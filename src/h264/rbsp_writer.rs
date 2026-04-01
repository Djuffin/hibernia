use bitstream_io::{BigEndian, BitWrite, BitWriter};
use std::io;

pub type WriteResult = std::result::Result<(), String>;

fn map_io_error(err: io::Error) -> String {
    format!("IO error: {}", err)
}

pub struct RbspWriter {
    writer: BitWriter<Vec<u8>, BigEndian>,
}

impl Default for RbspWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl RbspWriter {
    pub fn new() -> RbspWriter {
        RbspWriter {
            writer: BitWriter::new(Vec::new()),
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.writer.into_writer()
    }

    pub fn align(&mut self) -> WriteResult {
        self.writer.byte_align().map_err(map_io_error)
    }

    pub fn is_aligned(&self) -> bool {
        self.writer.byte_aligned()
    }

    pub fn f(&mut self, value: bool) -> WriteResult {
        self.writer.write_bit(value).map_err(map_io_error)
    }

    pub fn u(&mut self, bits: u8, value: u32) -> WriteResult {
        self.writer.write(bits as u32, value).map_err(map_io_error)
    }

    pub fn ue(&mut self, value: u32) -> WriteResult {
        let x = (value as u64) + 1;
        let length = 63 - x.leading_zeros();
        for _ in 0..length {
            self.writer.write_bit(false).map_err(map_io_error)?;
        }
        self.writer.write(length + 1, x).map_err(map_io_error)
    }

    pub fn se(&mut self, value: i32) -> WriteResult {
        let value_i64 = value as i64;
        let mapped = if value_i64 <= 0 {
            (-2 * value_i64) as u32
        } else {
            (2 * value_i64 - 1) as u32
        };
        self.ue(mapped)
    }

    pub fn te(&mut self, range_max: u32, value: u32) -> WriteResult {
        if range_max > 1 {
            self.ue(value)
        } else {
            self.f(value == 0)
        }
    }

    pub fn rbsp_trailing_bits(&mut self) -> WriteResult {
        self.f(true)?;
        self.align()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ue() {
        let mut writer = RbspWriter::new();
        writer.ue(0).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b10000000]);

        let mut writer = RbspWriter::new();
        writer.ue(1).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b01000000]);

        let mut writer = RbspWriter::new();
        writer.ue(2).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b01100000]);

        let mut writer = RbspWriter::new();
        writer.ue(3).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00100000]);

        let mut writer = RbspWriter::new();
        writer.ue(4).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00101000]);

        let mut writer = RbspWriter::new();
        writer.ue(5).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00110000]);

        let mut writer = RbspWriter::new();
        writer.ue(6).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00111000]);

        let mut writer = RbspWriter::new();
        writer.ue(7).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00010000]);

        let mut writer = RbspWriter::new();
        writer.ue(8).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00010010]);

        let mut writer = RbspWriter::new();
        writer.ue(9).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00010100]);

        let mut writer = RbspWriter::new();
        writer.ue(255).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00000000, 0b10000000, 0]);

        let mut writer = RbspWriter::new();
        writer.ue(u32::MAX).unwrap();
        writer.align().unwrap();
        assert_eq!(
            writer.into_inner(),
            vec![
                0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b10000000, 0b00000000, 0b00000000,
                0b00000000, 0
            ]
        );
    }

    #[test]
    pub fn test_se() {
        let mut writer = RbspWriter::new();
        writer.se(0).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b10000000]);

        let mut writer = RbspWriter::new();
        writer.se(1).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b01000000]);

        let mut writer = RbspWriter::new();
        writer.se(-1).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b01100000]);

        let mut writer = RbspWriter::new();
        writer.se(2).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00100000]);

        let mut writer = RbspWriter::new();
        writer.se(-2).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00101000]);

        let mut writer = RbspWriter::new();
        writer.se(3).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00110000]);

        let mut writer = RbspWriter::new();
        writer.se(-3).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00111000]);

        let mut writer = RbspWriter::new();
        writer.se(4).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00010000]);

        let mut writer = RbspWriter::new();
        writer.se(-4).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00010010]);

        let mut writer = RbspWriter::new();
        writer.se(5).unwrap();
        writer.align().unwrap();
        assert_eq!(writer.into_inner(), vec![0b00010100]);
    }

    #[test]
    pub fn test_writer_reader_roundtrip() {
        use super::super::rbsp::RbspReader;
        let mut writer = RbspWriter::new();
        writer.u(8, 0x55).unwrap();
        writer.f(true).unwrap();
        writer.f(false).unwrap();
        writer.f(true).unwrap();
        writer.f(true).unwrap();
        writer.u(4, 0xA).unwrap();
        writer.ue(12345).unwrap();
        writer.se(-54321).unwrap();
        writer.te(5, 3).unwrap();
        writer.te(1, 0).unwrap();
        writer.rbsp_trailing_bits().unwrap();

        let data = writer.into_inner();
        let mut reader = RbspReader::new(&data);

        assert_eq!(reader.u(8).unwrap(), 0x55);
        assert_eq!(reader.f().unwrap(), true);
        assert_eq!(reader.f().unwrap(), false);
        assert_eq!(reader.f().unwrap(), true);
        assert_eq!(reader.f().unwrap(), true);
        assert_eq!(reader.u(4).unwrap(), 0xA);
        assert_eq!(reader.ue(32).unwrap(), 12345);
        assert_eq!(reader.se().unwrap(), -54321);
        assert_eq!(reader.te(5).unwrap(), 3);
        assert_eq!(reader.te(1).unwrap(), 0);
        
        assert_eq!(reader.f().unwrap(), true);
        reader.align();
        assert_eq!(reader.is_aligned(), true);
    }
}
