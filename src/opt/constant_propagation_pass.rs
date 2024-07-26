use std::collections::{HashMap, HashSet};
use std::{rc::Weak, cell::RefCell};
use crate::opt::{cfg::*, pass::CompilerPass};
use crate::fe::ast::*;
 
pub struct ConstantPropagationPass;

impl CompilerPass for ConstantPropagationPass {
	fn run_on_function(&self, cfg: &mut ControlFlowGraph) {
		let mut vis: HashSet<BasicBlockRef> = HashSet::new();
		let mut context: HashMap<BasicBlockRef, HashMap<String, Value>> = HashMap::new();
		loop {
			let mut new_context: HashMap<BasicBlockRef, HashMap<String, Value>> = context.clone();
			vis.clear();
			update_evaluations(BasicBlockRef(cfg.basic_blocks[cfg.entry].clone()), &mut vis, &mut new_context);
			if context == new_context {
				break;
			}
			context = new_context;
		}
		vis.clear();
		update_cfg(BasicBlockRef(cfg.basic_blocks[cfg.entry].clone()), &mut vis, &context);
	}
}

fn update_evaluations(basic_block: BasicBlockRef, vis: &mut HashSet<BasicBlockRef>,
					  context: &mut HashMap<BasicBlockRef, HashMap<String, Value>>) {
	vis.insert(basic_block.clone());
	let basic_block_context = meet_operator(basic_block.borrow().prevs.clone(), &*context);
	context.insert(basic_block.clone(), basic_block_context);	
	for inst in basic_block.borrow().insts.iter() {
		inst.borrow_mut().update_evaluations(context.get_mut(&basic_block.clone()).unwrap());
	}
	for succ in basic_block.borrow().successors().iter() {
		if !vis.contains(&BasicBlockRef(succ.upgrade().unwrap())) {
			update_evaluations(BasicBlockRef(succ.upgrade().unwrap()), vis, context);
		}
	}
}

fn update_cfg(basic_block: BasicBlockRef, vis: &mut HashSet<BasicBlockRef>,
			  context: &HashMap<BasicBlockRef, HashMap<String, Value>>) {
	vis.insert(basic_block.clone());
	for inst in basic_block.borrow().insts.iter() {
		inst.borrow_mut().reduce(context.get(&basic_block.clone()).unwrap());
	}
	for succ in basic_block.borrow().successors().iter() {
		if !vis.contains(&BasicBlockRef(succ.upgrade().unwrap())) {
			update_cfg(BasicBlockRef(succ.upgrade().unwrap()), vis, context);
		}
	}
}

fn meet_operator(basic_blocks: Vec<Weak<RefCell<BasicBlock>>>,
				 context: &HashMap<BasicBlockRef, HashMap<String, Value>>
) -> HashMap<String, Value>{
	let mut res: HashMap<String, Value> = HashMap::new();
	for basic_block in basic_blocks.iter() {
		if let Some(ctx) = context.get(&BasicBlockRef(basic_block.upgrade().unwrap())) {
			for (iden, value) in ctx {
				let entry = res.entry(iden.clone()).or_insert(value.clone());
				*entry = value_join(entry.clone(), value.clone());
			}
		}
	}
	res
}
