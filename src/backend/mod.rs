///! Backend module of Mini VM.
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

pub mod bytecode;
pub mod assembler;

/// Trait for a virtual machine.
pub struct VirtualMachine {
    code: CodeMemory,
    instruction_pointer: usize,
}

impl VirtualMachine {
    pub fn new(byte_code: Vec<u8>) -> VirtualMachine {
        VirtualMachine {
            code: CodeMemory::new(byte_code),
            instruction_pointer: 0,
        }
    }

    /// Run a given program.
    pub fn run(&self) -> Result<(), &str> {
        loop {
            self.fetch();
            self.decode();
            self.execute();
        }
        Ok(())
    }

    fn fetch(&self) {}
    fn decode(&self) {}
    fn execute(&self) {}
}

struct CodeMemory {
    byte_code: Vec<u8>,
}

impl CodeMemory {
    pub fn new(byte_code: Vec<u8>) -> CodeMemory {
        CodeMemory { byte_code }
    }

    pub fn fetch(&self, index: usize) -> Result<u8, &str> {
        if index < self.byte_code.len() {
            Ok(self.byte_code[index])
        } else {
            Err("Index out of bounds!")
        }
    }

    pub fn fetch_integer(&self, index: usize) -> Result<i64, &str> {
        let end_index = index + 8;

        if end_index < self.byte_code.len() {
            let mut reader = Cursor::new(&self.byte_code[index..end_index]);
            match reader.read_i64::<BigEndian>() {
                Ok(val) => Ok(val),
                Err(_) => Err("Bad bytes to read i64 from!"),
            }
        } else {
            Err("Index out of bounds!")
        }
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