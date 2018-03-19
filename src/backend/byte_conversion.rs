// https://stackoverflow.com/questions/29307474/how-can-i-convert-a-buffer-of-a-slice-of-bytes-u8-to-an-integer

/// Architectural word size in bytes.
pub const WORD_SIZE: usize = 8;

pub fn bytes_to_word(bytes: [u8;8]) -> u64 {
    unimplemented!();
}

pub fn word_to_integer(word: u64) -> i64 {
    unimplemented!();
}

pub fn integer_to_word(value: i64) -> u64 {
    unimplemented!();
}

