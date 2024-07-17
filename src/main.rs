use crate::fe::{lexer::Lexer, parser::Parser};
use crate::mw::default_ast_pass_manager::*;

pub mod fe;
pub mod mw;

fn main() {
	let lexer = Lexer::new(String::from("./eg/fib.irl"));
	let mut parser = Parser::new(lexer.tokens);
	run_default_ast_pass_manager(&mut parser.nodes);

	for node in parser.nodes.iter() {
		println!("{}", node);
	}
}
