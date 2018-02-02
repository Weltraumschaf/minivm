use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct IdentifierLexer {}

impl IdentifierLexer {
    pub fn new() -> IdentifierLexer {
        IdentifierLexer {}
    }

    fn collect_alpha_numeric_characters(&self, input: &mut CharacterStream) -> String {
        let mut consumed = String::new();

        loop {
            if END_OF_FILE == input.current() {
                // 0 Indicates EOF
                break;
            }

            if !CharacterHelper::is_alphanumeric(input.current()) {
                // no more alphanumeric characters
                break;
            }

            consumed.push(input.current());
            input.next(); // consume character
        }

        consumed
    }

    fn is_bool(&self, literal: &String) -> bool {
        "true" == literal || "false" == literal
    }

    fn is_keyword(&self, literal: &String) -> bool {
        "and" == literal ||
            "or" == literal ||
            "not" == literal ||
            "if" == literal ||
            "else" == literal ||
            "while" == literal ||
            "const" == literal ||
            "var" == literal
    }
}

impl SubLexer for IdentifierLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        let position = input.position();
        let literal = self.collect_alpha_numeric_characters(input);

        if self.is_bool(&literal) {
            let value = literal.parse::<bool>().unwrap();
            Token::new(position, TokenType::BOOL(value), literal.clone())
        } else if self.is_keyword(&literal) {
            let keyword = Keyword::for_literal(&literal);
            Token::new(position, TokenType::KEYWORD(keyword), literal.clone())
        } else if literal.is_empty() {
            Token::new(position, TokenType::EOF, String::from(""))
        } else {
            Token::new(
                position,
                TokenType::IDENTIFIER(literal.clone()),
                literal.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn collect_alpha_numeric_characters_empty() {
        let mut src = CharacterStream::new(String::from(""));
        let sut = IdentifierLexer::new();

        let token = sut.collect_alpha_numeric_characters(&mut src);

        assert_that!(token, is(equal_to(String::from(""))));
    }

    #[test]
    fn collect_alpha_numeric_characters() {
        let mut src = CharacterStream::new(String::from("h3ll0"));
        let sut = IdentifierLexer::new();

        let token = sut.collect_alpha_numeric_characters(&mut src);

        assert_that!(token, is(equal_to(String::from("h3ll0"))));
    }

    #[test]
    fn collect_alpha_numeric_characters_trailing_whitespace() {
        let mut src = CharacterStream::new(String::from("h3ll0 "));
        let sut = IdentifierLexer::new();

        let token = sut.collect_alpha_numeric_characters(&mut src);

        assert_that!(token, is(equal_to(String::from("h3ll0"))));
    }

    #[test]
    fn collect_alpha_numeric_characters_trailing_minus() {
        let mut src = CharacterStream::new(String::from("h3ll0-"));
        let sut = IdentifierLexer::new();

        let token = sut.collect_alpha_numeric_characters(&mut src);

        assert_that!(token, is(equal_to(String::from("h3ll0"))));
    }

    #[test]
    fn is_bool() {
        let sut = IdentifierLexer::new();

        assert_that!(sut.is_bool(&String::from("true")), is(true));
        assert_that!(sut.is_bool(&String::from("false")), is(true));
        assert_that!(sut.is_bool(&String::from("snafu")), is(false));
        assert_that!(sut.is_bool(&String::from("")), is(false));
    }

    #[test]
    fn is_keyword() {
        let sut = IdentifierLexer::new();

        assert_that!(sut.is_keyword(&String::from("and")), is(true));
        assert_that!(sut.is_keyword(&String::from("or")), is(true));
        assert_that!(sut.is_keyword(&String::from("not")), is(true));
        assert_that!(sut.is_keyword(&String::from("if")), is(true));
        assert_that!(sut.is_keyword(&String::from("else")), is(true));
        assert_that!(sut.is_keyword(&String::from("while")), is(true));
        assert_that!(sut.is_keyword(&String::from("const")), is(true));
        assert_that!(sut.is_keyword(&String::from("var")), is(true));
        assert_that!(sut.is_keyword(&String::from("snafu")), is(false));
        assert_that!(sut.is_keyword(&String::from("")), is(false));
    }

    #[test]
    fn scan_true() {
        let mut src = CharacterStream::new(String::from("true"));
        let sut = IdentifierLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1), TokenType::BOOL(true), String::from("true"))
        )));
    }

    #[test]
    fn scan_false() {
        let mut src = CharacterStream::new(String::from("false"));
        let sut = IdentifierLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1), TokenType::BOOL(false), String::from("false"))
        )));
    }

    #[test]
    fn scan_keyword() {
        let mut src = CharacterStream::new(String::from("var"));
        let sut = IdentifierLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1), TokenType::KEYWORD(Keyword::VAR), String::from("var"))
        )));
    }

    #[test]
    fn scan_identifier() {
        let mut src = CharacterStream::new(String::from("snafu"));
        let sut = IdentifierLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::IDENTIFIER(String::from("snafu")),
            String::from("snafu"))
        )));
    }

    #[test]
    fn scan_empty() {
        let mut src = CharacterStream::new(String::from(""));
        let sut = IdentifierLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::EOF,
            String::from(""))
        )));
    }
}