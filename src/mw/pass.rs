use crate::fe::ast::AstNode;
use crate::cli::CliOptions;

pub trait AstPass {
	fn apply(&self, nodes: &mut Vec<AstNode>);
	fn name(&self) -> String;
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
	pub fn run(&self, nodes: &mut Vec<AstNode>, options: &CliOptions) {
		for pass in self.passes.iter() {
			options.verbose_message(format!("running '{}'", pass.name()));
			pass.apply(nodes);
		}
	}
}
