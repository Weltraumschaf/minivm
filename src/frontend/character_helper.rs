/// Provides several helper functions to recognize characters.
pub struct CharacterHelper {}

impl CharacterHelper {
    /// Determines if a given character is alphabetic.
    pub fn is_alphabetic(ch: char) -> bool {
        ch.is_alphabetic()
    }

    /// Determines if a given character is numeric.
    pub fn is_numeric(ch: char) -> bool {
        ch.is_numeric()
    }

    /// Determines if a given character is alphabetic or numeric.
    pub fn is_alphanumeric(ch: char) -> bool {
        (CharacterHelper::is_alphabetic(ch) || CharacterHelper::is_numeric(ch))
    }

    /// Determines if a given character is a double quote.
    pub fn is_double_quote(ch: char) -> bool {
        ch == '"'
    }

    /// Determines if a given character is a single quote.
    pub fn is_single_quote(ch: char) -> bool {
        ch == '\''
    }

    /// Determines if a given character is an operator.
    pub fn is_operator(ch: char) -> bool {
        ch == '+' ||
            ch == '-' ||
            ch == '*' ||
            ch == '/' ||
            ch == '%' ||
            ch == '=' ||
            ch == '!' ||
            ch == '<' ||
            ch == '>' ||
            ch == ',' ||
            ch == '(' ||
            ch == ')' ||
            ch == '{' ||
            ch == '}' ||
            ch == '[' ||
            ch == ']'
    }

    /// Determines if a given character is a white space.
    pub fn is_white_space(ch: char) -> bool {
        ch == ' ' || ch == '\t'
    }

    /// Determines if a given character is a new line.
    pub fn is_new_line(ch: char) -> bool {
        ch == '\n'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn is_alphabetic() {
        assert_that!(CharacterHelper::is_alphabetic('a'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('A'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('b'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('B'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('c'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('C'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('x'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('X'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('y'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('Y'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('z'), is(true));
        assert_that!(CharacterHelper::is_alphabetic('Z'), is(true));

        assert_that!(CharacterHelper::is_alphabetic('0'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('1'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('2'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('3'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('4'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('5'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('6'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('7'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('8'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('9'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('!'), is(false));
        assert_that!(CharacterHelper::is_alphabetic('+'), is(false));
        assert_that!(CharacterHelper::is_alphabetic(' '), is(false));
        assert_that!(CharacterHelper::is_alphabetic('\n'), is(false));
    }

    #[test]
    fn is_numeric() {
        assert_that!(CharacterHelper::is_numeric('0'), is(true));
        assert_that!(CharacterHelper::is_numeric('1'), is(true));
        assert_that!(CharacterHelper::is_numeric('2'), is(true));
        assert_that!(CharacterHelper::is_numeric('3'), is(true));
        assert_that!(CharacterHelper::is_numeric('4'), is(true));
        assert_that!(CharacterHelper::is_numeric('5'), is(true));
        assert_that!(CharacterHelper::is_numeric('6'), is(true));
        assert_that!(CharacterHelper::is_numeric('7'), is(true));
        assert_that!(CharacterHelper::is_numeric('8'), is(true));
        assert_that!(CharacterHelper::is_numeric('9'), is(true));

        assert_that!(CharacterHelper::is_numeric('a'), is(false));
        assert_that!(CharacterHelper::is_numeric('!'), is(false));
        assert_that!(CharacterHelper::is_numeric('+'), is(false));
        assert_that!(CharacterHelper::is_numeric(' '), is(false));
        assert_that!(CharacterHelper::is_numeric('\n'), is(false));
        assert_that!(CharacterHelper::is_numeric(0 as char), is(false));
    }

    #[test]
    fn is_alphanumeric() {
        assert_that!(CharacterHelper::is_alphanumeric('0'), is(true));
        assert_that!(CharacterHelper::is_alphanumeric('5'), is(true));
        assert_that!(CharacterHelper::is_alphanumeric('a'), is(true));
        assert_that!(CharacterHelper::is_alphanumeric('g'), is(true));
        assert_that!(CharacterHelper::is_alphanumeric('Z'), is(true));

        assert_that!(CharacterHelper::is_alphanumeric('-'), is(false));
        assert_that!(CharacterHelper::is_alphanumeric('_'), is(false));
    }

    #[test]
    fn is_double_quote() {
        assert_that!(CharacterHelper::is_double_quote('"'), is(true));

        assert_that!(CharacterHelper::is_double_quote('\''), is(false));
        assert_that!(CharacterHelper::is_double_quote('a'), is(false));
        assert_that!(CharacterHelper::is_double_quote('Z'), is(false));
        assert_that!(CharacterHelper::is_double_quote('!'), is(false));
        assert_that!(CharacterHelper::is_double_quote(' '), is(false));
        assert_that!(CharacterHelper::is_double_quote('\n'), is(false));
    }

    #[test]
    fn is_single_quote() {
        assert_that!(CharacterHelper::is_single_quote('\''), is(true));

        assert_that!(CharacterHelper::is_single_quote('"'), is(false));
        assert_that!(CharacterHelper::is_single_quote('a'), is(false));
        assert_that!(CharacterHelper::is_single_quote('Z'), is(false));
        assert_that!(CharacterHelper::is_single_quote('!'), is(false));
        assert_that!(CharacterHelper::is_single_quote(' '), is(false));
        assert_that!(CharacterHelper::is_single_quote('\n'), is(false));
    }

    #[test]
    fn is_operator() {
        assert_that!(CharacterHelper::is_operator('+'), is(true));
        assert_that!(CharacterHelper::is_operator('-'), is(true));
        assert_that!(CharacterHelper::is_operator('*'), is(true));
        assert_that!(CharacterHelper::is_operator('/'), is(true));
        assert_that!(CharacterHelper::is_operator('%'), is(true));
        assert_that!(CharacterHelper::is_operator('='), is(true));
        assert_that!(CharacterHelper::is_operator('!'), is(true));
        assert_that!(CharacterHelper::is_operator('<'), is(true));
        assert_that!(CharacterHelper::is_operator('>'), is(true));
        assert_that!(CharacterHelper::is_operator(','), is(true));
        assert_that!(CharacterHelper::is_operator('('), is(true));
        assert_that!(CharacterHelper::is_operator(')'), is(true));
        assert_that!(CharacterHelper::is_operator('{'), is(true));
        assert_that!(CharacterHelper::is_operator('}'), is(true));
        assert_that!(CharacterHelper::is_operator('['), is(true));
        assert_that!(CharacterHelper::is_operator(']'), is(true));

        assert_that!(CharacterHelper::is_operator('\''), is(false));
        assert_that!(CharacterHelper::is_operator('"'), is(false));
        assert_that!(CharacterHelper::is_operator('a'), is(false));
        assert_that!(CharacterHelper::is_operator('Z'), is(false));
        assert_that!(CharacterHelper::is_operator(' '), is(false));
        assert_that!(CharacterHelper::is_operator('\n'), is(false));
    }

    #[test]
    fn is_white_space() {
        assert_that!(CharacterHelper::is_white_space(' '), is(true));
        assert_that!(CharacterHelper::is_white_space('\t'), is(true));

        assert_that!(CharacterHelper::is_white_space('3'), is(false));
        assert_that!(CharacterHelper::is_white_space('a'), is(false));
        assert_that!(CharacterHelper::is_white_space('Z'), is(false));
        assert_that!(CharacterHelper::is_white_space('!'), is(false));
        assert_that!(CharacterHelper::is_white_space('\n'), is(false));
    }

    #[test]
    fn is_new_line() {
        assert_that!(CharacterHelper::is_new_line('\n'), is(true));

        assert_that!(CharacterHelper::is_new_line('3'), is(false));
        assert_that!(CharacterHelper::is_new_line('a'), is(false));
        assert_that!(CharacterHelper::is_new_line('Z'), is(false));
        assert_that!(CharacterHelper::is_new_line('!'), is(false));
        assert_that!(CharacterHelper::is_new_line(' '), is(false));
    }
}