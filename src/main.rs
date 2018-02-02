use std::fs::File;
use std::io::prelude::*;
use minivm::frontend::character_stream::CharacterStream;
use minivm::frontend::lexer::Lexer;
use minivm::frontend::parser::Parser;
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
        println!("Parsing file {} ...", file);

        let mut f = match File::open(file) {
            Ok(f) => f,
            Err(_) => {
                error("Failed to read file!");
                return;
            }
        };

        let mut content = String::new();
        match f.read_to_string(&mut content) {
            Ok(contents) => contents,
            Err(_) => {
                error("Something went wrong reading the file!");
                return;
            }
        };

        let input_stream = CharacterStream::new(content);
        let lexer = Lexer::new(input_stream);
        let parser = Parser::new(lexer);
        parser.parse();
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let file = matches.value_of("file").unwrap();
        println!("Compile file {} ...", file);
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let file = matches.value_of("file").unwrap();
        println!("Execute file {} ...", file);
    } else {
        error("No subcommand given!");
        return;
    }
}

fn error(msg: &str) {
    println!("ERROR: {}", msg);
}