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

pub fn bytes_to_int(bytes: &[u8]) -> Result<i64, &'static str> {
    let mut reader = Cursor::new(bytes);

    match reader.read_i64::<BigEndian>() {
        Ok(val) => Ok(val),
        Err(_) => Err("Bad bytes to read u64 from!"),
    }
}

pub fn bytes_to_float(bytes: &[u8]) -> Result<f64, &'static str> {
    let mut reader = Cursor::new(bytes);

    match reader.read_f64::<BigEndian>() {
        Ok(val) => Ok(val),
        Err(_) => Err("Bad bytes to read u64 from!"),
    }
}

pub fn int_to_bytes(value: i64) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_i64::<BigEndian>(value).unwrap();
    buffer
}

pub fn float_to_bytes(value: f64) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_f64::<BigEndian>(value).unwrap();
    buffer
}

pub fn word_to_int(word: u64) -> i64 {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_u64::<BigEndian>(word).unwrap();
    bytes_to_int(&buffer).unwrap()
}

pub fn word_to_float(word: u64) -> f64 {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_u64::<BigEndian>(word).unwrap();
    bytes_to_float(&buffer).unwrap()
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

    #[test]
    fn positive_integer() {
        let an_int: i64 = 918_733_457_491_587;
        let expected_bytes = vec![0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83];
        let mut bytes = int_to_bytes(an_int);

        assert_that!(&bytes, is(equal_to(&expected_bytes)));

        let word = bytes_to_word(&bytes).unwrap();
        assert_that!(word, is(equal_to(918_733_457_491_587)));

        assert_that!(word_to_int(word), is(equal_to(an_int)));
    }

    #[test]
    fn negative_integer() {
        let an_int: i64 = -918_733_457_491_587;
        let expected_bytes = vec![0xFF, 0xFC, 0xBC, 0x6A, 0xB2, 0x9F, 0x79, 0x7D];
        let mut bytes = int_to_bytes(an_int);

        assert_that!(&bytes, is(equal_to(&expected_bytes)));

        let word = bytes_to_word(&bytes).unwrap();
        assert_that!(word, is(equal_to(18_445_825_340_252_060_029)));

        assert_that!(word_to_int(word), is(equal_to(an_int)));
    }

    #[test]
    fn positive_float() {
        let an_float: f64 = 918_733_457_491_587.012;
        let expected_bytes = vec![67, 10, 28, 170, 107, 4, 52, 24];
        let mut bytes = float_to_bytes(an_float);

        assert_that!(&bytes, is(equal_to(&expected_bytes)));

        let word = bytes_to_word(&bytes).unwrap();
        assert_that!(word, is(equal_to(4830705068573733912)));

        assert_that!(word_to_float(word), is(equal_to(an_float)));
    }

    #[test]
    fn negative_float() {
        let an_float: f64 = -918_733_457_491_587.012;
        let expected_bytes = vec![195, 10, 28, 170, 107, 4, 52, 24];
        let mut bytes = float_to_bytes(an_float);

        assert_that!(&bytes, is(equal_to(&expected_bytes)));

        let word = bytes_to_word(&bytes).unwrap();
        assert_that!(word, is(equal_to(14_054_077_105_428_509_720)));

        assert_that!(word_to_float(word), is(equal_to(an_float)));
    }
}
