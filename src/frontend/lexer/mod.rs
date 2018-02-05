use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
use frontend::token::Token;
use frontend::token::TokenType::*;
use frontend::Position;
use frontend::lexer::character_lexer::CharacterLexer;
use frontend::lexer::identifier_lexer::IdentifierLexer;
use frontend::lexer::number_lexer::NumberLexer;
use frontend::lexer::operator_lexer::OperatorLexer;
use frontend::lexer::string_lexer::StringLexer;

mod character_lexer;
mod identifier_lexer;
mod number_lexer;
mod operator_lexer;
mod string_lexer;

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

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn next(&mut self) {
        if !self.input.has_next() {
            debug!("No more input to lex.");
            let position = self.input.position();
            self.current = Token::new(
                self.input.position(),
                EOF,
                String::from(""));
            return;
        }

        while self.input.has_next() {
            debug!("Lexer loop iteration start.");
            let position = self.input.position();
            let current = self.input.current();

            if CharacterHelper::is_alphabetic(current) {
                // scan for boolean/keyword/identifier
                debug!("Current char is '{}' at {}. Use identifier lexer.", current, position);
                self.current = IdentifierLexer::new().scan(&mut self.input);
                break;
            } else if CharacterHelper::is_numeric(current) {
                // scan for integer and real number
                debug!("Current char is '{}' at {}. Use number lexer.", current, position);
                self.current = NumberLexer::new().scan(&mut self.input);
                break;
            } else if CharacterHelper::is_double_quote(current) {
                // scan for string literal
                debug!("Current char is '{}' at {}. Use string lexer.", current, position);
                self.current = StringLexer::new().scan(&mut self.input);
                break;
            } else if CharacterHelper::is_single_quote(current) {
                // scan for single character literal
                debug!("Current char is '{}' at {}. Use character lexer.", current, position);
                self.current = CharacterLexer::new().scan(&mut self.input);
                break;
            } else if CharacterHelper::is_operator(current) {
                // scan for operator or delimiter
                debug!("Current char is '{}' at {}. Use operator lexer.", current, position);
                self.current = OperatorLexer::new().scan(&mut self.input);
                break;
            } else if CharacterHelper::is_white_space(current) {
                // ignore white spaces
                debug!("Current char is '{}' at {}. Ignoring whitespace.", current, position);
                self.input.next(); // consume space
                break;
            } else if CharacterHelper::is_new_line(current) {
                debug!("Current char is '{}' at {}. Detected EOL.", current, position);
                self.current = Token::new(
                    self.input.position(),
                    EOL,
                    String::from("\\n"));
                self.input.next(); // consume \n
                break;
            }
        }
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