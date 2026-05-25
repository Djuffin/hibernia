//! NAL splitters for the two H.264/AVC bitstream packaging formats.
//!
//! Both produce zero-copy `&[u8]` views into the original buffer with
//! the start codes / length prefixes stripped.

/// H.264/AVC bitstream packaging formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AvcBitstreamFormat {
    /// Bitstream with start codes (`0x000001` or `0x00000001`)
    /// separating NAL units. Common in raw bitstream files (.264) and
    /// MPEG-TS.
    #[default]
    AnnexB,
    /// Bitstream where each NAL unit is prefixed by its length
    /// (typically 4 bytes). Common in MP4, MKV, and WebM containers.
    Avc,
}

/// H.264/AVC-specific configuration parameters. Wrapped in
/// `DecoderConfig::custom_params` via `Box<dyn Any>`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct H264Config {
    pub bitstream_format: AvcBitstreamFormat,
    /// Out-of-band parameter sets. Either an ISO/IEC 14496-15
    /// `AVCDecoderConfigurationRecord` (a.k.a. avcC, recognized by
    /// `bytes[0] == 1`) or a concatenation of Annex-B-framed
    /// SPS+PPS NALs. Parsed once at construction; avcC's
    /// `lengthSizeMinusOne` overrides the default 4-byte NAL length
    /// prefix. See also [`crate::api::H264SetExtradata`] (runtime)
    /// and [`crate::api::build_avcc`].
    pub extradata: Option<Vec<u8>>,
}

/// Iterator yielding NAL unit byte ranges from an Annex-B byte stream.
/// Start codes (`0x000001` or `0x00000001`) are stripped from the
/// returned slice.
pub struct AnnexBSplitter<'a> {
    data: &'a [u8],
    cursor: usize,
}

impl<'a> AnnexBSplitter<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, cursor: 0 }
    }
}

impl<'a> Iterator for AnnexBSplitter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        let start = find_start_code(self.data, self.cursor)?;
        let payload_start = start.end;
        let next_start =
            find_start_code(self.data, payload_start).map(|m| m.start).unwrap_or(self.data.len());
        // Trim trailing zero bytes that belong to the inter-NAL gap.
        let mut end = next_start;
        while end > payload_start && self.data[end - 1] == 0 {
            end -= 1;
        }
        self.cursor = next_start;
        Some(&self.data[payload_start..end])
    }
}

struct StartCode {
    start: usize,
    end: usize,
}

fn find_start_code(data: &[u8], from: usize) -> Option<StartCode> {
    let mut i = from;
    while i + 2 < data.len() {
        if data[i] == 0 && data[i + 1] == 0 {
            if data[i + 2] == 1 {
                return Some(StartCode { start: i, end: i + 3 });
            }
            if data[i + 2] == 0 && i + 3 < data.len() && data[i + 3] == 1 {
                return Some(StartCode { start: i, end: i + 4 });
            }
        }
        i += 1;
    }
    None
}

/// Iterator yielding NAL unit byte ranges from an AVC length-prefixed
/// byte stream. The length prefix width is typically 4 bytes (set by
/// `nalu_length_size_minus_one` in the AVC decoder configuration
/// record).
pub struct AvcSplitter<'a> {
    data: &'a [u8],
    cursor: usize,
    length_size: usize,
}

impl<'a> AvcSplitter<'a> {
    pub fn new(data: &'a [u8], length_size: usize) -> Self {
        assert!(length_size >= 1 && length_size <= 4, "length_size must be 1..=4");
        Self { data, cursor: 0, length_size }
    }
}

impl<'a> Iterator for AvcSplitter<'a> {
    type Item = Result<&'a [u8], &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.data.len() {
            return None;
        }
        if self.cursor + self.length_size > self.data.len() {
            return Some(Err("truncated NAL length prefix"));
        }
        let mut len: usize = 0;
        for i in 0..self.length_size {
            len = (len << 8) | (self.data[self.cursor + i] as usize);
        }
        let payload_start = self.cursor + self.length_size;
        let payload_end = payload_start + len;
        if payload_end > self.data.len() {
            return Some(Err("truncated NAL payload"));
        }
        self.cursor = payload_end;
        Some(Ok(&self.data[payload_start..payload_end]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annexb_single_long_start_code() {
        let buf = [0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0x00];
        let nals: Vec<_> = AnnexBSplitter::new(&buf).collect();
        assert_eq!(nals.len(), 1);
        assert_eq!(nals[0], [0x67u8, 0x42].as_slice());
    }

    #[test]
    fn annexb_two_nals_mixed_start_codes() {
        let buf = [
            0x00, 0x00, 0x00, 0x01, 0x67, 0xFF, 0x00, 0x00, 0x01, 0x68, 0xEE, 0xCC,
        ];
        let nals: Vec<_> = AnnexBSplitter::new(&buf).collect();
        assert_eq!(nals.len(), 2);
        assert_eq!(nals[0], [0x67u8, 0xFF].as_slice());
        assert_eq!(nals[1], [0x68u8, 0xEE, 0xCC].as_slice());
    }

    #[test]
    fn annexb_strips_trailing_zeros() {
        let buf = [0x00, 0x00, 0x01, 0x67, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x01, 0x68, 0xAA];
        let nals: Vec<_> = AnnexBSplitter::new(&buf).collect();
        assert_eq!(nals[0], [0x67u8, 0xFF].as_slice());
        assert_eq!(nals[1], [0x68u8, 0xAA].as_slice());
    }

    #[test]
    fn annexb_empty() {
        let buf: [u8; 0] = [];
        assert!(AnnexBSplitter::new(&buf).next().is_none());
    }

    #[test]
    fn annexb_no_start_code() {
        let buf = [0x67, 0x42, 0x80];
        assert!(AnnexBSplitter::new(&buf).next().is_none());
    }

    #[test]
    fn avc_four_byte_length() {
        let buf = [
            0x00, 0x00, 0x00, 0x02, 0x67, 0x42,
            0x00, 0x00, 0x00, 0x03, 0x68, 0xEE, 0xCC,
        ];
        let nals: Result<Vec<_>, _> = AvcSplitter::new(&buf, 4).collect();
        let nals = nals.unwrap();
        assert_eq!(nals.len(), 2);
        assert_eq!(nals[0], [0x67u8, 0x42].as_slice());
        assert_eq!(nals[1], [0x68u8, 0xEE, 0xCC].as_slice());
    }

    #[test]
    fn avc_truncated_length() {
        let buf = [0x00, 0x00];
        let mut it = AvcSplitter::new(&buf, 4);
        assert!(matches!(it.next(), Some(Err(_))));
    }

    #[test]
    fn avc_truncated_payload() {
        let buf = [0x00, 0x00, 0x00, 0x05, 0x67];
        let mut it = AvcSplitter::new(&buf, 4);
        assert!(matches!(it.next(), Some(Err(_))));
    }
}
