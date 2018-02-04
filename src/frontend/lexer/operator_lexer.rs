use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct OperatorLexer {}

impl OperatorLexer {
    pub fn new() -> OperatorLexer {
        OperatorLexer {}
    }
}

impl SubLexer for OperatorLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        let position = input.position();

        match input.current() {
            '(' => {
                input.next(); //consume (
                Token::new(
                    position,
                    TokenType::L_PAREN,
                    String::from("("))
            },
            ')' => {
                input.next(); //consume )
                Token::new(
                    position,
                    TokenType::R_PAREN,
                    String::from(")"))
            },
            '[' => {
                input.next(); //consume [
                Token::new(
                    position,
                    TokenType::L_BRACK,
                    String::from("["))
            },
            ']' => {
                input.next(); //consume ]
                Token::new(
                    position,
                    TokenType::R_BRACK,
                    String::from("]"))
            },
            '{' => {
                input.next(); //consume {
                Token::new(
                    position,
                    TokenType::L_BRACE,
                    String::from("{"))
            },
            '}' => {
                input.next(); //consume }
                Token::new(
                    position,
                    TokenType::R_BRACE,
                    String::from("}"))
            },
            ',' => {
                input.next(); //consume ,
                Token::new(
                    position,
                    TokenType::COMMA,
                    String::from(","))
            },
            '=' => {
                input.next(); //consume =

                if '=' == input.current() {
                    input.next(); //consume =
                    Token::new(
                        position,
                        TokenType::OPERATOR(Operator::EQUAL),
                        String::from("=="))
                } else {
                    Token::new(
                        position,
                        TokenType::OPERATOR(Operator::ASSIGN),
                        String::from("="))
                }
            },
            '!' => {
                input.next(); //consume !

                if '=' == input.current() {
                    input.next(); //consume =
                    Token::new(
                        position,
                        TokenType::OPERATOR(Operator::NOT_EQUAL),
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
                        TokenType::OPERATOR(Operator::LESS_THAN_EQUAL),
                        String::from("<="))
                } else {
                    Token::new(
                        position,
                        TokenType::OPERATOR(Operator::LESS_THAN),
                        String::from("<"))
                }
            },
            '>' => {
                input.next(); //consume >

                if '=' == input.current() {
                    input.next(); //consume =
                    Token::new(
                        position,
                        TokenType::OPERATOR(Operator::GREATER_THAN_EQUAL),
                        String::from(">="))
                } else {
                    Token::new(
                        position,
                        TokenType::OPERATOR(Operator::GREATER_THAN),
                        String::from(">"))
                }
            },
            '+' => {
                input.next(); //consume +
                Token::new(
                    position,
                    TokenType::OPERATOR(Operator::PLUS),
                    String::from("+"))
            },
            '-' => {
                input.next(); //consume -
                Token::new(
                    position,
                    TokenType::OPERATOR(Operator::MINUS),
                    String::from("-"))
            },
            '*' => {
                input.next(); //consume *
                Token::new(
                    position,
                    TokenType::OPERATOR(Operator::STAR),
                    String::from("*"))
            },
            '/' => {
                input.next(); //consume /
                Token::new(
                    position,
                    TokenType::OPERATOR(Operator::SLASH),
                    String::from("/"))
            },
            '%' => {
                input.next(); //consume %
                Token::new(
                    position,
                    TokenType::OPERATOR(Operator::MOD),
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
            TokenType::L_PAREN,
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
            TokenType::R_PAREN,
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
            TokenType::L_BRACK,
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
            TokenType::R_BRACK,
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
            TokenType::L_BRACE,
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
            TokenType::R_BRACE,
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
            TokenType::COMMA,
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
            TokenType::OPERATOR(Operator::EQUAL),
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
            TokenType::OPERATOR(Operator::ASSIGN),
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
            TokenType::OPERATOR(Operator::NOT_EQUAL),
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
            TokenType::OPERATOR(Operator::LESS_THAN_EQUAL),
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
            TokenType::OPERATOR(Operator::LESS_THAN),
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
            TokenType::OPERATOR(Operator::GREATER_THAN_EQUAL),
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
            TokenType::OPERATOR(Operator::GREATER_THAN),
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
            TokenType::OPERATOR(Operator::PLUS),
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
            TokenType::OPERATOR(Operator::MINUS),
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
            TokenType::OPERATOR(Operator::STAR),
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
            TokenType::OPERATOR(Operator::SLASH),
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
            TokenType::OPERATOR(Operator::MOD),
            String::from("%"))
        )));
    }
}