use crate::{fe::ast::*, mw::pass::AstPass};

pub struct AddGotoPass;

impl AstPass for AddGotoPass {
	fn apply(&self, nodes: &mut Vec<AstNode>) {
		let _ = helper(nodes, 0);
	}
}

fn helper(nodes: &mut Vec<AstNode>, ix: usize) -> bool {
	if ix >= nodes.len() {
		return false;
	} 
	let add: bool = match &mut nodes[ix] {
		AstNode::Function(node) => helper(&mut node.body, 0),
		AstNode::Label(node) => match node.body[node.body.len()-1] {
			AstNode::Goto(_) => false,
			AstNode::Ret(_) => false,
			_ => true,
		},
		AstNode::Goto(_) => false,
		_ => if ix+1 < nodes.len() {matches!(nodes[ix+1], AstNode::Label(_))} else {false},
	};
	if add {
		if let Some(AstNode::Label(next_label_node)) = nodes.get(ix + 1) {
			let goto_node = AstNode::Goto(GotoAstNode {
				name: next_label_node.name.clone(),
				loc: next_label_node.loc.clone(),
			});
			if let AstNode::Label(current_label_node) = &mut nodes[ix] {
				current_label_node.body.push(goto_node);
			} else {
				nodes.insert(ix + 1, goto_node);
			}
		}
	}
	helper(nodes, ix+1)
}
