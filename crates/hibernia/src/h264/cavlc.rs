use super::parser::{BitReader, ParseResult};
use super::tables;
use crate::{cast_or_error, read_value};
use log::trace;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CoeffToken {
    pub total_coeffs: u8,
    pub trailing_ones: u8,
    pub pattern_len: u8,
}

impl CoeffToken {
    pub fn is_valid(&self) -> bool {
        self.pattern_len > 0
    }
}

pub(crate) type BitPattern = (/* bit pattern */ u16, /* length */ u8);

// Generates a 16KB Lookup Table (LUT) for `total_zeros` decoding at compile time.
//
// What is this LUT?
// It maps `(vlc_idx, top_9_bits_of_input)` to `(total_zeros, bit_length)`.
// `vlc_idx` (1..15) is determined by `total_coeffs`.
// The inner array has 512 entries, covering all possible 9-bit patterns (2^9 = 512).
//
// How it works:
// The table `TABLE9_7AND8` contains variable-length codes.
// Shorter codes are "expanded" to fill all 512-entry slots that start with that code's prefix.
// This allows O(1) lookup by simply indexing with the top 9 bits of the bitstream.
const fn init_total_zeros_lut() -> [[(u8, u8); 512]; 16] {
    let mut lut = [[(0, 0); 512]; 16];

    let mut vlc_idx = 1;
    while vlc_idx <= 15 {
        let mut row_idx = 0;
        while row_idx < 16 {
            let row = tables::TABLE9_7AND8[row_idx];
            let (pattern, len) = match vlc_idx {
                1 => row.1,
                2 => row.2,
                3 => row.3,
                4 => row.4,
                5 => row.5,
                6 => row.6,
                7 => row.7,
                8 => row.8,
                9 => row.9,
                10 => row.10,
                11 => row.11,
                12 => row.12,
                13 => row.13,
                14 => row.14,
                15 => row.15,
                _ => (0, 0),
            };

            if len > 0 {
                let total_zeros = row.0;
                let shift = 9 - len;
                let start = (pattern as usize) << shift;
                let end = ((pattern as usize) + 1) << shift;
                let mut i = start;
                while i < end {
                    lut[vlc_idx][i] = (total_zeros, len);
                    i += 1;
                }
            }
            row_idx += 1;
        }
        vlc_idx += 1;
    }
    lut
}

static TOTAL_ZEROS_LUT: [[(u8, u8); 512]; 16] = init_total_zeros_lut();

// Naive implementation of Tables 9-7, 9-8 lookup for total_zeros patterns
fn lookup_total_zeros(bits: u16, vlc_idx: u8) -> (u8, u8) {
    if vlc_idx == 0 || vlc_idx > 15 {
        return (0, 0);
    }
    let index = (bits >> 7) as usize;
    TOTAL_ZEROS_LUT[vlc_idx as usize][index]
}

// Generates a LUT for `total_zeros_chroma` decoding at compile time.
const fn init_total_zeros_chroma_lut() -> [[(u8, u8); 8]; 4] {
    let mut lut = [[(0, 0); 8]; 4];

    let mut vlc_idx = 1;
    while vlc_idx <= 3 {
        let mut row_idx = 0;
        while row_idx < 4 {
            let row = tables::TABLE9_9A[row_idx];
            let (pattern, len) = match vlc_idx {
                1 => row.1,
                2 => row.2,
                3 => row.3,
                _ => (0, 0),
            };

            if len > 0 {
                let total_zeros = row.0;
                let shift = 3 - len;
                let start = (pattern as usize) << shift;
                let end = ((pattern as usize) + 1) << shift;
                let mut i = start;
                while i < end {
                    lut[vlc_idx][i] = (total_zeros, len);
                    i += 1;
                }
            }
            row_idx += 1;
        }
        vlc_idx += 1;
    }
    lut
}

static TOTAL_ZEROS_CHROMA_LUT: [[(u8, u8); 8]; 4] = init_total_zeros_chroma_lut();

// Naive implementation of Tables 9-9 total_zeros patterns
fn lookup_total_zeros_chroma(bits: u16, vlc_idx: u8) -> (u8, u8) {
    if vlc_idx < 1 || vlc_idx > 3 {
        return (0, 0);
    }
    let index = (bits >> 13) as usize;
    TOTAL_ZEROS_CHROMA_LUT[vlc_idx as usize][index]
}

// Naive implementation of Table 9-10 – Tables for run_before
fn lookup_run_before(bits: u16, zeros_left: u8) -> (u8, u8) {
    for row in tables::TABLE9_10 {
        let (pattern, pattern_len) = match zeros_left {
            0 => (0, 0),
            1 => row.1,
            2 => row.2,
            3 => row.3,
            4 => row.4,
            5 => row.5,
            6 => row.6,
            7.. => row.7,
        };
        if pattern_len == 0 {
            break;
        }
        let shift = u16::BITS - pattern_len as u32;
        let meaningful_bits = bits >> shift;
        if meaningful_bits == pattern {
            return (row.0, pattern_len);
        }
    }
    (0, 0)
}

// Naive implementation of Table 9-5 lookup for coeff_token patterns
fn lookup_coeff_token(bits: u16, nc: i32) -> CoeffToken {
    for row in tables::TABLE95 {
        let (pattern, pattern_len) = match nc {
            0 | 1 => row.1,
            2 | 3 => row.2,
            4..=7 => row.3,
            8.. => row.4,
            -1 => row.5,
            -2 => row.6,
            _ => (0, 0),
        };
        if pattern_len == 0 {
            break;
        }
        let shift = u16::BITS - pattern_len as u32;
        let meaningful_bits = bits >> shift;
        if meaningful_bits == pattern {
            let (trailing_ones, total_coeffs) = row.0;
            return CoeffToken { total_coeffs, trailing_ones, pattern_len };
        }
    }
    CoeffToken::default()
}

// Section 9.2.2.1 Parsing process for level_prefix
fn parse_level_prefix(input: &mut BitReader) -> ParseResult<u32> {
    input.read_till_one().map_err(|_e| "leadingZeroBits".to_owned())
}

// Parses a block of residual coefficients into `coeff_level` and returns
// the number of non-zero coefficients.
pub fn parse_residual_block(
    input: &mut BitReader,
    coeff_level: &mut [i32],
    nc: i32,
) -> ParseResult<u8> {
    trace!("CAVLC residual_block: len: {} nc: {}", coeff_level.len(), nc);
    debug_assert!(matches!(coeff_level.len(), 4 | 15 | 16));
    let max_num_coeff = coeff_level.len();
    let next_16_bits = input.peek_or_pad16().map_err(|e| "coeff_token: ".to_owned() + &e)?;
    let coeff_token = lookup_coeff_token(next_16_bits, nc);
    if !coeff_token.is_valid() {
        return Err(format!("Unknown coeff_token value: {:#016b} nc:{}", next_16_bits, nc));
    }
    input.skip(coeff_token.pattern_len as u32)?;
    let total_coeffs = coeff_token.total_coeffs as usize;
    trace!(
        "coeff_token total_coeffs: {} t1s: {}, bits:{}",
        total_coeffs,
        coeff_token.trailing_ones,
        coeff_token.pattern_len
    );

    if total_coeffs == 0 {
        return Ok(0);
    }

    // Section 9.2.2 Parsing process for level information
    let mut levels = [0; 16];
    let (lower_levels, higher_levels) =
        levels[..total_coeffs].split_at_mut(coeff_token.trailing_ones as usize);
    for level in lower_levels {
        let trailing_ones_sign_flag: i32;
        read_value!(input, trailing_ones_sign_flag, u, 1);
        *level = 1 - 2 * trailing_ones_sign_flag;
    }

    let mut suffix_len = if total_coeffs > 10 && coeff_token.trailing_ones < 3 { 1 } else { 0 };
    for (i, level) in higher_levels.iter_mut().enumerate() {
        let level_prefix = parse_level_prefix(input)?;
        let level_suffix_size = if level_prefix == 14 && suffix_len == 0 {
            4
        } else if level_prefix >= 15 {
            level_prefix - 3
        } else {
            suffix_len
        };

        let mut level_suffix = 0u32;
        if level_suffix_size > 0 {
            read_value!(input, level_suffix, u, level_suffix_size as u8);
        }
        let mut level_code = (std::cmp::min(15, level_prefix) << suffix_len) + level_suffix;
        if level_prefix >= 15 && suffix_len == 0 {
            level_code += 15;
        }
        if level_prefix >= 16 {
            level_code += (1 << (level_prefix - 3)) - 4096;
        }
        if i == 0 && coeff_token.trailing_ones < 3 {
            level_code += 2;
        }
        if level_code % 2 == 0 {
            *level = (level_code as i32 + 2) >> 1;
        } else {
            *level = (-(level_code as i32) - 1) >> 1;
        }
        if suffix_len == 0 {
            suffix_len = 1;
        }
        if suffix_len < 6 && level.abs() > (3 << (suffix_len - 1)) {
            suffix_len += 1;
        }
        trace!(
            "i: {i} level_prefix: {level_prefix} level_suffix_size: {level_suffix_size} \
                suffix_len: {suffix_len} coeff_level: {}",
            *level
        );
    }

    // Section 9.2.3 Parsing process for run information
    let mut zeros_left = if total_coeffs < max_num_coeff {
        let next_16_bits = input.peek_or_pad16().map_err(|e| "total_zeros: ".to_owned() + &e)?;
        let tz_vlc_index = total_coeffs as u8;
        let lookup_tz =
            if max_num_coeff == 4 { lookup_total_zeros_chroma } else { lookup_total_zeros };
        let (total_zeros, bits) = lookup_tz(next_16_bits, tz_vlc_index);
        if bits == 0 {
            return Err(format!(
                "Unknown total_zeros value: {:#016b} tz_vlc_index:{}",
                next_16_bits, tz_vlc_index
            ));
        }
        input.skip(bits as u32)?;
        trace!("total_zeros: {} bits:{}", total_zeros, bits);
        total_zeros
    } else {
        0
    };

    let mut runs = [0; 16];
    for run in runs.iter_mut().take(total_coeffs - 1) {
        *run = if zeros_left > 0 {
            let next_16_bits = input.peek_or_pad16().map_err(|e| "run_before: ".to_owned() + &e)?;
            let (run_before, bits) = lookup_run_before(next_16_bits, zeros_left);
            if bits == 0 {
                return Err(format!(
                    "Unknown run_before value: {:#016b} zeros_left:{}",
                    next_16_bits, zeros_left
                ));
            }
            input.skip(bits as u32)?;
            zeros_left -= run_before;
            trace!("run_before: {} bits:{}", run_before, bits);
            run_before
        } else {
            0
        }
    }

    // Section 9.2.4 Combining level and run information
    runs[total_coeffs - 1] = zeros_left;

    let mut coeff_num = -1isize;
    for i in (0..total_coeffs).rev() {
        coeff_num += (runs[i] + 1) as isize;
        coeff_level[coeff_num as usize] = levels[i];
    }
    trace!("coeff_level: {:?}", coeff_level);
    Ok(coeff_token.total_coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_bits(bit_str: &str) -> u16 {
        let value = u16::from_str_radix(bit_str, 2).unwrap();
        value << (u16::BITS - bit_str.len() as u32)
    }

    fn prepare_bit_vec(bit_str: &str) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for byte in bit_str.split(' ') {
            result.push(u8::from_str_radix(byte, 2).unwrap());
        }

        result
    }

    #[test]
    pub fn test_parse_residual() {
        // Examples from the book:
        // The H.264 advanced video compression standard / Iain E. Richardson.
        let data = prepare_bit_vec("00001000 11100101 11101101");
        let mut output = [0i32; 16];

        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 5);
        assert_eq!(output, [0, 3, 0, 1, -1, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data = prepare_bit_vec("00000001 10100010 01000010 11100110 0");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 5);
        assert_eq!(output, [-2, 4, 3, -3, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data = prepare_bit_vec("00011100 01110010");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 3);
        assert_eq!(output, [0, 0, 0, 1, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0]);

        // Example from the article:
        // VHDL Implementation of an Efficient Context Adaptive
        // Variable Length Coding Algorithm of H.264 Video Codec
        let data = prepare_bit_vec("01101000 10001111 11001001 11011110 0");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 5).unwrap(), 8);
        assert_eq!(output, [1, 1, -2, -4, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0]);

        // Examples from the article:
        // IMPLEMENTATION OF CONTEXT ADAPTIVE VARIABLE LENGTH CODING AND DECODING
        // ALGORITHM FOR H.264 VIDEO CODEC USING MATLAB
        let data = prepare_bit_vec("01000100 01001000 01011100 11000000");
        output.fill(0);
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 4).unwrap(), 5);
        assert_eq!(output, [-2, 4, 3, -3, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    pub fn test_parse_residual_refcase1() {
        /*
            @254F: coeff_token (total_coeff =  5 / t1s =  3) :     : (5)  : [00110]
            @2554: trailing_ones_sign                        :  -1 : (1)  : [1]
            @2555: trailing_ones_sign                        :  -1 : (1)  : [1]
            @2556: trailing_ones_sign                        :   1 : (1)  : [0]
            @2557: coeff_level                               :  -1 : (2)  : [01]
            @2559: coeff_level                               :  -2 : (3)  : [011]
            @255C: total_zeros                               :  10 : (4)  : [0001]
            @2560: run_before                                :  10 : (7)  : [0000001]
        */
        let data = prepare_bit_vec("00110110 01011000 10000001");
        let mut output = [0i32; 16];
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 2).unwrap(), 5);
        // Output might be not 100% accurate :\
        assert_eq!(output, [-2, -1, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0]);
    }

    #[test]
    pub fn test_parse_residual_refcase2() {
        /*
            @68  : coeff_token (total_coeff =  4 / t1s =  1) :     : (9)  : [000000110]
            @71  : trailing_ones_sign                        :  -1 : (1)  : [1]
            @72  : coeff_level                               :  -5 : (10) : [0000000001]
            @7C  : coeff_level                               : -19 : (12) : [000000000101]
            @88  : coeff_level                               :   6 : (5)  : [01010]
            @8D  : total_zeros                               :   3 : (4)  : [0100]
            @91  : run_before                                :   0 : (2)  : [11]
            @93  : run_before                                :   3 : (2)  : [00]
        */
        let data = prepare_bit_vec("00000011 01000000 00010000 00000101 01010010 01100000");
        let mut output = [0i32; 16];
        assert_eq!(parse_residual_block(&mut BitReader::new(&data), &mut output, 0).unwrap(), 4);
        // Output might be not 100% accurate :\
        assert_eq!(output, [6, -19, 0, 0, 0, -6, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    pub fn test_lookup_run_before() {
        for i in 3..7 {
            assert_eq!(lookup_run_before(prepare_bits("11"), i), (0, 2));
        }

        assert_eq!(lookup_run_before(prepare_bits("010"), 5), (3, 3));
        assert_eq!(lookup_run_before(prepare_bits("0000001"), 7), (10, 7));
        assert_eq!(lookup_run_before(prepare_bits("0000000001"), 7), (13, 10));
        assert_eq!(lookup_run_before(prepare_bits("00000000001"), 7), (14, 11));
        assert_eq!(lookup_run_before(prepare_bits("110"), 7), (1, 3));
        assert_eq!(lookup_run_before(prepare_bits("10"), 5), (1, 2));
        assert_eq!(lookup_run_before(prepare_bits("101"), 6), (5, 3));
        assert_eq!(lookup_run_before(prepare_bits("00"), 2), (2, 2));
    }

    #[test]
    pub fn brute_force_test_lookup_total_zeros() {
        for vcl_idx in 1..16 {
            let mut recognizer_patterns = [false; 16];
            for bits in 0..=u16::MAX {
                let result = lookup_total_zeros(bits, vcl_idx);
                if result.1 == 0 {
                    continue;
                }
                assert!(result.1 <= 9);
                assert!(result.0 < 17 - vcl_idx);
                recognizer_patterns[result.0 as usize] = true;
            }
            for i in 0..(17 - vcl_idx) {
                assert!(recognizer_patterns[i as usize], "vcl_idx:{:} i:{:}", vcl_idx, i);
            }
        }
    }

    #[test]
    pub fn test_lookup_total_zeros() {
        assert_eq!(lookup_total_zeros(prepare_bits("00000010"), 1), (12, 8));
        assert_eq!(lookup_total_zeros(prepare_bits("110"), 3), (2, 3));
        assert_eq!(lookup_total_zeros(prepare_bits("000000"), 7), (9, 6));
        assert_eq!(lookup_total_zeros(prepare_bits("000001"), 6), (0, 6));

        assert_eq!(lookup_total_zeros(prepare_bits("000001"), 8), (0, 6));
        assert_eq!(lookup_total_zeros(prepare_bits("1"), 15), (1, 1));
        assert_eq!(lookup_total_zeros(prepare_bits("00000"), 10), (1, 5));
        assert_eq!(lookup_total_zeros(prepare_bits("0000"), 12), (0, 4));
        assert_eq!(lookup_total_zeros(prepare_bits("00001"), 9), (7, 5));
    }

    #[test]
    pub fn test_lookup_total_zeros_chroma() {
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("1"), 1), (0, 1));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("1"), 2), (0, 1));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("1"), 3), (0, 1));

        assert_eq!(lookup_total_zeros_chroma(prepare_bits("01"), 1), (1, 2));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("01"), 2), (1, 2));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("0"), 3), (1, 1));

        assert_eq!(lookup_total_zeros_chroma(prepare_bits("001"), 1), (2, 3));
        assert_eq!(lookup_total_zeros_chroma(prepare_bits("00"), 2), (2, 2));

        assert_eq!(lookup_total_zeros_chroma(prepare_bits("000"), 1), (3, 3));
    }

    #[test]
    pub fn test_lookup_coeff_token() {
        assert_eq!(
            lookup_coeff_token(prepare_bits("0000100"), 0),
            CoeffToken { total_coeffs: 5, trailing_ones: 3, pattern_len: 7 }
        );

        assert_eq!(
            lookup_coeff_token(prepare_bits("0000000110"), 0),
            CoeffToken { total_coeffs: 5, trailing_ones: 1, pattern_len: 10 }
        );

        assert_eq!(
            lookup_coeff_token(prepare_bits("00011"), 0),
            CoeffToken { total_coeffs: 3, trailing_ones: 3, pattern_len: 5 }
        );
    }

    #[test]
    pub fn brute_force_test_lookup_coeff_token() {
        for nc in [0, 2, 4, 8, -1, -2, -3] {
            let mut recognizer_patterns = [[false; 4]; 17];
            for bits in 0..=u16::MAX {
                let result = lookup_coeff_token(bits, nc);
                if !result.is_valid() {
                    continue;
                }
                if result.pattern_len == 1 {
                    assert!(bits & 0x8000 != 0);
                }
                recognizer_patterns[result.total_coeffs as usize][result.trailing_ones as usize] =
                    true;
                assert!(result.trailing_ones <= 3);
                match nc {
                    -1 => {
                        assert!(result.pattern_len <= 8);
                        assert!(result.total_coeffs <= 4);
                    }
                    -2 => {
                        assert!(result.pattern_len <= 13);
                        assert!(result.total_coeffs <= 8);
                    }
                    8 => {
                        assert_eq!(result.pattern_len, 6);
                        assert!(result.total_coeffs <= 16);
                    }
                    _ => {
                        assert!(result.pattern_len <= 16);
                        assert!(result.total_coeffs <= 16);
                    }
                };
            }

            match nc {
                -1 => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            let value = total <= 4 && ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
                -2 => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            let value = total <= 8 && ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
                -3 => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            assert!(!recognizer_patterns[total][ones]);
                        }
                    }
                }
                _ => {
                    for total in 0..16 {
                        for ones in 0..4 {
                            let value = ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
            };
        }
    }
}
