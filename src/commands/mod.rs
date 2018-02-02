pub mod compile_command;
pub mod parse_command;
pub mod run_command;

pub trait Command {
    fn execute(self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
