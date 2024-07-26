use crate::opt::{cfg::ControlFlowGraph, pass::*};
use crate::opt::reduce_pass::ReducePass;
use crate::opt::constant_propagation_pass::ConstantPropagationPass;

pub fn run_default_compiler_pass_manager(cfg_table: &mut Vec<ControlFlowGraph>) {
	let mut pass_manager: CompilerPassManager = CompilerPassManager::new();
	pass_manager.add(ReducePass{});
	pass_manager.add(ConstantPropagationPass{});
	for cfg in cfg_table.iter_mut() {
		pass_manager.run(cfg);
	}
}
