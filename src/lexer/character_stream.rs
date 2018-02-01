use lexer::Position;

#[derive(Debug)]
struct CharacterStream {
    input: String,
    index: usize,
    line: u64,
    column: u64,
    new_line_seen: bool
}

impl CharacterStream {
    fn new(input: String) -> CharacterStream {
        CharacterStream {
            input,
            index: 0,
            line: 1,
            column: 1,
            new_line_seen: false,
        }
    }

    fn next(&mut self) -> Option<char> {
        if !self.has_next() {
            return None
        }

        if self.position().at(Position::null()) {
            self.line = 1;
            self.column = 1;
        }

        let current = self.current();
        self.index += 1;

        if self.new_line_seen {
            self.line += 1;
            self.column = 1;
            self.new_line_seen = false;
        } else {
            self.column += 1;
        }

        if let Some(c) = current {
            if '\n' == c {
                self.new_line_seen = true;
            }
        }

        current
    }

    fn has_next(&self) -> bool {
        self.index < self.input.len()
    }

    fn current(&mut self) -> Option<char> {
        self.input.chars().nth(self.index)
    }

    fn peek(&mut self) -> Option<char> {
        let index_backup = self.index;
        let c = self.next();
        self.index = index_backup;
        c
    }

    fn position(&self) -> Position {
        Position { line: self.line, column: self.column }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    fn crete_sut(input: &str) -> CharacterStream {
        CharacterStream::new(String::from(input))
    }

    #[test]
    fn has_no_next_for_empty_string() {
        let sut = crete_sut("");

        assert_that!(sut.has_next(), is(false));
    }

    #[test]
    fn next_for_empty_string() {
        let mut sut = crete_sut("");

        assert_that!(sut.next(), is(equal_to(None)));
    }

    #[test]
    fn current_for_empty_string() {
        let mut sut = crete_sut("");

        assert_that!(sut.current(), is(equal_to(None)));
    }

    #[test]
    fn iterate_through_string() {
        let mut sut = crete_sut("Hello,\nWorld!");

        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('H'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('e'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('l'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('l'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('o'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some(','))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('\n'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('W'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('o'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('r'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('l'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('d'))));
        assert_that!(sut.has_next(), is(equal_to(true)));
        assert_that!(sut.next(), is(equal_to(Some('!'))));
        assert_that!(sut.has_next(), is(equal_to(false)));
        assert_that!(sut.next(), is(equal_to(None)));
    }

    #[test]
    fn peek() {
        let mut sut = crete_sut("Hello");

        sut.next();

        assert_that!(sut.peek(), is(equal_to(Some('e'))));
        assert_that!(sut.next(), is(equal_to(Some('e'))));

        assert_that!(sut.peek(), is(equal_to(Some('l'))));
        assert_that!(sut.next(), is(equal_to(Some('l'))));

        sut.next();

        assert_that!(sut.peek(), is(equal_to(Some('o'))));
        assert_that!(sut.next(), is(equal_to(Some('o'))));
        assert_that!(sut.peek(), is(equal_to(None)));
    }
}