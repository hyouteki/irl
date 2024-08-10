use crate::fe::{lexer::Lexer, parser::Parser};
use crate::mw::default_ast_pass_manager::*;
use crate::opt::{default_compiler_pass_manager::*, cfg::*};
use crate::cli::cli;
use std::process::Command;

pub mod fe;
pub mod mw;
pub mod opt;
pub mod cli;

fn main() {
	let match_result = cli().get_matches();
    let compile_args = match_result.subcommand_matches("compile");
    let filepath: String = compile_args.unwrap().get_one::<String>("filepath").unwrap().to_string();
	let cfg: bool = *compile_args.unwrap().get_one::<bool>("cfg").unwrap();
	let debug: bool = *compile_args.unwrap().get_one::<bool>("debug").unwrap();
	let verbose: bool = *compile_args.unwrap().get_one::<bool>("verbose").unwrap();
	
	let lexer = Lexer::new(filepath.clone());
	let mut parser = Parser::new(lexer.tokens);
	run_default_ast_pass_manager(&mut parser.nodes);

	if debug {	
		for node in parser.nodes.iter() {
			println!("{}", node);
		}
	}

	let mut cfg_table: Vec<ControlFlowGraph> = cfg_table_from_program(&parser.nodes);
	if debug {
		for cfg in cfg_table.iter_mut() {
			println!("Function: {}", cfg.function.name);
			println!("{}", cfg);
		}
	}
	
	run_default_compiler_pass_manager(&mut cfg_table);
	if debug {
		for cfg in cfg_table.iter_mut() {
			println!("Function: {}", cfg.function.name);
			println!("{}", cfg);
		}
	}

	if cfg {
		let dot_filepath: String = format!("{}.dot", filepath.clone());
		dump_cfg_table_to_svg(&cfg_table, dot_filepath.clone());
		Command::new("dot").arg("-Tsvg").arg("-O").arg(dot_filepath.clone());
		if verbose {
			println!("info: created control flow graph svg '{}'", dot_filepath.clone());
		}
	}
}
