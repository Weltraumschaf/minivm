#[derive(Debug)]
struct CharacterStream {
    input: String,
    index: u64,
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

    fn next(&self) -> char {
        panic!("Not implemented!");
    }

    fn has_next(&self) -> bool {
        false
    }

    fn peek(&self) -> char {
        panic!("Not implemented!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    #[ignore]
    fn next() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn has_next() {
        let sut = CharacterStream::new(String::from("Hello,\nWorld!"));
        assert_that!(1, is(equal_to(1)));
//        assert_that(sut.has_next(), is(false));
    }

    #[test]
    #[ignore]
    fn peek() {}
}