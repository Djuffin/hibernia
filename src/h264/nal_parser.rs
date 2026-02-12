use std::io::{BufRead, Error};

/// A parser that iterates over H.264 NAL units from a byte stream.
///
/// This parser implements the byte stream format specified in **ITU-T H.264 Annex B**.
/// It separates NAL units based on the start code prefix (`0x000001` or `0x00000001`).
///
/// # Behavior
/// - Scans for the first start code to begin parsing.
/// - Strips start code prefixes and trailing zero bytes (which are part of the gap between NAL units).
/// - Returns the NAL unit bytes (header + payload) as `Vec<u8>`.
pub struct NalParser<R> {
    reader: R,
    /// Accumulates the current NAL unit's data.
    buffer: Vec<u8>,
    /// Tracks consecutive zero bytes seen so far.
    /// These are held pending because they might be part of a start code.
    zeros_count: usize,
    /// Whether the first start code has been encountered.
    first_start_code_found: bool,
    /// Whether the end of the stream has been reached.
    eof: bool,
}

impl<R: BufRead> NalParser<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            zeros_count: 0,
            first_start_code_found: false,
            eof: false,
        }
    }
}

impl<R: BufRead> Iterator for NalParser<R> {
    type Item = Result<Vec<u8>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }

        loop {
            let available = match self.reader.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Some(Err(e)),
            };

            let len = available.len();
            if len == 0 {
                self.eof = true;
                if self.first_start_code_found && !self.buffer.is_empty() {
                    // Flush remaining data.
                    // Any pending zeros (`self.zeros_count`) at EOF are discarded,
                    // as a valid NAL unit must end with a non-zero byte (rbsp_trailing_bits).
                    let nal = std::mem::take(&mut self.buffer);
                    return Some(Ok(nal));
                }
                return None;
            }

            let mut split_idx = None;
            let mut nal_found = None;

            for (i, &byte) in available.iter().enumerate() {
                if byte == 0 {
                    self.zeros_count += 1;
                } else if byte == 1 && self.zeros_count >= 2 {
                    // Start code found: 00 00 01 (or 00 00 00 01)
                    // The `zeros_count` zeros are part of the start code prefix or padding.

                    if self.first_start_code_found {
                        // We found the END of the current NAL unit (and start of next).
                        // Emit the collected buffer.
                        let nal = self.buffer.clone();
                        self.buffer.clear();
                        nal_found = Some(Ok(nal));
                        split_idx = Some(i + 1);
                        // Reset zeros count for the next NAL.
                        self.zeros_count = 0;
                        break;
                    } else {
                        // We found the START of the first NAL unit.
                        self.first_start_code_found = true;
                        self.buffer.clear(); // Discard preamble/garbage
                        self.zeros_count = 0;
                        // Do not break; continue parsing the payload.
                    }
                } else {
                    // Regular byte, or 1 preceded by < 2 zeros.
                    if self.first_start_code_found {
                        // Push pending zeros that were NOT part of a start code.
                        if self.zeros_count > 0 {
                            self.buffer.resize(self.buffer.len() + self.zeros_count, 0);
                            self.zeros_count = 0;
                        }
                        self.buffer.push(byte);
                    } else {
                        // Still searching for first start code; discard.
                        self.zeros_count = 0;
                    }
                }
            }

            let amt = split_idx.unwrap_or(len);
            self.reader.consume(amt);

            if let Some(result) = nal_found {
                return Some(result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_single_nal() {
        let data = vec![0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0x00];
        let data = vec![0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0x80];
        let mut parser = NalParser::new(Cursor::new(data));
        let nal = parser.next().unwrap().unwrap();
        assert_eq!(nal, vec![0x67, 0x42, 0x80]);
        assert!(parser.next().is_none());
    }

    #[test]
    fn test_multiple_nals() {
        let data = vec![
            0x00, 0x00, 0x00, 0x01, 0x67, 0xFF,
            0x00, 0x00, 0x01, 0x68, 0xEE,
            0x00, 0x00, 0x00, 0x01, 0x65, 0xDD,
        ];
        let mut parser = NalParser::new(Cursor::new(data));

        assert_eq!(parser.next().unwrap().unwrap(), vec![0x67, 0xFF]);
        assert_eq!(parser.next().unwrap().unwrap(), vec![0x68, 0xEE]);
        assert_eq!(parser.next().unwrap().unwrap(), vec![0x65, 0xDD]);
        assert!(parser.next().is_none());
    }

    #[test]
    fn test_leading_garbage() {
        let data = vec![0xFF, 0x00, 0x00, 0x00, 0x01, 0x67];
        let mut parser = NalParser::new(Cursor::new(data));
        assert_eq!(parser.next().unwrap().unwrap(), vec![0x67]);
    }

    #[test]
    fn test_split_start_code() {
        struct OneByteReader {
            data: Vec<u8>,
            pos: usize,
        }
        impl std::io::Read for OneByteReader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                if self.pos >= self.data.len() {
                    return Ok(0);
                }
                buf[0] = self.data[self.pos];
                self.pos += 1;
                Ok(1)
            }
        }
        impl std::io::BufRead for OneByteReader {
            fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
                if self.pos >= self.data.len() {
                    return Ok(&[]);
                }
                Ok(&self.data[self.pos..self.pos + 1])
            }
            fn consume(&mut self, amt: usize) {
                self.pos += amt;
            }
        }

        let data = vec![0x00, 0x00, 0x00, 0x01, 0x67, 0x00, 0x00, 0x01, 0x68];
        let parser = NalParser::new(OneByteReader { data, pos: 0 });
        let nals: Vec<_> = parser.map(|r| r.unwrap()).collect();
        assert_eq!(nals.len(), 2);
        assert_eq!(nals[0], vec![0x67]);
        assert_eq!(nals[1], vec![0x68]);
    }
}
