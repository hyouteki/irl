use clap::{Arg, Command, ArgAction};

pub struct CliOptions {
	pub filepath: String,
	pub cfg: bool,
	pub debug: bool,
	pub verbose: bool,
	pub wat: bool,
	pub wasm: bool,
	pub fasm: bool,
	pub run: bool,
}

impl CliOptions {
	pub fn new() -> Self {
		let match_result = cli().get_matches();
		let compile_args = match_result.subcommand_matches("compile");
		Self {
			filepath: compile_args.unwrap().get_one::<String>("filepath").unwrap().to_string(),
			cfg: *compile_args.unwrap().get_one::<bool>("cfg").unwrap(),
			debug: *compile_args.unwrap().get_one::<bool>("debug").unwrap(),
			verbose: *compile_args.unwrap().get_one::<bool>("verbose").unwrap(),
			wat: *compile_args.unwrap().get_one::<bool>("wat").unwrap(),
			wasm: *compile_args.unwrap().get_one::<bool>("wasm").unwrap(),
			fasm: *compile_args.unwrap().get_one::<bool>("fasm").unwrap(),
			run: *compile_args.unwrap().get_one::<bool>("run").unwrap(),
		}
	}
	pub fn verbose_message(&self, message: String) {
		if self.verbose {println!("{}: info: {}", self.filepath, message);}
	}
	pub fn verbose_error(&self, message: String) {
		if self.verbose {println!("{}: error: {}", self.filepath, message);}
	}	
	pub fn run_command(&self, args:  &[&str]) {
		if args.is_empty() {return;}

		let binpath = args.get(0).cloned().unwrap_or_default();
		let mut command = std::process::Command::new(binpath);
		for arg in &args[1..] {
			command.arg(arg);
		}
		
		let output = command.output().expect("failed to execute command");
		if !output.status.success() {
			let stderr = std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 in stderr");
			self.verbose_message(format!("error: {}", stderr));
		}
	}
}

pub fn cli() -> Command {
    Command::new("irl")
		.version("1.0")
        .author("Hyouteki")
        .about("Intermediate Representation Language: Minimal implementation of LLVM")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("compile")
                .about("Compile source code to target(s)")
                .arg_required_else_help(true)
                .arg(Arg::new("filepath")
                     .short('f')
                     .long("filepath")
                     .required(true)
                     .action(ArgAction::Append)
                     .help("Source file path"))
				.arg(Arg::new("cfg")
					 .long("cfg")
                     .alias("control-flow-graph")
                     .required(false)
                     .action(ArgAction::SetTrue)
                     .help("Output control flow graph of the program as a svg"))
				.arg(Arg::new("debug")
					 .short('d')
                     .long("debug")
                     .required(false)
                     .action(ArgAction::SetTrue)
                     .help("Dumps debug info onto stdout"))
				.arg(Arg::new("verbose")
					 .short('v')
                     .long("verbose")
                     .required(false)
                     .action(ArgAction::SetTrue)
                     .help("Sets info level to verbose"))
				.arg(Arg::new("run")
					 .short('r')
                     .long("run")
                     .required(false)
                     .action(ArgAction::SetTrue)
                     .help("Runs the binary after compilation"))
				.arg(Arg::new("wat")
                     .long("wat")
                     .required(false)
                     .action(ArgAction::SetTrue)
                     .help("Generates WAT (Web Assembly Text)"))
				.arg(Arg::new("wasm")
                     .long("wasm")
                     .required(false)
                     .action(ArgAction::SetTrue)
                     .help("Generates WASM (Web Assembly)"))
				.arg(Arg::new("fasm")
                     .long("fasm-linux-x86_64")
                     .required(false)
                     .action(ArgAction::SetTrue)
                     .help("Generates FASM (Flat Assembly)"))
		)
}
