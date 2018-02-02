use std::io::prelude::*;
use std::fs::File;
use frontend::character_stream::CharacterStream;
use frontend::lexer::Lexer;
use frontend::parser::Parser;
use commands::Command;
use error;

pub struct ParserCommand {
    file: String,
}

impl ParserCommand {
    pub fn new(file: String) -> ParserCommand {
        ParserCommand { file }
    }
}

impl Command for ParserCommand {
    fn execute(self) {
        println!("Parsing file {} ...", self.file);

        let mut f = match File::open(self.file) {
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
}
