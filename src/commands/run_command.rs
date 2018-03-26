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
        let byte_code = read_file_as_bytes(Path::new(&self.file));
        let mut vm = VirtualMachine::new(byte_code);
        let _ = vm.run();
    }
}

