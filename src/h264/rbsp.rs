use bitstream_io::{BigEndian, BitRead, BitReader};
use std::io::{self, Cursor, SeekFrom};

pub type ParseResult<T> = std::result::Result<T, String>;

fn map_io_error(err: io::Error) -> String {
    format!("IO error: {}", err)
}

#[derive(Clone)]
pub struct RbspReader<'a> {
    reader: BitReader<Cursor<&'a [u8]>, BigEndian>,
}

impl<'a> RbspReader<'a> {
    pub fn new(data: &'a [u8]) -> RbspReader<'a> {
        let cursor = Cursor::new(data);
        RbspReader { reader: BitReader::new(cursor) }
    }

    pub fn align(&mut self) {
        self.reader.byte_align();
    }

    pub fn is_aligned(&self) -> bool {
        self.reader.byte_aligned()
    }

    pub fn f(&mut self) -> ParseResult<bool> {
        self.reader.read_bit().map_err(map_io_error)
    }

    pub fn u(&mut self, bits: u8) -> ParseResult<u32> {
        self.reader.read(bits as u32).map_err(map_io_error)
    }

    // Parsing process for Exp-Golomb codes. Section 9.1
    pub fn ue(&mut self, n: u8) -> ParseResult<u32> {
        if n > 32 {
            return Err(format!("ue(): too many ({}) bits requested", n));
        }

        let error_handler = |e: io::Error| e.to_string();
        let zero_bits = self.read_till_one()?;
        if zero_bits > n as u32 {
            return Err(format!("ue(): too many ({}) leading zeros", zero_bits));
        }

        let x: u64 = self.reader.read(zero_bits).map_err(error_handler)?;
        let result = (1u64 << zero_bits) - 1 + x;
        if result >= 1u64 << n {
            return Err(format!("ue(): value ({}) is too large to fit the variable", result));
        }
        Ok(result as u32)
    }

    // Mapping process for signed Exp-Golomb codes Section 9.1.1
    pub fn se(&mut self) -> ParseResult<i32> {
        let value = self.ue(32)?;
        let result =
            if value & 1 != 0 { ((value >> 1) + 1) as i32 } else { -((value >> 1) as i32) };
        Ok(result)
    }

    pub fn position(&mut self) -> u64 {
        self.reader.position_in_bits().expect("position_in_bits() error")
    }

    pub fn skip(&mut self, bits: u32) -> ParseResult<()> {
        self.reader.skip(bits).map_err(map_io_error)
    }

    pub fn go_back(&mut self, bits: i64) -> ParseResult<u64> {
        self.reader.seek_bits(SeekFrom::Current(-bits)).map_err(map_io_error)
    }

    pub fn read_till_one(&mut self) -> ParseResult<u32> {
        self.reader.read_unary1().map_err(map_io_error)
    }

    pub fn remaining(&mut self) -> u64 {
        let mut reader = self.reader.clone();
        reader.seek_bits(SeekFrom::End(0)).expect("seek error");
        let pos = reader.position_in_bits().expect("position_in_bits() error");
        pos - self.position()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn reader(bytes: &[u8]) -> RbspReader {
        RbspReader::new(bytes)
    }

    #[test]
    pub fn test_ue() {
        assert_eq!(0, reader(&[0b10000000]).ue(8).unwrap());
        assert_eq!(1, reader(&[0b01000000]).ue(8).unwrap());
        assert_eq!(2, reader(&[0b01100000]).ue(8).unwrap());
        assert_eq!(3, reader(&[0b00100000]).ue(8).unwrap());
        assert_eq!(4, reader(&[0b00101000]).ue(8).unwrap());
        assert_eq!(5, reader(&[0b00110000]).ue(8).unwrap());
        assert_eq!(6, reader(&[0b00111000]).ue(8).unwrap());
        assert_eq!(7, reader(&[0b00010000]).ue(8).unwrap());
        assert_eq!(8, reader(&[0b00010010]).ue(8).unwrap());
        assert_eq!(9, reader(&[0b00010100]).ue(8).unwrap());
        assert_eq!(255, reader(&[0b00000000, 0b10000000, 0]).ue(8).unwrap());
        assert_eq!(
            u32::MAX,
            reader(&[
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b10000000, 0b00000000, 0b00000000,
                0b00000000, 0
            ])
            .ue(32)
            .unwrap()
        );
    }

    #[test]
    pub fn test_se() {
        assert_eq!(0, reader(&[0b10000000]).se().unwrap());
        assert_eq!(1, reader(&[0b01000000]).se().unwrap());
        assert_eq!(-1, reader(&[0b01100000]).se().unwrap());
        assert_eq!(2, reader(&[0b00100000]).se().unwrap());
        assert_eq!(-2, reader(&[0b00101000]).se().unwrap());
        assert_eq!(3, reader(&[0b00110000]).se().unwrap());
        assert_eq!(-3, reader(&[0b00111000]).se().unwrap());
        assert_eq!(4, reader(&[0b00010000]).se().unwrap());
        assert_eq!(-4, reader(&[0b00010010]).se().unwrap());
        assert_eq!(5, reader(&[0b00010100]).se().unwrap());
    }
}
