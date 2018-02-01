pub mod character_stream;
pub mod character_helper;
pub mod lexer;
pub mod parser;
pub mod token;

#[derive(Debug)]
pub struct Position {
    line: u64,
    column: u64,
}

impl Position {
    fn null() -> Position {
        Position::new(0, 0)
    }

    fn new(line: u64, column: u64) -> Position {
        Position { line, column }
    }

    fn at(&self, p: Position) -> bool {
        p.line == self.line && p.column == self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn at() {
        let sut = Position::new(42, 23);

        assert_that!(sut.at(Position::null()), is(false));
        assert_that!(sut.at(Position::new(42, 23)), is(true));
    }

    #[test]
    fn null() {
        let sut = Position::null();

        assert_that(sut.at(Position::null()), is(true));
        assert_that(sut.at(Position::new(42, 23)), is(false));
    }
}
