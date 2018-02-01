use std::env;
use std::fs::File;
use std::io::prelude::*;
use frontend::character_stream::CharacterStream;
use frontend::lexer::Lexer;
use frontend::parser::Parser;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod frontend;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error("Exactly one argument expected!");
        return;
    }

    let file = &args[1];
    println!("Running file {} ...", file);

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
}

fn error(msg: &str) {
    println!("ERROR: {}", msg);
}