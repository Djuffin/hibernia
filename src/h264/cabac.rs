use super::cabac_tables::{
    get_init_table, RANGE_TAB_LPS, TRANS_IDX_LPS, TRANS_IDX_MPS,
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

    fn renorm(&mut self) -> ParseResult<()> {
        while self.range < 256 {
            self.range <<= 1;
            self.offset = (self.offset << 1) | (self.reader.u(1)? as u32);
        }
        Ok(())
    }
}
