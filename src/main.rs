use crate::fe::{lexer::Lexer, parser::Parser};
use crate::mw::default_ast_pass_manager::*;
use crate::opt::{default_compiler_pass_manager::*, cfg::*};
use crate::cli::cli;

pub mod fe;
pub mod mw;
pub mod opt;
pub mod cli;

fn main() {
	let match_result = cli().get_matches();
    let compile_args = match_result.subcommand_matches("compile");
    let filepath: String = compile_args.unwrap().get_one::<String>("filepath").unwrap().to_string();
	
	let lexer = Lexer::new(filepath);
	let mut parser = Parser::new(lexer.tokens);
	run_default_ast_pass_manager(&mut parser.nodes);

	for node in parser.nodes.iter() {
		println!("{}", node);
	}

	let mut cfg_table: Vec<ControlFlowGraph> = cfg_table_from_program(&parser.nodes);
	for cfg in cfg_table.iter_mut() {
		println!("Function: {}", cfg.function.name);
		println!("{}", cfg);
	}
	run_default_compiler_pass_manager(&mut cfg_table);
	for cfg in cfg_table.iter_mut() {
		println!("Function: {}", cfg.function.name);
		println!("{}", cfg);
	}

	// println!("Uses:");
	// let uses = cfg_table[0].get_uses(cfg_table[0].basic_blocks[1].borrow().insts[1].clone());
	// for x in uses.iter() {
	// 	print!("{}", x.borrow());
	// }
}
