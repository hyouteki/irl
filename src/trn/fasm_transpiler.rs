use std::collections::HashMap;
use crate::fe::ast::*;
use crate::trn::transpiler::*;

pub struct FasmTranspiler;

#[derive(Clone)]
enum Operand {
	Register(String),
	Memory(String),
	Immediate(i32),
}

impl Operand {
	fn value(&self) -> String {
		match self {
			Operand::Register(reg) => reg.clone(),
			Operand::Memory(mem) => mem.clone(),
			Operand::Immediate(val) => format!("{}", val),
		}
	}
}

struct Context {
	operands: HashMap<String, Operand>,
	value_operands_len: usize,
	entry_point: bool,
	function_name: String,
}

impl Context {
	fn new() -> Self {
		Self{operands: HashMap::new(), value_operands_len: 0,
			 entry_point: false, function_name: String::new()}
	}
}

// const MEM_REG: &str = "r14";

#[inline]
fn call_convention() -> Vec<String> {
    vec![String::from("rdi"), 
         String::from("rsi"), 
         String::from("rdx"), 
         String::from("rcx"), 
         String::from("r8"),  
         String::from("r9")]
}

fn inst(opcode: &str, to: Operand, from: Operand) -> String {
	format!("{} {}, {}", opcode, to.value(), from.value())
}

fn ast_node_to_operand(node: AstNode, context: &Context) -> Operand {
	match node {
		AstNode::Iden(iden_node) => match context.operands.get(&iden_node.name) {
			Some(operand) => operand.clone(),
			None => unreachable!()
		},
		AstNode::Num(num_node) => Operand::Immediate(num_node.num),
		_ => unreachable!()
	}
}

impl Transpiler for FasmTranspiler {
	fn transpile(&self, nodes: &Vec<AstNode>) -> Vec<String> {
		let mut lines: Vec<String> = Vec::new();
		lines.append(&mut header());
		lines.append(&mut top_level_transpilation(nodes, 0));
		lines
	}
}

fn header() -> Vec<String> {
	vec![String::from("format ELF64 executable 3"),
		 String::from("entry main"),
		 String::from("segment gnustack"),
		 String::from("segment executable")]
}

fn top_level_transpilation(nodes: &Vec<AstNode>, indent_sz: usize) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	for node in nodes.iter() {
		if let AstNode::Function(function_node) = node {
			let mut context: Context = Context::new();
			let value_operands = node.value_operands();
			context.value_operands_len = value_operands.len();
			context.entry_point = function_node.name == "main";
			context.function_name = function_node.name.clone();
			lines.push(indent(indent_sz, format!("{}:", function_node.name)));
			lines.push(indent(indent_sz+1, inst(
				"sub",
				Operand::Register(String::from("rsp")),
				Operand::Immediate(context.value_operands_len as i32 * 4 as i32))));
			for (ix, value_operand) in node.value_operands().iter().enumerate() {
				context.operands.insert(value_operand.to_string(), Operand::Memory(format!("[rsp+{}]", ix*4)));
			}
			lines.append(&mut transpile_nodes(&function_node.body, indent_sz+1, &context));
		} else {
			panic!("only function nodes are allowed in top level scope");
		}
	}	
	lines
}

fn transpile_nodes(nodes: &Vec<AstNode>, indent_sz: usize, context: &Context) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	for node in nodes.iter() {lines.append(&mut transpile_node(node, indent_sz, context))}
	lines
}

fn transpile_node(node: &AstNode, indent_sz: usize, context: &Context) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	match node {
		AstNode::Call(call_node) => {
			let call_convention: Vec<String> = call_convention();
			if call_node.params.len() > call_convention.len() {
				call_node.loc.error(format!("fasm target currently only supports '{}' parameters at max",
											call_convention.len()));
			}
			for (ix, param) in call_node.params.iter().enumerate() {
				lines.push(indent(indent_sz, inst(
					"mov",
					Operand::Register(call_convention[ix].clone()),
					ast_node_to_operand(param.clone(), context))));
			}
			lines.push(indent(indent_sz, format!("call {}", call_node.name)));
			lines.push(indent(indent_sz, inst(
				"mov",
				context.operands.get(&call_node.id).unwrap().clone(),
				Operand::Register(String::from("rax")))));
		}
		AstNode::Label(label_node) => {
			lines.push(indent(indent_sz, format!("{}_label_{}:", context.function_name, label_node.name)));
			lines.append(&mut transpile_nodes(&label_node.body, indent_sz+1, context));
		}
		AstNode::Goto(goto_node) => {
			lines.push(indent(indent_sz, format!("jmp {}_label_{}", context.function_name, goto_node.name)));
		}
		AstNode::Ret(ret_node) => {
			if context.entry_point {
				lines.push(indent(indent_sz, inst(
					"mov",
					Operand::Register(String::from("rax")),
					Operand::Immediate(60))));
				lines.push(indent(indent_sz, inst(
					"mov",
					Operand::Register(String::from("rdi")),
					ast_node_to_operand(*ret_node.var.clone(), context))));
				lines.push(indent(indent_sz, inst(
					"add",
					Operand::Register(String::from("rsp")),
					Operand::Immediate(context.value_operands_len as i32 * 4 as i32))));
				lines.push(indent(indent_sz, format!("syscall")));
			} else {
				lines.push(indent(indent_sz, inst(
					"mov",
					Operand::Register(String::from("rax")),
					ast_node_to_operand(*ret_node.var.clone(), context))));
				lines.push(indent(indent_sz, inst(
					"add",
					Operand::Register(String::from("rsp")),
					Operand::Immediate(context.value_operands_len as i32 * 4 as i32))));
				lines.push(indent(indent_sz, format!("ret")));
			}
		}
		_ => {}
	}
	lines
}
