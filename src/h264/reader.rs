use bitstream_io::{BigEndian, BitRead, BitReader};
use std::io::{Cursor, Read, Result, SeekFrom};

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

    pub fn read_bool(&mut self) -> Result<bool> {
        self.reader.read_bit()
    }

    pub fn read_u8(&mut self, bits: u8) -> Result<u8> {
        self.reader.read(bits as u32)
    }

    pub fn read_u16(&mut self, bits: u8) -> Result<u16> {
        self.reader.read(bits as u32)
    }

    pub fn peek_u16(&mut self, bits: u8) -> Result<u16> {
        let result = self.reader.read(bits as u32)?;
        self.reader.seek_bits(SeekFrom::Current(-(bits as i64)))?;
        Ok(result)
    }

    pub fn read_u32(&mut self, bits: u8) -> Result<u32> {
        self.reader.read(bits as u32)
    }

    pub fn peek_u32(&mut self, bits: u8) -> Result<u32> {
        let result = self.reader.read(bits as u32)?;
        self.reader.seek_bits(SeekFrom::Current(-(bits as i64)))?;
        Ok(result)
    }

    pub fn read_u64(&mut self, bits: u8) -> Result<u64> {
        self.reader.read(bits as u32)
    }

    pub fn position(&mut self) -> u64 {
        self.reader.position_in_bits().expect("position_in_bits() error")
    }

    pub fn skip(&mut self, bits: u32) -> Result<()> {
        self.reader.skip(bits)
    }

    pub fn read_till_one(&mut self) -> Result<u32> {
        self.reader.read_unary1()
    }

    pub fn remaining(&mut self) -> u64 {
        let mut reader = self.reader.clone();
        reader.seek_bits(SeekFrom::End(0)).expect("seek error");
        let pos = reader.position_in_bits().expect("position_in_bits() error");
        pos - self.position()
    }
}
