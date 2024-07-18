use std::collections::HashSet;
use crate::opt::pass::CompilerPass;
use crate::opt::cfg::{ControlFlowGraph, BasicBlockRef};

pub struct ReducePass;

impl CompilerPass for ReducePass {
	fn run_on_function(&self, cfg: &mut ControlFlowGraph) {
		let mut vis: HashSet<BasicBlockRef> = HashSet::new();
		dfs(BasicBlockRef(cfg.basic_blocks[cfg.entry].clone()), &mut vis);
		let mut unvisited_ixs: Vec<usize> = Vec::new();
		for (ix, bb) in cfg.basic_blocks.iter().enumerate() {
			// println!("{} {}", bb.borrow().name(), vis.contains(&BasicBlockRef(bb.clone())));
			if !vis.contains(&BasicBlockRef(bb.clone())) {
				unvisited_ixs.push(ix);
			}
		}
		for &ix in unvisited_ixs.iter().rev() {
			cfg.basic_blocks.remove(ix);
		}
		cfg.reindex_basic_blocks();
	}
}

fn dfs(bb: BasicBlockRef, vis: &mut HashSet<BasicBlockRef>) {
	vis.insert(bb.clone());
	for succ in bb.borrow().successors().iter() {
		if vis.contains(&BasicBlockRef(succ.upgrade().unwrap())) {
			continue;
		}
		dfs(BasicBlockRef(succ.upgrade().unwrap()), vis);
	}
}
