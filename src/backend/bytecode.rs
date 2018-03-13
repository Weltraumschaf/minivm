use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;
use backend::BytecodeError;

// Inspired by https://rustbyexample.com/custom_types/enum/testcase_linked_list.html
pub enum List {
    /// Tuple struct that wraps an element and a pointer to the next node.
    Cons(Instruction, Box<List>),
    /// A node that signifies the end of the linked list.
    Nil,
}

impl List {
    pub fn new() -> List {
        List::Nil
    }

    pub fn prepend(self, elem: Instruction) -> List {
        List::Cons(elem, Box::new(self))
    }

    pub fn append(self, elem: Instruction) -> List {
        unimplemented!()
    }

    pub fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            List::Cons(ref head, ref tail) => format!("{:?}, {}", head, tail.stringify()),
            List::Nil => format!("Nil"),
        }
    }
}

/// Defines the available byte code instructions.
///
/// Inspired by some others:
///
/// * [Java Byte Code](https://en.wikipedia.org/wiki/Java_bytecode_instruction_listings)
/// * [Microsoft Common Intermediate Language](https://en.wikipedia.org/wiki/List_of_CIL_instructions)
/// * [Lua 5.1 VM Instructions](http://underpop.free.fr/l/lua/docs/a-no-frills-introduction-to-lua-5.1-vm-instructions.pdf)
/// * [Smalltalk-80 Blue Book](http://stephane.ducasse.free.fr/FreeBooks/BlueBook/Bluebook.pdf)
/// * [How to Build a Virtual Machine](https://www.youtube.com/watch?v=OjaAToVkoTw)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    /// Perform no operation.
    /// Stack (before -> after): [no change]
    /// Other bytes (count: operand labels): -
    Nop,
    /// Pushes eight bytes onto the stack as an integer value.
    /// Stack (before -> after): [] -> [value]
    /// Other bytes (count: operand labels): 8: byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8
    IPush,
    /// Store integer value into variable #index.
    /// Stack (before -> after): [value] -> []
    /// Other bytes (count: operand labels): 1: index
    IStore,
    /// Load an integer value from a local variable #index.
    /// Stack (before -> after): [] -> [value]
    /// Other bytes (count: operand labels): 1: index
    ILoad,
    /// Add two integers.
    /// Stack (before -> after): [value1, value2] -> [result]
    /// Other bytes (count: operand labels): -
    IAdd,
    /// Subtract two integers.
    /// Stack (before -> after): [value1, value2] -> [result]
    /// Other bytes (count: operand labels): -
    ISub,
    /// Multiply two integers.
    /// Stack (before -> after): [value1, value2] -> [result]
    /// Other bytes (count: operand labels): -
    IMul,
    /// Divide two integers.
    /// Stack (before -> after): [value1, value2] -> [result]
    /// Other bytes (count: operand labels): -
    IDiv,
    /// Remainder of two integers.
    /// Stack (before -> after): [value1, value2] -> [result]
    /// Other bytes (count: operand labels): -
    IRem,
    /// Negate integer.
    /// Stack (before -> after): [value] -> [result]
    /// Other bytes (count: operand labels): -
    INeg,
    /// Print the value on top of the stack.
    /// Stack (before -> after): [value] -> []
    /// Other bytes (count: operand labels): -
    Print,
    /// Stops the VM so no mire code will be executed.
    /// Stack (before -> after): [] -> []
    /// Other bytes (count: operand labels): -
    Halt,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::IPush => write!(f, "ipush"),
            Instruction::IStore => write!(f, "istore"),
            Instruction::ILoad => write!(f, "iload"),
            Instruction::IAdd => write!(f, "iadd"),
            Instruction::ISub => write!(f, "isub"),
            Instruction::IMul => write!(f, "imul"),
            Instruction::IDiv => write!(f, "idiv"),
            Instruction::IRem => write!(f, "irem"),
            Instruction::INeg => write!(f, "ineg"),
            Instruction::Print => write!(f, "print"),
            Instruction::Halt => write!(f, "halt"),
        }
    }
}

impl From<Instruction> for u8 {
    fn from(original: Instruction) -> u8 {
        match original {
            Instruction::Nop => 0x01,
            Instruction::IPush => 0x02,
            Instruction::IStore => 0x03,
            Instruction::ILoad => 0x04,
            Instruction::IAdd => 0x05,
            Instruction::ISub => 0x06,
            Instruction::IMul => 0x07,
            Instruction::IDiv => 0x08,
            Instruction::IRem => 0x09,
            Instruction::INeg => 0x0a,
            Instruction::Print => 0x0b,
            Instruction::Halt => 0x0c,
        }
    }
}

impl TryFrom<u8> for Instruction {
    type Error = BytecodeError;

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0x01 => Ok(Instruction::Nop),
            0x02 => Ok(Instruction::IPush),
            0x03 => Ok(Instruction::IStore),
            0x04 => Ok(Instruction::ILoad),
            0x05 => Ok(Instruction::IAdd),
            0x06 => Ok(Instruction::ISub),
            0x07 => Ok(Instruction::IMul),
            0x08 => Ok(Instruction::IDiv),
            0x09 => Ok(Instruction::IRem),
            0x0a => Ok(Instruction::INeg),
            0x0b => Ok(Instruction::Print),
            0x0c => Ok(Instruction::Halt),
            n => Err(BytecodeError::UnknownInstruction(n)),
        }
    }
}

impl FromStr for Instruction {
    type Err = BytecodeError;

    fn from_str(original: &str) -> Result<Self, Self::Err> {
        match original {
            "nop" => Ok(Instruction::Nop),
            "ipush" => Ok(Instruction::IPush),
            "istore" => Ok(Instruction::IStore),
            "iload" => Ok(Instruction::ILoad),
            "iadd" => Ok(Instruction::IAdd),
            "isub" => Ok(Instruction::ISub),
            "imul" => Ok(Instruction::IMul),
            "idiv" => Ok(Instruction::IDiv),
            "irem" => Ok(Instruction::IRem),
            "ineg" => Ok(Instruction::INeg),
            "print" => Ok(Instruction::Print),
            "halt" => Ok(Instruction::Halt),
            m => Err(BytecodeError::UnknownMnemonic(m.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn instruction_from_enum_to_u8() {
        assert_that!(u8::from(Instruction::Nop), is(equal_to(0x01)));
        assert_that!(u8::from(Instruction::IPush), is(equal_to(0x02)));
        assert_that!(u8::from(Instruction::IStore), is(equal_to(0x03)));
        assert_that!(u8::from(Instruction::ILoad), is(equal_to(0x04)));
        assert_that!(u8::from(Instruction::IAdd), is(equal_to(0x05)));
        assert_that!(u8::from(Instruction::ISub), is(equal_to(0x06)));
        assert_that!(u8::from(Instruction::IMul), is(equal_to(0x07)));
        assert_that!(u8::from(Instruction::IDiv), is(equal_to(0x08)));
        assert_that!(u8::from(Instruction::IRem), is(equal_to(0x09)));
        assert_that!(u8::from(Instruction::INeg), is(equal_to(0x0a)));
        assert_that!(u8::from(Instruction::Print), is(equal_to(0x0b)));
        assert_that!(u8::from(Instruction::Halt), is(equal_to(0x0c)));
    }

    #[test]
    fn instruction_try_from_u8_to_enum() {
        assert_that!(Instruction::try_from(0x01), is(equal_to(Ok(Instruction::Nop))));
        assert_that!(Instruction::try_from(0x02), is(equal_to(Ok(Instruction::IPush))));
        assert_that!(Instruction::try_from(0x03), is(equal_to(Ok(Instruction::IStore))));
        assert_that!(Instruction::try_from(0x04), is(equal_to(Ok(Instruction::ILoad))));
        assert_that!(Instruction::try_from(0x05), is(equal_to(Ok(Instruction::IAdd))));
        assert_that!(Instruction::try_from(0x06), is(equal_to(Ok(Instruction::ISub))));
        assert_that!(Instruction::try_from(0x07), is(equal_to(Ok(Instruction::IMul))));
        assert_that!(Instruction::try_from(0x08), is(equal_to(Ok(Instruction::IDiv))));
        assert_that!(Instruction::try_from(0x09), is(equal_to(Ok(Instruction::IRem))));
        assert_that!(Instruction::try_from(0x0a), is(equal_to(Ok(Instruction::INeg))));
        assert_that!(Instruction::try_from(0x0b), is(equal_to(Ok(Instruction::Print))));
        assert_that!(Instruction::try_from(0x0c), is(equal_to(Ok(Instruction::Halt))));
        assert_that!(Instruction::try_from(0x0d), is(equal_to(Err(BytecodeError::UnknownInstruction(0x0d)))));
    }

    #[test]
    fn instruction_try_from_string_to_enum() {
        assert_that!(Instruction::from_str("nop"), is(equal_to(Ok(Instruction::Nop))));
        assert_that!(Instruction::from_str("ipush"), is(equal_to(Ok(Instruction::IPush))));
        assert_that!(Instruction::from_str("istore"), is(equal_to(Ok(Instruction::IStore))));
        assert_that!(Instruction::from_str("iload"), is(equal_to(Ok(Instruction::ILoad))));
        assert_that!(Instruction::from_str("iadd"), is(equal_to(Ok(Instruction::IAdd))));
        assert_that!(Instruction::from_str("isub"), is(equal_to(Ok(Instruction::ISub))));
        assert_that!(Instruction::from_str("imul"), is(equal_to(Ok(Instruction::IMul))));
        assert_that!(Instruction::from_str("idiv"), is(equal_to(Ok(Instruction::IDiv))));
        assert_that!(Instruction::from_str("irem"), is(equal_to(Ok(Instruction::IRem))));
        assert_that!(Instruction::from_str("ineg"), is(equal_to(Ok(Instruction::INeg))));
        assert_that!(Instruction::from_str("print"), is(equal_to(Ok(Instruction::Print))));
        assert_that!(Instruction::from_str("halt"), is(equal_to(Ok(Instruction::Halt))));
        assert_that!(Instruction::from_str("foo"), is(equal_to(Err(BytecodeError::UnknownMnemonic(String::from("foo"))))));
    }

    #[test]
    fn instruction_fmt() {
        assert_that!(&format!("{}", Instruction::Nop), is(equal_to("nop")));
        assert_that!(&format!("{}", Instruction::IPush), is(equal_to("ipush")));
        assert_that!(&format!("{}", Instruction::IStore), is(equal_to("istore")));
        assert_that!(&format!("{}", Instruction::ILoad), is(equal_to("iload")));
        assert_that!(&format!("{}", Instruction::IAdd), is(equal_to("iadd")));
        assert_that!(&format!("{}", Instruction::ISub), is(equal_to("isub")));
        assert_that!(&format!("{}", Instruction::IMul), is(equal_to("imul")));
        assert_that!(&format!("{}", Instruction::IDiv), is(equal_to("idiv")));
        assert_that!(&format!("{}", Instruction::IRem), is(equal_to("irem")));
        assert_that!(&format!("{}", Instruction::INeg), is(equal_to("ineg")));
        assert_that!(&format!("{}", Instruction::Print), is(equal_to("print")));
        assert_that!(&format!("{}", Instruction::Halt), is(equal_to("halt")));
    }
}