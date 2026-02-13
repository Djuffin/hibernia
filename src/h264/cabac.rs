use super::cabac_tables::{
    get_init_table, RANGE_TAB_LPS, TRANS_IDX_LPS, TRANS_IDX_MPS,
};
use super::macroblock::{
    MbAddr, MbNeighborName, MotionVector, PartitionInfo, SubMbType, CodedBlockPattern,
};
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
    pub fn get_ctx_idx_inc_ref_idx(
        _slice: &Slice,
        _mb_addr: MbAddr,
        _mb_part_idx: usize,
        _sub_mb_part_idx: usize,
        _list_idx: usize,
    ) -> usize {
        // TODO: Implement proper neighbor lookup (Section 6.4.11.7)
        // For now, return 0 as a placeholder.
        0
    }

    // 9.3.3.1.1.7 Derivation process of ctxIdxInc for the syntax elements mvd_l0 and mvd_l1
    pub fn get_ctx_idx_inc_mvd(
        _slice: &Slice,
        _mb_addr: MbAddr,
        _list_idx: usize,
        _comp_idx: usize,
        _blk_idx: usize, // 4x4 block index
    ) -> usize {
        // TODO: Implement proper neighbor lookup using mvd_l0 from PartitionInfo.
        0
    }
    
    // 9.3.3.1.1.4 Derivation process of ctxIdxInc for the syntax element coded_block_pattern
    pub fn get_ctx_idx_inc_cbp(
        _slice: &Slice,
        _mb_addr: MbAddr,
        _bin_idx: u32,
    ) -> usize {
        // TODO: Implement neighbor lookup based on binIdx and luma8x8BlkIdx.
        0
    }

    pub fn parse_mb_qp_delta_cabac(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<i32> {
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

    pub fn parse_coded_block_pattern_cabac(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<CodedBlockPattern> {
        let ctx_idx_offset = 73;
        
        // Prefix: Luma (4 bits)
        let mut cbp_luma = 0;
        for i in 0..4 {
            let ctx_idx_inc = Self::get_ctx_idx_inc_cbp(slice, mb_addr, i);
            let bit = self.decode_bin(ctx_idx_offset + ctx_idx_inc)?;
            cbp_luma |= (bit as u8) << i;
        }
        
        // Suffix: Chroma (2 bits max, TU cMax=2)
        let mut cbp_chroma = 0;
        if slice.sps.ChromaArrayType().is_chroma_subsampled() {
             let ctx_idx_offset_chroma = 77;
             // TODO: implement get_ctx_idx_inc_cbp_chroma
             let bit0 = self.decode_bin(ctx_idx_offset_chroma)?; // bin 0
             if bit0 == 1 {
                 let bit1 = self.decode_bin(ctx_idx_offset_chroma + 1)?; // bin 1
                 if bit1 == 1 {
                     cbp_chroma = 2; // 11 -> 2
                 } else {
                     cbp_chroma = 1; // 10 -> 1
                 }
             } else {
                 cbp_chroma = 0; // 0
             }
        }
        
        Ok(CodedBlockPattern::new(cbp_chroma, cbp_luma))
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
    pub fn parse_ref_idx_cabac(&mut self, slice: &Slice, mb_addr: MbAddr, list_idx: usize, num_ref_idx_active_minus1: u32, mb_part_idx: usize, sub_mb_part_idx: usize) -> ParseResult<u8> {
        let ctx_idx_offset = 54;
        let get_ctx_idx = |bin_idx| {
            let ctx_idx_inc = if bin_idx == 0 {
                Self::get_ctx_idx_inc_ref_idx(slice, mb_addr, mb_part_idx, sub_mb_part_idx, list_idx)
            } else {
                // Table 9-39: binIdx 1 -> 4, binIdx > 1 -> 5.
                if bin_idx == 1 { 4 } else { 5 }
            };
            ctx_idx_offset + ctx_idx_inc
        };
        
        let val = self.parse_truncated_unary_bin(num_ref_idx_active_minus1, get_ctx_idx)?;
        Ok(val as u8)
    }

    // MVD
    pub fn parse_mvd_cabac(&mut self, _slice: &Slice, _mb_addr: MbAddr, _list_idx: usize, comp_idx: usize, _blk_idx: usize) -> ParseResult<i16> {
        let base_offset = if comp_idx == 0 { 40 } else { 47 };
        
        let get_ctx_idx = |_bin_idx| {
            // TODO: Implement ctxIdxInc derivation for MVD (9.3.3.1.1.7)
            base_offset
        };
        
        let val = self.parse_ueg_k(9, 3, true, get_ctx_idx)?;
        Ok(val as i16)
    }

    // 9.3.3.1.3 Assignment process of ctxIdxInc for syntax elements significant_coeff_flag, last_significant_coeff_flag, and coeff_abs_level_minus1
    // And 9.3.3.1.1.9 for coded_block_flag
    pub fn get_ctx_idx_inc_coded_block_flag(_slice: &Slice, _mb_addr: MbAddr, _ctx_block_cat: usize, _blk_idx: usize) -> usize {
        // TODO: Implement neighbor lookup for coded_block_flag.
        0
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

    pub fn parse_residual_cabac(&mut self, slice: &Slice, mb: &mut super::macroblock::Macroblock) -> ParseResult<()> {
        let mut residual = Box::new(super::residual::Residual::default());
        residual.prediction_mode = match mb {
            super::macroblock::Macroblock::I(m) => if m.mb_type == super::macroblock::IMbType::I_NxN { super::macroblock::MbPredictionMode::Intra_4x4 } else { super::macroblock::MbPredictionMode::Intra_16x16 },
            super::macroblock::Macroblock::P(_) => super::macroblock::MbPredictionMode::Pred_L0,
            _ => super::macroblock::MbPredictionMode::None,
        };
        residual.coded_block_pattern = mb.get_coded_block_pattern();
        
        // 1. Luma DC (if Intra 16x16)
        if residual.prediction_mode == super::macroblock::MbPredictionMode::Intra_16x16 {
            // ctxBlockCat = 0
            if self.parse_residual_block_cabac(slice, &mut residual, 0, 0, 16)? {
                // Has coefficients
            }
        }
        
        // 2. Luma AC (if Intra 16x16) or Luma 4x4 (others)
        for i in 0..16 {
            let (ctx_block_cat, max_num_coeff) = if residual.prediction_mode == super::macroblock::MbPredictionMode::Intra_16x16 {
                (1, 15)
            } else {
                (2, 16)
            };
            
            if residual.coded_block_pattern.luma() & (1 << (i / 4)) != 0 {
                self.parse_residual_block_cabac(slice, &mut residual, ctx_block_cat, i, max_num_coeff)?;
            }
        }
        
        // 3. Chroma DC
        if slice.sps.ChromaArrayType().is_chroma_subsampled() && residual.coded_block_pattern.chroma() != 0 {
             // Cb DC: Cat 3
             self.parse_residual_block_cabac(slice, &mut residual, 3, 0, 4)?;
             // Cr DC: Cat 4
             self.parse_residual_block_cabac(slice, &mut residual, 4, 0, 4)?;
        }
        
        // 4. Chroma AC
        if slice.sps.ChromaArrayType().is_chroma_subsampled() && residual.coded_block_pattern.chroma() == 2 {
             for i in 0..4 {
                 // Cb AC: Cat 5
                 self.parse_residual_block_cabac(slice, &mut residual, 5, i, 15)?;
                 // Cr AC: Cat 6
                 self.parse_residual_block_cabac(slice, &mut residual, 6, i, 15)?;
             }
        }
        
        mb.set_residual(Some(residual));
        Ok(())
    }

    // 9.3.3.2.3 and 7.3.5.3.3
    pub fn parse_residual_block_cabac(&mut self, slice: &Slice, residual: &mut super::residual::Residual, ctx_block_cat: usize, blk_idx: usize, max_num_coeff: usize) -> ParseResult<bool> {
        // 1. coded_block_flag
        let ctx_idx_offset_cbf = match ctx_block_cat {
            0..=4 => 85,
            5 | 6 => 460,
            _ => 85,
        };
        
        let ctx_idx_inc = Self::get_ctx_idx_inc_coded_block_flag(slice, 0, ctx_block_cat, blk_idx);
        let cbf = self.decode_bin(ctx_idx_offset_cbf + ctx_idx_inc)? == 1;
        
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

    pub fn parse_macroblock(&mut self, slice: &mut Slice) -> ParseResult<super::macroblock::Macroblock> {
        let mb_addr = slice.get_next_mb_addr();

        if slice.header.slice_type == super::slice::SliceType::P {
             let skipped = self.parse_mb_skip_flag(slice, mb_addr)?;
             if skipped {
                 let motion = super::parser::calculate_motion(slice, mb_addr, super::macroblock::PMbType::P_Skip, &[super::macroblock::PartitionInfo::default(); 4], &[super::macroblock::SubMacroblock::default(); 4]);
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
                    }

                    if mb.transform_size_8x8_flag {
                        return Err("Intra 8x8 not supported".to_string());
                    } else {
                        for i in 0..16 {
                             let prev_intra_pred_mode_flag = self.decode_bin(68)? == 1;
                             let prev_mode = super::parser::calc_prev_intra4x4_pred_mode(slice, &mb, mb_addr, i);

                             if prev_intra_pred_mode_flag {
                                 mb.rem_intra4x4_pred_mode[i] = prev_mode;
                             } else {
                                 let rem_intra_pred_mode = self.decode_bin(69)? as u32
                                     | ((self.decode_bin(69)? as u32) << 1)
                                     | ((self.decode_bin(69)? as u32) << 2);
                                 
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
                if slice.sps.ChromaArrayType().is_chroma_subsampled() && i_type != super::macroblock::IMbType::I_PCM {
                    mb.intra_chroma_pred_mode = self.parse_intra_chroma_pred_mode(slice, mb_addr)?;
                }
                
                // CBP and QP
                if i_type == super::macroblock::IMbType::I_NxN {
                    mb.coded_block_pattern = self.parse_coded_block_pattern_cabac(slice, mb_addr)?;
                    if !mb.coded_block_pattern.is_zero() || mb.mb_type == super::macroblock::IMbType::I_PCM {
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
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }
                
                let mut macroblock = super::macroblock::Macroblock::I(mb);
                self.parse_residual_cabac(slice, &mut macroblock)?;
                
                Ok(macroblock)
            }
            CabacMbType::P(p_type) => {
                let mut mb = super::macroblock::PMb {
                    mb_type: p_type,
                    ..Default::default()
                };
                
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
                                let ref_idx = self.parse_ref_idx_cabac(slice, mb_addr, 0, num_ref_idx_l0_active_minus1, i, 0)?;
                                sub_mbs[i].partitions[0].ref_idx_l0 = ref_idx;
                                sub_mbs[i].partitions[1].ref_idx_l0 = ref_idx;
                                sub_mbs[i].partitions[2].ref_idx_l0 = ref_idx;
                                sub_mbs[i].partitions[3].ref_idx_l0 = ref_idx;
                            }
                        }
                    }
                    
                    // mvd
                    for i in 0..4 {
                        if sub_mbs[i].sub_mb_type != SubMbType::P_L0_8x8 {
                             let num_sub_part = sub_mbs[i].sub_mb_type.NumSubMbPart();
                             for j in 0..num_sub_part {
                                 let mvd_x = self.parse_mvd_cabac(slice, mb_addr, 0, 0, 0)?;
                                 let mvd_y = self.parse_mvd_cabac(slice, mb_addr, 0, 1, 0)?;
                                 
                                 let p_idx = match (sub_mbs[i].sub_mb_type, j) {
                                     (SubMbType::P_L0_8x8, 0) => 0,
                                     (SubMbType::P_L0_8x4, 0) => 0,
                                     (SubMbType::P_L0_8x4, 1) => 2,
                                     (SubMbType::P_L0_4x8, 0) => 0,
                                     (SubMbType::P_L0_4x8, 1) => 1,
                                     (SubMbType::P_L0_4x4, x) => x,
                                     _ => 0,
                                 };
                                 sub_mbs[i].partitions[p_idx].mvd_l0 = MotionVector { x: mvd_x, y: mvd_y };
                             }
                        }
                    }
                    
                } else {
                    let num_part = p_type.NumMbPart();
                    let num_ref_idx_l0_active_minus1 = slice.header.num_ref_idx_l0_active_minus1;
                    
                    if num_ref_idx_l0_active_minus1 > 0 || slice.header.field_pic_flag {
                        for i in 0..num_part {
                            let ref_idx = self.parse_ref_idx_cabac(slice, mb_addr, 0, num_ref_idx_l0_active_minus1, i, 0)?;
                            partitions[i].ref_idx_l0 = ref_idx;
                        }
                    }
                    
                    for i in 0..num_part {
                        let mvd_x = self.parse_mvd_cabac(slice, mb_addr, 0, 0, 0)?;
                        let mvd_y = self.parse_mvd_cabac(slice, mb_addr, 0, 1, 0)?;
                        partitions[i].mvd_l0 = MotionVector { x: mvd_x, y: mvd_y };
                    }
                }
                
                mb.coded_block_pattern = self.parse_coded_block_pattern_cabac(slice, mb_addr)?;
                if !mb.coded_block_pattern.is_zero() && mb.mb_type != super::macroblock::PMbType::P_Skip {
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }
                
                let mut macroblock = super::macroblock::Macroblock::P(mb);
                self.parse_residual_cabac(slice, &mut macroblock)?;
                Ok(macroblock)
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
