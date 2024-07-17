use crate::fe::ast::AstNode;

pub trait AstPass {
	fn apply(&self, nodes: &mut Vec<AstNode>);
}

pub struct AstPassManager {
	passes: Vec<Box<dyn AstPass>>,
}

impl AstPassManager {
	pub fn new() -> Self {
		Self{passes: vec![]}
	}
	pub fn add<T: AstPass + 'static>(&mut self, pass: T) {
		self.passes.push(Box::new(pass));
	}
	pub fn run(&self, nodes: &mut Vec<AstNode>) {
		for pass in self.passes.iter() {
			pass.apply(nodes);
		}
	}
}
