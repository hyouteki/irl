use std::{rc::{Rc, Weak}, cell::{Ref, RefCell}};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::fe::ast::*;

struct ConditionalJump {
	condition: AstNode,
	goto: Weak<RefCell<BasicBlock>>,
	otherwise: Weak<RefCell<BasicBlock>>,
}

enum Jump {
	Unconditional(Weak<RefCell<BasicBlock>>),
	Conditional(ConditionalJump),
}

pub struct BasicBlock {
	id: usize,
	label: Option<String>,
	pub insts: Vec<Rc<RefCell<AstNode>>>,
	pub prevs: Vec<Weak<RefCell<BasicBlock>>>,
	next: Option<Jump>,
}

impl PartialEq for BasicBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BasicBlock {}

impl Hash for BasicBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Clone)]
pub struct BasicBlockRef(pub Rc<RefCell<BasicBlock>>);

impl PartialEq for BasicBlockRef {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().eq(&other.0.borrow())
    }
}

impl Eq for BasicBlockRef {}

impl Hash for BasicBlockRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl BasicBlockRef {
	pub fn borrow(&self) -> Ref<BasicBlock> {
        self.0.borrow()
    }
}

impl BasicBlock {
	fn new(id: usize) -> Self {
		Self{id: id, label: None, insts: Vec::new(), prevs: Vec::new(), next: None}
	}
	fn add_inst(&mut self, node: AstNode) {
		self.insts.push(Rc::new(RefCell::new(node)));
	}
	fn add_prev(&mut self, prev: Weak<RefCell<BasicBlock>>) {
		self.prevs.push(prev);
	}
	fn set_unconditional_jump(&mut self, jump: Weak<RefCell<BasicBlock>>) {
		self.next = Some(Jump::Unconditional(jump));
	}
	fn set_conditional_jump(&mut self, jump: ConditionalJump) {
		self.next = Some(Jump::Conditional(jump));
	}
	pub fn successors(&self) -> Vec<Weak<RefCell<BasicBlock>>> {
		match &self.next {
			Some(jump) => match jump {
				Jump::Conditional(conditional_jump) => vec![conditional_jump.goto.clone(),
															conditional_jump.otherwise.clone()],
				Jump::Unconditional(unconditional_jump) => vec![unconditional_jump.clone()]
			},
			None => vec![]
		}
	}
	pub fn name(&self) -> String {
		format!("[{}{}]", self.id, match &self.label {
			Some(name) => format!("||{}", name),
			None => String::from(""),
		})
	}
	fn properties(&self) -> String {
		format!("{} [#Predecessor={}] [#Successor={}]\n", self.name(),
				self.prevs.len(), self.successors().len())
	}
}

impl std::fmt::Display for BasicBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for inst in self.insts.iter() {
			let _ = write!(f, "{}", inst.borrow());
		}
		write!(f, "")
	}
}

pub struct ControlFlowGraph {
	pub function: FunctionAstNode,
	pub entry: usize,
	pub basic_blocks: Vec<Rc<RefCell<BasicBlock>>>,
	label_table: HashMap<String, usize>,
}

impl ControlFlowGraph {
	fn new(function: FunctionAstNode, entry: Rc<RefCell<BasicBlock>>) -> Self {
		Self{function: function, entry: 0, basic_blocks: vec![entry], label_table: HashMap::new()}
	}
	fn add_basic_block(&mut self, basic_block: Rc<RefCell<BasicBlock>>) {
		self.basic_blocks.push(basic_block);
	}
	fn add_label_with_basic_block(&mut self, label: String, basic_block: Rc<RefCell<BasicBlock>>) {
		self.basic_blocks.push(basic_block);
		self.label_table.insert(label, self.basic_blocks.len()-1);
	}
	fn get_basic_block(&self, label: String) -> Option<Rc<RefCell<BasicBlock>>> {
		self.label_table.get(&label).and_then(|&ix| self.basic_blocks.get(ix).cloned())
	}
	fn get_new_id(&self) -> usize {
		self.basic_blocks.len()
	}
	pub fn reindex_basic_blocks(&mut self) {
		for (ix, basic_block) in self.basic_blocks.iter().enumerate() {
			basic_block.borrow_mut().id = ix;
		}
	}
	pub fn get_uses(&self, inst: Rc<RefCell<AstNode>>) -> Vec<Rc<RefCell<AstNode>>> {
		let production: Option<String> = inst.borrow().production();
		if let None = production {
			return vec![];
		}
		let mut res: Vec<Rc<RefCell<AstNode>>> = Vec::new();
		let mut vis: HashSet<BasicBlockRef> = HashSet::new();
		get_uses_(BasicBlockRef(self.basic_blocks[self.entry].clone()), production.unwrap(), &mut vis, &mut res);
		res
	}
}

fn get_uses_(basic_block: BasicBlockRef, production: String,
			 vis: &mut HashSet<BasicBlockRef>, res: &mut Vec<Rc<RefCell<AstNode>>>) {
	vis.insert(basic_block.clone());
	for inst in basic_block.borrow().insts.iter() {
		if inst.borrow().dependencies().contains(&production) {
			res.push(inst.clone());
		}
	}
	for succ in basic_block.borrow().successors() {
		if !vis.contains(&BasicBlockRef(succ.upgrade().unwrap())) {
			get_uses_(BasicBlockRef(succ.upgrade().unwrap()), production.clone(), vis, res);
		}
	}
}

impl std::fmt::Display for ControlFlowGraph {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let _ = write!(f, "=====================================\n");
		for (ix, basic_block) in self.basic_blocks.iter().enumerate() {
			if ix == self.entry {
				let _ = write!(f, "[Entry] ");
			}
			let _ = write!(f, "{}", basic_block.borrow().properties());
			let _ = write!(f, "{}", basic_block.borrow());
			if basic_block.borrow().prevs.len() > 0 {
				let _ = write!(f, "[Predecessors=");
				for pred in basic_block.borrow().prevs.iter() {
					let _ = write!(f, "{},", pred.upgrade().unwrap().borrow().name());
				}
				let _ = write!(f, "] ");
			}
			if basic_block.borrow().successors().len() > 0 {
				let _ = write!(f, "[Successors=");
				for succ in basic_block.borrow().successors().iter() {
					let _ = write!(f, "{},", succ.upgrade().unwrap().borrow().name());
				}
				let _ = write!(f, "]\n");
			} else {
				let _ = write!(f, "\n");
			}
			let _ = write!(f, "=====================================\n");
		}
		write!(f, "")
	}
}

enum Context {
	InsideLabel,
	InsideFunction,
}

fn process_body(body: &Vec<AstNode>, mut cur_bb: Rc<RefCell<BasicBlock>>,
				cfg: &mut ControlFlowGraph, context: Context) {
	let mut skip_inst: bool = false; 
	for node in body.iter() {
		match node {
			AstNode::Label(label) => {
				let new_bb: Rc<RefCell<BasicBlock>> = if let Some(bb) = cfg.get_basic_block(label.name.clone()) {
					bb
				} else {
					let new_bb_t: Rc<RefCell<BasicBlock>> = Rc::new(
						RefCell::new(BasicBlock::new(cfg.get_new_id())));
					new_bb_t.borrow_mut().label = Some(label.name.clone());
					cfg.add_label_with_basic_block(label.name.clone(), Rc::clone(&new_bb_t));
					// cur_bb.borrow_mut().set_unconditional_jump(Rc::downgrade(&new_bb_t));
					// new_bb_t.borrow_mut().add_prev(Rc::downgrade(&cur_bb));
					new_bb_t
				};
				process_body(label.body.clone().as_ref(), new_bb, cfg, Context::InsideLabel);
				skip_inst = false;
			},
			AstNode::Goto(goto) => {
				if skip_inst {
					continue;
				}
				let new_bb: Rc<RefCell<BasicBlock>> = if let Some(bb) = cfg.get_basic_block(goto.name.clone()) {
					bb
				} else {
					let new_bb_t: Rc<RefCell<BasicBlock>> = Rc::new(
						RefCell::new(BasicBlock::new(cfg.get_new_id())));
					new_bb_t.borrow_mut().label = Some(goto.name.clone());
					cfg.add_label_with_basic_block(goto.name.clone(), Rc::clone(&new_bb_t));
					new_bb_t
				};
				new_bb.borrow_mut().add_prev(Rc::downgrade(&cur_bb));
				cur_bb.borrow_mut().set_unconditional_jump(Rc::downgrade(&new_bb));
				if let Context::InsideLabel = context {
					return;
				} else {
					skip_inst = true;
				}
			},
			AstNode::Ret(_) => {
				if skip_inst {
					continue;
				}
				cur_bb.borrow_mut().add_inst(node.clone());
				if let Context::InsideLabel = context {
					return;
				} else {
					skip_inst = true;
				}
			},
			AstNode::If(if_inst) => {
				if skip_inst {
					continue;
				}
				let then_bb: Rc<RefCell<BasicBlock>> =
					if let Some(bb) = cfg.get_basic_block(if_inst.label.clone()) {bb} else {
						let new_bb_t: Rc<RefCell<BasicBlock>> = Rc::new(
							RefCell::new(BasicBlock::new(cfg.get_new_id())));
						new_bb_t.borrow_mut().label = Some(if_inst.label.clone());
						cfg.add_label_with_basic_block(if_inst.label.clone(), Rc::clone(&new_bb_t));
						new_bb_t
					};
				let else_bb: Rc<RefCell<BasicBlock>> = Rc::new(RefCell::new(BasicBlock::new(cfg.get_new_id())));
				cfg.add_basic_block(Rc::clone(&else_bb));
				cur_bb.borrow_mut().set_conditional_jump(ConditionalJump{
					condition: *if_inst.condition.clone(),
					goto: Rc::downgrade(&then_bb),
					otherwise: Rc::downgrade(&else_bb),
				});
				then_bb.borrow_mut().add_prev(Rc::downgrade(&cur_bb));
				else_bb.borrow_mut().add_prev(Rc::downgrade(&cur_bb));
				cur_bb = else_bb;
			},
			_ => if !skip_inst {
				cur_bb.borrow_mut().add_inst(node.clone());
			},
		}
	}
}

fn cfg_from_function(node: &FunctionAstNode) -> ControlFlowGraph {
	let entry: Rc<RefCell<BasicBlock>> = Rc::new(RefCell::new(BasicBlock::new(0)));
	let mut cfg: ControlFlowGraph = ControlFlowGraph::new(node.clone(), Rc::clone(&entry));
	let cur_bb: Rc<RefCell<BasicBlock>> = Rc::new(RefCell::new(BasicBlock::new(cfg.get_new_id())));
	cfg.add_basic_block(Rc::clone(&cur_bb));
	entry.borrow_mut().set_unconditional_jump(Rc::downgrade(&cur_bb));
	cur_bb.borrow_mut().add_prev(Rc::downgrade(&entry));
	process_body(node.body.clone().as_ref(), cur_bb, &mut cfg, Context::InsideFunction);
	cfg
}

pub fn cfg_table_from_program(nodes: &Vec<AstNode>) -> Vec<ControlFlowGraph> {
	let mut cfg_table: Vec<ControlFlowGraph> = Vec::new();
	for node in nodes.iter() {
		if let AstNode::Function(function_node) = node {
			cfg_table.push(cfg_from_function(function_node));
		}
	}
	cfg_table
}
