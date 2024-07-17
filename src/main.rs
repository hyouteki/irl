use crate::fe::{lexer::Lexer, parser::Parser};

pub mod fe;

fn main() {
	let lexer = Lexer::new(String::from("./eg/fib.irl"));
	let parser = Parser::new(lexer.tokens);
	for node in parser.nodes.iter() {
		println!("{}", nodec);
	}
}
