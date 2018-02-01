use frontend::Position;

#[derive(Debug)]
pub struct CharacterStream {
    input: String,
    index: usize,
    line: u64,
    column: u64,
    new_line_seen: bool
}

impl CharacterStream {
    pub fn new(input: String) -> CharacterStream {
        CharacterStream {
            input,
            index: 0,
            line: 1,
            column: 1,
            new_line_seen: false,
        }
    }

    pub fn next(&mut self) {
        if !self.has_next() {
            return;
        }

        if self.position().at(Position::null()) {
            self.line = 1;
            self.column = 1;
        }

        self.index += 1;

        if self.new_line_seen {
            self.line += 1;
            self.column = 1;
            self.new_line_seen = false;
        } else {
            self.column += 1;
        }

        if '\n' == self.current() {
            self.new_line_seen = true;
        }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.input.len()
    }

    pub fn current(&mut self) -> char {
        match self.input.chars().nth(self.index) {
            Some(ch) => ch,
            None => 0 as char
        }
    }

    pub fn peek(&mut self) -> char {
        let index_backup = self.index;
        self.next();
        let peeked = self.current();
        self.index = index_backup;
        peeked
    }

    pub fn position(&self) -> Position {
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

        assert_that!(sut.current(), is(equal_to(0 as char)));
        sut.next();
        assert_that!(sut.current(), is(equal_to(0 as char)));
    }

    #[test]
    fn current_for_empty_string() {
        let mut sut = crete_sut("");

        assert_that!(sut.current(), is(equal_to(0 as char)));
    }

    #[test]
    fn iterate_through_string() {
        let mut sut = crete_sut("Hello,\nWorld!");

        assert_that!(sut.current(), is(equal_to('H')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('e')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('l')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('l')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('o')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to(',')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('\n')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('W')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('o')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('r')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('l')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('d')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to('!')));
        assert_that!(sut.has_next(), is(equal_to(true)));
        sut.next();
        assert_that!(sut.current(), is(equal_to(0 as char)));
        assert_that!(sut.has_next(), is(equal_to(false)));
    }

    #[test]
    fn peek() {
        let mut sut = crete_sut("Hello");

        assert_that!(sut.peek(), is(equal_to('e')));
        assert_that!(sut.current(), is(equal_to('H')));
        sut.next();
        assert_that!(sut.current(), is(equal_to('e')));

        assert_that!(sut.peek(), is(equal_to('l')));
        assert_that!(sut.current(), is(equal_to('e')));
        sut.next();
        assert_that!(sut.current(), is(equal_to('l')));

        sut.next();

        assert_that!(sut.peek(), is(equal_to('o')));
        assert_that!(sut.current(), is(equal_to('l')));
        sut.next();
        assert_that!(sut.current(), is(equal_to('o')));
        assert_that!(sut.peek(), is(equal_to(0 as char)));
    }
}