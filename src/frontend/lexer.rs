use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
use frontend::token::Token;
use frontend::token::TokenType::*;
use frontend::Position;

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

    fn next(&mut self) -> Token {
        let mut token = Token::new(
            Position::null(),
            EOF,
            String::from(""));

        while self.input.has_next() {
            if let Some(current) = self.input.next() {
                if CharacterHelper::is_alphabetic(current) {

                } else if CharacterHelper::is_numeric(current) {

                } else if CharacterHelper::is_double_quote(current) {

                } else if CharacterHelper::is_single_quote(current) {

                } else if CharacterHelper::is_operator(current) {

                } else if CharacterHelper::is_white_space(current) {

                } else if CharacterHelper::is_new_line(current) {
                    token = Token::new(
                        self.input.position(),
                        EOL,
                        String::from("\n"));
                    break;
                }
            } else {
                token = Token::new(
                    self.input.position(),
                    EOF,
                    String::from(""));
            }
        }

        token
    }

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