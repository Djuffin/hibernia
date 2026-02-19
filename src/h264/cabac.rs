use log::trace;
use super::tables::{
    get_init_table, RANGE_TAB_LPS, TRANS_IDX_LPS, TRANS_IDX_MPS,
};
use super::macroblock::{
    CbfInfo, CodedBlockPattern, MbAddr, MbMotion, MbNeighborName, MotionVector, PartitionInfo,
    PcmMb, SubMbType,
};
use super::parser::{BitReader, ParseResult};
use super::residual::Residual;
use super::slice::Slice;
use std::cmp::min;

struct CurrentMbInfo {
    mb_type: CabacMbType,
    motion: MbMotion,
    coded_block_pattern: CodedBlockPattern,
    transform_size_8x8_flag: bool,
    cbf: CbfInfo,
}

struct NeighborAccessor<'a> {
    slice: &'a Slice,
    mb_addr: MbAddr,
    curr_mb: &'a CurrentMbInfo,
}

impl<'a> NeighborAccessor<'a> {
    fn new(slice: &'a Slice, mb_addr: MbAddr, curr_mb: &'a CurrentMbInfo) -> Self {
        NeighborAccessor {
            slice,
            mb_addr,
            curr_mb,
        }
    }

    fn get_mb_type_is_intra(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => match self.curr_mb.mb_type {
                CabacMbType::I(_) => true,
                CabacMbType::P(_) => false,
            },
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| mb.is_intra())
                .unwrap_or(false),
        }
    }

    fn get_mb_type_is_skipped(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => match self.curr_mb.mb_type {
                CabacMbType::P(super::macroblock::PMbType::P_Skip) => true,
                _ => false,
            },
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| mb.is_skipped())
                .unwrap_or(false),
        }
    }

    fn is_available(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => true, // In current MB
            Some(nb_name) => self.slice.has_mb_neighbor(self.mb_addr, nb_name),
        }
    }

    fn is_intra_16x16(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => match self.curr_mb.mb_type {
                CabacMbType::I(t) => {
                    t != super::macroblock::IMbType::I_NxN
                        && t != super::macroblock::IMbType::I_PCM
                }
                _ => false,
            },
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| match mb {
                    super::macroblock::Macroblock::I(m) => {
                        m.mb_type != super::macroblock::IMbType::I_NxN
                            && m.mb_type != super::macroblock::IMbType::I_PCM
                    }
                    _ => false,
                })
                .unwrap_or(false),
        }
    }

    fn get_ref_idx(&self, blk_idx: u8, neighbor_name: MbNeighborName, list_idx: usize) -> Option<u8> {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => {
                // Current MB
                let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                let x = (p.x / 4) as usize;
                let y = (p.y / 4) as usize;
                let p_info = &self.curr_mb.motion.partitions[y][x];
                // Only L0 supported for now
                if list_idx == 0 {
                    Some(p_info.ref_idx_l0)
                } else {
                    unimplemented!("B-slice L1 reference support in get_ref_idx")
                }
            }
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| match mb {
                    super::macroblock::Macroblock::P(pmb) => {
                        let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                        let x = (p.x / 4) as usize;
                        let y = (p.y / 4) as usize;
                        // Only L0 supported for now
                        if list_idx == 0 {
                            pmb.motion.partitions[y][x].ref_idx_l0
                        } else {
                            unimplemented!("B-slice L1 reference support in get_ref_idx")
                        }
                    }
                    super::macroblock::Macroblock::I(_)
                    | super::macroblock::Macroblock::PCM(_) => 0, // Intra has no ref_idx
                }),
        }
    }

    fn get_mvd(
        &self,
        blk_idx: u8,
        neighbor_name: MbNeighborName,
        list_idx: usize,
    ) -> Option<MotionVector> {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => {
                let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                let x = (p.x / 4) as usize;
                let y = (p.y / 4) as usize;
                let p_info = &self.curr_mb.motion.partitions[y][x];
                if list_idx == 0 {
                    Some(p_info.mvd_l0)
                } else {
                    unimplemented!("B-slice L1 reference support in get_mvd")
                }
            }
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| match mb {
                    super::macroblock::Macroblock::P(pmb) => {
                        let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                        let x = (p.x / 4) as usize;
                        let y = (p.y / 4) as usize;
                        if list_idx == 0 {
                            pmb.motion.partitions[y][x].mvd_l0
                        } else {
                            unimplemented!("B-slice L1 reference support in get_mvd")
                        }
                    }
                    _ => MotionVector::default(),
                }),
        }
    }

    fn get_cbp(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> Option<CodedBlockPattern> {
         let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => Some(self.curr_mb.coded_block_pattern),
            Some(nb_name) => self.slice.get_mb_neighbor(self.mb_addr, nb_name).map(|mb| mb.get_coded_block_pattern()),
        }
    }

    fn is_pcm(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
         let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => matches!(self.curr_mb.mb_type, CabacMbType::I(super::macroblock::IMbType::I_PCM)),
            Some(nb_name) => self.slice.get_mb_neighbor(self.mb_addr, nb_name).map(|mb| matches!(mb, super::macroblock::Macroblock::PCM(_))).unwrap_or(false),
        }
    }

    fn get_cbf(
        &self,
        blk_idx: u8,
        neighbor_name: MbNeighborName,
        ctx_block_cat: usize,
        comp_idx: usize,
    ) -> Option<bool> {
        let (neighbor_blk_idx, mb_neighbor) = if ctx_block_cat == 3 || ctx_block_cat == 4 {
            super::macroblock::get_4x4chroma_block_neighbor(blk_idx, neighbor_name)
        } else {
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name)
        };

        match mb_neighbor {
            None => {
                match ctx_block_cat {
                    0 => Some(self.curr_mb.cbf.luma_dc),
                    1 | 2 | 5 => Some((self.curr_mb.cbf.luma_ac >> neighbor_blk_idx) & 1 != 0),
                    3 => {
                        if comp_idx == 0 {
                            Some(self.curr_mb.cbf.cb_dc)
                        } else {
                            Some(self.curr_mb.cbf.cr_dc)
                        }
                    }
                    4 => {
                        if comp_idx == 0 {
                            Some((self.curr_mb.cbf.cb_ac >> neighbor_blk_idx) & 1 != 0)
                        } else {
                            Some((self.curr_mb.cbf.cr_ac >> neighbor_blk_idx) & 1 != 0)
                        }
                    }
                    _ => Some(false),
                }
            }
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| {
                    let cbf_info = mb.get_cbf_info();
                    match ctx_block_cat {
                        0 => cbf_info.luma_dc,
                        1 | 2 | 5 => (cbf_info.luma_ac >> neighbor_blk_idx) & 1 != 0,
                        3 => if comp_idx == 0 { cbf_info.cb_dc } else { cbf_info.cr_dc },
                        4 => if comp_idx == 0 { (cbf_info.cb_ac >> neighbor_blk_idx) & 1 != 0 } else { (cbf_info.cr_ac >> neighbor_blk_idx) & 1 != 0 },
                        _ => false,
                    }
                }),
        }
    }

    fn get_transform_size_8x8_flag(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => self.curr_mb.transform_size_8x8_flag,
            Some(nb_name) => self.slice.get_mb_neighbor(self.mb_addr, nb_name).map(|mb| match mb {
                super::macroblock::Macroblock::I(m) => m.transform_size_8x8_flag,
                super::macroblock::Macroblock::P(m) => m.transform_size_8x8_flag,
                super::macroblock::Macroblock::PCM(_) => false,
            }).unwrap_or(false),
        }
    }
}

pub struct CabacContext<'a, 'b> {
    reader: &'a mut BitReader<'b>,
    range: u32,  // codIRange
    offset: u32, // codIOffset
    // Context models: (pStateIdx, valMPS)
    // 1024 context models as per spec (max ctxIdx is 1023)
    // Stored as u8: bit 0 is valMPS, bits 1-7 are pStateIdx
    ctx_table: [u8; 1024],
}

/// Parameters required to derive the Context Index Increment (ctxIdxInc) for a given bin.
/// This encapsulates the state dependencies specified in Table 9-39 (and related clauses 9.3.3.1.x)
/// of the H.264 specification.
#[derive(Clone, Copy, Debug)]
pub enum CtxIncParams {
    /// Standard context derivation where the `ctxIdxInc` for the first bin (binIdx 0)
    /// is derived from neighboring blocks (A and B), and subsequent bins use fixed increments.
    /// Used by: MbQpDelta, RefIdx, Mvd, IntraChromaPredMode.
    Standard(usize),

    /// Specialized context derivation for `coeff_abs_level_minus1`.
    /// Depends on the number of previously decoded coefficients with absolute level > 1 (`gt1`)
    /// and equal to 1 (`eq1`). See clause 9.3.3.1.3.
    AbsLevel { gt1: usize, eq1: usize },

    /// Context derivation for `mb_type`, where `ctxIdxInc` depends on the value of previously
    /// decoded bins (`prior`) within the same syntax element's binarization tree.
    /// See Table 9-39 (ctxIdxOffset 3, 14, 17) and clause 9.3.3.1.2.
    MbType {
        prior: u8,
    },
}

impl<'a, 'b> CabacContext<'a, 'b> {
    pub fn new(reader: &'a mut BitReader<'b>, slice: &Slice) -> ParseResult<Self> {
        let mut ctx = CabacContext {
            reader,
            range: 510,
            offset: 0,
            ctx_table: [0; 1024],
        };

        ctx.init_context_variables(slice);
        ctx.reader.align();
        ctx.init_decoding_engine()?;

        Ok(ctx)
    }

    // 9.3.1.1 Initialization process for context variables
    fn init_context_variables(&mut self, slice: &Slice) {
        let slice_qp_y = slice.slice_qp_y();
        let cabac_init_idc = slice.header.cabac_init_idc;

        let init_table = get_init_table(slice.header.slice_type, cabac_init_idc as u8);

        let qp_clipped = slice_qp_y.clamp(0, 51);

        for ctx_idx in 0..1024 {
            let init_val = init_table[ctx_idx];
            let m = init_val.0 as i32;
            let n = init_val.1 as i32;

            let pre_ctx_state = ((m * qp_clipped >> 4) + n).clamp(1, 126);

            let (p_state_idx, val_mps) = if pre_ctx_state <= 63 {
                (63 - pre_ctx_state, 0)
            } else {
                (pre_ctx_state - 64, 1)
            };

            self.ctx_table[ctx_idx] = (p_state_idx as u8) << 1 | (val_mps as u8);
        }
    }

    // 9.3.1.2 Initialization process for the arithmetic decoding engine
    fn init_decoding_engine(&mut self) -> ParseResult<()> {
        self.range = 510;
        self.offset = self.reader.u(9)? as u32;

        if self.offset == 510 || self.offset == 511 {
            // "The bitstream shall not contain data that result in a value of codIOffset being equal to 510 or 511."
            return Err("codIOffset equal to 510 or 511 is illegal".to_string());
        }

        Ok(())
    }

    // 9.3.2.1 Unary (U) binarization process
    pub fn parse_unary_bin(
        &mut self,
        se: SyntaxElement,
        ctx: CtxIncParams,
    ) -> ParseResult<u32> {
        let props = get_syntax_element_properties(se);
        let max_bin_idx_ctx = props.max_bin_idx_ctx;
        let ctx_idx_offset = props.ctx_idx_offset as usize;

        let mut bin_idx = 0;
        loop {
            // 9.3.3.1: All bins with binIdx greater than maxBinIdxCtx are parsed using the value of ctxIdx being assigned to binIdx equal to maxBinIdxCtx.
            let effective_bin_idx = std::cmp::min(bin_idx, max_bin_idx_ctx);
            let ctx_idx_inc = Self::get_ctx_idx_inc(se, effective_bin_idx, ctx);
            let ctx_idx = ctx_idx_offset + ctx_idx_inc;
            let bin = self.decode_bin(ctx_idx)?;

            if bin == 0 {
                trace!("parse_unary_bin se={:?} val={}", se, bin_idx);
                return Ok(bin_idx);
            }
            bin_idx += 1;
        }
    }

    // 9.3.2.2 Truncated unary (TU) binarization process
    pub fn parse_truncated_unary_bin(
        &mut self,
        se: SyntaxElement,
        c_max_override: Option<u32>,
        ctx: CtxIncParams,
    ) -> ParseResult<u32> {
        let props = get_syntax_element_properties(se);
        let max_bin_idx_ctx = props.max_bin_idx_ctx;
        let ctx_idx_offset = props.ctx_idx_offset as usize;
        let c_max = c_max_override.unwrap_or_else(|| {
            if let BinarizationType::TU { c_max } = props.binarization {
                c_max
            } else {
                panic!("parse_truncated_unary_bin called on non-TU syntax element without override: {:?}", se);
            }
        });

        let mut bin_idx = 0;
        while bin_idx < c_max {
            let effective_bin_idx = std::cmp::min(bin_idx, max_bin_idx_ctx);
            let ctx_idx_inc = Self::get_ctx_idx_inc(se, effective_bin_idx, ctx);
            let ctx_idx = ctx_idx_offset + ctx_idx_inc;
            let bin = self.decode_bin(ctx_idx)?;

            if bin == 0 {
                trace!("parse_truncated_unary_bin se={:?} val={}", se, bin_idx);
                return Ok(bin_idx);
            }
            bin_idx += 1;
        }
        trace!("parse_truncated_unary_bin se={:?} val={}", se, c_max);
        Ok(c_max)
    }

    // 9.3.2.3 Concatenated unary/ k-th order Exp-Golomb (UEGk) binarization process
    pub fn parse_ueg_k(
        &mut self,
        se: SyntaxElement,
        ctx: CtxIncParams,
    ) -> ParseResult<i32> {
        let props = get_syntax_element_properties(se);
        let (u_coff, k_val, signed_val_flag) = if let BinarizationType::UEGk {
            u_coff,
            k,
            signed_val_flag,
        } = props.binarization
        {
            (u_coff, k, signed_val_flag)
        } else {
            panic!("parse_ueg_k called on non-UEGk syntax element: {:?}", se);
        };

        // Prefix: TU with cMax = uCoff
        let prefix = self.parse_truncated_unary_bin(se, Some(u_coff), ctx)?;

        if prefix < u_coff {
            let val = prefix as i32;
            let final_val = if signed_val_flag && val != 0 {
                let sign = self.decode_bypass()?;
                if sign == 1 { -val } else { val }
            } else {
                val
            };
            trace!("parse_ueg_k se={:?} val={}", se, final_val);
            return Ok(final_val);
        }

        // Suffix: EGk
        let mut suffix_val = 0;
        let mut k = k_val;
        loop {
            let bit = self.decode_bypass()?;
            if bit == 1 {
                suffix_val += 1 << k;
                k += 1;
            } else {
                let mut rem = 0;
                for _ in 0..k {
                    rem = (rem << 1) | self.decode_bypass()? as u32;
                }
                suffix_val += rem;
                break;
            }
        }

        let val = (prefix + suffix_val) as i32;

        let final_val = if signed_val_flag && val != 0 {
            let sign = self.decode_bypass()?;
            if sign == 1 { -val } else { val }
        } else {
            val
        };
        trace!("parse_ueg_k se={:?} val={}", se, final_val);
        Ok(final_val)
    }

    // 9.3.3.2 Arithmetic decoding process
    pub fn decode_bin(&mut self, ctx_idx: usize) -> ParseResult<u8> {
        let ctx_state = self.ctx_table[ctx_idx];
        let mut p_state_idx = (ctx_state >> 1) as usize;
        let mut val_mps = ctx_state & 1;

        let q_cod_i_range_idx = (self.range >> 6) & 3;
        let cod_i_range_lps = RANGE_TAB_LPS[p_state_idx][q_cod_i_range_idx as usize] as u32;

        self.range -= cod_i_range_lps;

        let bin_val;
        if self.offset >= self.range {
            bin_val = 1 - val_mps;
            self.offset -= self.range;
            self.range = cod_i_range_lps;
        } else {
            bin_val = val_mps;
        }

        // 9.3.3.2.1.1 State transition process
        if bin_val == val_mps {
            p_state_idx = TRANS_IDX_MPS[p_state_idx] as usize;
        } else {
            if p_state_idx == 0 {
                val_mps = 1 - val_mps;
            }
            p_state_idx = TRANS_IDX_LPS[p_state_idx] as usize;
        }

        self.ctx_table[ctx_idx] = (p_state_idx as u8) << 1 | val_mps;

        // 9.3.3.2.2 Renormalization process
        self.renorm()?;
        trace!("decode_bin ctxIdx={} bin={}", ctx_idx, bin_val);
        Ok(bin_val)
    }

    // 9.3.3.2.3 Bypass decoding process
    pub fn decode_bypass(&mut self) -> ParseResult<u8> {
        self.offset = (self.offset << 1) | (self.reader.u(1)? as u32);

        let bin_val;
        if self.offset >= self.range {
            bin_val = 1;
            self.offset -= self.range;
        } else {
            bin_val = 0;
        }

        trace!("decode_bypass bin={}", bin_val);
        Ok(bin_val)
    }

    // 9.3.3.2.4 Decoding process for binary decisions before termination
    pub fn decode_terminate(&mut self) -> ParseResult<bool> {
        self.range -= 2;

        let bin_val = if self.offset >= self.range {
            true
        } else {
            self.renorm()?;
            false
        };
        trace!("decode_terminate bin={}", bin_val);
        Ok(bin_val)
    }

    // 9.3.3.1.1.1 Derivation process of ctxIdxInc for the syntax element mb_skip_flag
    pub fn get_ctx_idx_inc_mb_skip_flag(slice: &Slice, mb_addr: MbAddr) -> usize {
        let cond_term_flag_a = slice
            .get_mb_neighbor(mb_addr, MbNeighborName::A)
            .map(|mb| !mb.is_skipped())
            .unwrap_or(false);
        let cond_term_flag_b = slice
            .get_mb_neighbor(mb_addr, MbNeighborName::B)
            .map(|mb| !mb.is_skipped())
            .unwrap_or(false);

        (cond_term_flag_a as usize) + (cond_term_flag_b as usize)
    }

    // 9.3.3.1.1.5 Derivation process of ctxIdxInc for the syntax element mb_qp_delta
    pub fn get_ctx_idx_inc_mb_qp_delta(slice: &Slice, mb_addr: MbAddr) -> usize {
        if mb_addr <= slice.header.first_mb_in_slice {
            return 0;
        }
        let prev_mb_addr = mb_addr - 1;
        let prev_mb = match slice.get_mb(prev_mb_addr) {
            Some(mb) => mb,
            None => return 0,
        };

        if prev_mb.is_skipped() {
            return 0;
        }
        if matches!(prev_mb, super::macroblock::Macroblock::PCM(_)) {
            return 0;
        }

        // We need to separate Intra_16x16 check from CBP check
        let is_intra_16x16 = match prev_mb {
            super::macroblock::Macroblock::I(m) => {
                m.mb_type != super::macroblock::IMbType::I_NxN
                    && m.mb_type != super::macroblock::IMbType::I_PCM
            }
            _ => false,
        };

        let (cbp_luma, cbp_chroma, mb_qp_delta) = match prev_mb {
            super::macroblock::Macroblock::I(m) => {
                (m.coded_block_pattern.luma(), m.coded_block_pattern.chroma(), m.mb_qp_delta)
            }
            super::macroblock::Macroblock::P(m) => {
                (m.coded_block_pattern.luma(), m.coded_block_pattern.chroma(), m.mb_qp_delta)
            }
            _ => unreachable!(),
        };

        // Condition: ( mb_type != Intra_16x16 ) AND ( CBP == 0 )
        if !is_intra_16x16 && cbp_luma == 0 && cbp_chroma == 0 {
            return 0;
        }

        if mb_qp_delta == 0 {
            0
        } else {
            1
        }
    }

    // 9.3.3.1.1.6 Derivation process of ctxIdxInc for the syntax elements ref_idx_l0 and ref_idx_l1
    fn get_ctx_idx_inc_ref_idx(
        accessor: &NeighborAccessor,
        mb_part_idx: usize,
        list_idx: usize,
    ) -> usize {
        let blk_idx = match accessor.curr_mb.mb_type {
            CabacMbType::P(p_type) => match p_type {
                super::macroblock::PMbType::P_L0_16x16 => 0,
                super::macroblock::PMbType::P_L0_L0_16x8 => {
                    if mb_part_idx == 0 {
                        0
                    } else {
                        8
                    }
                }
                super::macroblock::PMbType::P_L0_L0_8x16 => {
                    if mb_part_idx == 0 {
                        0
                    } else {
                        4
                    }
                }
                super::macroblock::PMbType::P_8x8 => match mb_part_idx {
                    0 => 0,
                    1 => 4,
                    2 => 8,
                    3 => 12,
                    _ => 0,
                },
                _ => 0,
            },
            _ => 0,
        };

        let check_neighbor = |nb: MbNeighborName| -> usize {
            if !accessor.is_available(blk_idx, nb) {
                return 0;
            }
            if accessor.get_mb_type_is_skipped(blk_idx, nb) {
                return 0;
            }
            if accessor.get_mb_type_is_intra(blk_idx, nb) {
                return 0;
            }
            if accessor.slice.MbaffFrameFlag() {
                unimplemented!("MBAFF field macroblock logic for RefIdx context derivation (Eq 9-12)");
            }
            // predModeEqualFlagN: P slices always Pred_L0 unless B slice logic (not impl)
            // refIdxZeroFlagN: ref_idx > 0
            let ref_idx = accessor.get_ref_idx(blk_idx, nb, list_idx).unwrap_or(0);
            if ref_idx > 0 {
                1
            } else {
                0
            }
        };

        let cond_term_flag_a = check_neighbor(MbNeighborName::A);
        let cond_term_flag_b = check_neighbor(MbNeighborName::B);

        cond_term_flag_a + 2 * cond_term_flag_b
    }

    // 9.3.3.1.1.7 Derivation process of ctxIdxInc for the syntax elements mvd_l0 and mvd_l1
    fn get_ctx_idx_inc_mvd(
        accessor: &NeighborAccessor,
        list_idx: usize,
        comp_idx: usize,
        blk_idx: usize, // 4x4 block index
    ) -> usize {
        let check_neighbor = |nb: MbNeighborName| -> usize {
            if !accessor.is_available(blk_idx as u8, nb) {
                return 0;
            }
            if accessor.get_mb_type_is_skipped(blk_idx as u8, nb) {
                return 0;
            }
            // predModeEqualFlagN: P slices always Pred_L0 unless B slice logic
            if accessor.slice.MbaffFrameFlag() {
                unimplemented!("MBAFF field macroblock scaling for MVD context derivation (Eq 9-15, 9-16)");
            }
            let mvd = accessor
                .get_mvd(blk_idx as u8, nb, list_idx)
                .unwrap_or_default();
            let val = if comp_idx == 0 { mvd.x } else { mvd.y };
            val.abs() as usize
        };

        let abs_mvd_comp_a = check_neighbor(MbNeighborName::A);
        let abs_mvd_comp_b = check_neighbor(MbNeighborName::B);

        if abs_mvd_comp_a > 32 || abs_mvd_comp_b > 32 {
            2
        } else {
            let sum = abs_mvd_comp_a + abs_mvd_comp_b;
            if sum > 32 {
                2
            } else if sum > 2 {
                1
            } else {
                0
            }
        }
    }

    // 9.3.3.1.1.4 Derivation process of ctxIdxInc for the syntax element coded_block_pattern
    fn get_ctx_idx_inc_cbp_luma(accessor: &NeighborAccessor, bin_idx: u32) -> usize {
        let blk_idx = bin_idx * 4; // Top-left 4x4 block of the 8x8 block

        let check_neighbor = |nb: MbNeighborName| -> usize {
            // "If any of the following conditions are true, condTermFlagN is set equal to 0"
            // mbAddrN is not available
            if !accessor.is_available(blk_idx as u8, nb) {
                return 0;
            }
            // mb_type is I_PCM
            if accessor.is_pcm(blk_idx as u8, nb) {
                return 0;
            }
            // mb_type is P_Skip or B_Skip (and not current)
            // If neighbor is current, skip check is irrelevant (current is not skipped if we are parsing CBP)
            let (neighbor_blk_idx, mb_neighbor) =
                super::macroblock::get_4x4luma_block_neighbor(blk_idx as u8, nb);

            if mb_neighbor.is_some() && accessor.get_mb_type_is_skipped(blk_idx as u8, nb) {
                return 1;
            }

            // Check bit
            if let Some(cbp) = accessor.get_cbp(blk_idx as u8, nb) {
                // neighbor_blk_idx is the index in the neighbor MB.
                // 8x8 block index = neighbor_blk_idx / 4
                let bit_idx = neighbor_blk_idx / 4;
                if (cbp.luma() >> bit_idx) & 1 != 0 {
                    0
                } else {
                    1
                }
            } else {
                1
            }
        };

        let cond_term_flag_a = check_neighbor(MbNeighborName::A);
        let cond_term_flag_b = check_neighbor(MbNeighborName::B);

        cond_term_flag_a + 2 * cond_term_flag_b
    }

    fn get_ctx_idx_inc_cbp_chroma(accessor: &NeighborAccessor, bin_idx: u32) -> usize {
        // Neighbors A and B (macroblock neighbors)
        // We can use blk_idx 0 and check A and B
        let blk_idx = 0;

        let check_neighbor = |nb: MbNeighborName| -> usize {
            if accessor.is_pcm(blk_idx, nb) {
                return 1;
            }
            if !accessor.is_available(blk_idx, nb) {
                return 0;
            }
            if accessor.get_mb_type_is_skipped(blk_idx, nb) {
                return 0;
            }

            if let Some(cbp) = accessor.get_cbp(blk_idx, nb) {
                let chroma = cbp.chroma();
                if bin_idx == 0 {
                    if chroma != 0 {
                        1
                    } else {
                        0
                    }
                } else {
                    if chroma == 2 {
                        1
                    } else {
                        0
                    }
                }
            } else {
                0
            }
        };

        let cond_term_flag_a = check_neighbor(MbNeighborName::A);
        let cond_term_flag_b = check_neighbor(MbNeighborName::B);

        cond_term_flag_a + 2 * cond_term_flag_b + if bin_idx == 1 { 4 } else { 0 }
    }

    // 9.3.3.1.1.10 Derivation process of ctxIdxInc for the syntax element transform_size_8x8_flag
    fn get_ctx_idx_inc_transform_size_8x8_flag(accessor: &NeighborAccessor) -> usize {
        let blk_idx = 0;
        let cond_term_flag_a = accessor.get_transform_size_8x8_flag(blk_idx, MbNeighborName::A) as usize;
        let cond_term_flag_b = accessor.get_transform_size_8x8_flag(blk_idx, MbNeighborName::B) as usize;

        cond_term_flag_a + cond_term_flag_b
    }

    fn parse_mb_qp_delta_cabac(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<i32> {
        let ctx_idx_inc = Self::get_ctx_idx_inc_mb_qp_delta(slice, mb_addr);

        // Table 9-34 says maxBinIdxCtx=2 for MbQpDelta.
        // 9.3.2.7 says it's Unary binarization.
        let mapped_val = self.parse_unary_bin(SyntaxElement::MbQpDelta, CtxIncParams::Standard(ctx_idx_inc))?;

        // Map back to signed value (Table 9-3)
        let delta = decode_signed_mapping(mapped_val);

        trace!("parse_mb_qp_delta_cabac delta={}", delta);
        Ok(delta)
    }

    fn parse_coded_block_pattern_cabac(
        &mut self,
        slice: &Slice,
        mb_addr: MbAddr,
        curr_mb: &mut CurrentMbInfo,
    ) -> ParseResult<CodedBlockPattern> {
        let props = get_syntax_element_properties(SyntaxElement::CodedBlockPattern);
        let ctx_idx_offset = props.ctx_idx_offset as usize;

        // Prefix: Luma (4 bits)
        let mut cbp_luma = 0;
        for i in 0..4 {
            let accessor = NeighborAccessor::new(slice, mb_addr, curr_mb);
            let ctx_idx_inc = Self::get_ctx_idx_inc_cbp_luma(&accessor, i);
            drop(accessor);

            let bin = self.decode_bin(ctx_idx_offset + ctx_idx_inc)?;
            cbp_luma |= (bin as u8) << i;

            // Update partial CBP in current MB so subsequent bits can reference it
            curr_mb.coded_block_pattern = CodedBlockPattern::new(0, cbp_luma);
        }

        // Suffix: Chroma (2 bits max, TU cMax=2)
        let mut cbp_chroma = 0;
        if slice.sps.ChromaArrayType().is_chroma_subsampled() {
            let ctx_idx_offset_chroma = props.ctx_idx_offset_suffix.unwrap() as usize;

            let accessor = NeighborAccessor::new(slice, mb_addr, curr_mb);
            let ctx_idx_inc_0 = Self::get_ctx_idx_inc_cbp_chroma(&accessor, 0);
            drop(accessor);

            let bit0 = self.decode_bin(ctx_idx_offset_chroma + ctx_idx_inc_0)?; // bin 0
            if bit0 == 1 {
                let accessor = NeighborAccessor::new(slice, mb_addr, curr_mb);
                let ctx_idx_inc_1 = Self::get_ctx_idx_inc_cbp_chroma(&accessor, 1);
                drop(accessor);

                let bit1 = self.decode_bin(ctx_idx_offset_chroma + ctx_idx_inc_1)?; // bin 1
                if bit1 == 1 {
                    cbp_chroma = 2; // 11 -> 2
                } else {
                    cbp_chroma = 1; // 10 -> 1
                }
            } else {
                cbp_chroma = 0; // 0
            }
        }

        let cbp = CodedBlockPattern::new(cbp_chroma, cbp_luma);
        curr_mb.coded_block_pattern = cbp;
        trace!("parse_coded_block_pattern_cabac cbp={:?}", cbp);
        Ok(cbp)
    }

    pub fn parse_mb_skip_flag(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<bool> {
        let ctx_idx_inc = Self::get_ctx_idx_inc_mb_skip_flag(slice, mb_addr);
        let se = if slice.header.slice_type == super::slice::SliceType::B {
            SyntaxElement::MbSkipFlagB
        } else {
            SyntaxElement::MbSkipFlagP
        };
        let props = get_syntax_element_properties(se);
        let bin = self.decode_bin((props.ctx_idx_offset as usize) + ctx_idx_inc)?;
        let skip = bin == 1;
        trace!("parse_mb_skip_flag skip={}", skip);
        Ok(skip)
    }

    // Ref Idx
    fn parse_ref_idx_cabac(
        &mut self,
        accessor: &NeighborAccessor,
        list_idx: usize,
        num_ref_idx_active_minus1: u32,
        mb_part_idx: usize,
    ) -> ParseResult<u8> {
        let ctx_idx_inc = Self::get_ctx_idx_inc_ref_idx(accessor, mb_part_idx, list_idx);

        // Table 9-34 specifies U binarization for ref_idx_l0/l1.
        let val = self.parse_unary_bin(SyntaxElement::RefIdx(list_idx), CtxIncParams::Standard(ctx_idx_inc))?;

        if val > num_ref_idx_active_minus1 {
            return Err(format!(
                "ref_idx {} exceeds num_ref_idx_active_minus1 {}",
                val, num_ref_idx_active_minus1
            ));
        }

        let ref_idx = val as u8;
        trace!("parse_ref_idx_cabac list={} part={} val={}", list_idx, mb_part_idx, ref_idx);
        Ok(ref_idx)
    }

    // MVD
    fn parse_mvd_cabac(
        &mut self,
        accessor: &NeighborAccessor,
        list_idx: usize,
        comp_idx: usize,
        blk_idx: usize,
    ) -> ParseResult<i16> {
        let ctx_idx_inc_0 = Self::get_ctx_idx_inc_mvd(accessor, list_idx, comp_idx, blk_idx);

        let val = self.parse_ueg_k(SyntaxElement::Mvd(list_idx, comp_idx), CtxIncParams::Standard(ctx_idx_inc_0))?;
        let mvd = val as i16;
        trace!("parse_mvd_cabac list={} comp={} blk={} val={}", list_idx, comp_idx, blk_idx, mvd);
        Ok(mvd)
    }

    // 9.3.3.1.3 Assignment process of ctxIdxInc for syntax elements significant_coeff_flag, last_significant_coeff_flag, and coeff_abs_level_minus1
    // And 9.3.3.1.1.9 for coded_block_flag
    fn get_ctx_idx_inc_coded_block_flag(
        accessor: &NeighborAccessor,
        ctx_block_cat: usize,
        blk_idx: usize,
        comp_idx: usize,
    ) -> usize {
        let check_neighbor = |nb: MbNeighborName| -> usize {
            if !accessor.is_available(blk_idx as u8, nb) {
                let is_current_intra = match accessor.curr_mb.mb_type {
                    CabacMbType::I(_) => true,
                    CabacMbType::P(_) => false,
                };
                if is_current_intra {
                    return 1;
                } else {
                    return 0;
                }
            }

            if accessor.is_pcm(blk_idx as u8, nb) {
                return 1;
            }

            if accessor.get_mb_type_is_skipped(blk_idx as u8, nb) {
                return 0;
            }

            if let Some(cbp) = accessor.get_cbp(blk_idx as u8, nb) {
                let (neighbor_blk_idx, _) = if ctx_block_cat == 3 || ctx_block_cat == 4 {
                    super::macroblock::get_4x4chroma_block_neighbor(blk_idx as u8, nb)
                } else {
                    super::macroblock::get_4x4luma_block_neighbor(blk_idx as u8, nb)
                };

                match ctx_block_cat {
                    0 | 6 | 10 => {
                        // Intra16x16 DC
                        if accessor.is_intra_16x16(blk_idx as u8, nb) {
                            accessor
                                .get_cbf(blk_idx as u8, nb, ctx_block_cat, 0)
                                .unwrap_or(false) as usize
                        } else {
                            0
                        }
                    }
                    1 | 2 => {
                        // Luma AC / 4x4
                        let bit_idx = neighbor_blk_idx / 4;
                        if (cbp.luma() >> bit_idx) & 1 == 0 {
                            return 0;
                        }
                        let is_8x8 = accessor.get_transform_size_8x8_flag(blk_idx as u8, nb);
                        if is_8x8 {
                            accessor.get_cbf(blk_idx as u8, nb, 5, 0).unwrap_or(false) as usize
                        } else {
                            accessor
                                .get_cbf(blk_idx as u8, nb, ctx_block_cat, 0)
                                .unwrap_or(false) as usize
                        }
                    }
                    3 => {
                        // Chroma DC
                        if cbp.chroma() == 0 {
                            return 0;
                        }
                        accessor
                            .get_cbf(blk_idx as u8, nb, 3, comp_idx)
                            .unwrap_or(false) as usize
                    }
                    4 => {
                        // Chroma AC
                        if cbp.chroma() != 2 {
                            return 0;
                        }
                        accessor
                            .get_cbf(blk_idx as u8, nb, 4, comp_idx)
                            .unwrap_or(false) as usize
                    }
                    5 => {
                        // Luma 8x8
                        let bit_idx = neighbor_blk_idx / 4;
                        if (cbp.luma() >> bit_idx) & 1 == 0 {
                            return 0;
                        }
                        let is_8x8 = accessor.get_transform_size_8x8_flag(blk_idx as u8, nb);
                        if !is_8x8 {
                            return 0;
                        }
                        accessor.get_cbf(blk_idx as u8, nb, 5, 0).unwrap_or(false) as usize
                    }
                    6..=13 => {
                        unimplemented!("Coded block flag context derivation for categories 6-13 (ChromaArrayType 3)");
                    }
                    _ => 0,
                }
            } else {
                0
            }
        };

        let cond_term_flag_a = check_neighbor(MbNeighborName::A);
        let cond_term_flag_b = check_neighbor(MbNeighborName::B);

        cond_term_flag_a + 2 * cond_term_flag_b
    }

    pub fn get_ctx_idx_inc_sig_coeff_flag(ctx_block_cat: usize, scanning_pos: usize) -> usize {
        // Section 9.3.3.1.3
        if ctx_block_cat == 3 {
            // Chroma DC
            // Eq 9-22: Min( levelListIdx / NumC8x8, 2 )
            min(scanning_pos, 2)
        } else if matches!(ctx_block_cat, 5 | 9 | 13) {
            unimplemented!("Table 9-43 context mapping for 8x8 blocks");
        } else {
            // Luma DC, AC, 4x4 and Chroma AC
            scanning_pos
        }
    }

    pub fn get_ctx_idx_inc_last_sig_coeff_flag(ctx_block_cat: usize, scanning_pos: usize) -> usize {
        // Section 9.3.3.1.3
        if ctx_block_cat == 3 {
            min(scanning_pos, 2)
        } else if matches!(ctx_block_cat, 5 | 9 | 13) {
            unimplemented!("Table 9-43 context mapping for 8x8 blocks");
        } else {
            scanning_pos
        }
    }

    pub fn get_ctx_idx_inc_abs_level(ctx_block_cat: usize, bin_idx: u32, num_decod_abs_level_gt1: usize, num_decod_abs_level_eq1: usize) -> usize {
        // Section 9.3.3.1.3
        if bin_idx == 0 {
            if num_decod_abs_level_gt1 != 0 {
                0
            } else {
                min(4, 1 + num_decod_abs_level_eq1)
            }
        } else {
            let limit = if ctx_block_cat == 3 { 3 } else { 4 };
            5 + min(limit, num_decod_abs_level_gt1)
        }
    }

    // Helper to derive ctxIdxInc for syntax elements in Table 9-39 that depend on binIdx.
    fn get_ctx_idx_inc(
        se: SyntaxElement,
        bin_idx: u32,
        ctx: CtxIncParams,
    ) -> usize {
        match (se, ctx) {
            (SyntaxElement::MbQpDelta, CtxIncParams::Standard(initial)) => {
                if bin_idx == 0 {
                    initial
                } else if bin_idx == 1 {
                    2
                } else {
                    3
                }
            }
            (SyntaxElement::RefIdx(_), CtxIncParams::Standard(initial)) => {
                if bin_idx == 0 {
                    initial
                } else if bin_idx == 1 {
                    4
                } else {
                    5
                }
            }
            (SyntaxElement::Mvd(_, _), CtxIncParams::Standard(initial)) => {
                if bin_idx == 0 {
                    initial
                } else {
                    min(bin_idx as usize + 2, 6)
                }
            }
            (SyntaxElement::IntraChromaPredMode, CtxIncParams::Standard(initial)) => {
                if bin_idx == 0 {
                    initial
                } else {
                    3
                }
            }
            (SyntaxElement::CoeffAbsLevelMinus1(cat), CtxIncParams::AbsLevel { gt1, eq1 }) => {
                Self::get_ctx_idx_inc_abs_level(cat, bin_idx, gt1, eq1)
            }
            (SyntaxElement::MbTypeI, CtxIncParams::Standard(initial)) => {
                // Table 9-39 Offset 3, bin 0 only
                if bin_idx == 0 { initial } else { unreachable!("MbTypeI bin {} needs CtxIncParams::MbType", bin_idx) }
            }
            (SyntaxElement::MbTypeI, CtxIncParams::MbType { prior }) => {
                // Table 9-39 Offset 3
                match bin_idx {
                    2 => 3,
                    3 => 4,
                    4 => if prior == 0 { 6 } else { 5 },
                    5 => if prior == 0 { 7 } else { 6 },
                    6 => 7,
                    _ => unreachable!("Invalid binIdx {} for MbTypeI with prior", bin_idx),
                }
            }
            (SyntaxElement::MbTypeP, CtxIncParams::Standard(_)) => {
                // Table 9-39 Offset 14
                match bin_idx {
                    0 => 0,
                    1 => 1,
                    _ => unreachable!("MbTypeP bin {} needs CtxIncParams::MbType", bin_idx)
                }
            }
            (SyntaxElement::MbTypeP, CtxIncParams::MbType { prior }) => {
                // Table 9-39 Offset 14
                match bin_idx {
                    2 => if prior == 1 { 3 } else { 2 },
                    _ => unreachable!("Invalid binIdx {} for MbTypeP with prior", bin_idx),
                }
            }
            (SyntaxElement::MbTypeISuffix, CtxIncParams::Standard(initial)) => {
                // Table 9-39 Offset 17 (P-slice Intra suffix)
                // Bin 0 of the suffix corresponds to the I_NxN check.
                if bin_idx == 0 { initial } else { unreachable!("MbTypeISuffix bin {} needs CtxIncParams::MbType", bin_idx) }
            }
            (SyntaxElement::MbTypeISuffix, CtxIncParams::MbType { prior }) => {
                // Table 9-39 Offset 17
                match bin_idx {
                    2 => 1,
                    3 => 2,
                    4 => if prior == 0 { 3 } else { 2 },
                    5 => 3,
                    6 => 3,
                    _ => unreachable!("Invalid binIdx {} for MbTypeISuffix", bin_idx),
                }
            }
            (SyntaxElement::MbSkipFlagB, _) | (SyntaxElement::MbTypeB, _) | (SyntaxElement::SubMbTypeB, _) => {
                unimplemented!("B-slice context derivation for {:?}", se);
            }
            (SyntaxElement::MbTypeSI, _) => {
                unimplemented!("SI-slice context derivation for {:?}", se);
            }
            _ => unreachable!("get_ctx_idx_inc mismatch: se={:?}, ctx={:?}", se, ctx),
        }
    }



    fn parse_residual_cabac(
        &mut self,
        slice: &Slice,
        mb_addr: MbAddr,
        curr_mb: &mut CurrentMbInfo,
        residual: &mut super::residual::Residual,
    ) -> ParseResult<()> {
        residual.prediction_mode = match curr_mb.mb_type {
            CabacMbType::I(m) => {
                if m == super::macroblock::IMbType::I_NxN {
                    super::macroblock::MbPredictionMode::Intra_4x4
                } else {
                    super::macroblock::MbPredictionMode::Intra_16x16
                }
            }
            CabacMbType::P(_) => super::macroblock::MbPredictionMode::Pred_L0,
        };
        residual.coded_block_pattern = curr_mb.coded_block_pattern;

        // 1. Luma DC (if Intra 16x16)
        if residual.prediction_mode == super::macroblock::MbPredictionMode::Intra_16x16 {
            // ctxBlockCat = 0
            self.parse_residual_block_cabac(
                slice, mb_addr, curr_mb, residual, 0, 0, 0, 16,
            )?;
        }

        // 2. Luma AC (if Intra 16x16) or Luma 4x4 (others)
        for i in 0..16 {
            let (ctx_block_cat, max_num_coeff) =
                if residual.prediction_mode == super::macroblock::MbPredictionMode::Intra_16x16 {
                    (1, 15)
                } else {
                    (2, 16)
                };

            if residual.coded_block_pattern.luma() & (1 << (i / 4)) != 0 {
                self.parse_residual_block_cabac(
                    slice,
                    mb_addr,
                    curr_mb,
                    residual,
                    ctx_block_cat,
                    i,
                    0,
                    max_num_coeff,
                )?;
            }
        }

        // 3. Chroma DC
        if slice.sps.ChromaArrayType().is_chroma_subsampled()
            && residual.coded_block_pattern.chroma() != 0
        {
            if slice.sps.ChromaArrayType() == super::ChromaFormat::YUV422 {
                 unimplemented!("Chroma DC residual parsing for YUV422 (NumC8x8 > 1)");
            }
            // Cb DC: Cat 3, comp_idx 0
            self.parse_residual_block_cabac(slice, mb_addr, curr_mb, residual, 3, 0, 0, 4)?;
            // Cr DC: Cat 3, comp_idx 1
            self.parse_residual_block_cabac(slice, mb_addr, curr_mb, residual, 3, 0, 1, 4)?;
        } else if slice.sps.ChromaArrayType() == super::ChromaFormat::YUV444 {
            unimplemented!("Chroma DC/AC residual parsing for YUV444 (Categories 6-13)");
        }

        // 4. Chroma AC
        if slice.sps.ChromaArrayType().is_chroma_subsampled()
            && residual.coded_block_pattern.chroma() == 2
        {
            for i in 0..4 {
                // Cb AC: Cat 4, comp_idx 0
                self.parse_residual_block_cabac(
                    slice, mb_addr, curr_mb, residual, 4, i, 0, 15,
                )?;
            }
            for i in 0..4 {
                // Cr AC: Cat 4, comp_idx 1
                self.parse_residual_block_cabac(
                    slice, mb_addr, curr_mb, residual, 4, i, 1, 15,
                )?;
            }
        }

        if curr_mb.transform_size_8x8_flag {
            unimplemented!("Luma 8x8 residual parsing (Category 5)");
        }

        Ok(())
    }

    // 9.3.3.2.3 and 7.3.5.3.3
    fn parse_residual_block_cabac(
        &mut self,
        slice: &Slice,
        mb_addr: MbAddr,
        curr_mb: &mut CurrentMbInfo,
        residual: &mut super::residual::Residual,
        ctx_block_cat: usize,
        blk_idx: usize,
        comp_idx: usize,
        max_num_coeff: usize,
    ) -> ParseResult<bool> {
        trace!("parse_residual_block_cabac cat={} blk={} comp={}", ctx_block_cat, blk_idx, comp_idx);
        // 1. coded_block_flag
        if slice.header.field_pic_flag {
            unimplemented!("Field-coded macroblock support (ctxIdxOffset 277 etc)");
        }
        let cbf_props = get_syntax_element_properties(SyntaxElement::CodedBlockFlag(ctx_block_cat));
        let ctx_idx_offset_cbf = cbf_props.ctx_idx_offset as usize;

        let cbf = if max_num_coeff != 64
            || slice.sps.ChromaArrayType() == super::ChromaFormat::YUV444
        {
            let accessor = NeighborAccessor::new(slice, mb_addr, curr_mb);
            let ctx_idx_inc = Self::get_ctx_idx_inc_coded_block_flag(
                &accessor,
                ctx_block_cat,
                blk_idx,
                comp_idx,
            );
            drop(accessor);

            self.decode_bin(ctx_idx_offset_cbf + ctx_idx_inc)? == 1
        } else {
            true
        };

        // Update CbfInfo
        match ctx_block_cat {
            0 => curr_mb.cbf.luma_dc = cbf,
            1 | 2 => {
                if cbf {
                    curr_mb.cbf.luma_ac |= 1 << blk_idx;
                }
            }
            3 => {
                if comp_idx == 0 {
                    curr_mb.cbf.cb_dc = cbf
                } else {
                    curr_mb.cbf.cr_dc = cbf
                }
            }
            4 => {
                if cbf {
                    if comp_idx == 0 {
                        curr_mb.cbf.cb_ac |= 1 << blk_idx;
                    } else {
                        curr_mb.cbf.cr_ac |= 1 << blk_idx;
                    }
                }
            }
            _ => {}
        }

        trace!("parse_residual_block_cabac cbf={}", cbf);
        if !cbf {
            return Ok(false);
        }

        // 2. significant_coeff_flag and last_significant_coeff_flag
        let mut significant_coeff_flag = [false; 64];
        let mut last_significant_coeff_flag = [false; 64];
        let mut num_coeff = 0;

        let sig_props = get_syntax_element_properties(SyntaxElement::SignificantCoeffFlag(ctx_block_cat));
        let ctx_idx_offset_sig = sig_props.ctx_idx_offset as usize;

        let last_props = get_syntax_element_properties(SyntaxElement::LastSignificantCoeffFlag(ctx_block_cat));
        let ctx_idx_offset_last = last_props.ctx_idx_offset as usize;

        let mut last_scan_pos = -1;

        for i in 0..max_num_coeff {
            if i == max_num_coeff - 1 {
                significant_coeff_flag[i] = true;
                last_scan_pos = i as i32;
                num_coeff += 1;
                break;
            }

            let ctx_idx_inc_sig = Self::get_ctx_idx_inc_sig_coeff_flag(ctx_block_cat, i);
            let sig = self.decode_bin(ctx_idx_offset_sig + ctx_idx_inc_sig)? == 1;
            significant_coeff_flag[i] = sig;
            trace!("parse_residual_block_cabac sig_coeff[{}]={}", i, sig);
            if sig {
                num_coeff += 1;

                let ctx_idx_inc_last = Self::get_ctx_idx_inc_last_sig_coeff_flag(ctx_block_cat, i);
                let last = self.decode_bin(ctx_idx_offset_last + ctx_idx_inc_last)? == 1;
                last_significant_coeff_flag[i] = last;
                trace!("parse_residual_block_cabac last_sig_coeff[{}]={}", i, last);
                if last {
                    last_scan_pos = i as i32;
                    break;
                }
            }
        }

        // 3. coeff_abs_level_minus1
        let mut num_decod_abs_level_eq1 = 0;
        let mut num_decod_abs_level_gt1 = 0;
        let mut coeff_level = [0i32; 64];

        let abs_props = get_syntax_element_properties(SyntaxElement::CoeffAbsLevelMinus1(ctx_block_cat));
        let ctx_idx_offset_abs = abs_props.ctx_idx_offset as usize;

        // Reverse scan
        for i in (0..=last_scan_pos as usize).rev() {
            if significant_coeff_flag[i] {
                let val_minus1 = self.parse_abs_level_minus1(ctx_block_cat, ctx_idx_offset_abs, num_decod_abs_level_gt1, num_decod_abs_level_eq1)?;
                let abs_level = (val_minus1 + 1) as i32;

                // Update counters
                if abs_level == 1 {
                    num_decod_abs_level_eq1 += 1;
                } else {
                    num_decod_abs_level_gt1 += 1;
                }

                // Sign
                let sign = self.decode_bypass()?;
                let level = if sign == 1 { -abs_level } else { abs_level };
                trace!("parse_residual_block_cabac level[{}]={}", i, level);
                coeff_level[i] = level;
            }
        }

        // Store coefficients in Residual
        match ctx_block_cat {
            0 => { // Luma DC (16 coeffs)
                for i in 0..16 { residual.dc_level16x16[i] = coeff_level[i]; }
            },
            1 => { // Luma AC (15 coeffs)
                residual.ac_level16x16[blk_idx].copy_from_slice(&coeff_level[0..15]);
                residual.ac_level16x16_nc[blk_idx] = num_coeff as u8;
            },
            2 => { // Luma 4x4 (16 coeffs)
                residual.luma_level4x4[blk_idx].copy_from_slice(&coeff_level[0..16]);
                residual.luma_level4x4_nc[blk_idx] = num_coeff as u8;
            },
            3 => { // Chroma DC Cb/Cr
                let levels = residual.get_dc_levels_for(if comp_idx == 0 { super::ColorPlane::Cb } else { super::ColorPlane::Cr });
                levels.copy_from_slice(&coeff_level[0..4]);
            },
            4 => { // Chroma AC Cb/Cr
                let (levels, nc) = residual.get_ac_levels_for(blk_idx as u8, if comp_idx == 0 { super::ColorPlane::Cb } else { super::ColorPlane::Cr });
                levels.copy_from_slice(&coeff_level[0..15]);
                *nc = num_coeff as u8;
            },
            _ => {},
        }

        Ok(true)
    }

    fn parse_abs_level_minus1(&mut self, ctx_block_cat: usize, _ctx_idx_offset_abs: usize, num_decod_abs_level_gt1: usize, num_decod_abs_level_eq1: usize) -> ParseResult<u32> {
        let val = self.parse_ueg_k(
            SyntaxElement::CoeffAbsLevelMinus1(ctx_block_cat),
            CtxIncParams::AbsLevel { gt1: num_decod_abs_level_gt1, eq1: num_decod_abs_level_eq1 },
        )?;
        let val_u32 = val as u32;
        trace!("parse_abs_level_minus1 cat={} val={}", ctx_block_cat, val_u32);
        Ok(val_u32)
    }


    // 9.3.3.1.1.3 Derivation process of ctxIdxInc for the syntax element mb_type
    fn get_ctx_idx_inc_mb_type_i(slice: &Slice, mb_addr: MbAddr) -> usize {
        let get_flag = |mb: &super::macroblock::Macroblock| match mb {
            super::macroblock::Macroblock::I(m) => m.mb_type != super::macroblock::IMbType::I_NxN,
            super::macroblock::Macroblock::PCM(_) => true, // I_PCM is not I_NxN
            super::macroblock::Macroblock::P(_) => true,   // P is not I_NxN
        };

        let cond_term_flag_a = slice
            .get_mb_neighbor(mb_addr, MbNeighborName::A)
            .map(get_flag)
            .unwrap_or(false);

        let cond_term_flag_b = slice
            .get_mb_neighbor(mb_addr, MbNeighborName::B)
            .map(get_flag)
            .unwrap_or(false);

        (cond_term_flag_a as usize) + (cond_term_flag_b as usize)
    }

    pub fn parse_mb_type_i(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<super::macroblock::IMbType> {
        let props = get_syntax_element_properties(SyntaxElement::MbTypeI);
        let ctx_idx_offset = props.ctx_idx_offset as usize;

        // Bin 0
        let ctx_idx_inc_0 = Self::get_ctx_idx_inc_mb_type_i(slice, mb_addr);
        let inc0 = Self::get_ctx_idx_inc(SyntaxElement::MbTypeI, 0, CtxIncParams::Standard(ctx_idx_inc_0));
        if self.decode_bin(ctx_idx_offset + inc0)? == 0 {
            trace!("parse_mb_type_i type={:?}", super::macroblock::IMbType::I_NxN);
            return Ok(super::macroblock::IMbType::I_NxN);
        }

        // Bin 1: I_PCM check (Terminal)
        if self.decode_terminate()? {
            trace!("parse_mb_type_i type={:?}", super::macroblock::IMbType::I_PCM);
            return Ok(super::macroblock::IMbType::I_PCM);
        }

        // Bins 2-6
        let res = self.parse_i_16x16_params(ctx_idx_offset, SyntaxElement::MbTypeI)?;
        trace!("parse_mb_type_i type={:?}", res);
        Ok(res)
    }

    fn parse_i_16x16_params(&mut self, ctx_idx_offset: usize, se: SyntaxElement) -> ParseResult<super::macroblock::IMbType> {
        let inc2 = Self::get_ctx_idx_inc(se, 2, CtxIncParams::MbType { prior: 0 });
        let cbpl = self.decode_bin(ctx_idx_offset + inc2)?;

        let inc3 = Self::get_ctx_idx_inc(se, 3, CtxIncParams::MbType { prior: 0 });
        let b3 = self.decode_bin(ctx_idx_offset + inc3)?;

        let cbpc;
        let imode;
        if b3 == 0 {
            cbpc = 0;
            let inc4 = Self::get_ctx_idx_inc(se, 4, CtxIncParams::MbType { prior: 0 });
            let imode_bit1 = self.decode_bin(ctx_idx_offset + inc4)?;
            let inc5 = Self::get_ctx_idx_inc(se, 5, CtxIncParams::MbType { prior: 0 });
            let imode_bit0 = self.decode_bin(ctx_idx_offset + inc5)?;
            imode = (imode_bit1 << 1) | imode_bit0;
        } else {
            let inc4 = Self::get_ctx_idx_inc(se, 4, CtxIncParams::MbType { prior: 1 });
            let b4 = self.decode_bin(ctx_idx_offset + inc4)?;
            cbpc = if b4 == 0 { 1 } else { 2 };
            let inc5 = Self::get_ctx_idx_inc(se, 5, CtxIncParams::MbType { prior: 1 });
            let imode_bit1 = self.decode_bin(ctx_idx_offset + inc5)?;
            let inc6 = Self::get_ctx_idx_inc(se, 6, CtxIncParams::MbType { prior: 0 });
            let imode_bit0 = self.decode_bin(ctx_idx_offset + inc6)?;
            imode = (imode_bit1 << 1) | imode_bit0;
        }
        let mb_type_val = 1 + imode as u32 + 4 * cbpc as u32 + 12 * cbpl as u32;
        let res = super::macroblock::IMbType::try_from(mb_type_val).map_err(|e| e);
        if let Ok(ref t) = res {
            trace!("parse_i_16x16_params type={:?}", t);
        }
        res
    }

    pub fn parse_mb_type_p(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<CabacMbType> {
        let props = get_syntax_element_properties(SyntaxElement::MbTypeP);
        let ctx_idx_offset = props.ctx_idx_offset as usize;
        let ctx_idx_offset_suffix = props.ctx_idx_offset_suffix.unwrap() as usize;

        // Prefix part: ctxIdxOffset 14
        // Bin 0
        let inc0 = Self::get_ctx_idx_inc(SyntaxElement::MbTypeP, 0, CtxIncParams::Standard(0));
        if self.decode_bin(ctx_idx_offset + inc0)? == 1 {
            // Intra. Suffix part: ctxIdxOffset 17
            let i_mb_type = self.parse_mb_type_i_suffix(ctx_idx_offset_suffix, slice, mb_addr)?;
            trace!("parse_mb_type_p type=I({:?})", i_mb_type);
            return Ok(CabacMbType::I(i_mb_type));
        }

        // Bin 1
        let inc1 = Self::get_ctx_idx_inc(SyntaxElement::MbTypeP, 1, CtxIncParams::Standard(0));
        let b1 = self.decode_bin(ctx_idx_offset + inc1)?;

        // Bin 2: Table 9-39, ctxIdxOffset 14, binIdx 2 uses ctxIdxInc 2 or 3
        let inc2 = Self::get_ctx_idx_inc(SyntaxElement::MbTypeP, 2, CtxIncParams::MbType { prior: b1 });
        let b2 = self.decode_bin(ctx_idx_offset + inc2)?;

        let res = if b1 == 1 {
            if b2 == 1 {
                Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_L0_16x8))
            } else {
                Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_L0_8x16))
            }
        } else {
            if b2 == 1 {
                Ok(CabacMbType::P(super::macroblock::PMbType::P_8x8))
            } else {
                Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_16x16))
            }
        };
        if let Ok(ref t) = res {
            trace!("parse_mb_type_p type={:?}", t);
        }
        res
    }

    // Helper for P-slice Intra suffix
    fn parse_mb_type_i_suffix(&mut self, ctx_idx_offset: usize, _slice: &Slice, _mb_addr: MbAddr) -> ParseResult<super::macroblock::IMbType> {
        // Bin 0 (of suffix): I_NxN check. Table 9-39 Row 17 Col 3 => ctxIdxInc = 0
        let inc0 = Self::get_ctx_idx_inc(SyntaxElement::MbTypeISuffix, 0, CtxIncParams::Standard(0));
        if self.decode_bin(ctx_idx_offset + inc0)? == 0 {
            // Bin 0 = 0 -> I_NxN
            trace!("parse_mb_type_i_suffix type={:?}", super::macroblock::IMbType::I_NxN);
            return Ok(super::macroblock::IMbType::I_NxN);
        }

        // Bin 1 (of suffix): I_PCM check. Table 9-39 Row 17 Col 4 => ctxIdx = 276 (Termination)
        if self.decode_terminate()? {
            trace!("parse_mb_type_i_suffix type={:?}", super::macroblock::IMbType::I_PCM);
            return Ok(super::macroblock::IMbType::I_PCM);
        }

        // Remaining bins of I_16x16
        let res = self.parse_i_16x16_params(ctx_idx_offset, SyntaxElement::MbTypeISuffix)?;
        trace!("parse_mb_type_i_suffix type={:?}", res);
        Ok(res)
    }

    pub fn parse_sub_mb_type_p(&mut self, _slice: &Slice, _mb_addr: MbAddr) -> ParseResult<super::macroblock::SubMbType> {
        let props = get_syntax_element_properties(SyntaxElement::SubMbTypeP);
        let ctx_idx_offset = props.ctx_idx_offset as usize;

        // Bin 0 (ctxIdx 21)
        let sub_mb_type = if self.decode_bin(ctx_idx_offset)? == 1 {
            super::macroblock::SubMbType::P_L0_8x8
        } else {
            // Bin 1 (ctxIdx 22)
            if self.decode_bin(ctx_idx_offset + 1)? == 0 {
                super::macroblock::SubMbType::P_L0_8x4
            } else {
                // Bin 2 (ctxIdx 23)
                if self.decode_bin(ctx_idx_offset + 2)? == 1 {
                    super::macroblock::SubMbType::P_L0_4x8
                } else {
                    super::macroblock::SubMbType::P_L0_4x4
                }
            }
        };
        trace!("parse_sub_mb_type_p type={:?}", sub_mb_type);
        Ok(sub_mb_type)
    }

    // 9.3.3.1.1.8 Derivation process of ctxIdxInc for the syntax element intra_chroma_pred_mode
    fn get_ctx_idx_inc_intra_chroma_pred_mode(slice: &Slice, mb_addr: MbAddr) -> usize {
        let get_cond_term_flag = |mb: &super::macroblock::Macroblock| match mb {
             super::macroblock::Macroblock::PCM(_) => false, // Spec: condTermFlagN = 0 for I_PCM
             super::macroblock::Macroblock::I(m) => m.intra_chroma_pred_mode as u32 != 0,
             super::macroblock::Macroblock::P(_) => false,
        };

        let cond_term_flag_a = slice
            .get_mb_neighbor(mb_addr, MbNeighborName::A)
            .map(get_cond_term_flag)
            .unwrap_or(false);

        let cond_term_flag_b = slice
            .get_mb_neighbor(mb_addr, MbNeighborName::B)
            .map(get_cond_term_flag)
            .unwrap_or(false);

        (cond_term_flag_a as usize) + (cond_term_flag_b as usize)
    }

    pub fn parse_intra_chroma_pred_mode(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<super::macroblock::Intra_Chroma_Pred_Mode> {
        let ctx_idx_inc = Self::get_ctx_idx_inc_intra_chroma_pred_mode(slice, mb_addr);

        // Table 9-34: maxBinIdxCtx = 1.
        let val = self.parse_truncated_unary_bin(SyntaxElement::IntraChromaPredMode, None, CtxIncParams::Standard(ctx_idx_inc))?;
        let mode = super::macroblock::Intra_Chroma_Pred_Mode::try_from(val).map_err(|e| e)?;
        trace!("parse_intra_chroma_pred_mode mode={:?}", mode);
        Ok(mode)
    }

    pub fn parse_macroblock(
        &mut self,
        slice: &mut Slice,
    ) -> ParseResult<super::macroblock::Macroblock> {
        let mb_addr = slice.get_next_mb_addr();
        trace!("parse_macroblock addr={}", mb_addr);

        if slice.MbaffFrameFlag() {
             unimplemented!("MBAFF mb_field_decoding_flag parsing");
        }

        if slice.header.slice_type == super::slice::SliceType::P {
            let skipped = self.parse_mb_skip_flag(slice, mb_addr)?;
            if skipped {
                let motion = super::parser::calculate_motion(
                    slice,
                    mb_addr,
                    super::macroblock::PMbType::P_Skip,
                    &[super::macroblock::PartitionInfo::default(); 4],
                    &[super::macroblock::SubMacroblock::default(); 4],
                );
                let mb = super::macroblock::PMb {
                    mb_type: super::macroblock::PMbType::P_Skip,
                    motion,
                    coded_block_pattern: CodedBlockPattern::new(0, 0),
                    mb_qp_delta: 0,
                    qp: slice.slice_qp_y() as u8,
                    cbf_info: CbfInfo::default(),
                    ..Default::default()
                };
                return Ok(super::macroblock::Macroblock::P(mb));
            }
        } else if slice.header.slice_type == super::slice::SliceType::B {
            unimplemented!("B-slice mb_skip_flag parsing");
        }

        let mb_type = if slice.header.slice_type == super::slice::SliceType::I {
            CabacMbType::I(self.parse_mb_type_i(slice, mb_addr)?)
        } else if slice.header.slice_type == super::slice::SliceType::SI {
            unimplemented!("SI slice mb_type parsing");
        } else {
            self.parse_mb_type_p(slice, mb_addr)?
        };

        let mut curr_mb = CurrentMbInfo {
            mb_type,
            motion: MbMotion::default(),
            coded_block_pattern: CodedBlockPattern::new(0, 0),
            transform_size_8x8_flag: false,
            cbf: CbfInfo::default(),
        };

        match mb_type {
            CabacMbType::I(i_type) => {
                if i_type == super::macroblock::IMbType::I_PCM {
                    self.reader.align();

                    let mut pcm_mb = PcmMb { qp: 0, ..PcmMb::default() };

                    // Luma: 256 samples
                    let bit_depth_luma = slice.sps.bit_depth_luma_minus8 + 8;
                    let luma_size = 256;
                    pcm_mb.pcm_sample_luma.reserve(luma_size);
                    for _ in 0..luma_size {
                        pcm_mb.pcm_sample_luma.push(self.reader.u(bit_depth_luma)? as u8);
                    }

                    // Chroma
                    let chroma_format = slice.sps.ChromaArrayType();
                    if chroma_format != super::ChromaFormat::Monochrome {
                        let shift = chroma_format.get_chroma_shift();
                        let width_c = 16 >> shift.width;
                        let height_c = 16 >> shift.height;
                        let chroma_size = (width_c * height_c) as usize;
                        let bit_depth_chroma = slice.sps.bit_depth_chroma_minus8 + 8;

                        pcm_mb.pcm_sample_chroma_cb.reserve(chroma_size);
                        for _ in 0..chroma_size {
                            pcm_mb
                                .pcm_sample_chroma_cb
                                .push(self.reader.u(bit_depth_chroma)? as u8);
                        }

                        pcm_mb.pcm_sample_chroma_cr.reserve(chroma_size);
                        for _ in 0..chroma_size {
                            pcm_mb
                                .pcm_sample_chroma_cr
                                .push(self.reader.u(bit_depth_chroma)? as u8);
                        }
                    }

                    // Initialize decoding engine
                    self.init_decoding_engine()?;

                    return Ok(super::macroblock::Macroblock::PCM(pcm_mb));
                }

                let mut mb = super::macroblock::IMb {
                    mb_type: i_type,
                    ..Default::default()
                };

                // Intra prediction
                if i_type == super::macroblock::IMbType::I_NxN {
                    if slice.pps.transform_8x8_mode_flag {
                        let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                        let ctx_idx_inc = Self::get_ctx_idx_inc_transform_size_8x8_flag(&accessor);
                        drop(accessor);

                        let props = get_syntax_element_properties(SyntaxElement::TransformSize8x8Flag);
                        let flag = self.decode_bin((props.ctx_idx_offset as usize) + ctx_idx_inc)? == 1;
                        mb.transform_size_8x8_flag = flag;
                        curr_mb.transform_size_8x8_flag = flag;
                    }

                    if mb.transform_size_8x8_flag {
                        unimplemented!("Intra 8x8 prediction parsing (prev_intra8x8_pred_mode_flag, rem_intra8x8_pred_mode)");
                    } else {
                        let prev_intra_props = get_syntax_element_properties(SyntaxElement::PrevIntra4x4PredModeFlag);
                        let rem_intra_props = get_syntax_element_properties(SyntaxElement::RemIntra4x4PredMode);

                        for i in 0..16 {
                            let prev_intra_pred_mode_flag = self.decode_bin(prev_intra_props.ctx_idx_offset as usize)? == 1;
                            let prev_mode =
                                super::parser::calc_prev_intra4x4_pred_mode(slice, &mb, mb_addr, i);

                            if prev_intra_pred_mode_flag {
                                mb.rem_intra4x4_pred_mode[i] = prev_mode;
                            } else {
                                let rem_intra_offset = rem_intra_props.ctx_idx_offset as usize;
                                let rem_intra_pred_mode = self.decode_bin(rem_intra_offset)? as u32
                                    | ((self.decode_bin(rem_intra_offset)? as u32) << 1)
                                    | ((self.decode_bin(rem_intra_offset)? as u32) << 2);

                                if rem_intra_pred_mode < (prev_mode as u32) {
                                    mb.rem_intra4x4_pred_mode[i] =
                                        super::macroblock::Intra_4x4_SamplePredMode::try_from(
                                            rem_intra_pred_mode,
                                        )?;
                                } else {
                                    mb.rem_intra4x4_pred_mode[i] =
                                        super::macroblock::Intra_4x4_SamplePredMode::try_from(
                                            rem_intra_pred_mode + 1,
                                        )?;
                                }
                            }
                        }
                    }
                }

                // Intra Chroma Pred Mode
                if slice.sps.ChromaArrayType().is_chroma_subsampled()
                    && i_type != super::macroblock::IMbType::I_PCM
                {
                    mb.intra_chroma_pred_mode =
                        self.parse_intra_chroma_pred_mode(slice, mb_addr)?;
                }

                // CBP and QP
                if i_type == super::macroblock::IMbType::I_NxN {
                    mb.coded_block_pattern =
                        self.parse_coded_block_pattern_cabac(slice, mb_addr, &mut curr_mb)?;
                    if !mb.coded_block_pattern.is_zero()
                        || mb.mb_type == super::macroblock::IMbType::I_PCM
                    {
                        mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                    }
                } else {
                    // Intra_16x16: Derived CBP from mb_type (Table 7-11)
                    mb.coded_block_pattern = super::tables::mb_type_to_coded_block_pattern(i_type)
                        .expect("Intra_16x16 mb_type should have a valid CBP");
                    curr_mb.coded_block_pattern = mb.coded_block_pattern;
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }

                let mut residual = super::residual::Residual::default();
                self.parse_residual_cabac(slice, mb_addr, &mut curr_mb, &mut residual)?;
                mb.residual = Some(Box::new(residual));
                mb.cbf_info = curr_mb.cbf;

                Ok(super::macroblock::Macroblock::I(mb))
            }
            CabacMbType::P(p_type) => {
                let mut partitions = [PartitionInfo::default(); 4];
                let mut sub_mbs = [super::macroblock::SubMacroblock::default(); 4];

                if slice.header.slice_type == super::slice::SliceType::B {
                    unimplemented!("B-slice mb_type and partition parsing");
                }

                // Sub MB pred
                if p_type == super::macroblock::PMbType::P_8x8 {
                    for i in 0..4 {
                        sub_mbs[i].sub_mb_type = self.parse_sub_mb_type_p(slice, mb_addr)?;
                    }

                    let num_ref_idx_l0_active_minus1 = slice.header.num_ref_idx_l0_active_minus1;
                    if num_ref_idx_l0_active_minus1 > 0 || slice.header.field_pic_flag {
                        for i in 0..4 {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx = self.parse_ref_idx_cabac(
                                &accessor,
                                0,
                                num_ref_idx_l0_active_minus1,
                                i,
                            )?;
                            drop(accessor);

                            sub_mbs[i].partitions[0].ref_idx_l0 = ref_idx;
                            sub_mbs[i].partitions[1].ref_idx_l0 = ref_idx;
                            sub_mbs[i].partitions[2].ref_idx_l0 = ref_idx;
                            sub_mbs[i].partitions[3].ref_idx_l0 = ref_idx;

                            // Update curr_mb.motion
                            // P_8x8 -> 8x8 block i.
                            let start_blk_idx = match i {
                                0 => 0,
                                1 => 4,
                                2 => 8,
                                3 => 12,
                                _ => 0,
                            };
                            let p =
                                super::macroblock::get_4x4luma_block_location(start_blk_idx);
                            let start_y = (p.y / 4) as usize;
                            let start_x = (p.x / 4) as usize;
                            for y in 0..2 {
                                for x in 0..2 {
                                    curr_mb.motion.partitions[start_y + y][start_x + x]
                                        .ref_idx_l0 = ref_idx;
                                }
                            }
                        }
                    }

                    // mvd
                    for i in 0..4 {
                        let num_sub_part = sub_mbs[i].sub_mb_type.NumSubMbPart();
                        for j in 0..num_sub_part {
                            let p_idx = match (sub_mbs[i].sub_mb_type, j) {
                                (SubMbType::P_L0_8x8, 0) => 0,
                                (SubMbType::P_L0_8x4, 0) => 0,
                                (SubMbType::P_L0_8x4, 1) => 2,
                                (SubMbType::P_L0_4x8, 0) => 0,
                                (SubMbType::P_L0_4x8, 1) => 1,
                                (SubMbType::P_L0_4x4, x) => x,
                                _ => 0,
                            };
                            // i*4 + p_idx constructs the Z-scan index of the sub-partition within the MB
                            // But p_idx here is relative to 8x8 block Z-scan.
                            // 8x8 block i starts at Z-scan index: 0, 4, 8, 12.
                            let base_blk_idx = match i {
                                0 => 0,
                                1 => 4,
                                2 => 8,
                                3 => 12,
                                _ => 0,
                            };
                            let blk_idx = base_blk_idx + p_idx;

                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let mvd_x = self.parse_mvd_cabac(&accessor, 0, 0, blk_idx)?;
                            let mvd_y = self.parse_mvd_cabac(&accessor, 0, 1, blk_idx)?;
                            drop(accessor);

                            sub_mbs[i].partitions[p_idx].mvd_l0 =
                                MotionVector { x: mvd_x, y: mvd_y };

                            // Update curr_mb.motion
                            let (w, h) = match sub_mbs[i].sub_mb_type {
                                SubMbType::P_L0_8x8 => (2, 2),
                                SubMbType::P_L0_8x4 => (2, 1),
                                SubMbType::P_L0_4x8 => (1, 2),
                                SubMbType::P_L0_4x4 => (1, 1),
                            };
                            let p =
                                super::macroblock::get_4x4luma_block_location(blk_idx as u8);
                            let start_blk_y = (p.y / 4) as usize;
                            let start_blk_x = (p.x / 4) as usize;
                            for y in 0..h {
                                for x in 0..w {
                                    curr_mb.motion.partitions[start_blk_y + y][start_blk_x + x]
                                        .mvd_l0 = MotionVector { x: mvd_x, y: mvd_y };
                                }
                            }
                        }
                    }
                } else {
                    let num_part = p_type.NumMbPart();
                    let num_ref_idx_l0_active_minus1 = slice.header.num_ref_idx_l0_active_minus1;

                    if num_ref_idx_l0_active_minus1 > 0 || slice.header.field_pic_flag {
                        for i in 0..num_part {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx = self.parse_ref_idx_cabac(
                                &accessor,
                                0,
                                num_ref_idx_l0_active_minus1,
                                i,
                            )?;
                            drop(accessor);
                            partitions[i].ref_idx_l0 = ref_idx;

                            // Update curr_mb.motion
                            let (w, h) = match p_type {
                                super::macroblock::PMbType::P_L0_16x16 => (4, 4),
                                super::macroblock::PMbType::P_L0_L0_16x8 => (4, 2),
                                super::macroblock::PMbType::P_L0_L0_8x16 => (2, 4),
                                _ => (0, 0),
                            };
                            let start_blk_idx = match p_type {
                                super::macroblock::PMbType::P_L0_L0_16x8 => i * 8,
                                super::macroblock::PMbType::P_L0_L0_8x16 => i * 4,
                                _ => 0,
                            };
                            let p =
                                super::macroblock::get_4x4luma_block_location(start_blk_idx as u8);
                            let start_blk_y = (p.y / 4) as usize;
                            let start_blk_x = (p.x / 4) as usize;
                            for y in 0..h {
                                for x in 0..w {
                                    curr_mb.motion.partitions[start_blk_y + y][start_blk_x + x]
                                        .ref_idx_l0 = ref_idx;
                                }
                            }
                        }
                    }

                    for i in 0..num_part {
                        let blk_idx = match p_type {
                            super::macroblock::PMbType::P_L0_L0_16x8 => i * 8,
                            super::macroblock::PMbType::P_L0_L0_8x16 => i * 4,
                            _ => 0,
                        };
                        let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                        let mvd_x = self.parse_mvd_cabac(&accessor, 0, 0, blk_idx)?;
                        let mvd_y = self.parse_mvd_cabac(&accessor, 0, 1, blk_idx)?;
                        drop(accessor);
                        partitions[i].mvd_l0 = MotionVector { x: mvd_x, y: mvd_y };

                        // Update curr_mb.motion
                        let (w, h) = match p_type {
                            super::macroblock::PMbType::P_L0_16x16 => (4, 4),
                            super::macroblock::PMbType::P_L0_L0_16x8 => (4, 2),
                            super::macroblock::PMbType::P_L0_L0_8x16 => (2, 4),
                            _ => (0, 0),
                        };
                        let p = super::macroblock::get_4x4luma_block_location(blk_idx as u8);
                        let start_blk_y = (p.y / 4) as usize;
                        let start_blk_x = (p.x / 4) as usize;
                        for y in 0..h {
                            for x in 0..w {
                                curr_mb.motion.partitions[start_blk_y + y][start_blk_x + x]
                                    .mvd_l0 = MotionVector { x: mvd_x, y: mvd_y };
                            }
                        }
                    }
                }

                let cbp = self.parse_coded_block_pattern_cabac(slice, mb_addr, &mut curr_mb)?;
                let mut mb = super::macroblock::PMb {
                    mb_type: p_type,
                    motion: super::parser::calculate_motion(
                        slice, mb_addr, p_type, &partitions, &sub_mbs,
                    ),
                    coded_block_pattern: cbp,
                    mb_qp_delta: 0,
                    qp: slice.slice_qp_y() as u8,
                    transform_size_8x8_flag: false,
                    residual: None,
                    cbf_info: CbfInfo::default(),
                };

                if !mb.coded_block_pattern.is_zero()
                    && mb.mb_type != super::macroblock::PMbType::P_Skip
                {
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }

                let mut residual = super::residual::Residual::default();
                self.parse_residual_cabac(slice, mb_addr, &mut curr_mb, &mut residual)?;
                mb.residual = Some(Box::new(residual));
                mb.cbf_info = curr_mb.cbf;

                Ok(super::macroblock::Macroblock::P(mb))
            }
        }
    }

    fn renorm(&mut self) -> ParseResult<()> {
        while self.range < 256 {
            self.range <<= 1;
            self.offset = (self.offset << 1) | (self.reader.u(1)? as u32);
        }
        Ok(())
    }
}

/// Map CABAC unsigned value back to signed value (Table 9-3 / clause 9.1.1)
fn decode_signed_mapping(val: u32) -> i32 {
    let val = val as i32;
    if val == 0 {
        0
    } else if val % 2 == 0 {
        -(val / 2)
    } else {
        (val + 1) / 2
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CabacMbType {
    I(super::macroblock::IMbType),
    P(super::macroblock::PMbType),
}

#[cfg(test)]
mod tests {
    use super::super::pps::PicParameterSet;
    use super::super::slice::{Slice, SliceHeader};
    use super::super::sps::SequenceParameterSet;
    use super::*;

    fn make_dummy_slice() -> Slice {
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let header = SliceHeader::default();
        Slice::new(sps, pps, header)
    }

    #[test]
    fn test_cabac_init() {
        let data = [0u8; 10]; // enough zeros
        let mut reader = BitReader::new(&data);
        let slice = make_dummy_slice();

        let ctx = CabacContext::new(&mut reader, &slice);
        assert!(ctx.is_ok());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxElement {
    MbTypeSI,
    MbTypeI,
    MbTypeISuffix,
    MbSkipFlagP,
    MbTypeP,
    SubMbTypeP,
    MbSkipFlagB,
    MbTypeB,
    SubMbTypeB,
    Mvd(usize, usize), // list_idx, comp_idx
    RefIdx(usize), // list_idx
    MbQpDelta,
    IntraChromaPredMode,
    PrevIntra4x4PredModeFlag,
    RemIntra4x4PredMode,
    MbFieldDecodingFlag,
    CodedBlockPattern,
    CodedBlockFlag(usize), // ctxBlockCat
    SignificantCoeffFlag(usize), // ctxBlockCat
    LastSignificantCoeffFlag(usize), // ctxBlockCat
    CoeffAbsLevelMinus1(usize), // ctxBlockCat
    EndOfSliceFlag,
    TransformSize8x8Flag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinarizationType {
    FL { c_max: u32 },
    U,
    TU { c_max: u32 },
    UEGk { k: u32, signed_val_flag: bool, u_coff: u32 },
    PrefixSuffix, // For MbType and CBP
    Custom, // For others
}

pub struct CabacTableEntry {
    pub binarization: BinarizationType,
    pub max_bin_idx_ctx: u32,
    pub ctx_idx_offset: u32,
    // For prefix/suffix types, we might need secondary values
    pub max_bin_idx_ctx_suffix: Option<u32>,
    pub ctx_idx_offset_suffix: Option<u32>,
}

pub fn get_syntax_element_properties(se: SyntaxElement) -> CabacTableEntry {
    match se {
        SyntaxElement::MbTypeSI => CabacTableEntry {
            binarization: BinarizationType::PrefixSuffix,
            max_bin_idx_ctx: 0,
            ctx_idx_offset: 0,
            max_bin_idx_ctx_suffix: Some(6),
            ctx_idx_offset_suffix: Some(3),
        },
        SyntaxElement::MbTypeI => CabacTableEntry {
            binarization: BinarizationType::Custom,
            max_bin_idx_ctx: 6,
            ctx_idx_offset: 3,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::MbTypeISuffix => CabacTableEntry {
            binarization: BinarizationType::Custom,
            max_bin_idx_ctx: 6,
            ctx_idx_offset: 17,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::MbSkipFlagP => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: 11,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::MbTypeP => CabacTableEntry {
            binarization: BinarizationType::PrefixSuffix,
            max_bin_idx_ctx: 2,
            ctx_idx_offset: 14,
            max_bin_idx_ctx_suffix: Some(5),
            ctx_idx_offset_suffix: Some(17),
        },
        SyntaxElement::SubMbTypeP => CabacTableEntry {
            binarization: BinarizationType::Custom,
            max_bin_idx_ctx: 2,
            ctx_idx_offset: 21,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::MbSkipFlagB => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: 24,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::MbTypeB => CabacTableEntry {
            binarization: BinarizationType::PrefixSuffix,
            max_bin_idx_ctx: 3,
            ctx_idx_offset: 27,
            max_bin_idx_ctx_suffix: Some(5),
            ctx_idx_offset_suffix: Some(32),
        },
        SyntaxElement::SubMbTypeB => CabacTableEntry {
            binarization: BinarizationType::Custom,
            max_bin_idx_ctx: 3,
            ctx_idx_offset: 36,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::Mvd(_, comp_idx) => {
            let offset = if comp_idx == 0 { 40 } else { 47 };
            CabacTableEntry {
                binarization: BinarizationType::UEGk { k: 3, signed_val_flag: true, u_coff: 9 },
                max_bin_idx_ctx: 4,
                ctx_idx_offset: offset,
                max_bin_idx_ctx_suffix: None,
                ctx_idx_offset_suffix: None,
            }
        },
        SyntaxElement::RefIdx(_) => CabacTableEntry {
            binarization: BinarizationType::U,
            max_bin_idx_ctx: 2,
            ctx_idx_offset: 54,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::MbQpDelta => CabacTableEntry {
            binarization: BinarizationType::Custom, // U of mapped value
            max_bin_idx_ctx: 2,
            ctx_idx_offset: 60,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::IntraChromaPredMode => CabacTableEntry {
            binarization: BinarizationType::TU { c_max: 3 },
            max_bin_idx_ctx: 1,
            ctx_idx_offset: 64,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::PrevIntra4x4PredModeFlag | SyntaxElement::MbFieldDecodingFlag => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: if matches!(se, SyntaxElement::MbFieldDecodingFlag) { 70 } else { 68 },
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::RemIntra4x4PredMode => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 7 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: 69,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::CodedBlockPattern => CabacTableEntry {
            binarization: BinarizationType::PrefixSuffix,
            max_bin_idx_ctx: 3,
            ctx_idx_offset: 73,
            max_bin_idx_ctx_suffix: Some(1),
            ctx_idx_offset_suffix: Some(77),
        },
        SyntaxElement::CodedBlockFlag(cat) => {
             // Table 9-34 and Table 9-40
             let offset = match cat {
                 0..=4 => 85 + match cat {
                     0 => 0,
                     1 => 4,
                     2 => 8,
                     3 => 12,
                     4 => 16,
                     _ => unreachable!(),
                 },
                 5 => 1012, // ctxBlockCat == 5
                 6..=8 => 460 + (cat - 6) * 4, // 5 < ctxBlockCat < 9
                 9 => 1016, // 1012 + 4
                 10..=12 => 472 + (cat - 10) * 4, // 9 < ctxBlockCat < 13
                 13 => 1020, // 1012 + 8
                 _ => 85,
             };
             CabacTableEntry {
                binarization: BinarizationType::FL { c_max: 1 },
                max_bin_idx_ctx: 0,
                ctx_idx_offset: offset as u32,
                max_bin_idx_ctx_suffix: None,
                ctx_idx_offset_suffix: None,
            }
        },
        SyntaxElement::SignificantCoeffFlag(cat) => {
             // Table 9-34 and Table 9-40
             let offset = match cat {
                 0..=4 => 105 + match cat {
                     0 => 0,
                     1 => 15,
                     2 => 29,
                     3 => 44,
                     4 => 47,
                     _ => unreachable!(),
                 },
                 5 => 402,
                 6..=8 => 484 + match cat {
                     6 => 0,
                     7 => 15,
                     8 => 29,
                     _ => unreachable!(),
                 },
                 9 => 660,
                 10..=12 => 528 + match cat {
                     10 => 0,
                     11 => 15,
                     12 => 29,
                     _ => unreachable!(),
                 },
                 13 => 718,
                 _ => 105,
             };
             CabacTableEntry {
                binarization: BinarizationType::FL { c_max: 1 },
                max_bin_idx_ctx: 0,
                ctx_idx_offset: offset as u32,
                max_bin_idx_ctx_suffix: None,
                ctx_idx_offset_suffix: None,
            }
        },
        SyntaxElement::LastSignificantCoeffFlag(cat) => {
             // Table 9-34 and Table 9-40
             let offset = match cat {
                 0..=4 => 166 + match cat {
                     0 => 0,
                     1 => 15,
                     2 => 29,
                     3 => 44,
                     4 => 47,
                     _ => unreachable!(),
                 },
                 5 => 417,
                 6..=8 => 572 + match cat {
                     6 => 0,
                     7 => 15,
                     8 => 29,
                     _ => unreachable!(),
                 },
                 9 => 690,
                 10..=12 => 616 + match cat {
                     10 => 0,
                     11 => 15,
                     12 => 29,
                     _ => unreachable!(),
                 },
                 13 => 748,
                 _ => 166,
             };
             CabacTableEntry {
                binarization: BinarizationType::FL { c_max: 1 },
                max_bin_idx_ctx: 0,
                ctx_idx_offset: offset as u32,
                max_bin_idx_ctx_suffix: None,
                ctx_idx_offset_suffix: None,
            }
        },
        SyntaxElement::CoeffAbsLevelMinus1(cat) => {
             // Table 9-34 and Table 9-40
             let offset = match cat {
                 0..=4 => 227 + match cat {
                     0 => 0,
                     1 => 10,
                     2 => 20,
                     3 => 30,
                     4 => 39,
                     _ => unreachable!(),
                 },
                 5 => 426,
                 6..=8 => 952 + match cat {
                     6 => 0,
                     7 => 10,
                     8 => 20,
                     _ => unreachable!(),
                 },
                 9 => 708,
                 10..=12 => 982 + match cat {
                     10 => 0,
                     11 => 10,
                     12 => 20,
                     _ => unreachable!(),
                 },
                 13 => 766,
                 _ => 227,
             };
             CabacTableEntry {
                binarization: BinarizationType::UEGk { k: 0, signed_val_flag: false, u_coff: 14 },
                max_bin_idx_ctx: 1,
                ctx_idx_offset: offset as u32,
                max_bin_idx_ctx_suffix: None,
                ctx_idx_offset_suffix: None,
            }
        },
        SyntaxElement::EndOfSliceFlag => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: 276,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::TransformSize8x8Flag => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: 399,
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
    }
}
