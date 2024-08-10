use clap::{Arg, Command, ArgAction};

pub fn cli() -> Command {
    Command::new("irl")
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
		)
}
