use std::convert::TryFrom;
use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use backend::bytecode::Instruction;

/// Generates byte code from assembly style code.
pub struct Assembler;

impl Assembler {
    pub fn new() -> Assembler { Assembler }

    /// Generates the byte code from the assembly style code.
    pub fn assemble(&self, assembly_code: &str) -> Vec<u8> {
        let asm = read_string(assembly_code);
        let byte_code = translate(asm);
        byte_code
    }

    /// Generates assembly style code from byte code.
    pub fn disassemble(&self, byte_code: Vec<u8>) -> String {
        let mut buffer = String::new();
        let mut index = 0;

        while index < byte_code.len() {
            let opcode = byte_code[index];
            let opcode = Instruction::try_from(opcode).unwrap();
            buffer.push_str(&format!("{}", opcode));
            index += 1;

            match opcode {
                Instruction::IPush => {
                    buffer.push(' ');
                    let mut reader = Cursor::new(&byte_code[index..index + 8]);
                    let argument = reader.read_i64::<BigEndian>().unwrap();
                    buffer.push_str(&format!("{}", argument));
                    index += 8;
                },
                _ => (),
            }

            buffer.push('\n');
        }

        buffer
    }
}

fn read_string(assembly_code: &str) -> Vec<Vec<String>> {
    let mut buffer = Vec::new();

    for line in assembly_code.lines() {
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
    let mut buffer: Vec<u8> = Vec::new();

    for line in asm {
        if line.is_empty() {
            continue;
        }

        if let Some((mnemonic, arguments)) = line.split_first() {
            let mnemonic = mnemonic.parse::<Instruction>().unwrap();

            match mnemonic {
                Instruction::IPush => {
                    if arguments.len() != 1 {
                        panic!("Expecting exactly one argument for ipush!");
                    }

                    buffer.push(u8::from(mnemonic));
                    let int = arguments[0].replace("_", "")
                        .parse::<i64>()
                        .expect("Bad integer given!");
                    buffer.write_i64::<BigEndian>(int).unwrap();
                },
                Instruction::IAdd => {
                    if arguments.len() != 0 {
                        panic!("Expecting exactly zero arguments for iadd!");
                    }

                    buffer.push(u8::from(mnemonic));
                },
                Instruction::Print => {
                    if arguments.len() != 0 {
                        panic!("Expecting exactly zero arguments for print!");
                    }

                    buffer.push(u8::from(mnemonic));
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
    fn assemble() {
        let sut = Assembler::new();

        let byte_code = sut.assemble(r#"
// Simple adition of two integers.

ipush       1_000   // Push integer 1000 onto stack
ipush       100
iadd

// Print theresult from the stack.
print
"#);

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
                0x0b, // print
            ])));
    }

    #[test]
    fn read_string_some_content() {
        assert_that!(
            read_string(r#"
// Simple adition of two integers.

ipush       1_000   // Push integer 1000 onto stack
ipush       100
iadd

// Print theresult from the stack.
print
"#),
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
    fn disassemble() {
        let byte_code = vec![
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
            0x0b, // print
        ];

        let sut = Assembler::new();
        let assembly = sut.disassemble(byte_code);

        assert_that!(&assembly, is(equal_to(
r#"ipush 1000
ipush 100
iadd
print
"#
        )));
    }
}