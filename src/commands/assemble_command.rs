use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use commands::Command;

/// Command to assemble to byte code.
pub struct AssembleCommand {
    file: String,
}

impl AssembleCommand {
    pub fn new(file: String) -> AssembleCommand {
        AssembleCommand { file }
    }
}

impl Command for AssembleCommand {
    fn execute(self) {
        unimplemented!();
//        let base_file_name = Path::new(&self.file).file_stem().unwrap();
//        let target = format!("{}.mcode", base_file_name.to_str().unwrap());
//        println!("Assemble file {} to {} ...", &self.file, target);
//        let asm = read_file(&self.file);
//        let byte_code = translate(asm);
//
//        let mut buffer = File::create(target).unwrap();
//        buffer.write(&byte_code).unwrap();
//        buffer.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

}