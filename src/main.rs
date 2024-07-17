use crate::fe::{lexer::Lexer, parser::Parser};
use crate::mw::{pass::*, validate_iden_pass::ValidateIdenPass};

pub mod fe;
pub mod mw;

fn main() {
	let lexer = Lexer::new(String::from("./eg/fib.irl"));
	let mut parser = Parser::new(lexer.tokens);
	for node in parser.nodes.iter() {
		println!("{}", node);
	}
	let mut ast_pass_manager: AstPassManager = AstPassManager::new();
	ast_pass_manager.add(ValidateIdenPass{});
	ast_pass_manager.run(&mut parser.nodes);
}
