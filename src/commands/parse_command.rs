use std::io::prelude::*;
use std::fs::File;
use frontend::character_stream::CharacterStream;
use frontend::lexer::Lexer;
use frontend::parser::Parser;
use frontend::token::TokenType;
use commands::Command;
use error;

pub struct ParserCommand {
    file: String,
    print_tokens: bool,
    print_ast: bool,
}

impl ParserCommand {
    pub fn new(file: String, print_tokens: bool, print_ast: bool) -> ParserCommand {
        ParserCommand { file, print_tokens, print_ast }
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

        debug!("Parsing source:");
        debug!("{}", content);
        debug!("--- source end ---");

        let input_stream = CharacterStream::new(content);

        if self.print_tokens {
            println!("Recognized tokens:");
            let mut lexer = Lexer::new(input_stream);

            loop {
                lexer.next();
                let token = lexer.current();
                println!("{}", token);

                if &TokenType::EOF == token.get_token_type() {
                    debug!("Got EOF token: Exiting token loop.");
                    break;
                }
            }
        }

        if self.print_ast {
            println!("Parsed AST:")
        }


//        let parser = Parser::new(lexer);
//        parser.parse();
    }
}
