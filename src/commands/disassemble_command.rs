use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use commands::Command;
use backend::assembler::Assembler;

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
        let byte_code_file = Path::new(&self.file);
        let mut input = File::open(byte_code_file)
            .expect("Can't read byte code file!");
        let mut byte_code :Vec<u8> = Vec::new();
        input.read_to_end(&mut byte_code)
            .expect("Can't read byte code vector!");

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
