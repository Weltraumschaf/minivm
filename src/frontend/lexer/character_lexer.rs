use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct CharacterLexer {}

impl CharacterLexer {
    pub fn new() -> CharacterLexer {
        CharacterLexer {}
    }
}

impl SubLexer for CharacterLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        let position = input.position();

        if !input.has_next() {
            panic!("Unterminated character literal");
        }

        input.next(); // consume '

        if CharacterHelper::is_single_quote(input.current()) {
            return Token::new(
                position,
                TokenType::CHARACTER(0 as char),
                String::from(""));
        }

        let ch = input.current();

        if !input.has_next() {
            panic!("Unterminated character literal");
        }

        input.next(); // consume character

        if !CharacterHelper::is_single_quote(input.current()) {
            panic!("Unterminated character literal!");
        }

        Token::new(position, TokenType::CHARACTER(ch), format!("'{}'", ch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn scan_single_char() {
        let mut src = CharacterStream::new(String::from("'c'"));
        let sut = CharacterLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::CHARACTER('c'),
            String::from("'c'"))
        )));
    }

    #[test]
    fn scan_empty_char() {
        let mut src = CharacterStream::new(String::from("''"));
        let sut = CharacterLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::CHARACTER(0 as char),
            String::from(""))
        )));
    }

    #[test]
    #[ignore] // Panics as expected, but test fails in IDE, not on CLI
    #[should_panic]
    fn scan_unterminated_char() {
        let mut src = CharacterStream::new(String::from("'c"));
        let sut = CharacterLexer::new();

        let token = sut.scan(&mut src);
    }
}