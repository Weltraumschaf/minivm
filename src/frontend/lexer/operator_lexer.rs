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

        // delimiters
        if '(' == input.current() {
            input.next(); //consume (
            return Token::new(
                position,
                TokenType::L_PAREN,
                String::from("("));
        } else if ')' == input.current() {
            input.next(); //consume )
            return Token::new(
                position,
                TokenType::R_PAREN,
                String::from(")"));
        } else if '[' == input.current() {
            input.next(); //consume [
            return Token::new(
                position,
                TokenType::L_BRACK,
                String::from("["));
        } else if ']' == input.current() {
            input.next(); //consume ]
            return Token::new(
                position,
                TokenType::R_BRACK,
                String::from("]"));
        } else if '{' == input.current() {
            input.next(); //consume {
            return Token::new(
                position,
                TokenType::L_BRACE,
                String::from("{"));
        } else if '}' == input.current() {
            input.next(); //consume }
            return Token::new(
                position,
                TokenType::R_BRACE,
                String::from("}"));
        } else if ',' == input.current() {
            input.next(); //consume ,
            return Token::new(
                position,
                TokenType::COMMA,
                String::from(","));
        } else if '=' == input.current() {
            input.next(); //consume =

            if '=' == input.current() {
                input.next(); //consume =
                return Token::new(
                    position,
                    TokenType::OPERATOR(Operator::EQUAL),
                    String::from("=="));
            } else {
                return Token::new(
                    position,
                    TokenType::OPERATOR(Operator::ASSIGN),
                    String::from("="));
            }
        } else if '!' == input.current() {
            input.next(); //consume !

            if '=' == input.current() {
                input.next(); //consume =
                return Token::new(
                    position,
                    TokenType::OPERATOR(Operator::NOT_EQUAL),
                    String::from("!="));
            } else {
                panic!("Expecting = after ! for != operator!");
            }
        } else if '<' == input.current() {
            input.next(); //consume <

            if '=' == input.current() {
                input.next(); //consume =
                return Token::new(
                    position,
                    TokenType::OPERATOR(Operator::LESS_THAN_EQUAL),
                    String::from("<="));
            } else {
                return Token::new(
                    position,
                    TokenType::OPERATOR(Operator::LESS_THAN),
                    String::from("<"));
            }
        } else if '>' == input.current() {
            input.next(); //consume >

            if '=' == input.current() {
                input.next(); //consume =
                return Token::new(
                    position,
                    TokenType::OPERATOR(Operator::GREATER_THAN_EQUAL),
                    String::from(">="));
            } else {
                return Token::new(
                    position,
                    TokenType::OPERATOR(Operator::GREATER_THAN),
                    String::from(">"));
            }
        } else if '+' == input.current() {
            input.next(); //consume +
            return Token::new(
                position,
                TokenType::OPERATOR(Operator::PLUS),
                String::from("+"));
        } else if '-' == input.current() {
            input.next(); //consume -
            return Token::new(
                position,
                TokenType::OPERATOR(Operator::MINUS),
                String::from("-"));
        } else if '*' == input.current() {
            input.next(); //consume *
            return Token::new(
                position,
                TokenType::OPERATOR(Operator::STAR),
                String::from("*"));
        } else if '/' == input.current() {
            input.next(); //consume /
            return Token::new(
                position,
                TokenType::OPERATOR(Operator::SLASH),
                String::from("/"));
        } else if '%' == input.current() {
            input.next(); //consume %
            return Token::new(
                position,
                TokenType::OPERATOR(Operator::MOD),
                String::from("%"));
        } else {
            panic!("Unrecognized operator '{}'!", input.current());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
}