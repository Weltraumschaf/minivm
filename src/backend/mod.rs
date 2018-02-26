///! Backend module of Mini VM.

/// Trait for a virtual machine.
pub trait VirtualMachine {
    /// Run a given program.
    fn run(self);
}