use commands::Command;

/// Command to compile to byte code.
pub struct CompileCommand {
    file: String,
}

impl CompileCommand {
    pub fn new(file: String) -> CompileCommand {
        CompileCommand { file }
    }
}

impl Command for CompileCommand {
    fn execute(self) {
        println!("Compile file {} ...", self.file);
    }
}