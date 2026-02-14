use super::cabac_tables::{
    get_init_table, RANGE_TAB_LPS, TRANS_IDX_LPS, TRANS_IDX_MPS,
};
use super::macroblock::{
    CodedBlockPattern, MbAddr, MbMotion, MbNeighborName, MotionVector, PartitionInfo, SubMbType,
};
use super::parser::{BitReader, ParseResult};
use super::residual::Residual;
use super::slice::Slice;
use std::cmp::min;

#[derive(Default)]
struct CbfInfo {
    luma_dc: bool,
    luma_ac: u16, // 16 bits
    cb_dc: bool,
    cb_ac: u8, // 4 bits
    cr_dc: bool,
    cr_ac: u8, // 4 bits
}

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
                .unwrap_or(false), // Unavailable -> 0 for condTermFlag usually, but this is just is_intra
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
                    None // TODO: L1
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
                            0 // TODO
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
                    None
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
                            MotionVector::default()
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
                // In current MB
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
                    mb.get_nc(
                        neighbor_blk_idx,
                        if ctx_block_cat >= 3 {
                            if comp_idx == 0 {
                                super::ColorPlane::Cb
                            } else {
                                super::ColorPlane::Cr
                            }
                        } else {
                            super::ColorPlane::Y
                        },
                    ) > 0
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

impl<'a, 'b> CabacContext<'a, 'b> {
    pub fn new(reader: &'a mut BitReader<'b>, slice: &Slice) -> ParseResult<Self> {
        let mut ctx = CabacContext {
            reader,
            range: 510,
            offset: 0,
            ctx_table: [0; 1024],
        };

        ctx.init_context_variables(slice);
        ctx.init_decoding_engine()?;

        Ok(ctx)
    }

    // 9.3.1.1 Initialization process for context variables
    fn init_context_variables(&mut self, slice: &Slice) {
        let slice_qp_y = slice.slice_qp_y();
        let cabac_init_idc = slice.header.cabac_init_idc;

        let init_table = get_init_table(slice.header.slice_type, cabac_init_idc as u8);

        for ctx_idx in 0..1024 {
            let init_val = init_table[ctx_idx];
            let m = init_val.0 as i32;
            let n = init_val.1 as i32;

            let pre_ctx_state = min(126, ((m * (min(51, slice_qp_y) as i32)) >> 4) + n);
            let pre_ctx_state = if pre_ctx_state < 1 { 1 } else { pre_ctx_state };

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
        self.reader.align();
        self.range = 510;
        self.offset = self.reader.u(9)? as u32;

        if self.offset == 510 || self.offset == 511 {
            // "The bitstream shall not contain data that result in a value of codIOffset being equal to 510 or 511."
            return Err("codIOffset equal to 510 or 511 is illegal".to_string());
        }

        Ok(())
    }

    // 9.3.2.1 Unary (U) binarization process
    pub fn parse_unary_bin<F>(&mut self, mut get_ctx_idx: F) -> ParseResult<u32>
    where
        F: FnMut(u32) -> usize,
    {
        let mut bin_idx = 0;
        loop {
            let ctx_idx = get_ctx_idx(bin_idx);
            let bin = self.decode_bin(ctx_idx)?;
            if bin == 0 {
                return Ok(bin_idx);
            }
            bin_idx += 1;
        }
    }

    // 9.3.2.2 Truncated unary (TU) binarization process
    pub fn parse_truncated_unary_bin<F>(
        &mut self,
        c_max: u32,
        mut get_ctx_idx: F,
    ) -> ParseResult<u32>
    where
        F: FnMut(u32) -> usize,
    {
        let mut bin_idx = 0;
        while bin_idx < c_max {
            let ctx_idx = get_ctx_idx(bin_idx);
            let bin = self.decode_bin(ctx_idx)?;
            if bin == 0 {
                return Ok(bin_idx);
            }
            bin_idx += 1;
        }
        Ok(c_max)
    }

    // 9.3.2.3 Concatenated unary/ k-th order Exp-Golomb (UEGk) binarization process
    pub fn parse_ueg_k<F>(
        &mut self,
        u_coff: u32,
        k_val: u32,
        signed_val_flag: bool,
        mut get_ctx_idx: F,
    ) -> ParseResult<i32>
    where
        F: FnMut(u32) -> usize,
    {
        // Prefix: TU with cMax = uCoff
        let prefix = self.parse_truncated_unary_bin(u_coff, &mut get_ctx_idx)?;

        if prefix < u_coff {
            let val = prefix as i32;
            return if signed_val_flag && val != 0 {
                let sign = self.decode_bypass()?;
                Ok(if sign == 1 { -val } else { val })
            } else {
                Ok(val)
            };
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

        if signed_val_flag && val != 0 {
            let sign = self.decode_bypass()?;
            Ok(if sign == 1 { -val } else { val })
        } else {
            Ok(val)
        }
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

        Ok(bin_val)
    }

    // 9.3.3.2.4 Decoding process for binary decisions before termination
    pub fn decode_terminate(&mut self) -> ParseResult<bool> {
        self.range -= 2;

        if self.offset >= self.range {
            Ok(true)
        } else {
            self.renorm()?;
            Ok(false)
        }
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
        let get_flag = |mb: &super::macroblock::Macroblock| match mb {
            super::macroblock::Macroblock::I(m) => m.mb_qp_delta != 0,
            super::macroblock::Macroblock::P(m) => m.mb_qp_delta != 0,
            super::macroblock::Macroblock::PCM(_) => false,
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
            if accessor.get_mb_type_is_intra(blk_idx as u8, nb) {
                return 0;
            }
            // predModeEqualFlagN: P slices always Pred_L0 unless B slice logic
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
                return 0;
            }

            // Check bit
            if let Some(cbp) = accessor.get_cbp(blk_idx as u8, nb) {
                // neighbor_blk_idx is the index in the neighbor MB.
                // 8x8 block index = neighbor_blk_idx / 4
                let bit_idx = neighbor_blk_idx / 4;
                if (cbp.luma() >> bit_idx) & 1 != 0 {
                    1
                } else {
                    0
                }
            } else {
                0
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

    fn parse_mb_qp_delta_cabac(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<i32> {
        let ctx_idx_offset = 60;
        let ctx_idx_inc = Self::get_ctx_idx_inc_mb_qp_delta(slice, mb_addr);
        
        // Table 9-39: binIdx 0 uses ctxIdxInc, binIdx > 0 uses ctxIdxInc = 2.
        let get_ctx_idx = |bin_idx| {
            if bin_idx == 0 {
                ctx_idx_offset + ctx_idx_inc
            } else {
                ctx_idx_offset + 2
            }
        };
        
        let prefix = self.parse_unary_bin(get_ctx_idx)?;
        
        // Map back to signed value (Table 9-3)
        let val = prefix as i32;
        let delta = if val == 0 {
            0
        } else if val % 2 == 0 {
            -(val / 2)
        } else {
            (val + 1) / 2
        };
        
        Ok(delta)
    }

    fn parse_coded_block_pattern_cabac(
        &mut self,
        slice: &Slice,
        mb_addr: MbAddr,
        curr_mb: &mut CurrentMbInfo,
    ) -> ParseResult<CodedBlockPattern> {
        let ctx_idx_offset = 73;

        // Prefix: Luma (4 bits)
        let mut cbp_luma = 0;
        for i in 0..4 {
            let accessor = NeighborAccessor::new(slice, mb_addr, curr_mb);
            let ctx_idx_inc = Self::get_ctx_idx_inc_cbp_luma(&accessor, i);
            drop(accessor);

            let bit = self.decode_bin(ctx_idx_offset + ctx_idx_inc)?;
            cbp_luma |= (bit as u8) << i;

            // Update partial CBP in current MB so subsequent bits can reference it
            curr_mb.coded_block_pattern = CodedBlockPattern::new(0, cbp_luma);
        }

        // Suffix: Chroma (2 bits max, TU cMax=2)
        let mut cbp_chroma = 0;
        if slice.sps.ChromaArrayType().is_chroma_subsampled() {
            let ctx_idx_offset_chroma = 77;

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
        Ok(cbp)
    }

    pub fn parse_mb_skip_flag(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<bool> {
        let ctx_idx_inc = Self::get_ctx_idx_inc_mb_skip_flag(slice, mb_addr);
        let ctx_idx_offset = if slice.header.slice_type == super::slice::SliceType::B {
            24
        } else {
            11
        };
        let bin = self.decode_bin(ctx_idx_offset + ctx_idx_inc)?;
        Ok(bin == 1)
    }

    // Ref Idx
    fn parse_ref_idx_cabac(
        &mut self,
        accessor: &NeighborAccessor,
        list_idx: usize,
        num_ref_idx_active_minus1: u32,
        mb_part_idx: usize,
    ) -> ParseResult<u8> {
        let ctx_idx_offset = 54;
        let get_ctx_idx = |bin_idx| {
            let ctx_idx_inc = if bin_idx == 0 {
                Self::get_ctx_idx_inc_ref_idx(accessor, mb_part_idx, list_idx)
            } else {
                // Table 9-39: binIdx 1 -> 4, binIdx > 1 -> 5.
                if bin_idx == 1 {
                    4
                } else {
                    5
                }
            };
            ctx_idx_offset + ctx_idx_inc
        };

        let val = self.parse_truncated_unary_bin(num_ref_idx_active_minus1, get_ctx_idx)?;
        Ok(val as u8)
    }

    // MVD
    fn parse_mvd_cabac(
        &mut self,
        accessor: &NeighborAccessor,
        list_idx: usize,
        comp_idx: usize,
        blk_idx: usize,
    ) -> ParseResult<i16> {
        let base_offset = if comp_idx == 0 { 40 } else { 47 };

        let get_ctx_idx = |bin_idx| {
            let ctx_idx_inc = if bin_idx < 3 {
                Self::get_ctx_idx_inc_mvd(accessor, list_idx, comp_idx, blk_idx)
            } else if bin_idx == 3 {
                3
            } else {
                4
            };
            base_offset + ctx_idx_inc
        };

        let val = self.parse_ueg_k(9, 3, true, get_ctx_idx)?;
        Ok(val as i16)
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
        // Table 9-40
        match ctx_block_cat {
            0 | 1 | 2 => scanning_pos, // Luma DC, AC, 4x4
            3 | 4 => min(scanning_pos, 2), // Chroma DC
            5 | 6 => min(scanning_pos, 2) + 12, // Chroma AC
            _ => scanning_pos,
        }
    }

    pub fn get_ctx_idx_inc_last_sig_coeff_flag(ctx_block_cat: usize, scanning_pos: usize) -> usize {
        // Table 9-41
        match ctx_block_cat {
            0 | 1 | 2 => scanning_pos,
            3 | 4 => min(scanning_pos, 2),
            5 | 6 => scanning_pos,
            _ => scanning_pos,
        }
    }

    pub fn get_ctx_idx_inc_abs_level(ctx_block_cat: usize, num_decod_abs_level_gt1: usize, num_decod_abs_level_eq1: usize) -> usize {
        // Table 9-42
        let base = if num_decod_abs_level_gt1 != 0 {
            0
        } else {
            min(4, 1 + num_decod_abs_level_eq1)
        };
        
        match ctx_block_cat {
            0 | 1 | 2 | 5 | 6 => 5 + base,
            3 | 4 => min(4, 1 + num_decod_abs_level_eq1), // Chroma DC
            _ => base,
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
            // Cb DC: Cat 3, comp_idx 0
            self.parse_residual_block_cabac(slice, mb_addr, curr_mb, residual, 3, 0, 0, 4)?;
            // Cr DC: Cat 3, comp_idx 1
            self.parse_residual_block_cabac(slice, mb_addr, curr_mb, residual, 3, 0, 1, 4)?;
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
                // Cr AC: Cat 4, comp_idx 1
                self.parse_residual_block_cabac(
                    slice, mb_addr, curr_mb, residual, 4, i, 1, 15,
                )?;
            }
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
        // 1. coded_block_flag
        let ctx_idx_offset_cbf = match ctx_block_cat {
            0..=4 => 85,
            5 | 6 => 460,
            _ => 85,
        };

        let accessor = NeighborAccessor::new(slice, mb_addr, curr_mb);
        let ctx_idx_inc = Self::get_ctx_idx_inc_coded_block_flag(
            &accessor,
            ctx_block_cat,
            blk_idx,
            comp_idx,
        );
        drop(accessor);

        let cbf = self.decode_bin(ctx_idx_offset_cbf + ctx_idx_inc)? == 1;

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
            5 => {
                if cbf {
                    // Luma 8x8. blk_idx is 0..3 (8x8 block index).
                    let start_x = (blk_idx & 1) * 2;
                    let start_y = (blk_idx >> 1) * 2;
                    for y in 0..2 {
                        for x in 0..2 {
                            let idx4x4 = (start_y + y) * 4 + (start_x + x);
                            curr_mb.cbf.luma_ac |= 1 << idx4x4;
                        }
                    }
                }
            }
            _ => {}
        }

        if !cbf {
            return Ok(false);
        }

        // 2. significant_coeff_flag and last_significant_coeff_flag
        let mut significant_coeff_flag = [false; 64]; 
        let mut last_significant_coeff_flag = [false; 64];
        let mut num_coeff = 0;
        
        // Table 9-34
        let ctx_idx_offset_sig = match ctx_block_cat {
            0..=4 => 105,
            5 => 402,
            6 => 484, // Table 9-34: 5 < ctxBlockCat < 9 -> 484
            _ => 105,
        };
        
        let ctx_idx_offset_last = match ctx_block_cat {
            0..=4 => 166,
            5 => 417,
            6 => 572,
            _ => 166,
        };
        
        let mut last_scan_pos = -1;
        
        for i in 0..max_num_coeff {
            let ctx_idx_inc_sig = Self::get_ctx_idx_inc_sig_coeff_flag(ctx_block_cat, i);
            let sig = self.decode_bin(ctx_idx_offset_sig + ctx_idx_inc_sig)? == 1;
            significant_coeff_flag[i] = sig;
            if sig {
                num_coeff += 1;
                if i == max_num_coeff - 1 {
                    last_scan_pos = i as i32;
                    break;
                }
                
                let ctx_idx_inc_last = Self::get_ctx_idx_inc_last_sig_coeff_flag(ctx_block_cat, i);
                let last = self.decode_bin(ctx_idx_offset_last + ctx_idx_inc_last)? == 1;
                last_significant_coeff_flag[i] = last;
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
        
        // Reverse scan
        for i in (0..=last_scan_pos as usize).rev() {
            if significant_coeff_flag[i] {
                let ctx_idx_offset_abs = match ctx_block_cat {
                    0..=4 => 227,
                    5 => 426,
                    6 => 952,
                    _ => 227,
                };
                
                let get_ctx_idx = |bin_idx: u32| {
                    if bin_idx == 0 {
                        let inc = Self::get_ctx_idx_inc_abs_level(ctx_block_cat, num_decod_abs_level_gt1, num_decod_abs_level_eq1);
                        ctx_idx_offset_abs + inc
                    } else {
                        // Suffix (UEG0) uses Bypass
                        0
                    }
                };
                
                let val_minus1 = self.parse_ueg_k(14, 0, false, get_ctx_idx)? as i32;
                let abs_level = val_minus1 + 1;
                
                // Update counters
                if abs_level == 1 {
                    num_decod_abs_level_eq1 += 1;
                } else {
                    num_decod_abs_level_gt1 += 1;
                }
                
                // Sign
                let sign = self.decode_bypass()?;
                coeff_level[i] = if sign == 1 { -abs_level } else { abs_level };
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
            3 => { // Chroma DC Cb (4 coeffs)
                residual.chroma_cb_dc_level.copy_from_slice(&coeff_level[0..4]);
            },
            4 => { // Chroma DC Cr (4 coeffs)
                residual.chroma_cr_dc_level.copy_from_slice(&coeff_level[0..4]);
            },
            5 => { // Chroma AC Cb (15 coeffs)
                residual.chroma_cb_ac_level[blk_idx].copy_from_slice(&coeff_level[0..15]);
                residual.chroma_cb_level4x4_nc[blk_idx] = num_coeff as u8;
            },
            6 => { // Chroma AC Cr (15 coeffs)
                residual.chroma_cr_ac_level[blk_idx].copy_from_slice(&coeff_level[0..15]);
                residual.chroma_cr_level4x4_nc[blk_idx] = num_coeff as u8;
            },
            _ => {},
        }
        
        Ok(true)
    }


    // 9.3.3.1.1.3 Derivation process of ctxIdxInc for the syntax element mb_type
    fn get_ctx_idx_inc_mb_type_i(slice: &Slice, mb_addr: MbAddr) -> usize {
        let get_flag = |mb: &super::macroblock::Macroblock| match mb {
            super::macroblock::Macroblock::I(m) => m.mb_type != super::macroblock::IMbType::I_NxN,
            super::macroblock::Macroblock::PCM(_) => true, // I_PCM is not I_NxN
            super::macroblock::Macroblock::P(_) => false,  // Should not happen in I slice
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
        let ctx_idx_offset = 3;

        // Bin 0
        let ctx_idx_inc_0 = Self::get_ctx_idx_inc_mb_type_i(slice, mb_addr);
        if self.decode_bin(ctx_idx_offset + ctx_idx_inc_0)? == 0 {
            return Ok(super::macroblock::IMbType::I_NxN);
        }

        // Bin 1: I_PCM check (Terminal)
        if self.decode_terminate()? {
            return Ok(super::macroblock::IMbType::I_PCM);
        }

        // Bin 2
        let ctx_idx_inc_2 = 3;
        if self.decode_bin(ctx_idx_offset + ctx_idx_inc_2)? == 0 {
            return self.parse_i_16x16_suffix(ctx_idx_offset, 0);
        } else {
            return self.parse_i_16x16_suffix(ctx_idx_offset, 12);
        }
    }

    fn parse_i_16x16_suffix(&mut self, ctx_idx_offset: usize, base_type: u32) -> ParseResult<super::macroblock::IMbType> {
        // Bins 3-6 (Fixed ctxIdx 3..6 + offset)
        let bit3 = self.decode_bin(ctx_idx_offset + 4)?;
        let bit4 = self.decode_bin(ctx_idx_offset + 5)?;
        let bit5 = self.decode_bin(ctx_idx_offset + 6)?;
        let bit6 = self.decode_bin(ctx_idx_offset + 7)?;

        let offset = (bit3 as u32) << 3 | (bit4 as u32) << 2 | (bit5 as u32) << 1 | (bit6 as u32);
        super::macroblock::IMbType::try_from(1 + base_type + offset).map_err(|e| e)
    }

    pub fn parse_mb_type_p(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<CabacMbType> {
        // Bin 0 (ctxIdx 14)
        if self.decode_bin(14)? == 1 {
            // Intra. Bin 1 corresponds to Bin 0 of I-slice mb_type
            let i_mb_type = self.parse_mb_type_i_suffix(17, slice, mb_addr)?;
            return Ok(CabacMbType::I(i_mb_type));
        }

        // Bin 1 (ctxIdx 15)
        if self.decode_bin(15)? == 1 {
             // Bin 2 (ctxIdx 16)
             if self.decode_bin(16)? == 1 {
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_L0_16x8));
             } else {
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_L0_8x16));
             }
        } else {
             // Bin 2 (ctxIdx 16)
             if self.decode_bin(16)? == 1 {
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_8x8));
             } else {
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_16x16));
             }
        }
    }

    // Helper for P-slice Intra suffix
    fn parse_mb_type_i_suffix(&mut self, ctx_idx_offset: usize, slice: &Slice, mb_addr: MbAddr) -> ParseResult<super::macroblock::IMbType> {
        // Bin 0 (of suffix, bin 1 of overall P mb_type)
        let ctx_idx_inc_0 = Self::get_ctx_idx_inc_mb_type_i(slice, mb_addr);
        if self.decode_bin(ctx_idx_offset + ctx_idx_inc_0)? == 0 {
            return Ok(super::macroblock::IMbType::I_NxN);
        }

        // Bin 1 (Terminal) -> I_PCM
        if self.decode_terminate()? {
            return Ok(super::macroblock::IMbType::I_PCM);
        }

        // Bin 2
        let ctx_idx_inc_2 = 3;
        if self.decode_bin(ctx_idx_offset + ctx_idx_inc_2)? == 0 {
            return self.parse_i_16x16_suffix(ctx_idx_offset, 0);
        } else {
            return self.parse_i_16x16_suffix(ctx_idx_offset, 12);
        }
    }

    pub fn parse_sub_mb_type_p(&mut self, _slice: &Slice, _mb_addr: MbAddr) -> ParseResult<super::macroblock::SubMbType> {
        let ctx_idx_offset = 21;

        // Bin 0 (ctxIdx 21)
        if self.decode_bin(ctx_idx_offset)? == 1 {
            return Ok(super::macroblock::SubMbType::P_L0_8x8);
        }

        // Bin 1 (ctxIdx 22)
        if self.decode_bin(ctx_idx_offset + 1)? == 0 {
            return Ok(super::macroblock::SubMbType::P_L0_8x4);
        }

        // Bin 2 (ctxIdx 23)
        if self.decode_bin(ctx_idx_offset + 2)? == 1 {
            return Ok(super::macroblock::SubMbType::P_L0_4x8);
        } else {
            return Ok(super::macroblock::SubMbType::P_L0_4x4);
        }
    }

    // 9.3.3.1.1.8 Derivation process of ctxIdxInc for the syntax element intra_chroma_pred_mode
    fn get_ctx_idx_inc_intra_chroma_pred_mode(slice: &Slice, mb_addr: MbAddr) -> usize {
        let get_cond_term_flag = |mb: &super::macroblock::Macroblock| match mb {
             super::macroblock::Macroblock::PCM(_) => true,
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
        let ctx_idx_offset = 64;
        let ctx_idx_inc = Self::get_ctx_idx_inc_intra_chroma_pred_mode(slice, mb_addr);

        let get_ctx_idx = |bin_idx| {
            if bin_idx == 0 {
                ctx_idx_offset + ctx_idx_inc
            } else {
                67 
            }
        };

        let val = self.parse_truncated_unary_bin(3, get_ctx_idx)?;
        super::macroblock::Intra_Chroma_Pred_Mode::try_from(val).map_err(|e| e)
    }

    pub fn parse_macroblock(
        &mut self,
        slice: &mut Slice,
    ) -> ParseResult<super::macroblock::Macroblock> {
        let mb_addr = slice.get_next_mb_addr();

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
                    ..Default::default()
                };
                return Ok(super::macroblock::Macroblock::P(mb));
            }
        }

        let mb_type = if slice.header.slice_type.is_intra() {
            CabacMbType::I(self.parse_mb_type_i(slice, mb_addr)?)
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
                    return Err("PCM not fully supported in CABAC yet".to_string());
                }

                let mut mb = super::macroblock::IMb {
                    mb_type: i_type,
                    ..Default::default()
                };

                // Intra prediction
                if i_type == super::macroblock::IMbType::I_NxN {
                    if slice.pps.transform_8x8_mode_flag {
                        let flag = self.decode_bin(399)? == 1;
                        mb.transform_size_8x8_flag = flag;
                        curr_mb.transform_size_8x8_flag = flag;
                    }

                    if mb.transform_size_8x8_flag {
                        return Err("Intra 8x8 not supported".to_string());
                    } else {
                        for i in 0..16 {
                            let prev_intra_pred_mode_flag = self.decode_bin(68)? == 1;
                            let prev_mode =
                                super::parser::calc_prev_intra4x4_pred_mode(slice, &mb, mb_addr, i);

                            if prev_intra_pred_mode_flag {
                                mb.rem_intra4x4_pred_mode[i] = prev_mode;
                            } else {
                                let rem_intra_pred_mode = self.decode_bin(69)? as u32
                                    | ((self.decode_bin(69)? as u32) << 1)
                                    | ((self.decode_bin(69)? as u32) << 2);

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
                    // Intra_16x16: Derived CBP
                    let type_val = i_type as u32 - 1;
                    let cbp_chroma = match (type_val / 4) % 3 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => 0,
                    };
                    mb.coded_block_pattern = CodedBlockPattern::new(cbp_chroma, 15);
                    curr_mb.coded_block_pattern = mb.coded_block_pattern;
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }

                let mut residual = super::residual::Residual::default();
                self.parse_residual_cabac(slice, mb_addr, &mut curr_mb, &mut residual)?;
                mb.residual = Some(Box::new(residual));

                Ok(super::macroblock::Macroblock::I(mb))
            }
            CabacMbType::P(p_type) => {
                let mut partitions = [PartitionInfo::default(); 4];
                let mut sub_mbs = [super::macroblock::SubMacroblock::default(); 4];

                // Sub MB pred
                if p_type == super::macroblock::PMbType::P_8x8 {
                    for i in 0..4 {
                        sub_mbs[i].sub_mb_type = self.parse_sub_mb_type_p(slice, mb_addr)?;
                    }

                    let num_ref_idx_l0_active_minus1 = slice.header.num_ref_idx_l0_active_minus1;
                    if num_ref_idx_l0_active_minus1 > 0 || slice.header.field_pic_flag {
                        for i in 0..4 {
                            if sub_mbs[i].sub_mb_type != SubMbType::P_L0_8x8 {
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
                    }

                    // mvd
                    for i in 0..4 {
                        if sub_mbs[i].sub_mb_type != SubMbType::P_L0_8x8 {
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
                };

                if !mb.coded_block_pattern.is_zero()
                    && mb.mb_type != super::macroblock::PMbType::P_Skip
                {
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }

                let mut residual = super::residual::Residual::default();
                self.parse_residual_cabac(slice, mb_addr, &mut curr_mb, &mut residual)?;
                mb.residual = Some(Box::new(residual));

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
