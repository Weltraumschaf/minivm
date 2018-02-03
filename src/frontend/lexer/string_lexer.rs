use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct StringLexer {}

impl StringLexer {
    pub fn new() -> StringLexer {
        StringLexer {}
    }
}

impl SubLexer for StringLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        let position = input.position();
        let mut value = String::new();

        if !input.has_next() {
            return Token::new(position, TokenType::EOF, String::from(""));
        }

        input.next(); // consume the "

        loop {
            if '"' == input.current() {
                input.next(); // consume the "
                break; // end of string literal
            }

            if END_OF_FILE == input.current() {
                panic!("Unterminated string literal!");
            }

            value.push(input.current());
            input.next(); // consume character
        }

        Token::new(
            position,
            TokenType::STRING(value.clone()),
            format!("\"{}\"", value.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn scan_empty_string() {
        let mut src = CharacterStream::new(String::from(""));
        let sut = StringLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::EOF,
            String::from(""))
        )));
    }

    #[test]
    #[ignore] // Panics as expected, but test fails in IDE, not on CLI
    #[should_panic]
    fn scan_with_unexpected_end() {
        let mut src = CharacterStream::new(String::from("\""));
        let sut = StringLexer::new();

        let token = sut.scan(&mut src);
    }

    #[test]
    fn scan_zero_length_string() {
        let mut src = CharacterStream::new(String::from("\"\""));
        let sut = StringLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::STRING(String::from("")),
            String::from("\"\""))
        )));
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn scan_unterminated_string() {
        let mut src = CharacterStream::new(String::from("\"foobar"));
        let sut = StringLexer::new();

        let token = sut.scan(&mut src);
    }

    #[test]
    fn scan_normal_string() {
        let mut src = CharacterStream::new(String::from("\"foo bar baz\""));
        let sut = StringLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(
            Position::new(1, 1),
            TokenType::STRING(String::from("foo bar baz")),
            String::from("\"foo bar baz\""))
        )));
    }
}