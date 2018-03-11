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
#[derive(Debug, Copy, Clone)]
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
    /// /// Stack (before -> after): [value] -> []
    /// Other bytes (count: operand labels): -
    Print,
}

impl Instruction {
    /// Returns the assembly mnemonic of this instruction.
    pub fn mnemonic(&self) -> String {
        format!("{:?}", self).to_ascii_lowercase()
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn mnemonic() {
        assert_that!(Instruction::IPush.mnemonic(), is(equal_to(String::from("ipush"))));
    }

    #[test]
    fn from_enum_to_u8() {
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
    }

    #[test]
    fn from_byte() {
//        assert_that!(Instruction::from_byte(0x07), is(uqeual_to(Instruction::IMul)));
    }
}