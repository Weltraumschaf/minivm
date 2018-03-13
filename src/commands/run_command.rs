use commands::Command;
use commands::read_file_as_bytes;
use std::path::Path;

use backend::VirtualMachine;

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
        let byte_code = read_file_as_bytes(Path::new(&self.file));
        let vm = VirtualMachine::new(byte_code);
        let result = vm.run();
    }
}

