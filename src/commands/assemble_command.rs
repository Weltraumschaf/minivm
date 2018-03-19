use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use commands::Command;
use backend::assembler::Assembler;

/// Command to translate assembly style code to byte code.
pub struct AssembleCommand {
    file: String,
}

impl AssembleCommand {
    pub fn new(file: String) -> AssembleCommand {
        AssembleCommand { file}
    }
}

impl Command for AssembleCommand {
    fn execute(&self) {
        let source_file = Path::new(&self.file);
        let base_file_name = source_file.file_stem().unwrap();
        let target_file = &format!("{}.mcode", base_file_name.to_str().unwrap());
        let target_file = Path::new(target_file);
        println!("Assemble file {:?} to {:?} ...", &source_file, &target_file);

        let mut input = File::open(source_file)
            .expect("Can't read source file!");
        let mut assembler_code = String::new();
        input.read_to_string(&mut assembler_code)
            .expect("Can't read assembler string!");

        let assembler = Assembler::new();
        let byte_code = assembler.assemble(&assembler_code);

        let mut output = File::create(target_file)
            .expect("Can't create target file!");
        output.write(&byte_code)
            .expect("Can't write target file!");
        let _ = output.flush();
    }
}

#[cfg(test)]
mod tests {
//    use super::*;
//    use hamcrest::prelude::*;
}