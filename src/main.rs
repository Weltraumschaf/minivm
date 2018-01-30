use std::env;
use std::fs::File;
use std::io::prelude::*;

mod lexer;

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

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(contents) => contents,
        Err(_) => {
            error("Something went wrong reading the file!");
            return;
        }
    };

    let bytes = contents.as_bytes();

    for (_, &item) in bytes.iter().enumerate() {
        println!("{}", item);
    }
}

fn error(msg: &str) {
    println!("ERROR: {}", msg);
}