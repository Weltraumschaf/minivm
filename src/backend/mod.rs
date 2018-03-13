///! Backend module of Mini VM.

pub mod bytecode;
pub mod assembler;

#[derive(Debug, PartialEq)]
pub enum BytecodeError {
    UnknownInstruction(u8),
    UnknownMnemonic(String),
}

/// Trait for a virtual machine.
pub trait VirtualMachine {
    /// Run a given program.
    fn run(self);
}