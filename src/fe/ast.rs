use std::collections::HashMap;
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

#[derive(Clone)]
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

impl ArithAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
	}
}

#[derive(Clone)]
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

impl RelopAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
	}
}

#[derive(Clone)]
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

impl UnaryAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}{}", self.op, self.var)
	}
}

#[derive(Clone)]
pub struct FunctionAstNode {
	pub name: String,
	pub args: Vec<AstNode>,
	pub body: Vec<AstNode>,
	pub loc: Loc,
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

impl RetAstNode {
	fn print(&self, f: &mut std::fmt::Formatter, indent_sz: usize) -> std::fmt::Result {
		print_indent(f, indent_sz);
		write!(f, "ret {}\n", self.var)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
	Constant(i32),
	Undefined,
	Nac
}

pub fn value_join(v1: Value, v2: Value) -> Value {
	match (v1, v2) {
		(Value::Constant(c1), Value::Constant(c2)) => if c1 == c2 {
			Value::Constant(c1)} else {Value::Nac},
		(Value::Constant(c1), Value::Undefined) => Value::Constant(c1),
		(Value::Undefined, Value::Constant(c2)) => Value::Constant(c2),
		(_, Value::Nac) | (Value::Nac, _) => Value::Nac,
		_ => Value::Undefined
	}
}

#[derive(Clone)]
pub enum AstNode {
	Iden(IdenAstNode),
	Num(NumAstNode),
	Call(CallAstNode),
	Arith(ArithAstNode),
	Relop(RelopAstNode),
	Unary(UnaryAstNode),
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
			AstNode::Iden(_) => vec![],
			AstNode::Num(_) => vec![],
			AstNode::Call(node) => {
				let mut res: Vec<String> = Vec::new();
				for param in node.params.iter() {
					if let AstNode::Iden(iden) = param {
						res.push(iden.name.clone());
					}
				}
				res
			},
			AstNode::Arith(node) => {
				let mut res: Vec<String> = Vec::new();
				if let AstNode::Iden(ref iden) = *node.lhs {
					res.push(iden.name.clone());
				}
				if let AstNode::Iden(ref iden) = *node.rhs {
					res.push(iden.name.clone());
				}
				res
			},
			AstNode::Relop(node) => {
				let mut res: Vec<String> = Vec::new();
				if let AstNode::Iden(ref iden) = *node.lhs {
					res.push(iden.name.clone());
				}
				if let AstNode::Iden(ref iden) = *node.rhs {
					res.push(iden.name.clone());
				}
				res
			},
			AstNode::Unary(node) => {
				let mut res: Vec<String> = Vec::new();
				if let AstNode::Iden(ref iden) = *node.var {
					res.push(iden.name.clone());
				}
				res
			},
			AstNode::Function(_) => vec![],
			AstNode::Assignment(node) => {
				let mut res: Vec<String> = Vec::new();
				if let AstNode::Iden(ref iden) = *node.var {
					res.push(iden.name.clone());
				}
				res
			},
			AstNode::Goto(_) => vec![],
			AstNode::Label(_) => vec![],
			AstNode::If(node) => {
				let mut res: Vec<String> = Vec::new();
				if let AstNode::Iden(ref iden) = *node.condition {
					res.push(iden.name.clone());
				}
				res
			},
			AstNode::Ret(node) => {
				let mut res: Vec<String> = Vec::new();
				if let AstNode::Iden(ref iden) = *node.var {
					res.push(iden.name.clone());
				}
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
				let mut res = node.clone();
				*res.lhs = res.lhs.reduced_version(state);
				*res.rhs = res.rhs.reduced_version(state);
				AstNode::Arith(*Box::new(res))
			},
			AstNode::Relop(node) => {
				let mut res = node.clone();
				*res.lhs = res.lhs.reduced_version(state);
				*res.rhs = res.rhs.reduced_version(state);
				AstNode::Relop(*Box::new(res))
			},
			AstNode::Unary(node) => {
				let mut res = node.clone();
				*res.var = res.var.reduced_version(state);
				AstNode::Unary(*Box::new(res))
			},
			AstNode::Function(_) => self.clone(),
			AstNode::Assignment(node) => {
				let mut res = node.clone();
				*res.var = res.var.reduced_version(state);
				AstNode::Assignment(*Box::new(res))
			},
			AstNode::Goto(_) => self.clone(),
			AstNode::Label(_) => self.clone(),
			AstNode::If(_) => self.clone(),
			AstNode::Ret(node) => {
				let mut res = node.clone();
				*res.var = res.var.reduced_version(state);
				AstNode::Ret(*Box::new(res))
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
}

impl std::fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.print(f, 0)
	}
}
