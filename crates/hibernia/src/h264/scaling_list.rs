//! Custom scaling matrices (Annex H.264 clauses 7.3.2.1.1.1, 7.3.2.2.1,
//! 7.4.2.1.1.1, 7.4.2.2.1, 8.5.9).
//!
//! Scaling lists are stored in zig-zag scan order (matching the bitstream
//! encoding). Index conventions follow Table 7-2:
//!   0: Sl_4x4_Intra_Y     3: Sl_4x4_Inter_Y
//!   1: Sl_4x4_Intra_Cb    4: Sl_4x4_Inter_Cb
//!   2: Sl_4x4_Intra_Cr    5: Sl_4x4_Inter_Cr
//!   6: Sl_8x8_Intra_Y     7: Sl_8x8_Inter_Y
//!   8: Sl_8x8_Intra_Cb    9: Sl_8x8_Inter_Cb     (only when chroma_format_idc == 3)
//!  10: Sl_8x8_Intra_Cr   11: Sl_8x8_Inter_Cr     (only when chroma_format_idc == 3)

use super::rbsp::{ParseResult, RbspReader};
use super::tables::{
    DEFAULT_SCALING_LIST_4X4_INTER, DEFAULT_SCALING_LIST_4X4_INTRA, DEFAULT_SCALING_LIST_8X8_INTER,
    DEFAULT_SCALING_LIST_8X8_INTRA,
};
use super::{ChromaFormat, ColorPlane};

/// 16 coefficients in zig-zag scan order.
pub type ScalingList4x4 = [u8; 16];

/// 64 coefficients in zig-zag scan order.
pub type ScalingList8x8 = [u8; 64];

/// Flat_4x4_16 -- all values 16 (spec equation 7-8). Used when no custom matrix.
pub const FLAT_4X4_16: ScalingList4x4 = [16; 16];

/// Flat_8x8_16 -- all values 16 (spec equation 7-9).
pub const FLAT_8X8_16: ScalingList8x8 = [16; 64];

/// A single scaling list entry as it appears in a raw SPS or PPS. Preserved
/// exactly for round-trip encoding.
///
/// The `Explicit` variant carries the list values as `Vec<u8>` purely to sidestep
/// serde's 32-element built-in array support cap -- the vector is always exactly
/// 16 elements long in well-formed inputs.
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, schemars::JsonSchema,
)]
pub enum ScalingList4x4Entry {
    /// `scaling_list_present_flag[i] = 0`: inferred via fallback rule.
    NotPresent,
    /// `scaling_list_present_flag[i] = 1` with `useDefaultScalingMatrixFlag = 1`.
    UseDefault,
    /// Explicit values from `delta_scale` loop (stored as absolute values, 16 long).
    Explicit(Vec<u8>),
}

/// `Explicit` variant carries 64-element `Vec<u8>` for the same reason as the 4x4
/// variant; serde does not derive for `[T; N]` with N > 32.
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, schemars::JsonSchema,
)]
pub enum ScalingList8x8Entry {
    NotPresent,
    UseDefault,
    Explicit(Vec<u8>),
}

impl Default for ScalingList4x4Entry {
    fn default() -> Self {
        ScalingList4x4Entry::NotPresent
    }
}

impl Default for ScalingList8x8Entry {
    fn default() -> Self {
        ScalingList8x8Entry::NotPresent
    }
}

/// `seq_scaling_matrix` as coded in the SPS.
///
/// The number of 8x8 entries depends on `chroma_format_idc`: 2 for YUV420/422
/// (luma only) and 6 for YUV444 (includes chroma 8x8).
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Default, schemars::JsonSchema,
)]
pub struct SeqScalingMatrix {
    pub lists_4x4: [ScalingList4x4Entry; 6],
    pub lists_8x8: Vec<ScalingList8x8Entry>,
}

/// `pic_scaling_matrix` as coded in the PPS. 8x8 entries are only present when
/// `transform_8x8_mode_flag = 1`.
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, Default, schemars::JsonSchema,
)]
pub struct PicScalingMatrix {
    pub lists_4x4: [ScalingList4x4Entry; 6],
    pub lists_8x8: Vec<ScalingList8x8Entry>,
}

/// Number of 8x8 scaling lists that the SPS must signal for a given chroma
/// format: 2 for 4:2:0/4:2:2 (luma only), 6 for 4:4:4.
pub fn num_sps_8x8_lists(chroma_format: ChromaFormat) -> usize {
    if chroma_format == ChromaFormat::YUV444 {
        6
    } else {
        2
    }
}

/// Number of 8x8 scaling lists that the PPS signals for a given chroma format
/// and `transform_8x8_mode_flag`.
pub fn num_pps_8x8_lists(chroma_format: ChromaFormat, transform_8x8_mode_flag: bool) -> usize {
    if !transform_8x8_mode_flag {
        0
    } else {
        num_sps_8x8_lists(chroma_format)
    }
}

/// Fully resolved picture-level scaling matrix, ready for use in inverse
/// quantization.
///
/// Indexing:
/// * `lists_4x4[i]` for i in 0..6 per Table 7-2.
/// * `lists_8x8[i]` for i in 0..6, where i = 0 is Intra_Y and i = 1 is Inter_Y;
///   for 4:2:0/4:2:2 only indices 0..2 are meaningful (chroma has no 8x8).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedScalingMatrix {
    pub lists_4x4: [ScalingList4x4; 6],
    pub lists_8x8: [ScalingList8x8; 6],
}

impl Default for ResolvedScalingMatrix {
    fn default() -> Self {
        ResolvedScalingMatrix { lists_4x4: [FLAT_4X4_16; 6], lists_8x8: [FLAT_8X8_16; 6] }
    }
}

impl ResolvedScalingMatrix {
    /// Map (is_inter, plane) to the 4x4 scaling list index per clause 8.5.9:
    /// `ScalingList4x4[iYCbCr + (mbIsInter ? 3 : 0)]`.
    #[inline]
    pub fn list_4x4(&self, is_inter: bool, plane: ColorPlane) -> &ScalingList4x4 {
        let i_ycbcr = plane as usize;
        let idx = i_ycbcr + if is_inter { 3 } else { 0 };
        &self.lists_4x4[idx]
    }

    /// Map (is_inter, plane) to the 8x8 scaling list index per clause 8.5.9:
    /// `ScalingList8x8[2 * iYCbCr + mbIsInter]`.
    #[inline]
    pub fn list_8x8(&self, is_inter: bool, plane: ColorPlane) -> &ScalingList8x8 {
        let i_ycbcr = plane as usize;
        let idx = 2 * i_ycbcr + if is_inter { 1 } else { 0 };
        &self.lists_8x8[idx]
    }

    /// DC-only scaling factor (position 0 of the relevant 4x4 list).
    #[inline]
    pub fn dc_4x4(&self, is_inter: bool, plane: ColorPlane) -> u8 {
        self.list_4x4(is_inter, plane)[0]
    }
}

/// Resolve the effective sequence-level scaling matrix from a raw SPS matrix.
///
/// When `sps_matrix` is `None`, falls back to flat 16 for every list. When
/// present, applies fallback rule set A (Table 7-2): missing lists inherit the
/// default matrix (for indices 0, 3, 6, 7) or the previously resolved list
/// (for 1, 2, 4, 5, 8..11).
pub fn resolve_seq_scaling_matrix(
    sps_matrix: Option<&SeqScalingMatrix>,
    chroma_format: ChromaFormat,
) -> ResolvedScalingMatrix {
    let Some(matrix) = sps_matrix else {
        return ResolvedScalingMatrix::default();
    };

    let mut out = ResolvedScalingMatrix::default();

    // 4x4 lists: indices 0..6.
    // Fallback chain (rule set A): 0 -> Default_4x4_Intra
    //                              1 -> list 0         2 -> list 1
    //                              3 -> Default_4x4_Inter
    //                              4 -> list 3         5 -> list 4
    for i in 0..6 {
        out.lists_4x4[i] = match &matrix.lists_4x4[i] {
            ScalingList4x4Entry::Explicit(v) => vec_to_list_4x4(v),
            ScalingList4x4Entry::UseDefault => {
                if i == 0 || i == 1 || i == 2 {
                    DEFAULT_SCALING_LIST_4X4_INTRA
                } else {
                    DEFAULT_SCALING_LIST_4X4_INTER
                }
            }
            ScalingList4x4Entry::NotPresent => match i {
                0 => DEFAULT_SCALING_LIST_4X4_INTRA,
                3 => DEFAULT_SCALING_LIST_4X4_INTER,
                _ => out.lists_4x4[i - 1],
            },
        };
    }

    // 8x8 lists. The SPS only signals 2 entries for 4:2:0/4:2:2 (luma only),
    // or 6 for 4:4:4. Missing/non-luma entries default to flat_8x8_16 but are
    // never indexed outside 4:4:4 so the value is inert.
    let n_sps = num_sps_8x8_lists(chroma_format);
    for i in 0..6 {
        if i >= n_sps {
            // Indices 2..6 only meaningful in 4:4:4; leave as flat.
            out.lists_8x8[i] = FLAT_8X8_16;
            continue;
        }
        let entry = matrix.lists_8x8.get(i).cloned().unwrap_or(ScalingList8x8Entry::NotPresent);
        out.lists_8x8[i] = match entry {
            ScalingList8x8Entry::Explicit(v) => vec_to_list_8x8(&v),
            ScalingList8x8Entry::UseDefault => {
                // For indices 6..11 (i.e. our i=0..5), intra = even index,
                // inter = odd index per Table 7-2.
                if i % 2 == 0 {
                    DEFAULT_SCALING_LIST_8X8_INTRA
                } else {
                    DEFAULT_SCALING_LIST_8X8_INTER
                }
            }
            ScalingList8x8Entry::NotPresent => match i {
                0 => DEFAULT_SCALING_LIST_8X8_INTRA,
                1 => DEFAULT_SCALING_LIST_8X8_INTER,
                _ => out.lists_8x8[i - 2],
            },
        };
    }

    out
}

fn vec_to_list_4x4(v: &[u8]) -> ScalingList4x4 {
    let mut out = FLAT_4X4_16;
    let n = v.len().min(16);
    out[..n].copy_from_slice(&v[..n]);
    out
}

fn vec_to_list_8x8(v: &[u8]) -> ScalingList8x8 {
    let mut out = FLAT_8X8_16;
    let n = v.len().min(64);
    out[..n].copy_from_slice(&v[..n]);
    out
}

/// Resolve the effective picture-level scaling matrix from the SPS-resolved
/// matrix and an optional PPS matrix.
///
/// Per clause 7.4.2.2.1, the fallback for a PPS entry with `pic_scaling_list_present_flag = 0`
/// depends on whether the referenced SPS signalled its own scaling matrix:
/// * `sps_has_matrix = true`: fallback rule set B (inherit from SPS-resolved).
/// * `sps_has_matrix = false`: fallback rule set A (inherit from Default_*).
///
/// `sps_resolved` must already reflect rule A's application at the SPS level
/// when `sps_has_matrix` is true. When `sps_has_matrix` is false the caller
/// typically passes a flat-16 matrix; this function still ignores it in favour
/// of the Default_* matrices for the base entries.
pub fn resolve_pic_scaling_matrix(
    sps_resolved: &ResolvedScalingMatrix,
    sps_has_matrix: bool,
    pps_matrix: Option<&PicScalingMatrix>,
    transform_8x8_mode_flag: bool,
    chroma_format: ChromaFormat,
) -> ResolvedScalingMatrix {
    let Some(matrix) = pps_matrix else {
        return sps_resolved.clone();
    };

    let mut out = sps_resolved.clone();

    // 4x4 lists. For each `NotPresent` entry, the base (indices 0 and 3) comes
    // from rule B (SPS-resolved) or rule A (Default_*) depending on
    // `sps_has_matrix`. Indices 1, 2, 4, 5 inherit from the prior resolved
    // picture-level list either way.
    for i in 0..6 {
        out.lists_4x4[i] = match &matrix.lists_4x4[i] {
            ScalingList4x4Entry::Explicit(v) => vec_to_list_4x4(v),
            ScalingList4x4Entry::UseDefault => {
                if i < 3 {
                    DEFAULT_SCALING_LIST_4X4_INTRA
                } else {
                    DEFAULT_SCALING_LIST_4X4_INTER
                }
            }
            ScalingList4x4Entry::NotPresent => match i {
                0 => {
                    if sps_has_matrix {
                        sps_resolved.lists_4x4[0]
                    } else {
                        DEFAULT_SCALING_LIST_4X4_INTRA
                    }
                }
                3 => {
                    if sps_has_matrix {
                        sps_resolved.lists_4x4[3]
                    } else {
                        DEFAULT_SCALING_LIST_4X4_INTER
                    }
                }
                _ => out.lists_4x4[i - 1],
            },
        };
    }

    // 8x8 lists: only signalled in the PPS if transform_8x8_mode_flag is set.
    let n_pps = num_pps_8x8_lists(chroma_format, transform_8x8_mode_flag);
    for i in 0..6 {
        if i >= n_pps {
            // Not signalled in PPS -- keep SPS-resolved value.
            continue;
        }
        let entry = matrix.lists_8x8.get(i).cloned().unwrap_or(ScalingList8x8Entry::NotPresent);
        out.lists_8x8[i] = match entry {
            ScalingList8x8Entry::Explicit(v) => vec_to_list_8x8(&v),
            ScalingList8x8Entry::UseDefault => {
                if i % 2 == 0 {
                    DEFAULT_SCALING_LIST_8X8_INTRA
                } else {
                    DEFAULT_SCALING_LIST_8X8_INTER
                }
            }
            ScalingList8x8Entry::NotPresent => match i {
                0 => {
                    if sps_has_matrix {
                        sps_resolved.lists_8x8[0]
                    } else {
                        DEFAULT_SCALING_LIST_8X8_INTRA
                    }
                }
                1 => {
                    if sps_has_matrix {
                        sps_resolved.lists_8x8[1]
                    } else {
                        DEFAULT_SCALING_LIST_8X8_INTER
                    }
                }
                _ => out.lists_8x8[i - 2],
            },
        };
    }

    out
}

/// Decode a single `scaling_list()` syntax block (clause 7.3.2.1.1.1). Returns
/// the decoded values (always 16 or 64 coefficients depending on caller) and
/// the `useDefaultScalingMatrixFlag`.
pub fn parse_scaling_list(
    reader: &mut RbspReader,
    size: usize,
) -> ParseResult<(Vec<u8>, bool)> {
    let mut scaling_list = vec![0u8; size];
    let mut last_scale: i32 = 8;
    let mut next_scale: i32 = 8;
    let mut use_default = false;
    for j in 0..size {
        if next_scale != 0 {
            let delta_scale: i32 = reader.se()?;
            if !(-128..=127).contains(&delta_scale) {
                return Err(format!(
                    "delta_scale {} out of range [-128, 127]",
                    delta_scale
                ));
            }
            next_scale = (last_scale + delta_scale + 256) % 256;
            if j == 0 && next_scale == 0 {
                use_default = true;
            }
        }
        scaling_list[j] = if next_scale == 0 { last_scale as u8 } else { next_scale as u8 };
        last_scale = scaling_list[j] as i32;
    }
    Ok((scaling_list, use_default))
}

/// Produce a 2D `[[u8; 8]; 8]` weight-scale matrix from a 1D zig-zag list, for
/// use in 8x8 inverse quantization (where the residual block is 2D).
pub fn weight_scale_8x8_2d(list: &ScalingList8x8) -> [[u8; 8]; 8] {
    let mut out = [[0u8; 8]; 8];
    for idx in 0..64 {
        let (r, c) = super::residual::un_zig_zag_8x8(idx);
        out[r][c] = list[idx];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults_not_confused() {
        // Sanity check: the four default matrices are all distinct.
        assert_ne!(DEFAULT_SCALING_LIST_4X4_INTRA, DEFAULT_SCALING_LIST_4X4_INTER);
        assert_ne!(DEFAULT_SCALING_LIST_8X8_INTRA, DEFAULT_SCALING_LIST_8X8_INTER);
        assert_eq!(DEFAULT_SCALING_LIST_4X4_INTRA.len(), 16);
        assert_eq!(DEFAULT_SCALING_LIST_4X4_INTER.len(), 16);
        assert_eq!(DEFAULT_SCALING_LIST_8X8_INTRA.len(), 64);
        assert_eq!(DEFAULT_SCALING_LIST_8X8_INTER.len(), 64);
    }

    #[test]
    fn test_resolve_sps_no_matrix_is_flat() {
        let r = resolve_seq_scaling_matrix(None, ChromaFormat::YUV420);
        for list in &r.lists_4x4 {
            assert_eq!(list, &FLAT_4X4_16);
        }
        for list in &r.lists_8x8 {
            assert_eq!(list, &FLAT_8X8_16);
        }
    }

    #[test]
    fn test_resolve_sps_fallback_rule_a_cascades() {
        let matrix = SeqScalingMatrix {
            lists_4x4: core::array::from_fn(|_| ScalingList4x4Entry::NotPresent),
            lists_8x8: vec![ScalingList8x8Entry::NotPresent; 2],
        };
        let r = resolve_seq_scaling_matrix(Some(&matrix), ChromaFormat::YUV420);
        // i=0 -> Default_4x4_Intra, i=1 -> list 0 = Default_4x4_Intra, i=2 -> same
        assert_eq!(r.lists_4x4[0], DEFAULT_SCALING_LIST_4X4_INTRA);
        assert_eq!(r.lists_4x4[1], DEFAULT_SCALING_LIST_4X4_INTRA);
        assert_eq!(r.lists_4x4[2], DEFAULT_SCALING_LIST_4X4_INTRA);
        // i=3 -> Default_4x4_Inter, i=4,5 cascade
        assert_eq!(r.lists_4x4[3], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_4x4[4], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_4x4[5], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_8x8[0], DEFAULT_SCALING_LIST_8X8_INTRA);
        assert_eq!(r.lists_8x8[1], DEFAULT_SCALING_LIST_8X8_INTER);
    }

    #[test]
    fn test_resolve_sps_use_default() {
        let matrix = SeqScalingMatrix {
            lists_4x4: core::array::from_fn(|_| ScalingList4x4Entry::UseDefault),
            lists_8x8: vec![ScalingList8x8Entry::UseDefault; 2],
        };
        let r = resolve_seq_scaling_matrix(Some(&matrix), ChromaFormat::YUV420);
        // Indices 0..3 are "intra variants" per table 7-2.
        assert_eq!(r.lists_4x4[0], DEFAULT_SCALING_LIST_4X4_INTRA);
        assert_eq!(r.lists_4x4[1], DEFAULT_SCALING_LIST_4X4_INTRA);
        assert_eq!(r.lists_4x4[2], DEFAULT_SCALING_LIST_4X4_INTRA);
        assert_eq!(r.lists_4x4[3], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_4x4[4], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_4x4[5], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_8x8[0], DEFAULT_SCALING_LIST_8X8_INTRA);
        assert_eq!(r.lists_8x8[1], DEFAULT_SCALING_LIST_8X8_INTER);
    }

    #[test]
    fn test_resolve_pps_no_matrix_copies_sps() {
        let sps = ResolvedScalingMatrix {
            lists_4x4: [DEFAULT_SCALING_LIST_4X4_INTRA; 6],
            lists_8x8: [DEFAULT_SCALING_LIST_8X8_INTRA; 6],
        };
        let r = resolve_pic_scaling_matrix(&sps, true, None, true, ChromaFormat::YUV420);
        assert_eq!(r, sps);
    }

    #[test]
    fn test_resolve_pps_fallback_rule_b_inherits_sps() {
        // Rule B: PPS list i with NotPresent inherits from prior PPS list (for
        // 1,2,4,5) or from matching SPS list (for 0, 3).
        let sps = ResolvedScalingMatrix {
            lists_4x4: [
                [100; 16], // SPS[0]
                [101; 16], // SPS[1]
                [102; 16], // SPS[2]
                [103; 16], // SPS[3]
                [104; 16], // SPS[4]
                [105; 16], // SPS[5]
            ],
            lists_8x8: [FLAT_8X8_16; 6],
        };
        let pps_matrix = PicScalingMatrix {
            lists_4x4: core::array::from_fn(|_| ScalingList4x4Entry::NotPresent),
            lists_8x8: vec![],
        };
        let r = resolve_pic_scaling_matrix(
            &sps,
            true,
            Some(&pps_matrix),
            false,
            ChromaFormat::YUV420,
        );
        assert_eq!(r.lists_4x4[0], [100; 16]); // from SPS[0]
        assert_eq!(r.lists_4x4[1], [100; 16]); // inherit prior PPS[0]
        assert_eq!(r.lists_4x4[2], [100; 16]); // inherit prior PPS[1]
        assert_eq!(r.lists_4x4[3], [103; 16]); // from SPS[3]
        assert_eq!(r.lists_4x4[4], [103; 16]);
        assert_eq!(r.lists_4x4[5], [103; 16]);
    }

    #[test]
    fn test_list_4x4_indexing_follows_spec() {
        let mut m = ResolvedScalingMatrix::default();
        for i in 0..6 {
            m.lists_4x4[i] = [i as u8; 16];
        }
        // Intra Y -> 0, Intra Cb -> 1, Intra Cr -> 2
        assert_eq!(m.list_4x4(false, ColorPlane::Y)[0], 0);
        assert_eq!(m.list_4x4(false, ColorPlane::Cb)[0], 1);
        assert_eq!(m.list_4x4(false, ColorPlane::Cr)[0], 2);
        // Inter Y -> 3, Inter Cb -> 4, Inter Cr -> 5
        assert_eq!(m.list_4x4(true, ColorPlane::Y)[0], 3);
        assert_eq!(m.list_4x4(true, ColorPlane::Cb)[0], 4);
        assert_eq!(m.list_4x4(true, ColorPlane::Cr)[0], 5);
    }

    #[test]
    fn test_list_8x8_indexing_follows_spec() {
        let mut m = ResolvedScalingMatrix::default();
        for i in 0..6 {
            m.lists_8x8[i] = [i as u8; 64];
        }
        assert_eq!(m.list_8x8(false, ColorPlane::Y)[0], 0);
        assert_eq!(m.list_8x8(true, ColorPlane::Y)[0], 1);
        assert_eq!(m.list_8x8(false, ColorPlane::Cb)[0], 2);
        assert_eq!(m.list_8x8(true, ColorPlane::Cb)[0], 3);
        assert_eq!(m.list_8x8(false, ColorPlane::Cr)[0], 4);
        assert_eq!(m.list_8x8(true, ColorPlane::Cr)[0], 5);
    }

    #[test]
    fn test_weight_scale_8x8_2d_inverse_zigzag() {
        let mut list: ScalingList8x8 = [0u8; 64];
        // Put a 1 at idx=0; after inverse zig-zag that should land at (0, 0).
        list[0] = 1;
        // Put a 2 at idx=1; after inverse zig-zag that should land at (0, 1).
        list[1] = 2;
        // Put a 3 at idx=2; after inverse zig-zag that should land at (1, 0).
        list[2] = 3;
        let m = weight_scale_8x8_2d(&list);
        assert_eq!(m[0][0], 1);
        assert_eq!(m[0][1], 2);
        assert_eq!(m[1][0], 3);
    }

    #[test]
    fn test_parse_scaling_list_flat_matrix_via_zero_deltas() {
        // 16 consecutive delta_scale = 0 (se(0) = one bit `1`).
        // With last_scale = next_scale = 8, all values stay at 8.
        let bytes = [0xFFu8, 0xFF]; // 16 ones
        let mut reader = RbspReader::new(&bytes);
        let (list, use_default) = parse_scaling_list(&mut reader, 16).unwrap();
        assert!(!use_default);
        assert_eq!(list, vec![8u8; 16]);
    }

    #[test]
    fn test_parse_scaling_list_use_default_sentinel() {
        // se(-8) encodes to ue(16) -> 9 bits: `000010001`, padded to 16 bits.
        // With last_scale = 8, delta = -8 -> next_scale = 0 at j = 0, which
        // sets useDefaultScalingMatrixFlag per spec.
        let bytes = [0b00001000u8, 0b10000000];
        let mut reader = RbspReader::new(&bytes);
        let (_list, use_default) = parse_scaling_list(&mut reader, 16).unwrap();
        assert!(use_default);
    }

    #[test]
    fn test_use_default_decodes_to_default_when_resolved() {
        // When the SPS matrix has UseDefault at index 0, the resolved list 0
        // should equal Default_4x4_Intra (not Flat_4x4_16).
        let matrix = SeqScalingMatrix {
            lists_4x4: {
                let mut arr: [ScalingList4x4Entry; 6] =
                    core::array::from_fn(|_| ScalingList4x4Entry::NotPresent);
                arr[0] = ScalingList4x4Entry::UseDefault;
                arr
            },
            lists_8x8: vec![ScalingList8x8Entry::NotPresent; 2],
        };
        let r = resolve_seq_scaling_matrix(Some(&matrix), ChromaFormat::YUV420);
        assert_eq!(r.lists_4x4[0], DEFAULT_SCALING_LIST_4X4_INTRA);
    }

    #[test]
    fn test_resolve_pps_without_sps_matrix_falls_back_to_defaults() {
        // When the PPS has `pic_scaling_matrix_present_flag = 1` but all the
        // list flags are 0 AND the SPS had no scaling matrix, rule A (not B)
        // applies: list 0 -> Default_4x4_Intra, list 3 -> Default_4x4_Inter,
        // list 6 -> Default_8x8_Intra, list 7 -> Default_8x8_Inter.
        let sps_flat = ResolvedScalingMatrix::default();
        let pps_matrix = PicScalingMatrix {
            lists_4x4: core::array::from_fn(|_| ScalingList4x4Entry::NotPresent),
            lists_8x8: vec![ScalingList8x8Entry::NotPresent; 2],
        };
        let r = resolve_pic_scaling_matrix(
            &sps_flat,
            false, // sps_has_matrix
            Some(&pps_matrix),
            true, // transform_8x8_mode_flag
            ChromaFormat::YUV420,
        );
        assert_eq!(r.lists_4x4[0], DEFAULT_SCALING_LIST_4X4_INTRA);
        assert_eq!(r.lists_4x4[1], DEFAULT_SCALING_LIST_4X4_INTRA); // cascade via prior
        assert_eq!(r.lists_4x4[2], DEFAULT_SCALING_LIST_4X4_INTRA);
        assert_eq!(r.lists_4x4[3], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_4x4[4], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_4x4[5], DEFAULT_SCALING_LIST_4X4_INTER);
        assert_eq!(r.lists_8x8[0], DEFAULT_SCALING_LIST_8X8_INTRA);
        assert_eq!(r.lists_8x8[1], DEFAULT_SCALING_LIST_8X8_INTER);
    }

    #[test]
    fn test_resolve_sps_explicit_4x4_values_round_through() {
        // Explicit list values should pass through the resolver unchanged
        // (modulo the Vec<u8> -> [u8; 16] conversion).
        let list: Vec<u8> = (1..=16).collect();
        let mut lists_4x4: [ScalingList4x4Entry; 6] =
            core::array::from_fn(|_| ScalingList4x4Entry::NotPresent);
        lists_4x4[0] = ScalingList4x4Entry::Explicit(list.clone());
        let matrix = SeqScalingMatrix {
            lists_4x4,
            lists_8x8: vec![ScalingList8x8Entry::NotPresent; 2],
        };
        let r = resolve_seq_scaling_matrix(Some(&matrix), ChromaFormat::YUV420);
        let mut expected = FLAT_4X4_16;
        expected.copy_from_slice(&list);
        assert_eq!(r.lists_4x4[0], expected);
    }
}
