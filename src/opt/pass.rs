use crate::opt::cfg::ControlFlowGraph;

pub trait CompilerPass {
	fn run_on_function(&self, cfg: &mut ControlFlowGraph);
}

pub struct CompilerPassManager {
	passes: Vec<Box<dyn CompilerPass>>,
}

impl CompilerPassManager {
	pub fn new() -> Self {
		Self{passes: Vec::new()}
	}
	pub fn add<T: CompilerPass + 'static>(&mut self, pass: T) {
		self.passes.push(Box::new(pass));
	}
	pub fn run(&self, cfg: &mut ControlFlowGraph) {
		for pass in self.passes.iter() {
			pass.run_on_function(cfg);
		}
	} 
}
