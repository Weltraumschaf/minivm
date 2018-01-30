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

    #[test]
    #[ignore]
    fn next() {
        let sut = CharacterStream::new(String::from("Hello,\nWorld!"));
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn has_next() {}

    #[test]
    #[ignore]
    fn peek() {}
}