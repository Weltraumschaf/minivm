use minivm::commands::*;
use minivm::error;
use clap::{Arg, App, SubCommand};

extern crate clap;
extern crate env_logger;
extern crate minivm;

fn main() {
    env_logger::init();

    // https://docs.rs/clap/2.29.2/clap/
    let matches = App::new("Mini Virtual Machine")
        .version("1.0.0")
        .author("Sven Strittmatter <ich@weltraumschaf.de>")
        .about("A minimalistic byte code compiler with executing virtual machine.")
        .subcommand(SubCommand::with_name("parse")
            .about("Only parses the given source code.")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The file to parse.")
                .required(true))
            .arg(Arg::with_name("tokens")
                .short("t")
                .long("tokens")
                .help("Prints the recognized tokens."))
            .arg(Arg::with_name("ast")
                .short("a")
                .long("ast")
                .help("Prints the parsed abstract syntax tree.")))
        .subcommand(SubCommand::with_name("compile")
            .about("Compiles the given source code file to byte code.")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The file to parse.")
                .required(true)))
        .subcommand(SubCommand::with_name("asm")
            .about("Transforms the given Minivm assembler into byte code.")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The file with assembly code.")
                .required(true)))
        .subcommand(SubCommand::with_name("disasm")
            .about("Prints human readable assembler style form of the givne byte code.")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The file with byte code.")
                .required(true)))
        .subcommand(SubCommand::with_name("run")
            .about("Executes a compiled byte code file.")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The byte code file to execute.")
                .required(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("parse") {
        let file = matches.value_of("file").unwrap();
        ParserCommand::new(
            file.to_string(),
            matches.is_present("tokens"),
            matches.is_present("ast"))
            .execute();
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let file = matches.value_of("file").unwrap();
        CompileCommand::new(file.to_string()).execute();
    } else if let Some(matches) = matches.subcommand_matches("asm") {
        let file = matches.value_of("file").unwrap();
        AssembleCommand::new(file.to_string()).execute();
    } else if let Some(matches) = matches.subcommand_matches("disasm") {
        let file = matches.value_of("file").unwrap();
        DisassembleCommand::new(file.to_string()).execute();
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let file = matches.value_of("file").unwrap();
        RunCommand::new(file.to_string()).execute();
    } else {
        error("No subcommand given!");
        return;
    }
}
