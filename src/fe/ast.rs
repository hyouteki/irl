use std::collections::{HashMap, HashSet};
use crate::fe::{loc::Loc, token::{Token, TokenKind}};

fn print_indent(f: &mut std::fmt::Formatter, indent_sz: usize) {	
	for _ in 0..indent_sz {
		let _ = write!(f, "    ");
	}
}

#[derive(Clone)]
pub struct IdenAstNode {
	pub name: String,
	pub loc: Loc,
}

impl PartialEq for IdenAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl IdenAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

#[derive(Clone)]
pub struct NumAstNode {
	pub num: i32,
	pub loc: Loc,
}

impl PartialEq for NumAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl NumAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.num)
	}
}

#[derive(Clone)]
pub struct CallAstNode {
	pub id: String,
	pub name: String,
	pub params: Vec<AstNode>,
	pub loc: Loc,
}

impl PartialEq for CallAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.params == other.params
    }
}

impl CallAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		for param in self.params.iter() {
			print_indent(f, indent_sz);
			let _ = write!(f, "param {}\n", param);
		}
		print_indent(f, indent_sz);
		write!(f, "{} = call {}, {}\n", self.id, self.name, self.params.len())
	}
}

#[derive(Clone, PartialEq)]
pub enum ArithOp {
	Sum, Sub, Mul, Div,
}

impl ArithOp {
	pub fn new(token: &Token) -> Self {
		match token.kind {
			TokenKind::Plus => ArithOp::Sum,
			TokenKind::Minus => ArithOp::Sub,
			TokenKind::Mul => ArithOp::Mul,
			TokenKind::Div => ArithOp::Div,
			_ => {
				token.error_token_kind_mismatch(
					vec![TokenKind::Plus, TokenKind::Minus,
						 TokenKind::Mul, TokenKind::Div]);
				unreachable!()
			},
		}	
	}
	pub fn to_string(&self) -> String {
		String::from(match self {
			ArithOp::Sum => "add",
			ArithOp::Sub => "sub",
			ArithOp::Mul => "mul",
			ArithOp::Div => "div",
		})
	}
}

impl std::fmt::Display for ArithOp {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			ArithOp::Sum => "+",
			ArithOp::Sub => "-",
			ArithOp::Mul => "*",
			ArithOp::Div => "/",
		})
	}
}

#[derive(Clone)]
pub struct ArithAstNode {
	pub op: ArithOp,
	pub lhs: Box<AstNode>,
	pub rhs: Box<AstNode>,
	pub loc: Loc,
}

impl PartialEq for ArithAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.op == other.op && self.rhs == other.rhs
    }
}

impl ArithAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
	}
}

#[derive(Clone, PartialEq)]
pub enum RelOp {
	Eq, Neq, Gt, Lt, Ge, Le,
}

impl RelOp {
	pub fn new(token: &Token) -> Self {
		match token.kind {
			TokenKind::Eq => RelOp::Eq,
			TokenKind::Neq => RelOp::Neq,
			TokenKind::Gt => RelOp::Gt,
			TokenKind::Lt => RelOp::Lt,
			TokenKind::Ge => RelOp::Ge,
			TokenKind::Le => RelOp::Le,
			_ => {
				token.error_token_kind_mismatch(
					vec![TokenKind::Eq, TokenKind::Neq, TokenKind::Gt,
						 TokenKind::Lt, TokenKind::Ge, TokenKind::Le]);
				unreachable!()
			},
		}
	}
}

impl std::fmt::Display for RelOp {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			RelOp::Eq => "==",
			RelOp::Neq => "!=",
			RelOp::Gt => ">",
			RelOp::Lt => "<",
			RelOp::Ge => ">=",
			RelOp::Le => "<=",
		})
	}
}

#[derive(Clone)]
pub struct RelopAstNode {
	pub op: RelOp,
	pub lhs: Box<AstNode>,
	pub rhs: Box<AstNode>,
	pub loc: Loc,
}

impl PartialEq for RelopAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.op == other.op && self.rhs == other.rhs
    }
}

impl RelopAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
	}
}

#[derive(Clone, PartialEq)]
pub enum UnaryOp {
	Neg,
}

impl UnaryOp {
	pub fn new(token: &Token) -> Self {
		match token.kind {
			TokenKind::Minus => UnaryOp::Neg,
			_ => {
				token.error_token_kind_mismatch(vec![TokenKind::Minus]);
				unreachable!()
			},
		}
	}
}

impl std::fmt::Display for UnaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", match self {
			UnaryOp::Neg => "-"
		})
	}
}

#[derive(Clone)]
pub struct UnaryAstNode {
	pub op: UnaryOp,
	pub var: Box<AstNode>,
	pub loc: Loc,
}

impl PartialEq for UnaryAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.var == other.var && self.op == other.op
    }
}

impl UnaryAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}{}", self.op, self.var)
	}
}

#[derive(Clone)]
pub struct AllocAstNode {
	pub size: Box<AstNode>,
	pub loc: Loc,
}

impl PartialEq for AllocAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl AllocAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "alloc {}", self.size)
	}
}

#[derive(Clone)]
pub struct LoadAstNode {
	pub ptr: String,
	pub loc: Loc,
}

impl PartialEq for LoadAstNode {
    fn eq(&self, other: &Self) -> bool {
		self.ptr == other.ptr
    }
}

impl LoadAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "load {}", self.ptr)
	}
}

#[derive(Clone)]
pub struct StoreAstNode {
	pub ptr: String,
	pub op: Box<AstNode>,
	pub loc: Loc,
}

impl PartialEq for StoreAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr && self.op == other.op
    }
}

impl StoreAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		write!(f, "store {}, {}\n", self.ptr, self.op)
	}
}

#[derive(Clone)]
pub struct FunctionAstNode {
	pub name: String,
	pub args: Vec<AstNode>,
	pub body: Vec<AstNode>,
	pub loc: Loc,
}

impl PartialEq for FunctionAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.args == other.args && self.body == other.body
    }
}

impl FunctionAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		let _ = write!(f, "function {}, {}\n", self.name, self.args.len());
		for arg in self.args.iter() {
			print_indent(f, indent_sz+1);
			let _ = write!(f, "arg {}\n", arg);
		}
		for node in self.body.iter() {
			let _ = node.print(f, indent_sz+1);
		}
		write!(f, "")
	}
}

#[derive(Clone)]
pub struct AssignmentAstNode {
	pub name: String,
	pub var: Box<AstNode>,
	pub loc: Loc,
}

impl PartialEq for AssignmentAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.var == other.var
    }
}

impl AssignmentAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		write!(f, "{} = {}\n", self.name, self.var)
	}
}

#[derive(Clone)]
pub struct GotoAstNode {
	pub name: String,
	pub loc: Loc,
}

impl PartialEq for GotoAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl GotoAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		write!(f, "goto {}\n", self.name)
	}
}

#[derive(Clone)]
pub struct LabelAstNode {
	pub name: String,
	pub body: Vec<AstNode>,
	pub loc: Loc,
}

impl PartialEq for LabelAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.body == other.body
    }
}

impl LabelAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		let _ = write!(f, "label {}\n", self.name);
		for node in self.body.iter() {
			let _ = node.print(f, indent_sz+1);
		}
		write!(f, "")
	}
}

#[derive(Clone)]
pub struct IfAstNode {
	pub condition: Box<AstNode>,
	pub label: String,
	pub loc: Loc,
}

impl PartialEq for IfAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.condition == other.condition && self.label == other.label
    }
}

impl IfAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		write!(f, "if ({}) goto {}\n", self.condition, self.label)
	}
}

#[derive(Clone)]
pub struct RetAstNode {
	pub var: Box<AstNode>,
	pub loc: Loc,
}

impl PartialEq for RetAstNode {
    fn eq(&self, other: &Self) -> bool {
        self.var == other.var
    }
}

impl RetAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		write!(f, "ret {}\n", self.var)
	}
}


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
	Constant(i32),
	Pointer(Box<Value>),
	Undefined,
	Nac
}

pub fn value_join(v1: Value, v2: Value) -> Value {
	match (v1, v2) {
		(Value::Constant(c1), Value::Constant(c2)) => if c1 == c2 {
			Value::Constant(c1)} else {Value::Nac},
		(Value::Constant(c1), Value::Undefined) => Value::Constant(c1),
		(Value::Undefined, Value::Constant(c2)) => Value::Constant(c2),
		(Value::Pointer(c1), Value::Pointer(c2)) => Value::Pointer(Box::new(value_join(*c1, *c2))),
		(Value::Pointer(c1), Value::Undefined) => Value::Pointer(c1),
		(Value::Undefined, Value::Pointer(c2)) => Value::Pointer(c2),
		(_, Value::Nac) | (Value::Nac, _) => Value::Nac,
		_ => Value::Undefined
	}
}

#[derive(Clone, PartialEq)]
pub enum AstNode {
	Iden(IdenAstNode),
	Num(NumAstNode),
	Call(CallAstNode),
	Arith(ArithAstNode),
	Relop(RelopAstNode),
	Unary(UnaryAstNode),
	Alloc(AllocAstNode),
	Load(LoadAstNode),
	Store(StoreAstNode),
	Function(FunctionAstNode),
	Assignment(AssignmentAstNode),
	Goto(GotoAstNode),
	Label(LabelAstNode),
	If(IfAstNode),
	Ret(RetAstNode),
}

impl AstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		match self {
			AstNode::Iden(node) => node.print(f),
			AstNode::Num(node) => node.print(f),
			AstNode::Call(node) => node.print(f, indent_sz),
			AstNode::Arith(node) => node.print(f),
			AstNode::Relop(node) => node.print(f),
			AstNode::Unary(node) => node.print(f),
			AstNode::Function(node) => node.print(f, indent_sz),
			AstNode::Assignment(node) => node.print(f, indent_sz),
			AstNode::Goto(node) => node.print(f, indent_sz),
			AstNode::Label(node) => node.print(f, indent_sz),
			AstNode::If(node) => node.print(f, indent_sz),
			AstNode::Ret(node) => node.print(f, indent_sz),
			AstNode::Alloc(node) => node.print(f),
			AstNode::Load(node) => node.print(f),
			AstNode::Store(node) => node.print(f, indent_sz),
		}
	}
	pub fn is_terminator(&self) -> bool {
		match self {
			AstNode::Goto(_) => true,
			AstNode::Label(_) => true,
			AstNode::If(_) => true,
			_ => false,
		}
	}
	pub fn dependencies(&self) -> Vec<String> {
		match self {
			AstNode::Iden(iden_node) => vec![iden_node.name.clone()],
			AstNode::Num(_) => Vec::new(),
			AstNode::Call(node) => {
				let mut res: Vec<String> = Vec::new();
				for param in node.params.iter() {
					res.append(&mut param.dependencies());
				}
				res
			},
			AstNode::Arith(node) => {
				let mut res: Vec<String> = Vec::new();
				res.append(&mut node.lhs.dependencies());
				res.append(&mut node.rhs.dependencies());
				res
			},
			AstNode::Relop(node) => {
				let mut res: Vec<String> = Vec::new();
				res.append(&mut node.lhs.dependencies());
				res.append(&mut node.rhs.dependencies());
				res
			},
			AstNode::Unary(node) => node.var.dependencies(),
			AstNode::Function(node) => {
				let mut res: Vec<String> = Vec::new();
				for arg in node.args.iter() {
					res.append(&mut arg.dependencies());
				}
				res
			},
			AstNode::Assignment(node) => node.var.dependencies(),
			AstNode::Goto(_) => Vec::new(),
			AstNode::Label(_) => Vec::new(),
			AstNode::If(node) => node.condition.dependencies(),
			AstNode::Ret(node) => node.var.dependencies(),
			AstNode::Alloc(node) => node.size.dependencies(),
			AstNode::Load(node) => vec![node.ptr.clone()],
			AstNode::Store(node) => {
				let mut res: Vec<String> = vec![node.ptr.clone()];
				res.append(&mut node.op.dependencies());
				res
			},
		}
	}
	pub fn production(&self) -> Option<String> {
		match self {
			AstNode::Iden(_) => None,
			AstNode::Num(_) => None,
			AstNode::Call(node) => Some(node.id.clone()),
			AstNode::Arith(_) => None,
			AstNode::Relop(_) => None,
			AstNode::Unary(_) => None,
			AstNode::Function(_) => None,
			AstNode::Assignment(node) => Some(node.name.clone()),
			AstNode::Goto(_) => None,
			AstNode::Label(_) => None,
			AstNode::If(_) => None,
			AstNode::Ret(_) => None,
			AstNode::Alloc(_) => None,
			AstNode::Load(_) => None,
			AstNode::Store(node) => Some(node.ptr.clone()),
		}
	}
	pub fn evaluate(&self) -> Value {
		match self {
			AstNode::Iden(_) => Value::Nac,
			AstNode::Num(c) => Value::Constant(c.num),
			AstNode::Call(_) => Value::Nac,
			AstNode::Arith(node) => {
				if let (Value::Constant(c1), Value::Constant(c2)) = (node.lhs.evaluate(), node.rhs.evaluate()) {
					match node.op {
						ArithOp::Sum => Value::Constant(c1+c2),
						ArithOp::Sub => Value::Constant(c1-c2),
						ArithOp::Mul => Value::Constant(c1*c2),
						ArithOp::Div => Value::Constant(c1/c2),
					}
				} else {Value::Nac}
			},
			AstNode::Relop(node) => {
				if let (Value::Constant(c1), Value::Constant(c2)) = (node.lhs.evaluate(), node.rhs.evaluate()) {
					match node.op {
						RelOp::Eq => Value::Constant((c1 == c2) as i32),
						RelOp::Neq => Value::Constant((c1 != c2) as i32),
						RelOp::Le => Value::Constant((c1 < c2) as i32),
						RelOp::Ge => Value::Constant((c1 > c2) as i32),
						RelOp::Lt => Value::Constant((c1 <= c2) as i32),
						RelOp::Gt => Value::Constant((c1 >= c2) as i32),
					}
				} else {Value::Nac}
			},
			AstNode::Unary(node) => {
				if let Value::Constant(c) = node.var.evaluate() {
					match node.op {
						UnaryOp::Neg => Value::Constant(-c),
					}
				} else {Value::Nac}
			},
			AstNode::Function(_) => Value::Nac,
			AstNode::Assignment(node) => node.var.evaluate(),
			AstNode::Goto(_) => Value::Nac,
			AstNode::Label(_) => Value::Nac,
			AstNode::If(_) => Value::Nac,
			AstNode::Ret(node) => node.var.evaluate(),
			AstNode::Alloc(_) => Value::Nac,
			AstNode::Load(_) => Value::Nac,
			AstNode::Store(_) => Value::Nac,
		}
	}
	pub fn reduced_version(&self, state: &HashMap<String, Value>) -> AstNode {
		match self {
			AstNode::Iden(node) => if let Some(Value::Constant(c)) = state.get(&node.name) {
				AstNode::Num(NumAstNode{num: *c, loc: node.loc.clone()})
			} else {self.clone()},
			AstNode::Num(_) => self.clone(),
			AstNode::Call(_) => self.clone(),
			AstNode::Arith(node) => {
				let mut new_node: ArithAstNode = node.clone();
				*new_node.lhs = new_node.lhs.reduced_version(state);
				*new_node.rhs = new_node.rhs.reduced_version(state);
				AstNode::Arith(new_node)
			},
			AstNode::Alloc(node) => {
				let mut new_node: AllocAstNode = node.clone();
				*new_node.size = new_node.size.reduced_version(state);
				AstNode::Alloc(new_node)
			},
			AstNode::Load(node) => {
				if let Some(Value::Pointer(boxed_value)) = state.get(&node.ptr) {
					if let Value::Constant(c) = **boxed_value {
						return AstNode::Num(NumAstNode{num: c, loc: node.loc.clone()});
					}
				}
				self.clone()
			},
			AstNode::Store(node) => {
				let mut new_node: StoreAstNode = node.clone();
				*new_node.op = new_node.op.reduced_version(state);
				AstNode::Store(new_node)
			},
			AstNode::Relop(node) => {
				let mut new_node: RelopAstNode = node.clone();
				*new_node.lhs = new_node.lhs.reduced_version(state);
				*new_node.rhs = new_node.rhs.reduced_version(state);
				AstNode::Relop(new_node)
			},
			AstNode::Unary(node) => {
				let mut new_node: UnaryAstNode = node.clone();
				*new_node.var = new_node.var.reduced_version(state);
				AstNode::Unary(new_node)
			},
			AstNode::Function(_) => self.clone(),
			AstNode::Assignment(node) => {
				let mut new_node: AssignmentAstNode = node.clone();
				*new_node.var = new_node.var.reduced_version(state);
				AstNode::Assignment(new_node)
			},
			AstNode::Goto(_) => self.clone(),
			AstNode::Label(_) => self.clone(),
			AstNode::If(_) => self.clone(),
			AstNode::Ret(node) => {
				let mut new_node: RetAstNode = node.clone();
				*new_node.var = new_node.var.reduced_version(state);
				AstNode::Ret(new_node)
			},
		}
	}
	pub fn reduce(&mut self, state: &HashMap<String, Value>) {
		*self = self.reduced_version(state);
	}
	pub fn update_evaluations(&mut self, state: &mut HashMap<String, Value>) {
		match self {
			AstNode::Iden(_) => {},
			AstNode::Num(_) => {},
			AstNode::Call(node) => {state.insert(node.id.clone(), Value::Nac);},
			AstNode::Arith(_) => {},
			AstNode::Alloc(_) => {},
			AstNode::Load(_) => {},
			AstNode::Store(node) => {
				state.insert(node.ptr.clone(), Value::Pointer(Box::new(
					node.op.reduced_version(&*state).evaluate())));
			},
			AstNode::Relop(_) => {},
			AstNode::Unary(_) => {},
			AstNode::Function(_) => {},
			AstNode::Assignment(node) => {
				state.insert(node.name.clone(), node.var.reduced_version(&*state).evaluate());
			},
			AstNode::Goto(_) => {},
			AstNode::Label(_) => {},
			AstNode::If(_) => {},
			AstNode::Ret(_) => {},
		};
	}
	pub fn value_operands(&self) -> HashSet<String> {
		match self {
			AstNode::Iden(iden_node) => HashSet::from([iden_node.name.clone()]),
			AstNode::Num(_) => HashSet::new(),
			AstNode::Call(call_node) => HashSet::from([call_node.id.clone()]),
			AstNode::Arith(arith_node) => {
				let mut res = arith_node.lhs.value_operands();
				res.extend(arith_node.rhs.value_operands());
				res
			},
			AstNode::Alloc(alloc_node) => alloc_node.size.value_operands(),
			AstNode::Load(load_node) => HashSet::from([load_node.ptr.clone()]),
			AstNode::Store(store_node) => {
				let mut res = HashSet::from([store_node.ptr.clone()]);
				res.extend(store_node.op.value_operands());
				res
			}
			AstNode::Relop(relop_node) => {
				let mut res = relop_node.lhs.value_operands();
				res.extend(relop_node.rhs.value_operands());
				res
			},
			AstNode::Unary(unary_node) => unary_node.var.value_operands(),
			AstNode::Function(function_node) => {
				let mut res = HashSet::new();
				for arg in function_node.args.iter() {res.extend(arg.value_operands())}
				for node in function_node.body.iter() {res.extend(node.value_operands())}
				res
			},
			AstNode::Assignment(assignment_node) => {
				let mut res = HashSet::from([assignment_node.name.clone()]);
				res.extend(assignment_node.var.value_operands());
				res
			},
			AstNode::Goto(_) => HashSet::new(),
			AstNode::Label(label_node) => {
				let mut res = HashSet::new();
				for node in label_node.body.iter() {res.extend(node.value_operands())}
				res
			},
			AstNode::If(if_node) => if_node.condition.value_operands(),
			AstNode::Ret(ret_node) => ret_node.var.value_operands(),
		}
	}
	pub fn loc(&self) -> Loc {
		match self {
			AstNode::Iden(node) => node.loc.clone(),
			AstNode::Num(node) => node.loc.clone(),
			AstNode::Call(node) => node.loc.clone(),
			AstNode::Arith(node) => node.loc.clone(),
			AstNode::Relop(node) => node.loc.clone(),
			AstNode::Unary(node) => node.loc.clone(),
			AstNode::Function(node) => node.loc.clone(),
			AstNode::Assignment(node) => node.loc.clone(),
			AstNode::Goto(node) => node.loc.clone(),
			AstNode::Label(node) => node.loc.clone(),
			AstNode::If(node) => node.loc.clone(),
			AstNode::Ret(node) => node.loc.clone(),
			AstNode::Alloc(node) => node.loc.clone(),
			AstNode::Load(node) => node.loc.clone(),
			AstNode::Store(node) => node.loc.clone(),
		}
	}
}

impl std::fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.print(f, 0)
	}
}
