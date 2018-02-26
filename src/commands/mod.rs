///! This module provides the various CLI command implementations.

pub mod compile_command;
pub mod parse_command;
pub mod run_command;

/// Trait for a command to execute.
pub trait Command {
    /// Executes the command.
    fn execute(self);
}
