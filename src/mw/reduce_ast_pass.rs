use std::collections::HashSet;
use crate::{fe::ast::*, mw::pass::AstPass};

pub struct ReduceAstPass;

/*
 * Currently this pass just removes redundent labels and gotos 
 */

impl AstPass for ReduceAstPass {
	fn apply(&self, nodes: &mut Vec<AstNode>) {
		for node in nodes.iter_mut() {
			if let AstNode::Function(ref mut function_node) = node {
				reduce_function_node(function_node);
			}
		}
	}
}

fn get_useful_labels(body: &Vec<AstNode>) -> HashSet<String> {
	let mut labels: HashSet<String> = HashSet::new();
	for node in body.iter() {
		match node {
			AstNode::Function(_) => unreachable!(),
			AstNode::Label(label_node) => labels.extend(get_useful_labels(&label_node.body)),
			AstNode::Goto(goto_node) => {labels.insert(goto_node.name.clone());},
			AstNode::If(if_node) => {labels.insert(if_node.label.clone());},
			_ => {}
		}
	}
	labels
}

fn reduce_function_node(function_node: &mut FunctionAstNode) {
	let useful_labels: HashSet<String> = get_useful_labels(&function_node.body);
	let mut new_body: Vec<AstNode> = Vec::new();
	for node in function_node.body.iter() {
		if let AstNode::Label(label_node) = node {
			if !useful_labels.contains(&label_node.name) {
				continue;
			}
		}
		new_body.push(node.clone());
	}
	function_node.body = new_body;
}
