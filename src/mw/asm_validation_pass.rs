use std::collections::HashSet;
use crate::{fe::ast::*, mw::pass::AstPass};

pub struct AsmValidationPass;

impl AstPass for AsmValidationPass {
	fn apply(&self, nodes: &mut Vec<AstNode>) {
		if nodes.len() == 0 {return;}
		let mut function_names: HashSet<String> = HashSet::new();
		for node in nodes.iter() {
			if let AstNode::Function(function_node) = node {
				function_names.insert(function_node.name.clone());
			} else {
				node.loc().error(String::from("expected top level function instruction"));
			}
		}
		if !function_names.contains(&String::from("main")) {
			nodes[0].loc().program_error(String::from("program entry point aka function 'main' not found"));
		}
	}
	fn name(&self) -> String {String::from("asm_validation_pass")}
}
