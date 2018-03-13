///! Backend module of Mini VM.
pub mod bytecode;
pub mod assembler;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::convert::TryFrom;

use backend::bytecode::Instruction;
use backend::bytecode::Error;

/// Architectural word size in bytes.
const WORD_SIZE: usize = 8;

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
    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let opcode = self.fetch();

            if opcode.is_err() {
                return Err(opcode.unwrap_err());
            }

            let instruction = self.decode(opcode.unwrap());

            if instruction.is_err() {
                return Err("Bad opcode!");
            }

            let instruction = instruction.unwrap();

            if instruction == Instruction::Halt {
                return Ok(());
            }

            self.execute(instruction);
        }
    }

    fn fetch(&self) -> Result<u8, &'static str> {
        self.code.fetch(self.instruction_pointer)
    }

    fn decode(&self, opcode: u8) -> Result<Instruction, Error> {
        Instruction::try_from(opcode)
    }

    fn execute(&mut self, instruction: Instruction) {
        self.inc_instruction_pointer();

        match instruction {
            Instruction::Nop => (),
            Instruction::IPush => {
                let result = self.code.fetch_integer(self.instruction_pointer);
            },
            Instruction::IStore => unimplemented!(),
            Instruction::ILoad => unimplemented!(),
            Instruction::IAdd => unimplemented!(),
            Instruction::ISub => unimplemented!(),
            Instruction::IMul => unimplemented!(),
            Instruction::IDiv => unimplemented!(),
            Instruction::IRem => unimplemented!(),
            Instruction::INeg => unimplemented!(),
            Instruction::Print => unimplemented!(),
            Instruction::Halt => panic!("The opcode 'halt' should exit the loop before execute!"),
        }
    }

    fn inc_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }
}

struct CodeMemory {
    byte_code: Vec<u8>,
}

impl CodeMemory {
    fn new(byte_code: Vec<u8>) -> CodeMemory {
        CodeMemory { byte_code }
    }

    fn fetch(&self, index: usize) -> Result<u8, &'static str> {
        if index < self.byte_code.len() {
            Ok(self.byte_code[index])
        } else {
            Err("Index out of bounds!")
        }
    }

    fn fetch_integer(&self, index: usize) -> Result<i64, &'static str> {
        let end_index = index + WORD_SIZE;

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