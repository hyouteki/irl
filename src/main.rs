use crate::fe::{lexer::Lexer, parser::Parser, ast::AstNode};
use crate::mw::default_ast_pass_manager::*;
use crate::opt::{default_compiler_pass_manager::*, cfg::*};
use crate::cli::CliOptions;
use std::process::Command;

pub mod fe;
pub mod mw;
pub mod opt;
pub mod cli;

fn main() {
	let options: CliOptions = CliOptions::new();

	// module: fe
	let lexer = Lexer::new(options.filepath.clone());
	options.verbose_message(String::from("lexing complete"));
	let parser = Parser::new(lexer.tokens);
	options.verbose_message(String::from("parsing complete"));
	let mut ast: Vec<AstNode> = parser.nodes;
	if options.debug {
		println!("Initial AST");
		println!("===========");
		for node in ast.iter() {
			println!("{}", node);
		}
	}
	options.verbose_message(String::from("FE over"));

	// module: mw
	run_default_ast_pass_manager(&mut ast);
	options.verbose_message(String::from("MW over"));
	if options.debug {
		println!("MW Optimized AST");
		println!("===========");
		for node in ast.iter() {
			println!("{}", node);
		}
	}

	// module: opt
	let mut cfg_table: Vec<ControlFlowGraph> = cfg_table_from_program(&ast);
	run_default_compiler_pass_manager(&mut cfg_table);
	if options.cfg {
		let dot_filepath: String = format!("{}.dot", options.filepath.clone());
		dump_cfg_table_to_svg(&cfg_table, dot_filepath.clone());
		Command::new("dot").arg("-Tsvg").arg("-O").arg(dot_filepath.clone());
		options.verbose_message(format!("created control flow graph svg '{}'", dot_filepath));
	}
	options.verbose_message(format!("OPT over"));
	
	ast.clear();
	for cfg in cfg_table.iter() {
		ast.push(cfg.generate_ast());
	}
	if options.debug {
		println!("OPT Optimized AST");
		println!("===========");
		for node in ast.iter() {
			println!("{}", node);
		}
	}
	
	// module: mw
	run_default_ast_pass_manager(&mut ast);	
	if options.debug {
		println!("optimized AST");
		println!("=============");
		for node in ast.iter() {
			println!("{}", node);
			println!("");
		}
	}
}
