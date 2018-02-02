use minivm::commands::compile_command::CompileCommand;
use minivm::commands::parse_command::ParserCommand;
use minivm::commands::run_command::RunCommand;
use minivm::commands::Command;
use minivm::error;
use clap::{Arg, App, SubCommand};

#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate clap;
extern crate minivm;

// Make this a executable lib crate and move the main in out.
// https://doc.rust-lang.org/book/second-edition/ch07-02-controlling-visibility-with-pub.html
fn main() {
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
        ParserCommand::new(file.to_string()).execute();
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let file = matches.value_of("file").unwrap();
        CompileCommand::new(file.to_string()).execute();
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let file = matches.value_of("file").unwrap();
        RunCommand::new(file.to_string()).execute();
    } else {
        error("No subcommand given!");
        return;
    }
}
