use std::collections::HashSet;
use crate::fe::ast::*;
use crate::trn::transpiler::Transpiler;

pub struct WatTranspiler;

impl Transpiler for WatTranspiler {
	fn transpile(&self, nodes: &Vec<AstNode>) -> Vec<String> {
		let mut lines: Vec<String> = Vec::new();
		lines.push(String::from("(module"));
		lines.append(&mut transpile_nodes_to_wat(nodes, 1, &mut HashSet::new()));
		lines.push(String::from(")"));
		lines
	}
}

fn transpile_nodes_to_wat(nodes: &Vec<AstNode>, indent_sz: usize,
						  vis_labels: &mut HashSet<String>) -> Vec<String> {

	let mut lines: Vec<String> = Vec::new();
	for node in nodes.iter() {
		lines.append(&mut transpile_node_to_wat(node, indent_sz, vis_labels));
	}
	lines
}

fn make_line(indent_sz: usize, text: String) -> String {let mut line: String = String::new();
	for _ in 0..indent_sz {line.push_str("    ");}
	line.push_str(&text);
	line
}

fn transpile_node_to_wat(node: &AstNode, indent_sz: usize, vis_labels: &mut HashSet<String>) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	match node {
		AstNode::Function(function_node) => {
			let mut line: String = make_line(indent_sz, format!("(func ${} (export \"{}\")",
																function_node.name, function_node.name));
			for arg_name in function_node.args.iter() {
				line += &format!(" (param ${} i32)", arg_name);
			}
			line += " (result i32)";
			lines.push(line);
			lines.append(&mut transpile_nodes_to_wat(&function_node.body, indent_sz+1, vis_labels));
			lines.push(make_line(indent_sz+1, String::from("i32.const 0")));
			lines.push(make_line(indent_sz, String::from("    )")));
		}
		AstNode::Label(label_node) => {
			lines.push(make_line(indent_sz, format!("(block ${}", label_node.name)));
			vis_labels.insert(label_node.name.clone());
			lines.append(&mut transpile_nodes_to_wat(&label_node.body, indent_sz+1, vis_labels));
			lines.push(make_line(indent_sz, String::from(")")));
		}
		AstNode::Goto(goto_node) => {
			if vis_labels.contains(&goto_node.name) {
				lines.push(make_line(indent_sz, format!("br ${}", goto_node.name)));
			}
		}
		_ => {}
	}
	lines
}
