use std::collections::HashSet;
use crate::fe::ast::*;
use crate::trn::transpiler::*;

pub struct FasmTranspiler;

struct Context {
	function_names: HashSet<String>,
	label_names: HashSet<String>,
}

impl Context {
	fn new() -> Self {
		Self{function_names: HashSet::new(), label_names: HashSet::new()}
	}
}

impl Transpiler for FasmTranspiler {
	fn transpile(&self, nodes: &Vec<AstNode>) -> Vec<String> {
		let mut lines: Vec<String> = Vec::new();
		let mut context: Context = Context::new();
		generate_context(nodes, &mut context);
		lines.append(&mut header());
		lines.append(&mut transpile_nodes(nodes, 0, &context));
		lines
	}
}

fn generate_context(nodes: &Vec<AstNode>, context: &mut Context) {
	for node in nodes {
		generate_context_(&node, context);
	}
}

fn generate_context_(node: &AstNode, context: &mut Context) {
	match node {
		AstNode::Function(function_node) => {
			context.function_names.insert(function_node.name.clone());
			generate_context(&function_node.body, context);
		},
		AstNode::Label(label_node) => {
			context.label_names.insert(label_node.name.clone());
			generate_context(&label_node.body, context);
		},
		_ => {}
	}
}

fn header() -> Vec<String> {
	vec![String::from("format ELF64"),
		 String::from("section '.text' executable")]
}

fn transpile_nodes(nodes: &Vec<AstNode>, indent_sz: usize, context: &Context) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	for node in nodes.iter() {lines.append(&mut transpile_node(node, indent_sz, context));}
	lines
}

fn transpile_node(node: &AstNode, indent_sz: usize, context: &Context) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	match node {
		AstNode::Function(function_node) => {
			lines.push(indent(indent_sz, format!("function_{}:", function_node.name.clone())));
			lines.append(&mut transpile_nodes(&function_node.body, indent_sz+1, context));
		}
		AstNode::Label(label_node) => {
			lines.push(indent(indent_sz, format!("label_{}:", label_node.name.clone())));
			lines.append(&mut transpile_nodes(&label_node.body, indent_sz+1, context));
		}
		AstNode::Goto(goto_node) => {
			lines.push(indent(indent_sz, format!("goto label_{}", goto_node.name)));
		}
		_ => {}
	}
	lines
}
