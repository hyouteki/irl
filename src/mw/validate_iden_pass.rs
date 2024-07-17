use crate::{fe::{ast::*, loc::Loc}, mw::pass::AstPass};
use std::collections::HashSet;

pub struct ValidateIdenPass;

#[derive(Clone)]
struct Context {
	idens: HashSet<String>,
	labels: HashSet<String>,
}

impl Context {
	fn new() -> Self {
		Self{idens: HashSet::new(), labels: HashSet::new()}
	}
	fn validate_iden(&self, node: &IdenAstNode) {
		if !self.idens.contains(&node.name) {
			node.loc.error(format!("unknown identifier '{}'", node.name));
		}
	}
	fn insert_iden(&mut self, iden: String) {
		let _ = self.idens.insert(iden);
	}
	fn validate_label(&self, label: &String, loc: Loc) {
		if !self.labels.contains(label) {
			loc.error(format!("unknown label identifier '{}'", label));
		}
	}
	fn insert_label(&mut self, label: &String, loc: Loc) {
		if self.labels.contains(label) {
			loc.error(format!("label identifier already exists '{}'", label));
		}
		let _ = self.labels.insert(label.clone());
	}
}

impl AstPass for ValidateIdenPass {
	fn apply(&self, nodes: &mut Vec<AstNode>) {
		for node in nodes.iter() {
			let mut context: Context = Context::new();
			validate_node(node, &mut context);
			validate_label(node, &context);
		}
	}
}

fn validate_node(node: &AstNode, context: &mut Context) {
	match node {
		AstNode::Call(node) => {
			for param in node.params.iter() {
				if let AstNode::Iden(iden_node) = param {
					context.validate_iden(iden_node);
				}
			}
			context.insert_iden(node.id.clone());
		},
		AstNode::Arith(node) => {
			if let AstNode::Iden(ref iden_lhs) = *node.lhs {
				context.validate_iden(iden_lhs);
			}
			if let AstNode::Iden(ref iden_rhs) = *node.rhs {
				context.validate_iden(iden_rhs);
			}
		},
		AstNode::Relop(node) => {
			if let AstNode::Iden(ref iden_lhs) = *node.lhs {
				context.validate_iden(iden_lhs);
			}
			if let AstNode::Iden(ref iden_rhs) = *node.rhs {
				context.validate_iden(iden_rhs);
			}
		},
		AstNode::Unary(node) => {
			if let AstNode::Iden(ref iden_var) = *node.var {
				context.validate_iden(iden_var);
			}
		},
		AstNode::Function(node) => {
			for arg in node.args.iter() {
				if let AstNode::Iden(iden_node) = arg {
					context.insert_iden(iden_node.name.clone());
				}
			}
			for body_node in node.body.iter() {
				validate_node(body_node, context);
			}
		},
		AstNode::Assignment(node) => {
			validate_node(&*node.var, context);
			context.insert_iden(node.name.clone());
		},
		AstNode::Label(node) => {
			context.insert_label(&node.name, node.loc.clone());
			let mut inner_context: Context = context.clone(); 
			for body_node in node.body.iter() {
				validate_node(body_node, &mut inner_context);
			} 
		},
		AstNode::If(node) => validate_node(&*node.condition, context),
		AstNode::Ret(node) => {
			if let AstNode::Iden(ref iden_var) = *node.var {
				context.validate_iden(iden_var);
			}
		},
		_ => {},
	}
}

fn validate_label(node: &AstNode, context: &Context) {
	match node {
		AstNode::Function(node) => {
			for body_node in node.body.iter() {
				validate_label(body_node, context);
			}
		},
		AstNode::Label(node) => {
			for body_node in node.body.iter() {
				validate_label(body_node, context);
			} 
		},
		AstNode::If(node) => context.validate_label(&node.label, node.loc.clone()),
		AstNode::Goto(node) => context.validate_label(&node.name, node.loc.clone()),
		_ => {},
	}
}
