use frontend::token::*;
use frontend::lexer::SubLexer;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct NumberLexer {}

impl NumberLexer {
    pub fn new() -> NumberLexer {
        NumberLexer {}
    }

    // Consumes all unsigned inter digits from input until a non integer digit character occurs.
    //
    // @param input must not be {@code null}
    // @return all digits from input
    fn unsigned_integer_digits(&self, input: &mut CharacterStream) -> String {
        let mut digits = String::new();

        loop {
            if END_OF_FILE == input.current() {
                // 0 Indicates EOF
                break;
            }

            if !CharacterHelper::is_numeric(input.current()) {
                // no more digits
                break;
            }

            digits.push(input.current());
            input.next(); // consume digit
        }

        digits
    }
}

enum Type {
    INTEGER,
    REAL,
}

impl SubLexer for NumberLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        let position = input.position();
        debug!("Start number lexing at position {}.", position);
        let mut literal = String::new();
        let mut whole_digits = String::new();    // Digits before the decimal point.
        let mut fraction_digits = String::new(); // Digits after the decimal point.
        let mut exponent_digits = String::new(); // Exponent digits.

        let mut number_type = Type::INTEGER; // Assume INTEGER token type for now.

        // Extract the digits of the whole part of the number.
        whole_digits.push_str(self.unsigned_integer_digits(input).as_str());

        if whole_digits.is_empty() {
            panic!("At least one digit necessary!");
        }

        literal.push_str(whole_digits.as_str());

        // Is there a dot, so we have a floating point number.
        if '.' == input.current() {
            number_type = Type::REAL;
            literal.push(input.current());
            input.next(); // Consume decimal point.
            // Collect the digits of the fraction part of the number.
            fraction_digits.push_str(self.unsigned_integer_digits(input).as_str());

            if fraction_digits.is_empty() {
                panic!("At least one fraction digit necessary!");
            }

            literal.push_str(fraction_digits.as_str());
        }

        // Is there an exponent part?
        if input.current() == 'E' || input.current() == 'e' {
            number_type = Type::REAL;  // Exponent, so token type is FLOAT.
            literal.push(input.current());
            input.next(); // Consume 'E' or 'e'.

            // Exponent sign?
            if input.current() == '+' || input.current() == '-' {
                literal.push(input.current());
                input.next(); // Consume '+' or '-'.
            }

            // Extract the digits of the exponent.
            exponent_digits.push_str(self.unsigned_integer_digits(input).as_str());

            if exponent_digits.is_empty() {
                panic!("At least one exponent digit necessary!");
            }

            literal.push_str(exponent_digits.as_str());
        }

        let token_type;
        match number_type {
            Type::INTEGER => {
                // Compute the value of an integer number token.
                match literal.parse::<i64>() {
                    Ok(value) => token_type = TokenType::INTEGER(value),
                    Err(error) => panic!(format!("ERROR: {}: \"{}\"", error, literal))
                }
            },
            Type::REAL => {
                // Compute the value of a real number token.
                match literal.parse::<f64>() {
                    Ok(value) => token_type = TokenType::REAL(value),
                    Err(error) => panic!(format!("ERROR: {}: \"{}\"", error, literal))
                }
            }
        }

        Token::new(position, token_type, literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn unsigned_integer_digits_empty() {
        let mut src = CharacterStream::new(String::from(""));
        let sut = NumberLexer::new();

        let token = sut.unsigned_integer_digits(&mut src);

        assert_that!(token, is(equal_to(String::from(""))));
    }

    #[test]
    fn unsigned_integer_digits_only_numbers() {
        let mut src = CharacterStream::new(String::from("1234"));
        let sut = NumberLexer::new();

        let token = sut.unsigned_integer_digits(&mut src);

        assert_that!(token, is(equal_to(String::from("1234"))));
    }

    #[test]
    fn unsigned_integer_digits_numbers_with_trailing_whitespace() {
        let mut src = CharacterStream::new(String::from("1234 "));
        let sut = NumberLexer::new();

        let token = sut.unsigned_integer_digits(&mut src);

        assert_that!(token, is(equal_to(String::from("1234"))));
    }

    #[test]
    fn unsigned_integer_digits_numbers_with_trailing_dot() {
        let mut src = CharacterStream::new(String::from("12345678.9"));
        let sut = NumberLexer::new();

        let token = sut.unsigned_integer_digits(&mut src);

        assert_that!(token, is(equal_to(String::from("12345678"))));
    }

    #[test]
    fn integer() {
        let mut src = CharacterStream::new(String::from("42"));
        let sut = NumberLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token,
            is(equal_to(
                Token::new(Position::new(1, 1), TokenType::INTEGER(42), String::from("42")))
            )
        );
    }

    #[test]
    fn real() {
        let mut src = CharacterStream::new(String::from("3.14"));
        let sut = NumberLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token,
            is(equal_to(
                Token::new(Position::new(1, 1), TokenType::REAL(3.14), String::from("3.14")))
            )
        );
    }

    #[test]
    fn real_with_negative_exponent_lc() {
        let mut src = CharacterStream::new(String::from("7.0e-2"));
        let sut = NumberLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token,
            is(equal_to(
                Token::new(Position::new(1, 1), TokenType::REAL(0.07), String::from("7.0e-2")))
            )
        );
    }

    #[test]
    fn real_with_negative_exponent_uc() {
        let mut src = CharacterStream::new(String::from("7.0E-2"));
        let sut = NumberLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token,
            is(equal_to(
                Token::new(Position::new(1, 1), TokenType::REAL(0.07), String::from("7.0E-2")))
            )
        );
    }

    #[test]
    fn real_with_positive_exponent_lc() {
        let mut src = CharacterStream::new(String::from("7.0e2"));
        let sut = NumberLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token,
            is(equal_to(
                Token::new(Position::new(1, 1), TokenType::REAL(700.), String::from("7.0e2")))
            )
        );
    }

    #[test]
    fn real_with_positive_exponent_uc() {
        let mut src = CharacterStream::new(String::from("7.0E+2"));
        let sut = NumberLexer::new();

        let token = sut.scan(&mut src);

        assert_that!(token,
            is(equal_to(
                Token::new(Position::new(1, 1), TokenType::REAL(700.), String::from("7.0E+2")))
            )
        );
    }
}