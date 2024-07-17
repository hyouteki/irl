use crate::fe::{loc::Loc, token::{Token, TokenKind}};

fn print_indent(f: &mut std::fmt::Formatter, indent_sz: usize) {	
	for _ in 0..indent_sz {
		let _ = write!(f, "    ");
	}
}

pub struct IdenAstNode {
	pub name: String,
	pub loc: Loc,
}

impl IdenAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

pub struct NumAstNode {
	pub num: i32,
	pub loc: Loc,
}

impl NumAstNode {
	fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.num)
	}
}

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
}

impl std::fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.print(f, 0)
	}
}
