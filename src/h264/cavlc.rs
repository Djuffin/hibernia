use super::tables;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CoeffToken {
    pub total_coeffs: u8,
    pub trailing_ones: u8,
    pub pattern_len: u8,
}
pub type CoeffTokenPattern = (/* bit pattern */ u16, /* length */ u8);

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
    return CoeffToken::default();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_bits(bit_str: &str) -> u16 {
        let value = u16::from_str_radix(bit_str, 2).unwrap();
        value << (u16::BITS - bit_str.len() as u32)
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
                if result.pattern_len == 0 {
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
