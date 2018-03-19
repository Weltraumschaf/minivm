///! Backend module of Mini VM.
pub mod bytecode;
pub mod byte_conversion;
pub mod assembler;
pub mod memory;

use std::convert::TryFrom;

use backend::bytecode::{Instruction, BytecodeError};
use backend::byte_conversion::*;
use backend::memory::{CodeMemory, Stack};

/// Trait for a virtual machine.
pub struct VirtualMachine {
    code: CodeMemory,
    stack: Stack,
    instruction_pointer: usize,
}

impl VirtualMachine {
    pub fn new(byte_code: Vec<u8>) -> VirtualMachine {
        VirtualMachine {
            code: CodeMemory::new(byte_code),
            stack: Stack::new(),
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

    fn decode(&self, opcode: u8) -> Result<Instruction, BytecodeError> {
        Instruction::try_from(opcode)
    }

    fn execute(&mut self, instruction: Instruction) {
        self.inc_instruction_pointer();

        match instruction {
            Instruction::Nop => (),
            Instruction::IPush => {
                let result = self.code.fetch_word(self.instruction_pointer);

                match result {
                    Ok(value) => self.stack.push(value),
                    Err(msg) => panic!("Error: {}", msg),
                }

                self.inc_instruction_pointer_word();
            },
            Instruction::IStore => unimplemented!(),
            Instruction::ILoad => unimplemented!(),
            Instruction::IAdd => {
                let first_operand = self.stack.pop();
                let second_operand = self.stack.pop();
                let first_operand = word_to_integer(first_operand);
                let second_operand = word_to_integer(second_operand);
                let result = first_operand + second_operand;
                let result = integer_to_word(result);
                self.stack.push(result);
            },
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

    fn inc_instruction_pointer_word(&mut self) {
        self.instruction_pointer += WORD_SIZE;
    }
}

#[cfg(test)]
mod tests {
//    use super::*;
//    use hamcrest::prelude::*;
}