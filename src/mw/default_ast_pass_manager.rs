use crate::{fe::ast::AstNode, mw::pass::*};
use crate::cli::CliOptions;
use crate::mw::validate_iden_pass::ValidateIdenPass;
use crate::mw::add_goto_pass::AddGotoPass;
use crate::mw::asm_validation_pass::AsmValidationPass;

const PASS_MAX_APPLICATION_LIMIT: usize = 1;

pub fn run_default_ast_pass_manager(nodes: &mut Vec<AstNode>, options: &CliOptions) {
	let mut ast_pass_manager: AstPassManager = AstPassManager::new();
	ast_pass_manager.add(AsmValidationPass{});
	ast_pass_manager.add(ValidateIdenPass{});
	ast_pass_manager.add(AddGotoPass{});

	for _ in 0..PASS_MAX_APPLICATION_LIMIT {
		let prev_nodes: Vec<AstNode> = nodes.clone();
		ast_pass_manager.run(nodes, options);
		if prev_nodes == *nodes {break;}
	}
}
