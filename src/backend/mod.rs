///! Backend module of Mini VM.

pub mod bytecode;
pub mod assembler;

/// Trait for a virtual machine.
pub trait VirtualMachine {
    /// Run a given program.
    fn run(self);
}