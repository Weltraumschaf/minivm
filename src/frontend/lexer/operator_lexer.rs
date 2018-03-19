use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
#[cfg(test)]
use frontend::Position;

/// Recognizes a operator literal token.
pub struct OperatorLexer {}

impl OperatorLexer {
    pub fn new() -> OperatorLexer {
        OperatorLexer {}
    }
}

impl SubLexer for OperatorLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        let position = input.position();
        debug!("Start operator lexing at position {}.", position);

        match input.current() {
            '(' => {
                input.next(); //consume (
                Token::new(
                    position,
                    TokenType::LeftParen,
                    String::from("("))
            },
            ')' => {
                input.next(); //consume )
                Token::new(
                    position,
                    TokenType::RightParen,
                    String::from(")"))
            },
            '[' => {
                input.next(); //consume [
                Token::new(
                    position,
                    TokenType::LeftBracket,
                    String::from("["))
            },
            ']' => {
                input.next(); //consume ]
                Token::new(
                    position,
                    TokenType::RightBracket,
                    String::from("]"))
            },
            '{' => {
                input.next(); //consume {
                Token::new(
                    position,
                    TokenType::LeftBrace,
                    String::from("{"))
            },
            '}' => {
                input.next(); //consume }
                Token::new(
                    position,
                    TokenType::RightBrace,
                    String::from("}"))
            },
            ',' => {
                input.next(); //consume ,
                Token::new(
                    position,
                    TokenType::Comma,
                    String::from(","))
            },
            '=' => {
                input.next(); //consume =

                if '=' == input.current() {
                    input.next(); //consume =
                    Token::new(
                        position,
                        TokenType::Operator(Operator::Equal),
                        String::from("=="))
                } else {
                    Token::new(
                        position,
                        TokenType::Operator(Operator::Assign),
                        String::from("="))
                }
            },
            '!' => {
                input.next(); //consume !

                if '=' == input.current() {
                    input.next(); //consume =
                    Token::new(
                        position,
                        TokenType::Operator(Operator::NotEqual),
                        String::from("!="))
                } else {
                    panic!("Expecting = after ! for != operator!");
                }
            },
            '<' => {
                input.next(); //consume <

                if '=' == input.current() {
                    input.next(); //consume =
                    Token::new(
                        position,
                        TokenType::Operator(Operator::LessThanEqual),
                        String::from("<="))
                } else {
                    Token::new(
                        position,
                        TokenType::Operator(Operator::LessThan),
                        String::from("<"))
                }
            },
            '>' => {
                input.next(); //consume >

                if '=' == input.current() {
                    input.next(); //consume =
                    Token::new(
                        position,
                        TokenType::Operator(Operator::GreaterThanEqual),
                        String::from(">="))
                } else {
                    Token::new(
                        position,
                        TokenType::Operator(Operator::GreaterThan),
                        String::from(">"))
                }
            },
            '+' => {
                input.next(); //consume +
                Token::new(
                    position,
                    TokenType::Operator(Operator::Plus),
                    String::from("+"))
            },
            '-' => {
                input.next(); //consume -
                Token::new(
                    position,
                    TokenType::Operator(Operator::Minus),
                    String::from("-"))
            },
            '*' => {
                input.next(); //consume *
                Token::new(
                    position,
                    TokenType::Operator(Operator::Star),
                    String::from("*"))
            },
            '/' => {
                input.next(); //consume /
                Token::new(
                    position,
                    TokenType::Operator(Operator::Slash),
                    String::from("/"))
            },
            '%' => {
                input.next(); //consume %
                Token::new(
                    position,
                    TokenType::Operator(Operator::Mod),
                    String::from("%"))
            }
            _ => panic!("Unrecognized operator '{}'!", input.current())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn scan_l_paren() {
        let mut src = CharacterStream::new(String::from("("));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::LeftParen,
            String::from("("))
        )));
    }

    #[test]
    fn scan_r_paren() {
        let mut src = CharacterStream::new(String::from(")"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::RightParen,
            String::from(")"))
        )));
    }

    #[test]
    fn scan_l_brack() {
        let mut src = CharacterStream::new(String::from("["));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::LeftBracket,
            String::from("["))
        )));
    }

    #[test]
    fn scan_r_brack() {
        let mut src = CharacterStream::new(String::from("]"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::RightBracket,
            String::from("]"))
        )));
    }

    #[test]
    fn scan_l_brace() {
        let mut src = CharacterStream::new(String::from("{"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::LeftBrace,
            String::from("{"))
        )));
    }

    #[test]
    fn scan_r_brace() {
        let mut src = CharacterStream::new(String::from("}"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::RightBrace,
            String::from("}"))
        )));
    }

    #[test]
    fn scan_comma() {
        let mut src = CharacterStream::new(String::from(","));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Comma,
            String::from(","))
        )));
    }

    #[test]
    fn scan_equal() {
        let mut src = CharacterStream::new(String::from("=="));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::Equal),
            String::from("=="))
        )));
    }

    #[test]
    fn scan_assign() {
        let mut src = CharacterStream::new(String::from("="));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::Assign),
            String::from("="))
        )));
    }

    #[test]
    fn scan_not_euql() {
        let mut src = CharacterStream::new(String::from("!="));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::NotEqual),
            String::from("!="))
        )));
    }

    #[test]
    fn scan_less_than_equal() {
        let mut src = CharacterStream::new(String::from("<="));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::LessThanEqual),
            String::from("<="))
        )));
    }

    #[test]
    fn scan_less_than() {
        let mut src = CharacterStream::new(String::from("<"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::LessThan),
            String::from("<"))
        )));
    }

    #[test]
    fn scan_greater_than_equal() {
        let mut src = CharacterStream::new(String::from(">="));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::GreaterThanEqual),
            String::from(">="))
        )));
    }

    #[test]
    fn scan_greater_than() {
        let mut src = CharacterStream::new(String::from(">"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::GreaterThan),
            String::from(">"))
        )));
    }

    #[test]
    fn scan_plus() {
        let mut src = CharacterStream::new(String::from("+"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::Plus),
            String::from("+"))
        )));
    }

    #[test]
    fn scan_minus() {
        let mut src = CharacterStream::new(String::from("-"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::Minus),
            String::from("-"))
        )));
    }

    #[test]
    fn scan_star() {
        let mut src = CharacterStream::new(String::from("*"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::Star),
            String::from("*"))
        )));
    }

    #[test]
    fn scan_slash() {
        let mut src = CharacterStream::new(String::from("/"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::Slash),
            String::from("/"))
        )));
    }

    #[test]
    fn scan_mod() {
        let mut src = CharacterStream::new(String::from("%"));
        let sut = OperatorLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token, is(equal_to(
            Token::new(Position::new(1, 1),
            TokenType::Operator(Operator::Mod),
            String::from("%"))
        )));
    }
}