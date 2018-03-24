use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

// https://stackoverflow.com/questions/29307474/how-can-i-convert-a-buffer-of-a-slice-of-bytes-u8-to-an-integer

/// Architectural word size in bytes.
pub const WORD_SIZE: usize = 8;

pub fn bytes_to_word(bytes: &[u8]) -> Result<u64, &'static str> {
    let mut reader = Cursor::new(bytes);

    match reader.read_u64::<BigEndian>() {
        Ok(val) => Ok(val),
        Err(_) => Err("Bad bytes to read u64 from!"),
    }
}

pub fn int_to_bytes(value: i64) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_i64::<BigEndian>(value).unwrap();
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn fetch_word() {
        let bytes = vec![0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83];

        assert_that!(bytes_to_word(&bytes), is(equal_to(Ok(918733457491587))));
    }
}
