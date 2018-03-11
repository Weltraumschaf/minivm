///! This module provides the various CLI command implementations.

mod assemble_command;
mod compile_command;
mod disassemble_command;
mod parse_command;
mod run_command;

pub use self::assemble_command::AssembleCommand;
pub use self::compile_command::CompileCommand;
pub use self::disassemble_command::DisassembleCommand;
pub use self::parse_command::ParserCommand;
pub use self::run_command::RunCommand;

/// Trait for a command to execute.
pub trait Command {
    /// Executes the command.
    fn execute(&self);
}
