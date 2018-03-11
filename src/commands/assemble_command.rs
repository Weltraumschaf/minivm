use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use byteorder::{BigEndian, WriteBytesExt};
use commands::Command;
use backend::bytecode::Instruction;

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
        let base_file_name = Path::new(&self.file).file_stem().unwrap();
        let target = format!("{}.mcode", base_file_name.to_str().unwrap());
        println!("Assemble file {} to {} ...", &self.file, target);
        let asm = read_file(&self.file);
        let byte_code = translate(asm);

        let mut buffer = File::create(target).unwrap();
        buffer.write(&byte_code).unwrap();
        buffer.flush();
    }
}

fn read_file(file_name: &str) -> Vec<Vec<String>> {
    let file = File::open(file_name).expect(&format!("File not found: {}!", file_name));
    let reader = BufReader::new(&file);
    let mut buffer = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Can't read line!");

        if line.starts_with("//") {
            continue;
        }

        let trimmed = remove_trailing_comment(&line);

        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<String> = split_line(&trimmed);

        buffer.push(parts);
    }

    buffer
}

fn remove_trailing_comment(line: &str) -> String {
    match line.find("//") {
        Some(pos) => {
            let (code, _) = line.split_at(pos);
            code
        },
        None => line,
    }.trim().to_owned()
}

fn split_line(line: &str) -> Vec<String> {
    line.split(' ')
        .filter(|&x| x != "")
        .map(|x| x.to_owned())
        .collect()
}

fn translate(asm: Vec<Vec<String>>) -> Vec<u8> {
    let mut buffer = Vec::new();

    for line in asm {
        if line.is_empty() {
            continue;
        }

        if let Some((mnemonic, arguments)) = line.split_first() {
            match mnemonic.as_str() {
                "ipush" => {
                    if arguments.len() != 1 {
                        panic!("Expecting exactly one argument for ipush!");
                    }

                    buffer.push(Instruction::IPush as u8);
                    let int = arguments[0].replace("_", "")
                        .parse::<i64>()
                        .expect("Bad integer given!");
                    buffer.write_i64::<BigEndian>(int).unwrap();
                },
                "iadd" => {
                    if arguments.len() != 0 {
                        panic!("Expecting exactly zero arguments for iadd!");
                    }

                    buffer.push(Instruction::IAdd as u8);
                },
                "print" => {
                    if arguments.len() != 0 {
                        panic!("Expecting exactly zero arguments for print!");
                    }

                    buffer.push(Instruction::Print as u8);
                },
                _ => panic!(format!("Unrecognized mnemonic '{}'!", mnemonic)),
            }
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn read_file_some_content() {
        assert_that!(
            read_file("test/asm_command.masm"),
            is(equal_to(vec![
                vec![String::from("ipush"), String::from("1_000")],
                vec![String::from("ipush"), String::from("100")],
                vec![String::from("iadd")],
                vec![String::from("print")]
            ]))
        );
    }

    #[test]
    fn remove_trailing_comment_no_comment_to_remove() {
        assert_that!(
            remove_trailing_comment("iadd"),
            is(equal_to(String::from("iadd"))));
    }

    #[test]
    fn remove_trailing_comment_comment_to_remove() {
        assert_that!(
            remove_trailing_comment("iadd  // some comments // foo"),
            is(equal_to(String::from("iadd"))));
    }

    #[test]
    fn split_line_no_args() {
        assert_that!(
            split_line("foo"),
            is(equal_to(vec![String::from("foo")])));
    }

    #[test]
    fn split_line_one_arg() {
        assert_that!(
            split_line("foo   bar"),
            is(equal_to(vec![String::from("foo"), String::from("bar")])));
    }

    #[test]
    fn split_line_three_args() {
        assert_that!(
            split_line("foo   bar baz snafu"),
            is(equal_to(vec![String::from("foo"), String::from("bar"), String::from("baz"), String::from("snafu")])));
    }

    #[test]
    fn test_translate() {
        let asm = read_file("test/asm_command.masm");
        let byte_code = translate(asm);

        assert_that!(
            byte_code,
            is(equal_to(vec![
                0x02, // ipush
                0x00, // 1000
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x03,
                0xe8,
                0x02, // ipush
                0x00, // 100
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x64,
                0x05, // iadd
                0x06, // print
            ])));
    }
}