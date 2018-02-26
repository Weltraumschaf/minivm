///! This module provides various lexers.

use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
use frontend::token::Token;
use frontend::token::TokenType;
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

/// Does the lexical analysis to recognize tokens from a given source character stream.
pub struct Lexer {
    input: CharacterStream,
    current: Token,
}

impl Lexer {
    /// Creates a new lexer.
    pub fn new(input: CharacterStream) -> Lexer {
        Lexer {
            input,
            current: Token::new(
                Position::null(),
                TokenType::EOF,
                String::from(""))
        }
    }

    /// Get the input character stream.
    pub fn input(self) -> CharacterStream {
        self.input
    }

    /// Get the current recognized token.
    ///
    /// If not yet started the lexing [EOF](../token/enum.TokenType.html#variant.EOF) will be returned as default.
    pub fn current(&self) -> &Token {
        &self.current
    }

    /// Recognizes the next token.
    pub fn next(&mut self) {
        if !self.input.has_next() {
            debug!("No more input to lex.");
            let position = self.input.position();
            self.current = Token::new(
                self.input.position(),
                TokenType::EOF,
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
                continue;
            } else if CharacterHelper::is_new_line(current) {
                debug!("Current char is '{}' at {}. Detected EOL.", current, position);
                self.current = Token::new(
                    self.input.position(),
                    TokenType::EOL,
                    String::from("\\n"));
                self.input.next(); // consume \n
                break;
            }
        }
    }
}

trait SubLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token;
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
    use frontend::token::Keyword;

    #[test]
    fn lex_source() {
        let mut src = CharacterStream::new(String::from("var s = \"Hello, World!\"\n
var x = 1\n
var y = 2\n
z = x + y\n
println(z)\n"));
        let mut sut = Lexer::new(src);

        sut.next();
        let mut expected = Token::new(
            Position::new(1, 1),
            TokenType::KEYWORD(Keyword::VAR),
            String::from("var"));
        assert_that!(sut.current(), is(equal_to(&expected)));

        sut.next();
        expected = Token::new(
            Position::new(1, 5),
            TokenType::IDENTIFIER(String::from("s")),
            String::from("s"));
        assert_that!(sut.current(), is(equal_to(&expected)));
    }
}