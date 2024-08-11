use crate::{fe::ast::AstNode, mw::pass::*};
use crate::mw::validate_iden_pass::ValidateIdenPass;
use crate::mw::add_goto_pass::AddGotoPass;

const PASS_MAX_APPLICATION_LIMIT: usize = 1;

pub fn run_default_ast_pass_manager(nodes: &mut Vec<AstNode>) {
	let mut ast_pass_manager: AstPassManager = AstPassManager::new();
	ast_pass_manager.add(ValidateIdenPass{});
	ast_pass_manager.add(AddGotoPass{});

	for _ in 0..PASS_MAX_APPLICATION_LIMIT {
		let prev_nodes: Vec<AstNode> = nodes.clone();
		ast_pass_manager.run(nodes);
		if prev_nodes == *nodes {break;}
	}
}
