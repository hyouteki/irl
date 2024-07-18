use crate::{fe::ast::*, mw::pass::AstPass};

pub struct AddGotoPass;

impl AstPass for AddGotoPass {
	fn apply(&self, nodes: &mut Vec<AstNode>) {
		let _ = helper(nodes, 0);
	}
}

fn helper(nodes: &mut Vec<AstNode>, ix: usize) -> bool {
	if ix == nodes.len()-1 {
		return match nodes[ix] {
			AstNode::Goto(_) => false,
			AstNode::Ret(_) => false,
			_ => true,
		}
	}
	let add: bool = match &mut nodes[ix] {
		AstNode::Function(node) => helper(&mut node.body, 0),
		AstNode::Label(node) => helper(&mut node.body, 0),
		AstNode::Goto(_) => false,
		_ => matches!(nodes[ix+1], AstNode::Label(_)),
	};
	if add {
		if let AstNode::Label(label_node) = &nodes[ix+1] {
			nodes.insert(ix+1, AstNode::Goto(GotoAstNode{
				name: label_node.name.clone(), loc: label_node.loc.clone()}));
		}
	}
	return helper(nodes, ix+1);
}
