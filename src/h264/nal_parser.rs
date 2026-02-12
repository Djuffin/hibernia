use std::io::{BufRead, Error, ErrorKind};

pub struct NalParser<R> {
    reader: R,
    nal_buffer: Vec<u8>,
    zeros: usize,
    found_first_start_code: bool,
    eof: bool,
}

impl<R: BufRead> NalParser<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            nal_buffer: Vec::new(),
            zeros: 0,
            found_first_start_code: false,
            eof: false,
        }
    }
}

impl<R: BufRead> Iterator for NalParser<R> {
    type Item = Result<Vec<u8>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            if !self.nal_buffer.is_empty() {
                let nal = std::mem::take(&mut self.nal_buffer);
                return Some(Ok(nal));
            }
            return None;
        }

        loop {
            let buf = match self.reader.fill_buf() {
                Ok(b) => b,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Some(Err(e)),
            };

            if buf.is_empty() {
                self.eof = true;
                if !self.nal_buffer.is_empty() && self.found_first_start_code {
                    let nal = std::mem::take(&mut self.nal_buffer);
                    return Some(Ok(nal));
                }
                return None;
            }

            let mut consumed = buf.len();
            let mut found = false;
            let mut extra_zeros = 0;

            for (i, &byte) in buf.iter().enumerate() {
                if byte == 0 {
                    self.zeros += 1;
                } else if byte == 1 {
                    if self.zeros >= 2 {
                        found = true;
                        consumed = i + 1;
                        extra_zeros = if self.zeros > 2 { 1 } else { 0 };
                        self.zeros = 0;
                        break;
                    } else {
                        self.zeros = 0;
                    }
                } else {
                    self.zeros = 0;
                }
            }

            self.nal_buffer.extend_from_slice(&buf[..consumed]);
            self.reader.consume(consumed);

            if found {
                if self.found_first_start_code {
                    let trim = 3 + extra_zeros;
                    let new_len = self.nal_buffer.len().saturating_sub(trim);
                    self.nal_buffer.truncate(new_len);
                    let nal = std::mem::take(&mut self.nal_buffer);
                    return Some(Ok(nal));
                } else {
                    self.found_first_start_code = true;
                    self.nal_buffer.clear();
                    // Continue loop
                }
            }
        }
    }
}
