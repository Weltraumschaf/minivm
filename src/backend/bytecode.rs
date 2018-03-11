#![allow(dead_code)]

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
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    /// Perform no operation.
    /// Stack (before -> after): [no change]
    /// Other bytes (count: operand labels): -
    Nop = 0x01,
    /// Pushes eight bytes onto the stack as an integer value.
    /// Stack (before -> after): [] -> [value]
    /// Other bytes (count: operand labels): 8: byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8
    IPush = 0x02,
    /// Store integer value into variable #index.
    /// Stack (before -> after): [value] -> []
    /// Other bytes (count: operand labels): 1: index
    IStore = 0x03,
    /// Load an integer value from a local variable #index.
    /// Stack (before -> after): [] -> [value]
    /// Other bytes (count: operand labels): 1: index
    ILoad = 0x04,
    /// Add two integers.
    /// Stack (before -> after): [value1, value2] -> [result]
    /// Other bytes (count: operand labels): -
    IAdd = 0x05,
    /// Print the value on top of the stack.
    /// /// Stack (before -> after): [value] -> []
    /// Other bytes (count: operand labels): -
    Print = 0x06,
}