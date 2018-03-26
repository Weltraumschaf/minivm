use backend::byte_conversion::*;

/// Code memory holds the read only byte code to execute.
///
/// This struct is simply a wrapper around a long stream of bytes (`u8`).
pub struct CodeMemory {
    byte_code: Vec<u8>,
}

impl CodeMemory {
    pub fn new(byte_code: Vec<u8>) -> CodeMemory {
        CodeMemory { byte_code }
    }

    /// Fetches exactly one byte from the code memory at the given index.
    ///
    /// Returns an error result if the given index is beyond the number of available bytes.
    pub fn fetch(&self, index: usize) -> Result<u8, &'static str> {
        if index < self.byte_code.len() {
            Ok(self.byte_code[index])
        } else {
            Err("Index out of bounds!")
        }
    }

    /// Fetches a number of bytes from the code memory beginning from the given index.
    ///
    /// The number of bytes which will be fetched is defined in the `backend::byte_conversion::WORD_SIZE`
    /// constant. This method puts the number of bytes together as an long unsigned integer ('u64`)
    /// regardless of the the real underlying type. The decoding to the concrete type (float or
    /// integer or such) is done by the VM itself.
    ///
    /// Returns an error result if the given index is beyond the number of available bytes.
    pub fn fetch_word(&self, index: usize) -> Result<u64, &'static str> {
        let end_index = index + WORD_SIZE;

        if end_index <= self.byte_code.len() {
            let word = &self.byte_code[index..end_index];
            bytes_to_word(word)
        } else {
            Err("Index out of bounds!")
        }
    }
}

pub struct Stack {
    mem: Vec<u64>,
}

impl Stack {
    pub fn new() -> Stack { Stack { mem: Vec::new() } }

    pub fn push(&mut self, value: u64) {
        self.mem.push(value);
    }

    pub fn pop(&mut self) -> u64 {
        self.mem.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn code_memory_fetch() {
        let sut = CodeMemory::new(vec![0x01, 0x02, 0x03]);

        assert_that!(sut.fetch(0), is(equal_to(Ok(0x01))));
        assert_that!(sut.fetch(1), is(equal_to(Ok(0x02))));
        assert_that!(sut.fetch(2), is(equal_to(Ok(0x03))));
    }

    #[test]
    fn code_memory_fetch_out_of_bound() {
        let sut = CodeMemory::new(vec![0x01, 0x02, 0x03]);

        assert_that!(sut.fetch(3), is(equal_to(Err("Index out of bounds!"))));
    }

    #[test]
    fn code_memory_fetch_word() {
        let sut = CodeMemory::new(vec![0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83]);

        assert_that!(sut.fetch_word(0), is(equal_to(Ok(918733457491587))));
    }

    #[test]
    fn code_memory_fetch_word_out_of_bound() {
        let sut = CodeMemory::new(vec![0x01, 0x02, 0x03]);

        assert_that!(sut.fetch_word(3), is(equal_to(Err("Index out of bounds!"))));
    }

    #[test]
    fn stack_push_nd_pop() {
        let mut sut = Stack::new();
        sut.push(42);
        sut.push(23);

        assert_that!(sut.pop(), is(equal_to(23)));
        assert_that!(sut.pop(), is(equal_to(42)));
    }
}