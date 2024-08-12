use std::{fs::File, io::Write};
use crate::{fe::ast::*, cli::*};
use crate::trn::wat_transpiler::WatTranspiler;
use crate::trn::fasm_transpiler::FasmTranspiler;

pub trait Transpiler {
	fn transpile(&self, nodes: &Vec<AstNode>) -> Vec<String>;
}

fn remove_extension(filepath: String, ext: &str) -> String {
	filepath.as_str().trim_end_matches(ext).to_string()
}

pub fn replace_extension(filepath: String, old_ext: &str, new_ext: &str) -> String {
	format!("{}{}", remove_extension(filepath, old_ext), new_ext)
}

pub fn indent(indent_sz: usize, text: String) -> String {
	let mut line: String = String::new();
	for _ in 0..indent_sz {line.push_str("    ");}
	line.push_str(&text);
	line
}


pub fn transpilation_mode<T: Transpiler + 'static>(transpiler: T, nodes: &Vec<AstNode>,
										  output_filepath: String) {
	let mut file = File::create(output_filepath).expect("could not create file");
	for line in transpiler.transpile(nodes).iter() {
		file.write_all(line.as_bytes()).expect("could not write line");
		file.write_all(b"\n").expect("could not write new line");
	}
}

pub fn transpile(options: &CliOptions, nodes: &Vec<AstNode>, filepath: String) {
	if options.wat || options.wasm {
		let wat_filepath: String = replace_extension(filepath.clone(), "irl", "wat");
		transpilation_mode(WatTranspiler{}, nodes, wat_filepath.clone());
		if !options.wasm {return;}
		let wasm_filepath: String = replace_extension(filepath.clone(), "irl", "wasm");
		options.run_command(&["wat2wasm", wat_filepath.as_str(), "-o", wasm_filepath.as_str()]);
	}
	if options.fasm {
		let fasm_filepath: String = replace_extension(filepath.clone(), "irl", "fasm");
		transpilation_mode(FasmTranspiler{}, nodes, fasm_filepath.clone());
	}
}
