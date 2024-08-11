use crate::{fe::ast::AstNode, mw::pass::*};
use crate::mw::validate_iden_pass::ValidateIdenPass;
use crate::mw::add_goto_pass::AddGotoPass;
use crate::mw::reduce_ast_pass::ReduceAstPass;

const PASS_MAX_APPLICATION_LIMIT: usize = 100;

pub fn run_default_ast_pass_manager(nodes: &mut Vec<AstNode>) {
	let mut ast_pass_manager: AstPassManager = AstPassManager::new();
	ast_pass_manager.add(AddGotoPass{});
	// ast_pass_manager.add(ReduceAstPass{});
	// ast_pass_manager.add(ValidateIdenPass{});

	let mut new_nodes: Vec<AstNode> = Vec::new();
	let mut i: usize = 0;
	while i < PASS_MAX_APPLICATION_LIMIT && new_nodes != *nodes {
		new_nodes = nodes.clone();
		ast_pass_manager.run(nodes);
		i += 1;
	}

	*nodes = new_nodes;
}
