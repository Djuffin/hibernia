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

    fn renorm(&mut self) -> ParseResult<()> {
        while self.range < 256 {
            self.range <<= 1;
            self.offset = (self.offset << 1) | (self.reader.u(1)? as u32);
        }
        Ok(())
    }
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
