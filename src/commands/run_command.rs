use commands::Command;
use commands::read_file_as_bytes;
use std::path::Path;

/// Command to run byte code.
pub struct RunCommand {
    file: String,
}

impl RunCommand {
    pub fn new(file: String) -> RunCommand {
        RunCommand { file }
    }
}

impl Command for RunCommand {
    fn execute(&self) {
        println!("Execute file {} ...", self.file);
        let byte_code = read_file_as_bytes(Path::new(&self.file));
    }
}

struct CodeMemory {
    byte_code: Vec<u8>,
}

impl CodeMemory {
    pub fn new(byte_code: Vec<u8>) -> CodeMemory {
        CodeMemory { byte_code }
    }

    pub fn fetch(&self, index: usize) -> Result<u8, &str> {
        let len = self.byte_code.len();

        if index < len {
            Ok(self.byte_code[index])
        } else {
            Err("Index out of bounds!")
        }
    }

    pub fn fetch_integer(&self, index: usize) -> Result<i64, &str> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn fetch() {
        let sut = CodeMemory::new(vec![0x01, 0x02, 0x03]);

        assert_that!(sut.fetch(0), is(equal_to(Ok(0x01))));
        assert_that!(sut.fetch(1), is(equal_to(Ok(0x02))));
        assert_that!(sut.fetch(2), is(equal_to(Ok(0x03))));
    }

    #[test]
    fn fetch_out_of_bound() {
        let sut = CodeMemory::new(vec![0x01, 0x02, 0x03]);

        assert_that!(sut.fetch(3), is(equal_to(Err("Index out of bounds!"))));
    }

    #[test]
    fn fetch_integer() {}
}
