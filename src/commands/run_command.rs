use commands::Command;

/// Command to run byte code.
pub struct RunCommand {
    file: String,
}

impl RunCommand {
    pub fn new(file: String) -> RunCommand {
        RunCommand { file }
    }
}

impl Command for RunCommand {
    fn execute(&self) {
        println!("Execute file {} ...", self.file);
    }
}
