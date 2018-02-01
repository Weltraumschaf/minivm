use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
use frontend::token::Token;
use frontend::token::TokenType::*;
use frontend::Position;
use frontend::lexer::number_lexer::NumberLexer;

mod number_lexer;

pub struct Lexer {
    input: CharacterStream,
    current: Token,
}

impl Lexer {
    pub fn new(input: CharacterStream) -> Lexer {
        Lexer {
            input,
            current: Token::new(
                Position::null(),
                EOF,
                String::from(""))
        }
    }

    pub fn input(self) -> CharacterStream {
        self.input
    }

    fn next(&mut self) -> Token {
        let mut token = self.default();

        while self.input.has_next() {
            self.input.next();
            let current = self.input.current();

            if CharacterHelper::is_alphabetic(current) {
// TODO
            } else if CharacterHelper::is_numeric(current) {
                let lexer = NumberLexer::new();
                token = lexer.scan(&mut self.input);
            } else if CharacterHelper::is_double_quote(current) {
// TODO
            } else if CharacterHelper::is_single_quote(current) {
// TODO
            } else if CharacterHelper::is_operator(current) {
// TODO
            } else if CharacterHelper::is_white_space(current) {
// TODO
            } else if CharacterHelper::is_new_line(current) {
                token = Token::new(
                    self.input.position(),
                    EOL,
                    String::from("\n"));
                break;
            }
        }

        token
    }

    fn default(&self) -> Token {
        Token::new(
            self.input.position(),
            EOF,
            String::from(""))
    }
}

trait SubLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token;
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    fn crete_sut(input: &str) -> Lexer {
        Lexer::new(CharacterStream::new(String::from(input)))
    }

    #[test]
    fn next() {}
}