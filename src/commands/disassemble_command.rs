use std::path::Path;

use commands::Command;
use backend::assembler::Assembler;
use commands::read_file_as_bytes;

/// Command to translate byte code to assembly style code.
pub struct DisassembleCommand{
    file: String,
}

impl DisassembleCommand {
    pub fn new(file: String) -> DisassembleCommand {
        DisassembleCommand { file}
    }
}

impl Command for DisassembleCommand {
    fn execute(&self) {
        println!("{}:", &self.file);
        let byte_code :Vec<u8> = read_file_as_bytes(Path::new(&self.file));

        let assembler = Assembler::new();
        let asm = assembler.disassemble(byte_code);
        print!("{}", asm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
}
