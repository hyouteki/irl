use crate::{fe::ast::AstNode, mw::pass::*};
use crate::mw::validate_iden_pass::ValidateIdenPass;
use crate::mw::add_goto_pass::AddGotoPass;

pub fn run_default_ast_pass_manager(nodes: &mut Vec<AstNode>) {
	let mut ast_pass_manager: AstPassManager = AstPassManager::new();
	ast_pass_manager.add(ValidateIdenPass{});
	ast_pass_manager.add(AddGotoPass{});
	ast_pass_manager.run(nodes);
}
