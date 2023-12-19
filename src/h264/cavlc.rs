use super::parser::{self, parse_level_prefix, BitReader, ParseResult};
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

// Naive implementation of Tables 9-7, 9-8 lookup for total_zeros patterns
fn lookup_total_zeros(bits: u16, vlc_idx: u8) -> (u8, u8) {
    for row in tables::TABLE9_7AND8 {
        let (pattern, pattern_len) = match vlc_idx {
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

// Naive implementation of Tables 9-9 total_zeros patterns
fn lookup_total_zeros_chroma(bits: u16, vlc_idx: u8) -> (u8, u8) {
    for row in tables::TABLE9_9A {
        let (pattern, pattern_len) = match vlc_idx {
            1 => row.1,
            2 => row.2,
            3 => row.3,
            _ => (0, 0),
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

pub fn parse_residual_block(
    input: &mut BitReader,
    coeff_level: &mut [i32],
    max_num_coeff: usize,
) -> ParseResult<()> {
    let nc = 0;
    let next_16_bits = input.peek_u16(16).map_err(|e| "EOF at coeff_token".to_owned())?;
    let coeff_token = lookup_coeff_token(next_16_bits, nc);
    if (!coeff_token.is_valid()) {
        return Err(format!("Unknown coeff_token value: {:#016b} nc:{}", next_16_bits, nc));
    }
    input.skip(coeff_token.pattern_len as u64);
    let total_coeffs = coeff_token.total_coeffs as usize;
    if total_coeffs == 0 {
        return Ok(());
    }
    trace!("total_coeffs: {} trailing_ones: {}", total_coeffs, coeff_token.trailing_ones);

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
        trace!("i: {} level_prefix: {}", i, level_prefix);
        let level_suffix_size = if level_prefix == 14 && suffix_len == 0 {
            4
        } else if level_prefix >= 15 {
            level_prefix - 3
        } else {
            suffix_len
        };
        trace!("level_suffix_size: {} suffix_len: {}", level_suffix_size, suffix_len);
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
        trace!("level: {}", *level);
    }
    trace!("levels: {:?}", levels);

    // Section 9.2.3 Parsing process for run information
    let mut zeros_left = if total_coeffs < coeff_level.len() {
        let next_16_bits = input.peek_u16(16).map_err(|e| "EOF at total_zeros".to_owned())?;
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
        input.skip(bits as u64);
        trace!("total_zeros: {}", total_zeros);
        total_zeros
    } else {
        0
    };

    let mut runs = [0; 16];
    for run in runs.iter_mut().take(total_coeffs - 1) {
        *run = if zeros_left > 0 {
            let next_16_bits = input.peek_u16(16).map_err(|e| "EOF at run_before".to_owned())?;
            let (run_before, bits) = lookup_run_before(next_16_bits, zeros_left);
            if bits == 0 {
                return Err(format!(
                    "Unknown run_before value: {:#016b} zeros_left:{}",
                    next_16_bits, zeros_left
                ));
            }
            input.skip(bits as u64);
            zeros_left -= run_before;
            run_before
        } else {
            0
        }
    }
    runs[total_coeffs - 1] = zeros_left;
    trace!("runs: {:?}", runs);

    let mut coeff_num = -1isize;
    for i in (0..total_coeffs).rev() {
        coeff_num += (runs[i] + 1) as isize;
        coeff_level[coeff_num as usize] = levels[i];
    }
    trace!("coeff_level: {:?}", coeff_level);
    Ok(())
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

        // Extra padding for input reader
        result.push(0);
        result.push(0);
        result
    }

    #[test]
    pub fn test_parse_residual() {
        let data = prepare_bit_vec("00001000 11100101 11101101");
        let mut output = [0i32; 16];

        parse_residual_block(&mut BitReader::new(&data), &mut output, 16).unwrap();
        assert_eq!(output, [0, 3, 0, 1, -1, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data = prepare_bit_vec("00000001 10100010 01000010 11100110 0");
        output.fill(0);
        parse_residual_block(&mut BitReader::new(&data), &mut output, 16).unwrap();
        assert_eq!(output, [-2, 4, 3, -3, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data = prepare_bit_vec("00011100 01110010");
        output.fill(0);
        parse_residual_block(&mut BitReader::new(&data), &mut output, 16).unwrap();
        assert_eq!(output, [0, 0, 0, 1, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    pub fn test_lookup_run_before() {
        for i in 3..7 {
            assert_eq!(lookup_run_before(prepare_bits("11"), i), (0, 2));
        }

        assert_eq!(lookup_run_before(prepare_bits("010"), 5), (3, 3));
        assert_eq!(lookup_run_before(prepare_bits("000000000001"), 7), (14, 12));
        assert_eq!(lookup_run_before(prepare_bits("110"), 7), (1, 3));
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
                        for ones in 0..3 {
                            let value = total <= 4 && ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
                -2 => {
                    for total in 0..16 {
                        for ones in 0..3 {
                            let value = total <= 8 && ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
                -3 => {
                    for total in 0..16 {
                        for ones in 0..3 {
                            assert_eq!(recognizer_patterns[total][ones], false);
                        }
                    }
                }
                _ => {
                    for total in 0..16 {
                        for ones in 0..3 {
                            let value = ones <= total;
                            assert_eq!(recognizer_patterns[total][ones], value);
                        }
                    }
                }
            };
        }
    }
}
