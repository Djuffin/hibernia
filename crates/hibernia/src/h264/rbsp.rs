use bitstream_io::{BigEndian, BitRead, BitReader};
use std::io::{self, Cursor, SeekFrom};

pub type ParseResult<T> = std::result::Result<T, String>;

fn map_io_error(err: io::Error) -> String {
    format!("IO error: {}", err)
}

#[derive(Clone)]
pub struct RbspReader<'a> {
    reader: BitReader<Cursor<&'a [u8]>, BigEndian>,
    /// Length of the RBSP in bits.
    len_bits: u64,
    /// Bit position of the last bit equal to 1 in the RBSP, or `None` if every
    /// bit is 0. In a well-formed RBSP this is the `rbsp_stop_one_bit` (7.2).
    last_set_bit: Option<u64>,
}

impl<'a> RbspReader<'a> {
    pub fn new(data: &'a [u8]) -> RbspReader<'a> {
        let cursor = Cursor::new(data);
        RbspReader {
            reader: BitReader::new(cursor),
            len_bits: data.len() as u64 * 8,
            last_set_bit: last_set_bit(data),
        }
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
        // Bound to ue(31) so that the unsigned value fits in [0, 2^31 - 1].
        // After mapping, the signed result fits in i32 without overflow:
        // ue(32) could yield u32::MAX, where ((MAX >> 1) + 1) as i32 silently
        // produces i32::MIN.
        let value = self.ue(31)?;
        let result =
            if value & 1 != 0 { ((value >> 1) + 1) as i32 } else { -((value >> 1) as i32) };
        Ok(result)
    }

    // Truncated Exp-Golomb entropy coding. Section 9.1
    pub fn te(&mut self, range_max: u32) -> ParseResult<u32> {
        if range_max > 1 {
            self.ue(32)
        } else {
            let bit = self.f()?;
            Ok(if bit { 0 } else { 1 })
        }
    }

    pub fn peek_or_pad16(&mut self) -> ParseResult<u16> {
        let mut tmp_reader = self.reader.clone();
        if self.remaining() >= 16 {
            let result = tmp_reader.read(16).map_err(map_io_error)?;
            Ok(result)
        } else {
            let mut result = 0;
            let mut bits_read = 0;
            loop {
                match tmp_reader.read_bit() {
                    Ok(true) => {
                        result = (result << 1) | 1;
                    }
                    Ok(false) => {
                        result <<= 1;
                    }
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                        if bits_read == 0 {
                            return Err(map_io_error(e));
                        }
                        break;
                    }
                    Err(e) => {
                        return Err(map_io_error(e));
                    }
                }
                bits_read += 1;
            }
            result <<= 16 - bits_read;
            Ok(result)
        }
    }

    pub fn position(&mut self) -> u64 {
        self.reader.position_in_bits().expect("position_in_bits() error")
    }

    pub fn skip(&mut self, bits: u32) -> ParseResult<()> {
        self.reader.skip(bits).map_err(map_io_error)
    }

    /// Rewind the reader by `bits` bit positions. Used by CABAC to sync
    /// the underlying reader back to the logically-consumed position after
    /// pre-fetching bits into an internal buffer.
    pub fn rewind(&mut self, bits: u32) -> ParseResult<()> {
        self.reader
            .seek_bits(SeekFrom::Current(-(bits as i64)))
            .map_err(map_io_error)?;
        Ok(())
    }

    pub fn read_till_one(&mut self) -> ParseResult<u32> {
        self.reader.read_unary1().map_err(map_io_error)
    }

    pub fn remaining(&mut self) -> u64 {
        self.len_bits - self.position()
    }

    /// Section 7.2 `more_rbsp_data()`: false if no bits remain or if the next
    /// bit is the last bit equal to 1 in the RBSP (the `rbsp_stop_one_bit`);
    /// true otherwise. An RBSP that ends in zero bits, which is malformed,
    /// therefore reports more data at any position past its last 1 bit.
    pub fn more_rbsp_data(&mut self) -> bool {
        let pos = self.position();
        pos < self.len_bits && Some(pos) != self.last_set_bit
    }
}

/// Bit position of the last bit equal to 1 in `data`, counting bits
/// MSB-first from the start of the buffer; `None` if every bit is 0.
fn last_set_bit(data: &[u8]) -> Option<u64> {
    let idx = data.iter().rposition(|&b| b != 0)?;
    Some(idx as u64 * 8 + 7 - u64::from(data[idx].trailing_zeros()))
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn reader(bytes: &[u8]) -> RbspReader<'_> {
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

    #[test]
    pub fn test_peek_or_pad16() {
        let mut r = reader(&[0b11100111, 0b11100011]);
        assert_eq!(r.peek_or_pad16().unwrap(), 0b11100111_11100011);

        let mut r = reader(&[0b11100111]);
        assert_eq!(r.remaining(), 8);
        assert_eq!(r.peek_or_pad16().unwrap(), 0b11100111_00000000);
        assert_eq!(r.remaining(), 8);

        let mut r = reader(&[0b11100111]);
        assert_eq!(r.u(7).unwrap(), 0b1110011);
        assert_eq!(r.remaining(), 1);
        assert_eq!(r.peek_or_pad16().unwrap(), 0b10000000_00000000);
        assert_eq!(r.remaining(), 1);

        let mut r = reader(&[]);
        assert!(r.peek_or_pad16().is_err());
    }

    /// The original `remaining()`: seek a copy of the reader to the end.
    fn remaining_reference(r: &RbspReader) -> u64 {
        let end = r.reader.clone().seek_bits(SeekFrom::End(0)).expect("seek");
        end - r.clone().position()
    }

    /// The original `more_rbsp_data()` from parser.rs, kept as a reference: on
    /// a copy of the reader, try to parse `rbsp_trailing_bits()` (7.3.2.11) and
    /// then look for any later non-zero byte.
    fn more_rbsp_data_reference(r: &RbspReader) -> bool {
        let mut tmp = r.clone();
        if remaining_reference(&tmp) == 0 {
            return false;
        }
        // rbsp_stop_one_bit, then rbsp_alignment_zero_bit up to byte alignment.
        if tmp.u(1) != Ok(1) {
            return true;
        }
        while !tmp.is_aligned() {
            if tmp.u(1) != Ok(0) {
                return true;
            }
        }
        loop {
            match tmp.u(8) {
                Ok(value) if value > 0 => return true,
                Ok(_) => {}
                Err(_) => return false,
            }
        }
    }

    /// Deterministic xorshift64 bytes, so the test needs no RNG dependency.
    fn pseudo_random_bytes(state: &mut u64, len: usize) -> Vec<u8> {
        (0..len)
            .map(|_| {
                *state ^= *state << 13;
                *state ^= *state >> 7;
                *state ^= *state << 17;
                state.to_be_bytes()[0]
            })
            .collect()
    }

    fn more_rbsp_data_at(buf: &[u8], pos: u32) -> bool {
        let mut r = reader(buf);
        r.skip(pos).expect("skip");
        r.more_rbsp_data()
    }

    #[test]
    pub fn test_more_rbsp_data_matches_reference_at_every_bit() {
        let mut buffers: Vec<Vec<u8>> = vec![
            vec![],
            vec![0x80],
            vec![0x01],
            vec![0xFF],
            vec![0x00],
            vec![0x00; 5],
            vec![0xA5, 0x80],
            // Stop bit followed by cabac_zero_words (7.3.4).
            vec![0x12, 0x34, 0x80, 0x00, 0x00, 0x00, 0x00],
            vec![0x5C, 0x40, 0x00, 0x00],
            vec![0x80, 0x00, 0x80],
            vec![0x00, 0x00, 0x01],
        ];
        let mut state = 0x2545_F491_4F6C_DD1D;
        for len in 1..=32 {
            let mut buf = pseudo_random_bytes(&mut state, len);
            buffers.push(buf.clone());
            // The same bytes with a zero tail.
            buf.extend_from_slice(&[0x00, 0x00, 0x00]);
            buffers.push(buf);
        }

        for buf in &buffers {
            let len_bits = u32::try_from(buf.len() * 8).expect("test buffers are small");
            for pos in 0..=len_bits {
                let mut r = reader(buf);
                r.skip(pos).expect("skip");
                assert_eq!(r.position(), u64::from(pos));
                assert_eq!(
                    r.remaining(),
                    remaining_reference(&r),
                    "remaining: {buf:02x?} at bit {pos}"
                );
                assert_eq!(
                    r.more_rbsp_data(),
                    more_rbsp_data_reference(&r),
                    "more_rbsp_data: {buf:02x?} at bit {pos}"
                );
            }
        }
    }

    #[test]
    pub fn test_more_rbsp_data_examples() {
        // One payload byte, then the stop bit at bit 8.
        let buf = [0xA5, 0x80];
        assert!((0..8).all(|pos| more_rbsp_data_at(&buf, pos)));
        assert!(!more_rbsp_data_at(&buf, 8));
        // Past the stop bit only zeros remain: malformed, reported as more data.
        assert!(more_rbsp_data_at(&buf, 9));
        assert!(!more_rbsp_data_at(&buf, 16));
        // Trailing zero bytes (cabac_zero_words) don't hide the stop bit.
        assert!(!more_rbsp_data_at(&[0x5C, 0x40, 0x00, 0x00], 9));
        // No bit set anywhere: more data until the end.
        assert!(more_rbsp_data_at(&[0x00, 0x00], 0));
        assert!(!more_rbsp_data_at(&[0x00, 0x00], 16));
        assert!(!more_rbsp_data_at(&[], 0));
    }
}
