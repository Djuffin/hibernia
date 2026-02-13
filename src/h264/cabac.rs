use super::cabac_tables::{
    get_init_table, RANGE_TAB_LPS, TRANS_IDX_LPS, TRANS_IDX_MPS,
};
use super::macroblock::{MbAddr, MbNeighborName};
use super::parser::{BitReader, ParseResult};
use super::slice::Slice;
use std::cmp::min;

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
        get_ctx_idx: F,
    ) -> ParseResult<i32>
    where
        F: FnMut(u32) -> usize,
    {
        // Prefix: TU with cMax = uCoff
        let prefix = self.parse_truncated_unary_bin(u_coff, get_ctx_idx)?;

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
    pub fn get_ctx_idx_inc_mb_skip_flag(&self, slice: &Slice, mb_addr: MbAddr) -> usize {
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
    pub fn get_ctx_idx_inc_mb_qp_delta(&self, slice: &Slice, mb_addr: MbAddr) -> usize {
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

    pub fn parse_mb_skip_flag(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<bool> {
        let ctx_idx_inc = self.get_ctx_idx_inc_mb_skip_flag(slice, mb_addr);
        let ctx_idx_offset = if slice.header.slice_type == super::slice::SliceType::B {
            24
        } else {
            11
        };
        let bin = self.decode_bin(ctx_idx_offset + ctx_idx_inc)?;
        Ok(bin == 1)
    }

    // 9.3.3.1.1.3 Derivation process of ctxIdxInc for the syntax element mb_type
    fn get_ctx_idx_inc_mb_type_i(&self, slice: &Slice, mb_addr: MbAddr) -> usize {
        let get_flag = |mb: &super::macroblock::Macroblock| match mb {
            super::macroblock::Macroblock::I(m) => m.mb_type != super::macroblock::IMbType::I_NxN,
            super::macroblock::Macroblock::PCM(_) => true, // I_PCM is not I_NxN
            super::macroblock::Macroblock::P(_) => false,  // Should not happen in I slice, but valid for P slice Intra check
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
        let ctx_idx_inc_0 = self.get_ctx_idx_inc_mb_type_i(slice, mb_addr);
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
            // I_16x16_0_0_0 to I_16x16_3_2_0 (Values 1 to 12)
            // Coded as 1 0 (0/1) ...
            return self.parse_i_16x16_suffix(ctx_idx_offset, 0);
        } else {
            // I_16x16_0_0_1 to I_16x16_3_2_1 (Values 13 to 24)
            // Coded as 1 0 1 ...
            return self.parse_i_16x16_suffix(ctx_idx_offset, 12);
        }
    }

    fn parse_i_16x16_suffix(&mut self, ctx_idx_offset: usize, base_type: u32) -> ParseResult<super::macroblock::IMbType> {
        // Bin 3
        let bit3 = self.decode_bin(ctx_idx_offset + 4)?;
        // Bin 4
        let bit4 = self.decode_bin(ctx_idx_offset + 5)?;
        // Bin 5
        let bit5 = self.decode_bin(ctx_idx_offset + 6)?; // spec says 6 or 7, but table 9-39 says 6 for binIdx 5
        // Bin 6
        let bit6 = self.decode_bin(ctx_idx_offset + 7)?;

        let offset = (bit3 as u32) << 3 | (bit4 as u32) << 2 | (bit5 as u32) << 1 | (bit6 as u32);
        // Table 9-36:
        // 0000 -> I_16x16_0_0_x -> Value 1 + base
        // ...
        // Value is 1 + base + offset
        super::macroblock::IMbType::try_from(1 + base_type + offset).map_err(|e| e)
    }

    pub fn parse_mb_type_p(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<CabacMbType> {
        let ctx_idx_offset = 14;

        // Bin 0 (ctxIdx 14)
        if self.decode_bin(14)? == 1 {
            // Intra
            // Bin 1 of P-slice mb_type corresponds to Bin 0 of I-slice mb_type (ctxIdx 17 + ctxIdxInc(0))
            let i_mb_type = self.parse_mb_type_i_suffix(17, slice, mb_addr)?;
            return Ok(CabacMbType::I(i_mb_type));
        }

        // 0 ...
        // Bin 1 (ctxIdx 15)
        if self.decode_bin(15)? == 1 {
             // 0 1 ...
             // Bin 2 (ctxIdx 16)
             if self.decode_bin(16)? == 1 {
                 // 0 1 1 -> P_L0_L0_16x8
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_L0_16x8));
             } else {
                 // 0 1 0 -> P_L0_L0_8x16
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_L0_8x16));
             }
        } else {
             // 0 0 ...
             // Bin 2 (ctxIdx 16)
             if self.decode_bin(16)? == 1 {
                 // 0 0 1 -> P_8x8
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_8x8));
             } else {
                 // 0 0 0 -> P_L0_16x16
                 return Ok(CabacMbType::P(super::macroblock::PMbType::P_L0_16x16));
             }
        }
    }

    // Helper for P-slice Intra suffix (which mimics I-slice tree)
    fn parse_mb_type_i_suffix(&mut self, ctx_idx_offset: usize, slice: &Slice, mb_addr: MbAddr) -> ParseResult<super::macroblock::IMbType> {
        // Bin 0 (of suffix, bin 1 of overall P mb_type)
        // Uses get_ctx_idx_inc_mb_type_i logic but with offset 17
        let ctx_idx_inc_0 = self.get_ctx_idx_inc_mb_type_i(slice, mb_addr);
        if self.decode_bin(ctx_idx_offset + ctx_idx_inc_0)? == 0 {
            return Ok(super::macroblock::IMbType::I_NxN);
        }

        // Bin 1 (Terminal) -> I_PCM
        if self.decode_terminate()? {
            return Ok(super::macroblock::IMbType::I_PCM);
        }

        // Bin 2 (ctxIdxOffset + 3)
        let ctx_idx_inc_2 = 3;
        if self.decode_bin(ctx_idx_offset + ctx_idx_inc_2)? == 0 {
            return self.parse_i_16x16_suffix(ctx_idx_offset, 0);
        } else {
            return self.parse_i_16x16_suffix(ctx_idx_offset, 12);
        }
    }

    pub fn parse_sub_mb_type_p(&mut self, _slice: &Slice, _mb_addr: MbAddr) -> ParseResult<super::macroblock::SubMbType> {
        let ctx_idx_offset = 21;
        // Table 9-38
        // 0: P_L0_8x8 -> 1
        // 1: P_L0_8x4 -> 0 0
        // 2: P_L0_4x8 -> 0 1 1
        // 3: P_L0_4x4 -> 0 1 0

        // Bin 0 (ctxIdx 21)
        if self.decode_bin(ctx_idx_offset)? == 1 {
            return Ok(super::macroblock::SubMbType::P_L0_8x8);
        }

        // 0 ...
        // Bin 1 (ctxIdx 22)
        if self.decode_bin(ctx_idx_offset + 1)? == 0 {
            return Ok(super::macroblock::SubMbType::P_L0_8x4);
        }

        // 0 1 ...
        // Bin 2 (ctxIdx 23)
        if self.decode_bin(ctx_idx_offset + 2)? == 1 {
            return Ok(super::macroblock::SubMbType::P_L0_4x8);
        } else {
            return Ok(super::macroblock::SubMbType::P_L0_4x4);
        }
    }

    // 9.3.3.1.1.8 Derivation process of ctxIdxInc for the syntax element intra_chroma_pred_mode
    fn get_ctx_idx_inc_intra_chroma_pred_mode(&self, slice: &Slice, mb_addr: MbAddr) -> usize {
        let get_flag = |mb: &super::macroblock::Macroblock| match mb {
            super::macroblock::Macroblock::I(m) => m.intra_chroma_pred_mode as u32 != 0,
            super::macroblock::Macroblock::P(m) => false, // P block has no chroma pred mode? Wait, I blocks in P slice do.
            super::macroblock::Macroblock::PCM(_) => false, // ??? Spec doesn't say.
        };
        // Spec 9.3.3.1.1.8:
        // condTermFlagN = ( mb_type_N is I_PCM || ( mb_type_N is Intra && intra_chroma_pred_mode_N != 0 ) ) ? 1 : 0
        // Ah, P blocks are not Intra.
        // But if P block contains Intra MB, it's Macroblock::I.
        // So checking if mb is I or PCM.

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
        let ctx_idx_inc = self.get_ctx_idx_inc_intra_chroma_pred_mode(slice, mb_addr);

        // TU with cMax=3.
        // binIdx=0 uses ctxIdx = 64 + ctxIdxInc
        // binIdx>0 uses ctxIdx = 67 (bypass-like fixed context)

        let get_ctx_idx = |bin_idx| {
            if bin_idx == 0 {
                ctx_idx_offset + ctx_idx_inc
            } else {
                67 // Table 9-34: 64..67. 67 is for suffix? No, maxBinIdxCtx=1 in table?
                // Table 9-34 says "maxBinIdxCtx = 1" for intra_chroma_pred_mode.
                // Wait, cMax=3.
                // Table 9-39:
                // binIdx 0: ctxIdxInc = ...
                // binIdx 1: ctxIdxInc = 3.
                // binIdx 2: ctxIdxInc = 3.
                // So ctxIdx = 64 + 3 = 67 for bins 1 and 2.
            }
        };

        let val = self.parse_truncated_unary_bin(3, get_ctx_idx)?;
        super::macroblock::Intra_Chroma_Pred_Mode::try_from(val).map_err(|e| e)
    }

    pub fn parse_macroblock(&mut self, slice: &mut Slice) -> ParseResult<super::macroblock::Macroblock> {
        // Implement parsing logic (Phase 3)
        let mb_addr = slice.get_next_mb_addr();

        if slice.header.slice_type == super::slice::SliceType::P {
             let skipped = self.parse_mb_skip_flag(slice, mb_addr)?;
             if skipped {
                 // Create P_Skip
                 // Motion vector reconstruction for P_Skip is handled in calculate_motion
                 // But we need to construct a basic PMb with P_Skip type
                 let motion = super::parser::calculate_motion(slice, mb_addr, super::macroblock::PMbType::P_Skip, &[super::macroblock::PartitionInfo::default(); 4], &[super::macroblock::SubMacroblock::default(); 4]);
                 let mb = super::macroblock::PMb {
                     mb_type: super::macroblock::PMbType::P_Skip,
                     motion,
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

        match mb_type {
            CabacMbType::I(i_type) => {
                if i_type == super::macroblock::IMbType::I_PCM {
                    // Spec 9.3.1.2

                    return Err("PCM not fully supported in CABAC yet".to_string());
                }

                let mut mb = super::macroblock::IMb {
                    mb_type: i_type,
                    ..Default::default()
                };

                // Intra prediction
                if i_type == super::macroblock::IMbType::I_NxN {
                    // prev_intra4x4_pred_mode_flag: ctxIdx 68
                    // rem_intra4x4_pred_mode: ctxIdx 69

                    // transform_size_8x8_flag if present (ctxIdx 399)
                    if slice.pps.transform_8x8_mode_flag {
                        // ctxIdx 399
                        let flag = self.decode_bin(399)? == 1;
                        mb.transform_size_8x8_flag = flag;
                    }

                    if mb.transform_size_8x8_flag {
                        // Intra 8x8
                        // TODO
                        return Err("Intra 8x8 not supported".to_string());
                    } else {
                        // Intra 4x4
                        for i in 0..16 {
                             let prev_intra_pred_mode_flag = self.decode_bin(68)? == 1;
                             let prev_mode = super::parser::calc_prev_intra4x4_pred_mode(slice, &mb, mb_addr, i);

                             if prev_intra_pred_mode_flag {
                                 mb.rem_intra4x4_pred_mode[i] = prev_mode;
                             } else {
                                 let rem_intra_pred_mode = self.decode_bin(69)? as u32
                                     | ((self.decode_bin(69)? as u32) << 1)
                                     | ((self.decode_bin(69)? as u32) << 2);
                                 // "rem_intra4x4_pred_mode" is FL cMax=7.
                                 // Wait, FL binarization with cMax=7 is 3 bits fixed length.
                                 // Table 9-34 says "FL, cMax=7"
                                 // But ctxIdxOffset = 69.
                                 // Table 9-39: all bins use ctxIdxInc=0 (so ctxIdx 69).

                                 if rem_intra_pred_mode < (prev_mode as u32) {
                                     mb.rem_intra4x4_pred_mode[i] = super::macroblock::Intra_4x4_SamplePredMode::try_from(rem_intra_pred_mode)?;
                                 } else {
                                     mb.rem_intra4x4_pred_mode[i] = super::macroblock::Intra_4x4_SamplePredMode::try_from(rem_intra_pred_mode + 1)?;
                                 }
                             }
                        }
                    }
                }

                // Intra Chroma Pred Mode
                if slice.sps.ChromaArrayType().is_chroma_subsampled() {
                    mb.intra_chroma_pred_mode = self.parse_intra_chroma_pred_mode(slice, mb_addr)?;
                }

                // Continue with Phase 4 (CBP, QP, Residual)
                // For now, return incomplete MB
                Ok(super::macroblock::Macroblock::I(mb))
            }
            CabacMbType::P(p_type) => {
                let mb = super::macroblock::PMb {
                    mb_type: p_type,
                    ..Default::default()
                };

                // Sub MB pred
                if p_type == super::macroblock::PMbType::P_8x8 {
                    let mut sub_mbs = [super::macroblock::SubMacroblock::default(); 4];
                    for i in 0..4 {
                        sub_mbs[i].sub_mb_type = self.parse_sub_mb_type_p(slice, mb_addr)?;
                    }
                    // Phase 4: ref_idx, mvd for sub partitions
                } else {
                    // ref_idx, mvd
                    // Phase 4
                }

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
