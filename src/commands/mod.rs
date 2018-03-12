///! This module provides the various CLI command implementations.

mod assemble_command;
mod compile_command;
mod disassemble_command;
mod parse_command;
mod run_command;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

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

/// This function reads a given file name into a byte vector.
pub fn read_file_as_bytes(file: &Path) -> Vec<u8> {
    let mut input = File::open(file)
        .expect("Can't read byte code file!");

    let mut byte_code: Vec<u8> = Vec::new();
    input.read_to_end(&mut byte_code)
        .expect("Can't read byte code vector!");

    byte_code
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    #[ignore]
    fn read_file_as_bytes_file_does_not_exist() {}

    #[test]
    fn read_file_as_bytes_file_is_empty() {
        let bytes = read_file_as_bytes(Path::new("test/read_file_as_bytes_empty"));

        assert_that!(&bytes, is(of_len(0)));
    }

    #[test]
    fn read_file_as_bytes_usual_file() {
        let bytes = read_file_as_bytes(Path::new("test/read_file_as_bytes"));

        assert_that!(bytes, is(equal_to(vec![0x68, 0x65, 0x6c,0x6c, 0x6f])));
    }
}
