use super::tables;

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
pub fn lookup_total_zeros(bits: u16, vlc_idx: u8) -> (u8, u8) {
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
pub fn lookup_total_zeros_chroma(bits: u16, vlc_idx: u8) -> (u8, u8) {
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
pub fn lookup_run_before(bits: u16, zeros_left: u8) -> (u8, u8) {
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
pub fn lookup_coeff_token(bits: u16, nc: i32) -> CoeffToken {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_bits(bit_str: &str) -> u16 {
        let value = u16::from_str_radix(bit_str, 2).unwrap();
        value << (u16::BITS - bit_str.len() as u32)
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
