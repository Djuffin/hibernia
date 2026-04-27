use super::macroblock::{
    CbfInfo, CodedBlockPattern, MbAddr, MbMotion, MbNeighborName, MotionVector, PartitionInfo,
    PcmMb, SubMbType,
};
use super::parser::{BitReader, ParseResult};
use super::residual::Residual;
use super::slice::Slice;
use super::tables::{get_init_table, RANGE_TAB_LPS, TRANS_IDX_LPS, TRANS_IDX_MPS};
use log::trace;
use std::cmp::min;

struct CurrentMbInfo {
    mb_type: CabacMbType,
    motion: MbMotion,
    coded_block_pattern: CodedBlockPattern,
    transform_size_8x8_flag: bool,
    cbf: CbfInfo,
}

/// MB-level snapshot of one neighbor (A or B) used by the CBF context derivation
/// in `parse_residual_block_cabac`. The same neighbor MB is consulted for all 27
/// possible cbf decodes within the current MB, so we precompute everything that
/// doesn't depend on `blk_idx` once and avoid repeated `slice.get_mb_neighbor`
/// lookups and per-block match arms.
#[derive(Default, Clone, Copy)]
struct CachedNeighborMb {
    available: bool,
    is_pcm: bool,
    is_skipped: bool,
    is_intra_16x16: bool,
    transform_size_8x8_flag: bool,
    cbp: CodedBlockPattern,
    cbf: CbfInfo,
}

struct ResidualNeighborCache {
    a: CachedNeighborMb,
    b: CachedNeighborMb,
}

impl ResidualNeighborCache {
    fn build(slice: &Slice, mb_addr: MbAddr) -> Self {
        ResidualNeighborCache {
            a: snapshot_neighbor_mb(slice, mb_addr, MbNeighborName::A),
            b: snapshot_neighbor_mb(slice, mb_addr, MbNeighborName::B),
        }
    }

    #[inline]
    fn get(&self, dir: MbNeighborName) -> &CachedNeighborMb {
        match dir {
            MbNeighborName::A => &self.a,
            MbNeighborName::B => &self.b,
            // CBF context derivation only consults A and B.
            _ => unreachable!("ResidualNeighborCache only stores A and B"),
        }
    }
}

fn snapshot_neighbor_mb(
    slice: &Slice,
    mb_addr: MbAddr,
    dir: MbNeighborName,
) -> CachedNeighborMb {
    let mb = match slice.get_mb_neighbor(mb_addr, dir) {
        Some(m) => m,
        None => return CachedNeighborMb::default(),
    };
    let (transform_size_8x8_flag, is_intra_16x16, is_pcm) = match mb {
        super::macroblock::Macroblock::I(m) => (
            m.transform_size_8x8_flag,
            m.mb_type != super::macroblock::IMbType::I_NxN
                && m.mb_type != super::macroblock::IMbType::I_PCM,
            false,
        ),
        super::macroblock::Macroblock::P(m) => (m.transform_size_8x8_flag, false, false),
        super::macroblock::Macroblock::B(m) => (m.transform_size_8x8_flag, false, false),
        super::macroblock::Macroblock::PCM(_) => (false, false, true),
    };
    CachedNeighborMb {
        available: true,
        is_pcm,
        is_skipped: mb.is_skipped(),
        is_intra_16x16,
        transform_size_8x8_flag,
        cbp: mb.get_coded_block_pattern(),
        cbf: mb.get_cbf_info(),
    }
}

struct NeighborAccessor<'a> {
    slice: &'a Slice,
    mb_addr: MbAddr,
    curr_mb: &'a CurrentMbInfo,
}

impl<'a> NeighborAccessor<'a> {
    fn new(slice: &'a Slice, mb_addr: MbAddr, curr_mb: &'a CurrentMbInfo) -> Self {
        NeighborAccessor { slice, mb_addr, curr_mb }
    }

    fn get_mb_type_is_intra(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => match self.curr_mb.mb_type {
                CabacMbType::I(_) => true,
                CabacMbType::P(_) | CabacMbType::B(_) => false,
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
                CabacMbType::B(super::macroblock::BMbType::B_Skip) => true,
                _ => false,
            },
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| mb.is_skipped())
                .unwrap_or(false),
        }
    }

    /// Check if neighbor block is in "direct" prediction mode.
    /// This includes B_Skip, B_Direct_16x16, and B_Direct_8x8 sub-partitions.
    /// Used by ref_idx context derivation (clause 9.3.3.1.1.6) which treats
    /// direct-predicted neighbors as condTermFlagN = 0.
    fn is_direct_mode(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => match self.curr_mb.mb_type {
                CabacMbType::B(super::macroblock::BMbType::B_Skip)
                | CabacMbType::B(super::macroblock::BMbType::B_Direct_16x16) => true,
                _ => false,
            },
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| match mb {
                    super::macroblock::Macroblock::B(bmb) => {
                        match bmb.mb_type {
                            super::macroblock::BMbType::B_Skip
                            | super::macroblock::BMbType::B_Direct_16x16 => true,
                            super::macroblock::BMbType::B_8x8 => {
                                // Check if the specific 8x8 block is B_Direct_8x8
                                // by examining the sub_mb_types stored in the MB
                                let p =
                                    super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                                let b8_idx = ((p.x / 8) + (p.y / 8) * 2) as usize;
                                bmb.sub_mb_types[b8_idx]
                                    == super::macroblock::BSubMbType::B_Direct_8x8
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                })
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

    fn get_ref_idx(
        &self,
        blk_idx: u8,
        neighbor_name: MbNeighborName,
        list_idx: usize,
    ) -> Option<u8> {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => {
                // Current MB
                let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                let x = (p.x / 4) as usize;
                let y = (p.y / 4) as usize;
                let p_info = &self.curr_mb.motion.partitions[y][x];
                if list_idx == 0 {
                    Some(p_info.ref_idx_l0)
                } else {
                    Some(p_info.ref_idx_l1)
                }
            }
            Some(nb_name) => self.slice.get_mb_neighbor(self.mb_addr, nb_name).map(|mb| {
                let motion = match mb {
                    super::macroblock::Macroblock::P(pmb) => &pmb.motion,
                    super::macroblock::Macroblock::B(bmb) => &bmb.motion,
                    super::macroblock::Macroblock::I(_) | super::macroblock::Macroblock::PCM(_) => {
                        return 0
                    }
                };
                let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                let x = (p.x / 4) as usize;
                let y = (p.y / 4) as usize;
                if list_idx == 0 {
                    motion.partitions[y][x].ref_idx_l0
                } else {
                    motion.partitions[y][x].ref_idx_l1
                }
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
                    Some(p_info.mvd_l1)
                }
            }
            Some(nb_name) => self.slice.get_mb_neighbor(self.mb_addr, nb_name).map(|mb| {
                let motion = match mb {
                    super::macroblock::Macroblock::P(pmb) => &pmb.motion,
                    super::macroblock::Macroblock::B(bmb) => &bmb.motion,
                    _ => return MotionVector::default(),
                };
                let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                let x = (p.x / 4) as usize;
                let y = (p.y / 4) as usize;
                if list_idx == 0 {
                    motion.partitions[y][x].mvd_l0
                } else {
                    motion.partitions[y][x].mvd_l1
                }
            }),
        }
    }

    fn get_pred_mode(
        &self,
        blk_idx: u8,
        neighbor_name: MbNeighborName,
    ) -> super::macroblock::MbPredictionMode {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => {
                // Current MB
                let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                let x = (p.x / 4) as usize;
                let y = (p.y / 4) as usize;
                self.curr_mb.motion.partitions[y][x].pred_mode
            }
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| {
                    let motion = match mb {
                        super::macroblock::Macroblock::P(pmb) => &pmb.motion,
                        super::macroblock::Macroblock::B(bmb) => &bmb.motion,
                        super::macroblock::Macroblock::I(_)
                        | super::macroblock::Macroblock::PCM(_) => {
                            return super::macroblock::MbPredictionMode::None
                        }
                    };
                    let p = super::macroblock::get_4x4luma_block_location(neighbor_blk_idx);
                    let x = (p.x / 4) as usize;
                    let y = (p.y / 4) as usize;
                    motion.partitions[y][x].pred_mode
                })
                .unwrap_or(super::macroblock::MbPredictionMode::None),
        }
    }

    fn get_cbp(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> Option<CodedBlockPattern> {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => Some(self.curr_mb.coded_block_pattern),
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| mb.get_coded_block_pattern()),
        }
    }

    fn is_pcm(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => {
                matches!(self.curr_mb.mb_type, CabacMbType::I(super::macroblock::IMbType::I_PCM))
            }
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| matches!(mb, super::macroblock::Macroblock::PCM(_)))
                .unwrap_or(false),
        }
    }

    fn get_transform_size_8x8_flag(&self, blk_idx: u8, neighbor_name: MbNeighborName) -> bool {
        let (neighbor_blk_idx, mb_neighbor) =
            super::macroblock::get_4x4luma_block_neighbor(blk_idx, neighbor_name);

        match mb_neighbor {
            None => self.curr_mb.transform_size_8x8_flag,
            Some(nb_name) => self
                .slice
                .get_mb_neighbor(self.mb_addr, nb_name)
                .map(|mb| match mb {
                    super::macroblock::Macroblock::I(m) => m.transform_size_8x8_flag,
                    super::macroblock::Macroblock::P(m) => m.transform_size_8x8_flag,
                    super::macroblock::Macroblock::B(m) => m.transform_size_8x8_flag,
                    super::macroblock::Macroblock::PCM(_) => false,
                })
                .unwrap_or(false),
        }
    }
}

pub struct CabacContext<'a, 'b> {
    reader: &'a mut BitReader<'b>,
    range: u32,  // codIRange
    offset: u32, // codIOffset
    // Pre-fetched bits from the underlying reader. Valid bits occupy the
    // highest `n_bits` positions of `bit_buf`; lower bits are zero. This
    // removes the per-call `bitstream_io` overhead from the CABAC hot path:
    // `renorm` and `decode_bypass` drain bits from here and the buffer is
    // refilled 32 bits at a time. Pre-fetched bits not yet consumed by CABAC
    // must be pushed back to the reader before any direct reader access
    // (see `sync_reader_position`, used by the I_PCM path).
    bit_buf: u64,
    n_bits: u32,
    // Bits still available in the underlying reader (not yet pulled into
    // `bit_buf`). We track this ourselves because `bitstream_io::BitReader`
    // consumes partial bits and poisons itself when a read request exceeds
    // what is available, which would otherwise eat the tail of the stream.
    // Refill paths consult this counter before each call to guarantee the
    // read will succeed.
    reader_remaining: u64,
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
    MbType { prior: u8 },
}

impl<'a, 'b> CabacContext<'a, 'b> {
    pub fn new(reader: &'a mut BitReader<'b>, slice: &Slice) -> ParseResult<Self> {
        let mut ctx = CabacContext {
            reader,
            range: 510,
            offset: 0,
            bit_buf: 0,
            n_bits: 0,
            reader_remaining: 0,
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

            let pre_ctx_state = (((m * qp_clipped) >> 4) + n).clamp(1, 126);

            let (p_state_idx, val_mps) =
                if pre_ctx_state <= 63 { (63 - pre_ctx_state, 0) } else { (pre_ctx_state - 64, 1) };

            self.ctx_table[ctx_idx] = (p_state_idx as u8) << 1 | (val_mps as u8);
        }
    }

    // 9.3.1.2 Initialization process for the arithmetic decoding engine
    fn init_decoding_engine(&mut self) -> ParseResult<()> {
        self.range = 510;
        // Re-sync the remaining-bits counter against the reader's actual
        // position. Required when this is called after I_PCM raw reads
        // advanced the reader outside our tracking; harmless on first init.
        self.reader_remaining = self.reader.remaining();
        self.offset = self.read_bits(9)?;

        if self.offset == 510 || self.offset == 511 {
            // "The bitstream shall not contain data that result in a value of codIOffset being equal to 510 or 511."
            return Err("codIOffset equal to 510 or 511 is illegal".to_string());
        }

        Ok(())
    }

    // Pull bits from the underlying reader into `bit_buf`. The fast path does
    // one 32-bit read to amortise `bitstream_io` call overhead over many
    // CABAC bins; the cold path walks byte-by-byte (and then bit-by-bit)
    // near EOF. We only ask the reader for N bits when we know N bits are
    // available — `bitstream_io` poisons the reader on a failed read, which
    // would otherwise swallow any leftover bits.
    #[inline]
    fn refill_bits(&mut self) -> ParseResult<()> {
        // Refuse to refill once the buffer already holds > 32 bits, otherwise
        // the 32-bit fast path would shift new bits past position 0.
        if self.n_bits > 32 {
            return Ok(());
        }
        if self.reader_remaining >= 32 {
            let bits = self.reader.u(32)?;
            self.reader_remaining -= 32;
            // Place new 32 bits immediately below the existing valid region.
            self.bit_buf |= (bits as u64) << (32 - self.n_bits);
            self.n_bits += 32;
            return Ok(());
        }
        self.refill_slow()
    }

    // Cold-path refill for end-of-stream. Pulls remaining whole bytes, then
    // any sub-byte tail bit-by-bit, stopping when either the reader is
    // exhausted or the buffer is full.
    #[cold]
    #[inline(never)]
    fn refill_slow(&mut self) -> ParseResult<()> {
        while self.reader_remaining >= 8 && self.n_bits <= 56 {
            let byte = self.reader.u(8)?;
            self.reader_remaining -= 8;
            self.bit_buf |= (byte as u64) << (56 - self.n_bits);
            self.n_bits += 8;
        }
        while self.reader_remaining > 0 && self.n_bits <= 63 {
            let bit = self.reader.u(1)?;
            self.reader_remaining -= 1;
            self.bit_buf |= (bit as u64) << (63 - self.n_bits);
            self.n_bits += 1;
        }
        Ok(())
    }

    // Rewind the underlying reader so that any bits still held in the
    // CABAC pre-fetch buffer are "unread". Call this before switching from
    // CABAC-driven reads to direct byte-aligned reads (I_PCM payload).
    // After the direct reads finish, `init_decoding_engine` re-primes the
    // buffer.
    fn sync_reader_position(&mut self) -> ParseResult<()> {
        if self.n_bits > 0 {
            self.reader.rewind(self.n_bits)?;
            self.reader_remaining += self.n_bits as u64;
            self.bit_buf = 0;
            self.n_bits = 0;
        }
        Ok(())
    }

    // Take `n` bits off the top of the buffer, refilling from the reader
    // when necessary. Callers must pass 1 <= n <= 32.
    #[inline(always)]
    fn read_bits(&mut self, n: u32) -> ParseResult<u32> {
        debug_assert!(n >= 1 && n <= 32);
        if self.n_bits < n {
            self.refill_bits()?;
            if self.n_bits < n {
                return Err(format!(
                    "CABAC: needed {} bits, only {} available",
                    n, self.n_bits
                ));
            }
        }
        let bits = (self.bit_buf >> (64 - n)) as u32;
        self.bit_buf <<= n;
        self.n_bits -= n;
        Ok(bits)
    }

    // 9.3.2.1 Unary (U) binarization process
    pub fn parse_unary_bin(&mut self, se: SyntaxElement, ctx: CtxIncParams) -> ParseResult<u32> {
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
        self.parse_truncated_unary_bin_with(se, c_max_override, ctx, &props)
    }

    /// Same as [`parse_truncated_unary_bin`] but takes a prefetched table entry,
    /// avoiding the per-call lookup. Used by per-coefficient hot paths where the
    /// caller has already resolved the entry once for the whole block.
    pub fn parse_truncated_unary_bin_with(
        &mut self,
        se: SyntaxElement,
        c_max_override: Option<u32>,
        ctx: CtxIncParams,
        props: &CabacTableEntry,
    ) -> ParseResult<u32> {
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
    pub fn parse_ueg_k(&mut self, se: SyntaxElement, ctx: CtxIncParams) -> ParseResult<i32> {
        let props = get_syntax_element_properties(se);
        self.parse_ueg_k_with(se, ctx, &props)
    }

    /// Same as [`parse_ueg_k`] but takes a prefetched table entry, avoiding the
    /// per-call lookup. Per-coefficient hot path: `parse_residual_block_cabac`
    /// fetches `CoeffAbsLevelMinus1` props once and reuses for every level.
    pub fn parse_ueg_k_with(
        &mut self,
        se: SyntaxElement,
        ctx: CtxIncParams,
        props: &CabacTableEntry,
    ) -> ParseResult<i32> {
        let (u_coff, k_val, signed_val_flag) =
            if let BinarizationType::UEGk { u_coff, k, signed_val_flag } = props.binarization {
                (u_coff, k, signed_val_flag)
            } else {
                panic!("parse_ueg_k called on non-UEGk syntax element: {:?}", se);
            };

        // Prefix: TU with cMax = uCoff
        let prefix = self.parse_truncated_unary_bin_with(se, Some(u_coff), ctx, props)?;

        if prefix < u_coff {
            let val = prefix as i32;
            let final_val = if signed_val_flag && val != 0 {
                let sign = self.decode_bypass()?;
                if sign == 1 {
                    -val
                } else {
                    val
                }
            } else {
                val
            };
            trace!("parse_ueg_k se={:?} val={}", se, final_val);
            return Ok(final_val);
        }

        // Suffix: EGk. The outer unary must be decoded one bin at a time
        // because we stop on the first 0, but the fixed-length `k`-bit tail
        // can be decoded in one batched pass through the bypass state
        // machine.
        let mut suffix_val = 0;
        let mut k = k_val;
        loop {
            let bit = self.decode_bypass()?;
            if bit == 1 {
                suffix_val += 1 << k;
                k += 1;
            } else {
                suffix_val += self.decode_bypass_bits(k)?;
                break;
            }
        }

        let val = (prefix + suffix_val) as i32;

        let final_val = if signed_val_flag && val != 0 {
            let sign = self.decode_bypass()?;
            if sign == 1 {
                -val
            } else {
                val
            }
        } else {
            val
        };
        trace!("parse_ueg_k se={:?} val={}", se, final_val);
        Ok(final_val)
    }

    // 9.3.3.2 Arithmetic decoding process
    #[inline(always)]
    pub fn decode_bin(&mut self, ctx_idx: usize) -> ParseResult<u8> {
        let ctx_state = self.ctx_table[ctx_idx];
        // Mask with 0x3F tells the optimizer p_state_idx < 64, eliminating
        // bounds checks on the fixed-size state/range tables below.
        let mut p_state_idx = ((ctx_state >> 1) & 0x3F) as usize;
        let mut val_mps = ctx_state & 1;

        let q_cod_i_range_idx = ((self.range >> 6) & 3) as usize;
        let cod_i_range_lps = RANGE_TAB_LPS[p_state_idx][q_cod_i_range_idx] as u32;

        self.range -= cod_i_range_lps;

        // Fuse the MPS/LPS decision with the 9.3.3.2.1.1 state transition so
        // each path only runs its own table lookup.
        let bin_val;
        if self.offset >= self.range {
            // LPS path
            bin_val = 1 - val_mps;
            self.offset -= self.range;
            self.range = cod_i_range_lps;
            if p_state_idx == 0 {
                val_mps = 1 - val_mps;
            }
            p_state_idx = TRANS_IDX_LPS[p_state_idx] as usize;
        } else {
            // MPS path
            bin_val = val_mps;
            p_state_idx = TRANS_IDX_MPS[p_state_idx] as usize;
        }

        self.ctx_table[ctx_idx] = (p_state_idx as u8) << 1 | val_mps;

        self.renorm()?;
        trace!("decode_bin ctxIdx={} bin={}", ctx_idx, bin_val);
        Ok(bin_val)
    }

    // 9.3.3.2.3 Bypass decoding process
    #[inline]
    pub fn decode_bypass(&mut self) -> ParseResult<u8> {
        self.offset = (self.offset << 1) | self.read_bits(1)?;

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

    // Decode `n` consecutive bypass-coded bins in one call. Returns them
    // packed MSB-first: bit `n-1` is the first decoded bin, bit `0` the
    // last. Amortises the bit-buffer read across all `n` bins instead of
    // paying one `read_bits(1)` per bin. Intended for the fixed-length
    // suffix in EGk (9.3.2.3).
    //
    // Caller must pass n <= 32. n == 0 is a no-op that returns 0.
    #[inline]
    fn decode_bypass_bits(&mut self, n: u32) -> ParseResult<u32> {
        if n == 0 {
            return Ok(0);
        }
        debug_assert!(n <= 32);
        // Left-justify the `n` stream bits to the top of a u32 so we can
        // extract the next bin with a constant `>> 31` on each iteration
        // rather than a variable shift.
        let mut remaining = self.read_bits(n)? << (32 - n);
        let mut result = 0u32;
        for _ in 0..n {
            let b = remaining >> 31;
            remaining <<= 1;
            self.offset = (self.offset << 1) | b;
            result <<= 1;
            if self.offset >= self.range {
                result |= 1;
                self.offset -= self.range;
            }
        }
        Ok(result)
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

        // Eq 9-7: condition combines `mb_type != Intra_16x16` with `CBP == 0`,
        // so the two predicates are derived independently below.
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
            super::macroblock::Macroblock::B(m) => {
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
        // Map mb_part_idx to a 4x4 block index for the top-left block of the partition.
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
            CabacMbType::B(b_type) => {
                if b_type == super::macroblock::BMbType::B_8x8 {
                    match mb_part_idx {
                        0 => 0,
                        1 => 4,
                        2 => 8,
                        3 => 12,
                        _ => 0,
                    }
                } else {
                    let (_, h) = b_type.MbPartSize();
                    if b_type.NumMbPart() <= 1 {
                        0
                    } else if h == 8 {
                        // 16x8: partition 0 at blk 0, partition 1 at blk 8
                        if mb_part_idx == 0 {
                            0
                        } else {
                            8
                        }
                    } else {
                        // 8x16: partition 0 at blk 0, partition 1 at blk 4
                        if mb_part_idx == 0 {
                            0
                        } else {
                            4
                        }
                    }
                }
            }
            _ => 0,
        };

        let check_neighbor = |nb: MbNeighborName| -> usize {
            if !accessor.is_available(blk_idx, nb) {
                return 0;
            }
            if accessor.get_mb_type_is_intra(blk_idx, nb) {
                return 0;
            }
            // Per clause 9.3.3.1.1.6: B_Skip B_Direct_16x16, and B_Direct_8x8 neighbors
            // have condTermFlagN = 0.
            // Their derived ref_idx from direct prediction is NOT used.
            if accessor.is_direct_mode(blk_idx, nb) {
                return 0;
            }
            if accessor.slice.MbaffFrameFlag() {
                unimplemented!(
                    "MBAFF field macroblock logic for RefIdx context derivation (Eq 9-12)"
                );
            }
            let pred_mode = accessor.get_pred_mode(blk_idx, nb);
            let pred_mode_equal = match list_idx {
                0 => matches!(
                    pred_mode,
                    super::macroblock::MbPredictionMode::Pred_L0
                        | super::macroblock::MbPredictionMode::BiPred
                        | super::macroblock::MbPredictionMode::None
                ),
                1 => matches!(
                    pred_mode,
                    super::macroblock::MbPredictionMode::Pred_L1
                        | super::macroblock::MbPredictionMode::BiPred
                ),
                _ => true,
            };
            if !pred_mode_equal {
                return 0;
            }
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
            // predModeEqualFlagN (clause 9.3.3.1.1.7):
            // If neighbor's partition doesn't use the current list, absMvdCompN = 0.
            let pred_mode = accessor.get_pred_mode(blk_idx as u8, nb);
            let pred_mode_equal = match list_idx {
                0 => matches!(
                    pred_mode,
                    super::macroblock::MbPredictionMode::Pred_L0
                        | super::macroblock::MbPredictionMode::BiPred
                        | super::macroblock::MbPredictionMode::None
                ),
                1 => matches!(
                    pred_mode,
                    super::macroblock::MbPredictionMode::Pred_L1
                        | super::macroblock::MbPredictionMode::BiPred
                ),
                _ => true,
            };
            if !pred_mode_equal {
                return 0;
            }
            if accessor.slice.MbaffFrameFlag() {
                unimplemented!(
                    "MBAFF field macroblock scaling for MVD context derivation (Eq 9-15, 9-16)"
                );
            }
            let mvd = accessor.get_mvd(blk_idx as u8, nb, list_idx).unwrap_or_default();
            let val = if comp_idx == 0 { mvd.x } else { mvd.y };
            val.unsigned_abs() as usize
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
        // CBP-chroma is a per-MB syntax element, so any blk_idx selects the
        // same A/B macroblock neighbors.
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
        let cond_term_flag_a =
            accessor.get_transform_size_8x8_flag(blk_idx, MbNeighborName::A) as usize;
        let cond_term_flag_b =
            accessor.get_transform_size_8x8_flag(blk_idx, MbNeighborName::B) as usize;

        cond_term_flag_a + cond_term_flag_b
    }

    fn parse_mb_qp_delta_cabac(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<i32> {
        let ctx_idx_inc = Self::get_ctx_idx_inc_mb_qp_delta(slice, mb_addr);

        // Table 9-34 says maxBinIdxCtx=2 for MbQpDelta.
        // 9.3.2.7 says it's Unary binarization.
        let mapped_val =
            self.parse_unary_bin(SyntaxElement::MbQpDelta, CtxIncParams::Standard(ctx_idx_inc))?;

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


            let bit0 = self.decode_bin(ctx_idx_offset_chroma + ctx_idx_inc_0)?; // bin 0
            if bit0 == 1 {
                let accessor = NeighborAccessor::new(slice, mb_addr, curr_mb);
                let ctx_idx_inc_1 = Self::get_ctx_idx_inc_cbp_chroma(&accessor, 1);


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
        let ctx_idx = (props.ctx_idx_offset as usize) + ctx_idx_inc;
        let bin = self.decode_bin(ctx_idx)?;
        let skip = bin == 1;
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

        // Table 9-34 specifies U (Unary) binarization for ref_idx_l0/l1.
        let val = self.parse_unary_bin(
            SyntaxElement::RefIdx(list_idx),
            CtxIncParams::Standard(ctx_idx_inc),
        )?;

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

        let val = self.parse_ueg_k(
            SyntaxElement::Mvd(list_idx, comp_idx),
            CtxIncParams::Standard(ctx_idx_inc_0),
        )?;
        let mvd = val as i16;
        trace!("parse_mvd_cabac list={} comp={} blk={} val={}", list_idx, comp_idx, blk_idx, mvd);
        Ok(mvd)
    }

    // 9.3.3.1.3 Assignment process of ctxIdxInc for syntax elements significant_coeff_flag, last_significant_coeff_flag, and coeff_abs_level_minus1
    // And 9.3.3.1.1.9 for coded_block_flag
    /// Cached variant of `get_ctx_idx_inc_coded_block_flag`. Reads neighbor
    /// MB-level state from a `ResidualNeighborCache` precomputed at the start of
    /// `parse_residual_cabac` instead of re-walking `slice.get_mb_neighbor` and
    /// pattern-matching the neighbor `Macroblock` on every cbf decode (28× per
    /// MB in the worst case). Behavior must match `get_ctx_idx_inc_coded_block_flag`.
    fn get_ctx_idx_inc_coded_block_flag_cached(
        cache: &ResidualNeighborCache,
        curr_mb: &CurrentMbInfo,
        ctx_block_cat: usize,
        blk_idx: usize,
        comp_idx: usize,
    ) -> usize {
        #[inline]
        fn cbf_bit(
            cbf: &CbfInfo,
            ctx_block_cat: usize,
            comp_idx: usize,
            neighbor_blk_idx: u8,
        ) -> bool {
            match ctx_block_cat {
                0 => cbf.luma_dc,
                1 | 2 | 5 => (cbf.luma_ac >> neighbor_blk_idx) & 1 != 0,
                3 => {
                    if comp_idx == 0 {
                        cbf.cb_dc
                    } else {
                        cbf.cr_dc
                    }
                }
                4 => {
                    if comp_idx == 0 {
                        (cbf.cb_ac >> neighbor_blk_idx) & 1 != 0
                    } else {
                        (cbf.cr_ac >> neighbor_blk_idx) & 1 != 0
                    }
                }
                _ => false,
            }
        }

        let check_neighbor = |dir: MbNeighborName| -> usize {
            let (neighbor_blk_idx, mb_neighbor) = if ctx_block_cat == 3 || ctx_block_cat == 4 {
                super::macroblock::get_4x4chroma_block_neighbor(blk_idx as u8, dir)
            } else {
                super::macroblock::get_4x4luma_block_neighbor(blk_idx as u8, dir)
            };
            let internal = mb_neighbor.is_none();
            let nb = cache.get(dir);

            // is_available
            if !internal && !nb.available {
                let is_current_intra = matches!(curr_mb.mb_type, CabacMbType::I(_));
                return if is_current_intra { 1 } else { 0 };
            }

            // is_pcm
            let is_pcm = if internal {
                matches!(
                    curr_mb.mb_type,
                    CabacMbType::I(super::macroblock::IMbType::I_PCM)
                )
            } else {
                nb.is_pcm
            };
            if is_pcm {
                return 1;
            }

            // is_skipped
            let is_skipped = if internal {
                matches!(
                    curr_mb.mb_type,
                    CabacMbType::P(super::macroblock::PMbType::P_Skip)
                        | CabacMbType::B(super::macroblock::BMbType::B_Skip)
                )
            } else {
                nb.is_skipped
            };
            if is_skipped {
                return 0;
            }

            // get_cbp — for the internal case the caller-side branch in the
            // original used Some(curr_mb.coded_block_pattern), so cbp is always
            // available once we reach this point.
            let cbp = if internal {
                curr_mb.coded_block_pattern
            } else {
                nb.cbp
            };
            let cbf_src = if internal { &curr_mb.cbf } else { &nb.cbf };

            match ctx_block_cat {
                0 | 6 | 10 => {
                    // Intra16x16 DC
                    let is_intra_16x16 = if internal {
                        matches!(
                            curr_mb.mb_type,
                            CabacMbType::I(t)
                                if t != super::macroblock::IMbType::I_NxN
                                    && t != super::macroblock::IMbType::I_PCM
                        )
                    } else {
                        nb.is_intra_16x16
                    };
                    if is_intra_16x16 {
                        cbf_bit(cbf_src, ctx_block_cat, 0, neighbor_blk_idx) as usize
                    } else {
                        0
                    }
                }
                1 | 2 => {
                    let bit_idx = neighbor_blk_idx / 4;
                    if (cbp.luma() >> bit_idx) & 1 == 0 {
                        return 0;
                    }
                    let is_8x8 = if internal {
                        curr_mb.transform_size_8x8_flag
                    } else {
                        nb.transform_size_8x8_flag
                    };
                    let cat = if is_8x8 { 5 } else { ctx_block_cat };
                    cbf_bit(cbf_src, cat, 0, neighbor_blk_idx) as usize
                }
                3 => {
                    if cbp.chroma() == 0 {
                        return 0;
                    }
                    cbf_bit(cbf_src, 3, comp_idx, neighbor_blk_idx) as usize
                }
                4 => {
                    if cbp.chroma() != 2 {
                        return 0;
                    }
                    cbf_bit(cbf_src, 4, comp_idx, neighbor_blk_idx) as usize
                }
                5 => {
                    let bit_idx = neighbor_blk_idx / 4;
                    if (cbp.luma() >> bit_idx) & 1 == 0 {
                        return 0;
                    }
                    let is_8x8 = if internal {
                        curr_mb.transform_size_8x8_flag
                    } else {
                        nb.transform_size_8x8_flag
                    };
                    if !is_8x8 {
                        return 0;
                    }
                    cbf_bit(cbf_src, 5, 0, neighbor_blk_idx) as usize
                }
                6..=13 => {
                    unimplemented!("Coded block flag context derivation for categories 6-13 (ChromaArrayType 3)");
                }
                _ => 0,
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
            // Table 9-43 (frame scan). Field scan is unsupported — callers reject
            // field_pic_flag upstream in parse_residual_block_cabac.
            super::tables::SIG_COEFF_FLAG_CTX_IDX_INC_8X8_FRAME[scanning_pos] as usize
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
            super::tables::LAST_SIG_COEFF_FLAG_CTX_IDX_INC_8X8[scanning_pos] as usize
        } else {
            scanning_pos
        }
    }

    pub fn get_ctx_idx_inc_abs_level(
        ctx_block_cat: usize,
        bin_idx: u32,
        num_decod_abs_level_gt1: usize,
        num_decod_abs_level_eq1: usize,
    ) -> usize {
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
    fn get_ctx_idx_inc(se: SyntaxElement, bin_idx: u32, ctx: CtxIncParams) -> usize {
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
                if bin_idx == 0 {
                    initial
                } else {
                    unreachable!("MbTypeI bin {} needs CtxIncParams::MbType", bin_idx)
                }
            }
            (SyntaxElement::MbTypeI, CtxIncParams::MbType { prior }) => {
                // Table 9-39 Offset 3
                match bin_idx {
                    2 => 3,
                    3 => 4,
                    4 => {
                        if prior == 0 {
                            6
                        } else {
                            5
                        }
                    }
                    5 => {
                        if prior == 0 {
                            7
                        } else {
                            6
                        }
                    }
                    6 => 7,
                    _ => unreachable!("Invalid binIdx {} for MbTypeI with prior", bin_idx),
                }
            }
            (SyntaxElement::MbTypeP, CtxIncParams::Standard(_)) => {
                // Table 9-39 Offset 14
                match bin_idx {
                    0 => 0,
                    1 => 1,
                    _ => unreachable!("MbTypeP bin {} needs CtxIncParams::MbType", bin_idx),
                }
            }
            (SyntaxElement::MbTypeP, CtxIncParams::MbType { prior }) => {
                // Table 9-39 Offset 14
                match bin_idx {
                    2 => {
                        if prior == 1 {
                            3
                        } else {
                            2
                        }
                    }
                    _ => unreachable!("Invalid binIdx {} for MbTypeP with prior", bin_idx),
                }
            }
            (SyntaxElement::MbTypeISuffix, CtxIncParams::Standard(initial)) => {
                // Table 9-39 Offset 17 (P-slice Intra suffix)
                // Bin 0 of the suffix corresponds to the I_NxN check.
                if bin_idx == 0 {
                    initial
                } else {
                    unreachable!("MbTypeISuffix bin {} needs CtxIncParams::MbType", bin_idx)
                }
            }
            (SyntaxElement::MbTypeISuffix, CtxIncParams::MbType { prior }) => {
                // Table 9-39 Offset 17
                match bin_idx {
                    2 => 1,
                    3 => 2,
                    4 => {
                        if prior == 0 {
                            3
                        } else {
                            2
                        }
                    }
                    5 => 3,
                    6 => 3,
                    _ => unreachable!("Invalid binIdx {} for MbTypeISuffix", bin_idx),
                }
            }
            (SyntaxElement::MbSkipFlagB, _) => {
                // Same as MbSkipFlagP — ctxIdxInc passed via Standard
                unreachable!("MbSkipFlagB should not call get_ctx_idx_inc");
            }
            (SyntaxElement::MbTypeB, CtxIncParams::Standard(initial)) => {
                // Table 9-39, ctxIdxOffset 27
                if bin_idx == 0 {
                    initial // condTermFlagA + condTermFlagB
                } else {
                    unreachable!("MbTypeB bin {} needs CtxIncParams::MbType", bin_idx)
                }
            }
            (SyntaxElement::MbTypeB, CtxIncParams::MbType { prior }) => {
                // Table 9-39/9-41, ctxIdxOffset 27
                match bin_idx {
                    1 => 3,
                    2 => {
                        // Table 9-41: ctxIdxInc = (b1 != 0) ? 4 : 5
                        if prior != 0 {
                            4
                        } else {
                            5
                        }
                    }
                    3 => 5,
                    _ => unreachable!("Invalid binIdx {} for MbTypeB", bin_idx),
                }
            }
            (SyntaxElement::SubMbTypeB, CtxIncParams::Standard(_)) => {
                // Table 9-39, ctxIdxOffset 36
                match bin_idx {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => unreachable!("Invalid binIdx {} for SubMbTypeB", bin_idx),
                }
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
                    if curr_mb.transform_size_8x8_flag {
                        super::macroblock::MbPredictionMode::Intra_8x8
                    } else {
                        super::macroblock::MbPredictionMode::Intra_4x4
                    }
                } else {
                    super::macroblock::MbPredictionMode::Intra_16x16
                }
            }
            CabacMbType::P(_) | CabacMbType::B(_) => super::macroblock::MbPredictionMode::Pred_L0,
        };
        residual.coded_block_pattern = curr_mb.coded_block_pattern;
        residual.transform_size_8x8_flag = curr_mb.transform_size_8x8_flag;

        // Initialize the luma variant; parse_residual_block_cabac requires the
        // matching layout to be set when it goes to write coefficients.
        if residual.prediction_mode == super::macroblock::MbPredictionMode::Intra_16x16 {
            residual.luma.init_intra_16x16();
        } else if curr_mb.transform_size_8x8_flag {
            residual.luma.init_8x8();
        } else {
            residual.luma.init_4x4();
        }

        // Snapshot A/B neighbor MBs once for all cbf-context decodes in this MB.
        let neighbor_cache = ResidualNeighborCache::build(slice, mb_addr);

        // 1. Luma DC (if Intra 16x16)
        if residual.prediction_mode == super::macroblock::MbPredictionMode::Intra_16x16 {
            // ctxBlockCat = 0
            self.parse_residual_block_cabac(
                slice,
                mb_addr,
                curr_mb,
                &neighbor_cache,
                residual,
                0,
                0,
                0,
                16,
            )?;
        }

        // 2. Luma AC (if Intra 16x16), Luma 8x8 (if transform_8x8_flag), or Luma 4x4 (others).
        // Intra_16x16 forbids the 8x8 transform, so the 8x8 path is mutually exclusive
        // with the DC/AC case above.
        if curr_mb.transform_size_8x8_flag {
            for i8x8 in 0..4 {
                if residual.coded_block_pattern.luma() & (1 << i8x8) != 0 {
                    self.parse_residual_block_cabac(
                        slice,
                        mb_addr,
                        curr_mb,
                        &neighbor_cache,
                        residual,
                        5,
                        i8x8,
                        0,
                        64,
                    )?;
                }
            }
        } else {
            for i in 0..16 {
                let (ctx_block_cat, max_num_coeff) = if residual.prediction_mode
                    == super::macroblock::MbPredictionMode::Intra_16x16
                {
                    (1, 15)
                } else {
                    (2, 16)
                };

                if residual.coded_block_pattern.luma() & (1 << (i / 4)) != 0 {
                    self.parse_residual_block_cabac(
                        slice,
                        mb_addr,
                        curr_mb,
                        &neighbor_cache,
                        residual,
                        ctx_block_cat,
                        i,
                        0,
                        max_num_coeff,
                    )?;
                }
            }
        }

        // 3. Chroma DC
        if slice.sps.ChromaArrayType().is_chroma_subsampled()
            && residual.coded_block_pattern.chroma() != 0
        {
            if slice.sps.ChromaArrayType() == super::ChromaFormat::YUV422 {
                return Err("YUV422 chroma DC residual parsing is not supported".into());
            }
            // Cb DC: Cat 3, comp_idx 0
            self.parse_residual_block_cabac(
                slice,
                mb_addr,
                curr_mb,
                &neighbor_cache,
                residual,
                3,
                0,
                0,
                4,
            )?;
            // Cr DC: Cat 3, comp_idx 1
            self.parse_residual_block_cabac(
                slice,
                mb_addr,
                curr_mb,
                &neighbor_cache,
                residual,
                3,
                0,
                1,
                4,
            )?;
        } else if slice.sps.ChromaArrayType() == super::ChromaFormat::YUV444 {
            return Err("YUV444 chroma residual parsing is not supported".into());
        }

        // 4. Chroma AC
        if slice.sps.ChromaArrayType().is_chroma_subsampled()
            && residual.coded_block_pattern.chroma() == 2
        {
            for i in 0..4 {
                // Cb AC: Cat 4, comp_idx 0
                self.parse_residual_block_cabac(
                    slice,
                    mb_addr,
                    curr_mb,
                    &neighbor_cache,
                    residual,
                    4,
                    i,
                    0,
                    15,
                )?;
            }
            for i in 0..4 {
                // Cr AC: Cat 4, comp_idx 1
                self.parse_residual_block_cabac(
                    slice,
                    mb_addr,
                    curr_mb,
                    &neighbor_cache,
                    residual,
                    4,
                    i,
                    1,
                    15,
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
        neighbor_cache: &ResidualNeighborCache,
        residual: &mut super::residual::Residual,
        ctx_block_cat: usize,
        blk_idx: usize,
        comp_idx: usize,
        max_num_coeff: usize,
    ) -> ParseResult<bool> {
        trace!(
            "parse_residual_block_cabac cat={} blk={} comp={}",
            ctx_block_cat,
            blk_idx,
            comp_idx
        );
        // 1. coded_block_flag
        if slice.header.field_pic_flag {
            return Err("field-coded macroblock decoding is not supported".into());
        }
        let cbf_props = get_syntax_element_properties(SyntaxElement::CodedBlockFlag(ctx_block_cat));
        let ctx_idx_offset_cbf = cbf_props.ctx_idx_offset as usize;

        let cbf = if max_num_coeff != 64
            || slice.sps.ChromaArrayType() == super::ChromaFormat::YUV444
        {
            let ctx_idx_inc = Self::get_ctx_idx_inc_coded_block_flag_cached(
                neighbor_cache,
                curr_mb,
                ctx_block_cat,
                blk_idx,
                comp_idx,
            );
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
            5 => {
                // For an 8x8 luma block, propagate CBF to the 4 constituent 4x4
                // positions in luma_ac so neighbor lookups (which index by 4x4
                // blkIdx) see a consistent non-zero CBF.
                if cbf {
                    curr_mb.cbf.luma_ac |= 0xF << (blk_idx * 4);
                }
            }
            _ => {}
        }

        trace!("parse_residual_block_cabac cbf={}", cbf);
        if !cbf {
            return Ok(false);
        }

        // 2. significant_coeff_flag / last_significant_coeff_flag.
        assert!(max_num_coeff <= 64);
        // Bitmask of significant coefficient positions. Bit `i` is 1 if scanning position `i` is non-zero.
        let mut sig_map = 0u64;

        let sig_props =
            get_syntax_element_properties(SyntaxElement::SignificantCoeffFlag(ctx_block_cat));
        let ctx_idx_offset_sig = sig_props.ctx_idx_offset as usize;

        let last_props =
            get_syntax_element_properties(SyntaxElement::LastSignificantCoeffFlag(ctx_block_cat));
        let ctx_idx_offset_last = last_props.ctx_idx_offset as usize;

        for i in 0..max_num_coeff {
            if i == max_num_coeff - 1 {
                sig_map |= 1 << i;
                break;
            }

            let ctx_idx_inc_sig = Self::get_ctx_idx_inc_sig_coeff_flag(ctx_block_cat, i);
            let sig = self.decode_bin(ctx_idx_offset_sig + ctx_idx_inc_sig)? == 1;
            trace!("parse_residual_block_cabac sig_coeff[{}]={}", i, sig);
            if sig {
                sig_map |= 1 << i;

                let ctx_idx_inc_last = Self::get_ctx_idx_inc_last_sig_coeff_flag(ctx_block_cat, i);
                let last = self.decode_bin(ctx_idx_offset_last + ctx_idx_inc_last)? == 1;
                trace!("parse_residual_block_cabac last_sig_coeff[{}]={}", i, last);
                if last {
                    break;
                }
            }
        }

        // 3. coeff_abs_level_minus1.
        let num_coeff = sig_map.count_ones() as u8;
        let mut num_decod_abs_level_eq1 = 0;
        let mut num_decod_abs_level_gt1 = 0;
        let mut coeff_level = [0i32; 64];

        let abs_props =
            get_syntax_element_properties(SyntaxElement::CoeffAbsLevelMinus1(ctx_block_cat));
        let ctx_idx_offset_abs = abs_props.ctx_idx_offset as usize;

        let mut remaining_sig = sig_map;
        while remaining_sig != 0 {
            let pos = 63 - remaining_sig.leading_zeros();
            remaining_sig ^= 1 << pos;

            let val_minus1 = self.parse_abs_level_minus1(
                ctx_block_cat,
                ctx_idx_offset_abs,
                num_decod_abs_level_gt1,
                num_decod_abs_level_eq1,
                &abs_props,
            )?;
            let abs_level = (val_minus1 + 1) as i32;

            if abs_level == 1 {
                num_decod_abs_level_eq1 += 1;
            } else {
                num_decod_abs_level_gt1 += 1;
            }

            let sign = self.decode_bypass()?;
            let level = if sign == 1 { -abs_level } else { abs_level };
            // Mask with 63 to completely guarantee to LLVM that no bounds check is needed
            coeff_level[(pos as usize) & 63] = level;
        }

        // 4. Store coefficients in Residual
        use super::residual::LumaResidual;
        match ctx_block_cat {
            0 => {
                // Luma DC (16 coeffs)
                let LumaResidual::Intra16x16 { dc, .. } = &mut residual.luma else {
                    unreachable!("Luma DC requires Intra_16x16 layout");
                };
                dc.copy_from_slice(&coeff_level[..16]);
            }
            1 => {
                // Luma AC (15 coeffs)
                let LumaResidual::Intra16x16 { ac, ac_nc, .. } = &mut residual.luma else {
                    unreachable!("Luma AC requires Intra_16x16 layout");
                };
                ac[blk_idx].copy_from_slice(&coeff_level[0..15]);
                ac_nc[blk_idx] = num_coeff;
            }
            2 => {
                // Luma 4x4 (16 coeffs)
                let LumaResidual::Block4x4 { levels, nc } = &mut residual.luma else {
                    unreachable!("Luma 4x4 requires Block4x4 layout");
                };
                levels[blk_idx].copy_from_slice(&coeff_level[0..16]);
                nc[blk_idx] = num_coeff;
            }
            3 => {
                // Chroma DC Cb/Cr
                let levels = residual.get_dc_levels_for(if comp_idx == 0 {
                    super::ColorPlane::Cb
                } else {
                    super::ColorPlane::Cr
                });
                levels.copy_from_slice(&coeff_level[0..4]);
            }
            4 => {
                // Chroma AC Cb/Cr
                let (levels, nc) = residual.get_ac_levels_for(
                    blk_idx as u8,
                    if comp_idx == 0 { super::ColorPlane::Cb } else { super::ColorPlane::Cr },
                );
                levels.copy_from_slice(&coeff_level[0..15]);
                *nc = num_coeff;
            }
            5 => {
                // Luma 8x8 (64 coeffs, zig-zag order). Consumed by the 8x8 inverse
                // transform path in Residual::restore via unzip_block_8x8.
                let LumaResidual::Block8x8 { levels, .. } = &mut residual.luma else {
                    unreachable!("Luma 8x8 requires Block8x8 layout");
                };
                levels[blk_idx].0.copy_from_slice(&coeff_level[..64]);
            }
            _ => {}
        }

        Ok(true)
    }

    fn parse_abs_level_minus1(
        &mut self,
        ctx_block_cat: usize,
        _ctx_idx_offset_abs: usize,
        num_decod_abs_level_gt1: usize,
        num_decod_abs_level_eq1: usize,
        abs_props: &CabacTableEntry,
    ) -> ParseResult<u32> {
        let val = self.parse_ueg_k_with(
            SyntaxElement::CoeffAbsLevelMinus1(ctx_block_cat),
            CtxIncParams::AbsLevel { gt1: num_decod_abs_level_gt1, eq1: num_decod_abs_level_eq1 },
            abs_props,
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
            super::macroblock::Macroblock::B(_) => true,   // B is not I_NxN
        };

        let cond_term_flag_a =
            slice.get_mb_neighbor(mb_addr, MbNeighborName::A).map(get_flag).unwrap_or(false);

        let cond_term_flag_b =
            slice.get_mb_neighbor(mb_addr, MbNeighborName::B).map(get_flag).unwrap_or(false);

        (cond_term_flag_a as usize) + (cond_term_flag_b as usize)
    }

    pub fn parse_mb_type_i(
        &mut self,
        slice: &Slice,
        mb_addr: MbAddr,
    ) -> ParseResult<super::macroblock::IMbType> {
        let props = get_syntax_element_properties(SyntaxElement::MbTypeI);
        let ctx_idx_offset = props.ctx_idx_offset as usize;

        // Bin 0
        let ctx_idx_inc_0 = Self::get_ctx_idx_inc_mb_type_i(slice, mb_addr);
        let inc0 =
            Self::get_ctx_idx_inc(SyntaxElement::MbTypeI, 0, CtxIncParams::Standard(ctx_idx_inc_0));
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

    fn parse_i_16x16_params(
        &mut self,
        ctx_idx_offset: usize,
        se: SyntaxElement,
    ) -> ParseResult<super::macroblock::IMbType> {
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
        let res = super::macroblock::IMbType::try_from(mb_type_val);
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
        let inc2 =
            Self::get_ctx_idx_inc(SyntaxElement::MbTypeP, 2, CtxIncParams::MbType { prior: b1 });
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

    // Table 9-37: Binarization for macroblock types in B slices
    pub fn parse_mb_type_b(&mut self, slice: &Slice, mb_addr: MbAddr) -> ParseResult<CabacMbType> {
        let props = get_syntax_element_properties(SyntaxElement::MbTypeB);
        let ctx_idx_offset = props.ctx_idx_offset as usize;
        let ctx_idx_offset_suffix = props.ctx_idx_offset_suffix.unwrap() as usize;

        // Context for bin 0: condTermFlagA + condTermFlagB
        // condTermFlagN = 0 if mbAddrN not available, or if mbN is B_Skip or B_Direct_16x16
        let get_flag = |mb: &super::macroblock::Macroblock| match mb {
            super::macroblock::Macroblock::B(m) => {
                m.mb_type != super::macroblock::BMbType::B_Skip
                    && m.mb_type != super::macroblock::BMbType::B_Direct_16x16
            }
            _ => true, // non-B MBs: condTermFlag = 1
        };
        let cond_a =
            slice.get_mb_neighbor(mb_addr, MbNeighborName::A).map(get_flag).unwrap_or(false);
        let cond_b =
            slice.get_mb_neighbor(mb_addr, MbNeighborName::B).map(get_flag).unwrap_or(false);
        let ctx_idx_inc_0 = (cond_a as usize) + (cond_b as usize);

        // Bin 0
        let inc0 =
            Self::get_ctx_idx_inc(SyntaxElement::MbTypeB, 0, CtxIncParams::Standard(ctx_idx_inc_0));
        let b0 = self.decode_bin(ctx_idx_offset + inc0)?;
        if b0 == 0 {
            trace!("parse_mb_type_b type=B_Direct_16x16");
            return Ok(CabacMbType::B(super::macroblock::BMbType::B_Direct_16x16));
        }

        // Bin 1
        let inc1 =
            Self::get_ctx_idx_inc(SyntaxElement::MbTypeB, 1, CtxIncParams::MbType { prior: 0 });
        let b1 = self.decode_bin(ctx_idx_offset + inc1)?;
        if b1 == 0 {
            // Bin 2
            let inc2 = Self::get_ctx_idx_inc(
                SyntaxElement::MbTypeB,
                2,
                CtxIncParams::MbType { prior: b1 },
            );
            let b2 = self.decode_bin(ctx_idx_offset + inc2)?;
            if b2 == 0 {
                trace!("parse_mb_type_b type=B_L0_16x16");
                return Ok(CabacMbType::B(super::macroblock::BMbType::B_L0_16x16));
            } else {
                trace!("parse_mb_type_b type=B_L1_16x16");
                return Ok(CabacMbType::B(super::macroblock::BMbType::B_L1_16x16));
            }
        }

        // b0=1, b1=1: Bin 2 (context, ctxIdxInc depends on b1 per Table 9-41)
        let inc2 =
            Self::get_ctx_idx_inc(SyntaxElement::MbTypeB, 2, CtxIncParams::MbType { prior: b1 });
        let b2 = self.decode_bin(ctx_idx_offset + inc2)?;
        // Bin 3 (context, ctxIdxInc = 5)
        let inc3 =
            Self::get_ctx_idx_inc(SyntaxElement::MbTypeB, 3, CtxIncParams::MbType { prior: 0 });
        if b2 == 0 {
            let b3 = self.decode_bin(ctx_idx_offset + inc3)?;
            // 2 context-coded bins for types 3-10 (Table 9-37)
            // Reference decoder uses biari_decode_symbol with ctx[6] (ctxIdxInc=5) for all bins after bin 2
            let bp0 = self.decode_bin(ctx_idx_offset + inc3)?;
            let bp1 = self.decode_bin(ctx_idx_offset + inc3)?;
            let idx = (bp0 * 2 + bp1) as u32;
            let mb_type = if b3 == 0 {
                // 1,1,0,0,xx → types 3-6
                match idx {
                    0 => super::macroblock::BMbType::B_Bi_16x16,
                    1 => super::macroblock::BMbType::B_L0_L0_16x8,
                    2 => super::macroblock::BMbType::B_L0_L0_8x16,
                    3 => super::macroblock::BMbType::B_L1_L1_16x8,
                    _ => unreachable!(),
                }
            } else {
                // 1,1,0,1,xx → types 7-10
                match idx {
                    0 => super::macroblock::BMbType::B_L1_L1_8x16,
                    1 => super::macroblock::BMbType::B_L0_L1_16x8,
                    2 => super::macroblock::BMbType::B_L0_L1_8x16,
                    3 => super::macroblock::BMbType::B_L1_L0_16x8,
                    _ => unreachable!(),
                }
            };
            trace!("parse_mb_type_b type={:?}", mb_type);
            return Ok(CabacMbType::B(mb_type));
        }

        // b0=1, b1=1, b2=1
        let b3 = self.decode_bin(ctx_idx_offset + inc3)?;
        if b3 == 0 {
            // 1,1,1,0,xxx → 3 context-coded bins → types 12-19
            // All use ctxIdxInc=5 (same as bin 3), matching reference decoder ctx[6]
            let bp0 = self.decode_bin(ctx_idx_offset + inc3)?;
            let bp1 = self.decode_bin(ctx_idx_offset + inc3)?;
            let bp2 = self.decode_bin(ctx_idx_offset + inc3)?;
            let idx = (bp0 * 4 + bp1 * 2 + bp2) as u32;
            let mb_type = match idx {
                0 => super::macroblock::BMbType::B_L0_Bi_16x8,
                1 => super::macroblock::BMbType::B_L0_Bi_8x16,
                2 => super::macroblock::BMbType::B_L1_Bi_16x8,
                3 => super::macroblock::BMbType::B_L1_Bi_8x16,
                4 => super::macroblock::BMbType::B_Bi_L0_16x8,
                5 => super::macroblock::BMbType::B_Bi_L0_8x16,
                6 => super::macroblock::BMbType::B_Bi_L1_16x8,
                7 => super::macroblock::BMbType::B_Bi_L1_8x16,
                _ => unreachable!(),
            };
            trace!("parse_mb_type_b type={:?}", mb_type);
            return Ok(CabacMbType::B(mb_type));
        }

        // b0=1, b1=1, b2=1, b3=1: context-coded subtree for types 11, 20-22, Intra
        // All bins use ctxIdxInc=5 (same context as bin 3), matching reference decoder ctx[6]
        let bp0 = self.decode_bin(ctx_idx_offset + inc3)?;
        if bp0 == 0 {
            let bp1 = self.decode_bin(ctx_idx_offset + inc3)?;
            if bp1 == 0 {
                // 1,1,1,1,0,0,x → types 20-21
                let bp2 = self.decode_bin(ctx_idx_offset + inc3)?;
                let mb_type = if bp2 == 0 {
                    super::macroblock::BMbType::B_Bi_Bi_16x8
                } else {
                    super::macroblock::BMbType::B_Bi_Bi_8x16
                };
                trace!("parse_mb_type_b type={:?}", mb_type);
                Ok(CabacMbType::B(mb_type))
            } else {
                // 1,1,1,1,0,1 → Intra MB in B slice (types 23-48)
                let i_mb_type =
                    self.parse_mb_type_i_suffix(ctx_idx_offset_suffix, slice, mb_addr)?;
                trace!("parse_mb_type_b type=I({:?})", i_mb_type);
                Ok(CabacMbType::I(i_mb_type))
            }
        } else {
            let bp1 = self.decode_bin(ctx_idx_offset + inc3)?;
            let mb_type = if bp1 == 0 {
                // 1,1,1,1,1,0 → B_L1_L0_8x16 (type 11)
                super::macroblock::BMbType::B_L1_L0_8x16
            } else {
                // 1,1,1,1,1,1 → B_8x8 (type 22)
                super::macroblock::BMbType::B_8x8
            };
            trace!("parse_mb_type_b type={:?}", mb_type);
            Ok(CabacMbType::B(mb_type))
        }
    }

    // Helper for P-slice Intra suffix
    fn parse_mb_type_i_suffix(
        &mut self,
        ctx_idx_offset: usize,
        _slice: &Slice,
        _mb_addr: MbAddr,
    ) -> ParseResult<super::macroblock::IMbType> {
        // Bin 0 (of suffix): I_NxN check. Table 9-39 Row 17 Col 3 => ctxIdxInc = 0
        let inc0 =
            Self::get_ctx_idx_inc(SyntaxElement::MbTypeISuffix, 0, CtxIncParams::Standard(0));
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

    pub fn parse_sub_mb_type_p(
        &mut self,
        _slice: &Slice,
        _mb_addr: MbAddr,
    ) -> ParseResult<super::macroblock::SubMbType> {
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

    // Table 9-38: Binarization for sub-macroblock types in B slices
    pub fn parse_sub_mb_type_b(
        &mut self,
        _slice: &Slice,
        _mb_addr: MbAddr,
    ) -> ParseResult<super::macroblock::BSubMbType> {
        let props = get_syntax_element_properties(SyntaxElement::SubMbTypeB);
        let ctx_idx_offset = props.ctx_idx_offset as usize;

        // Bin 0 (ctxIdxInc=0)
        let b0 = self.decode_bin(ctx_idx_offset)?;
        if b0 == 0 {
            trace!("parse_sub_mb_type_b type=B_Direct_8x8");
            return Ok(super::macroblock::BSubMbType::B_Direct_8x8);
        }

        // Bin 1 (ctxIdxInc=1)
        let b1 = self.decode_bin(ctx_idx_offset + 1)?;
        if b1 == 0 {
            // Bin 2 (Table 9-41: b1=0 → ctxIdxInc=3)
            let b2 = self.decode_bin(ctx_idx_offset + 3)?;
            if b2 == 0 {
                trace!("parse_sub_mb_type_b type=B_L0_8x8");
                return Ok(super::macroblock::BSubMbType::B_L0_8x8);
            } else {
                trace!("parse_sub_mb_type_b type=B_L1_8x8");
                return Ok(super::macroblock::BSubMbType::B_L1_8x8);
            }
        }

        // b0=1, b1=1
        // Bin 2 (Table 9-41: b1=1 → ctxIdxInc=2)
        let b2 = self.decode_bin(ctx_idx_offset + 2)?;
        if b2 == 0 {
            // Bin 3 (ctxIdxInc=3)
            let b3 = self.decode_bin(ctx_idx_offset + 3)?;
            // Bin 4 (ctxIdxInc=3, same as bin 3 — reference uses ctx[3] for all bins 3+)
            let bp0 = self.decode_bin(ctx_idx_offset + 3)?;
            let idx = b3 * 2 + bp0;
            let sub_type = match idx {
                0 => super::macroblock::BSubMbType::B_Bi_8x8, // 11000
                1 => super::macroblock::BSubMbType::B_L0_8x4, // 11001
                2 => super::macroblock::BSubMbType::B_L0_4x8, // 11010
                3 => super::macroblock::BSubMbType::B_L1_8x4, // 11011
                _ => unreachable!(),
            };
            trace!("parse_sub_mb_type_b type={:?}", sub_type);
            return Ok(sub_type);
        }

        // b0=1, b1=1, b2=1
        // Bin 3 (ctxIdxInc=3)
        let b3 = self.decode_bin(ctx_idx_offset + 3)?;
        if b3 == 0 {
            // Bins 4,5 (ctxIdxInc=3, same as bin 3 — reference uses ctx[3] for all bins 3+)
            let bp0 = self.decode_bin(ctx_idx_offset + 3)?;
            let bp1 = self.decode_bin(ctx_idx_offset + 3)?;
            let idx = bp0 * 2 + bp1;
            let sub_type = match idx {
                0 => super::macroblock::BSubMbType::B_L1_4x8, // 111000
                1 => super::macroblock::BSubMbType::B_Bi_8x4, // 111001
                2 => super::macroblock::BSubMbType::B_Bi_4x8, // 111010
                3 => super::macroblock::BSubMbType::B_L0_4x4, // 111011
                _ => unreachable!(),
            };
            trace!("parse_sub_mb_type_b type={:?}", sub_type);
            return Ok(sub_type);
        }

        // b0=1, b1=1, b2=1, b3=1
        // Bin 4 (ctxIdxInc=3, same as bin 3 — reference uses ctx[3] for all bins 3+)
        let bp0 = self.decode_bin(ctx_idx_offset + 3)?;
        if bp0 == 0 {
            trace!("parse_sub_mb_type_b type=B_L1_4x4");
            Ok(super::macroblock::BSubMbType::B_L1_4x4) // 11110
        } else {
            trace!("parse_sub_mb_type_b type=B_Bi_4x4");
            Ok(super::macroblock::BSubMbType::B_Bi_4x4) // 11111
        }
    }

    // 9.3.3.1.1.8 Derivation process of ctxIdxInc for the syntax element intra_chroma_pred_mode
    fn get_ctx_idx_inc_intra_chroma_pred_mode(slice: &Slice, mb_addr: MbAddr) -> usize {
        let get_cond_term_flag = |mb: &super::macroblock::Macroblock| match mb {
            super::macroblock::Macroblock::PCM(_) => false, // Spec: condTermFlagN = 0 for I_PCM
            super::macroblock::Macroblock::I(m) => m.intra_chroma_pred_mode as u32 != 0,
            super::macroblock::Macroblock::P(_) | super::macroblock::Macroblock::B(_) => false,
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

    pub fn parse_intra_chroma_pred_mode(
        &mut self,
        slice: &Slice,
        mb_addr: MbAddr,
    ) -> ParseResult<super::macroblock::Intra_Chroma_Pred_Mode> {
        let ctx_idx_inc = Self::get_ctx_idx_inc_intra_chroma_pred_mode(slice, mb_addr);

        // Table 9-34: maxBinIdxCtx = 1.
        let val = self.parse_truncated_unary_bin(
            SyntaxElement::IntraChromaPredMode,
            None,
            CtxIncParams::Standard(ctx_idx_inc),
        )?;
        let mode = super::macroblock::Intra_Chroma_Pred_Mode::try_from(val)?;
        trace!("parse_intra_chroma_pred_mode mode={:?}", mode);
        Ok(mode)
    }

    // Helpers for B sub-macroblock partition index derivation
    fn get_sub_mb_blk_idx_b(sub_mb_type: super::macroblock::BSubMbType, j: usize) -> usize {
        use super::macroblock::BSubMbType::*;
        match (sub_mb_type, j) {
            (B_Direct_8x8 | B_L0_8x8 | B_L1_8x8 | B_Bi_8x8, 0) => 0,
            (B_L0_8x4 | B_L1_8x4 | B_Bi_8x4, 0) => 0,
            (B_L0_8x4 | B_L1_8x4 | B_Bi_8x4, 1) => 2,
            (B_L0_4x8 | B_L1_4x8 | B_Bi_4x8, 0) => 0,
            (B_L0_4x8 | B_L1_4x8 | B_Bi_4x8, 1) => 1,
            (B_L0_4x4 | B_L1_4x4 | B_Bi_4x4, x) => x,
            _ => 0,
        }
    }

    fn get_sub_mb_grid_size_b(sub_mb_type: super::macroblock::BSubMbType) -> (usize, usize) {
        use super::macroblock::BSubMbType::*;
        match sub_mb_type {
            B_Direct_8x8 | B_L0_8x8 | B_L1_8x8 | B_Bi_8x8 => (2, 2),
            B_L0_8x4 | B_L1_8x4 | B_Bi_8x4 => (2, 1),
            B_L0_4x8 | B_L1_4x8 | B_Bi_4x8 => (1, 2),
            B_L0_4x4 | B_L1_4x4 | B_Bi_4x4 => (1, 1),
        }
    }

    fn get_mb_part_grid_size_b(b_type: super::macroblock::BMbType) -> (usize, usize) {
        let (w, h) = b_type.MbPartSize();
        ((w / 4) as usize, (h / 4) as usize)
    }

    fn get_mb_part_start_blk_idx_b(b_type: super::macroblock::BMbType, part_idx: usize) -> usize {
        let (_, part_h) = b_type.MbPartSize();
        if part_idx == 0 {
            0
        } else if part_h == 8 {
            // 16x8: second partition starts at blk_idx 8
            8
        } else {
            // 8x16: second partition starts at blk_idx 4
            4
        }
    }

    pub fn parse_macroblock(
        &mut self,
        slice: &mut Slice,
        pool: &mut super::residual::ResidualPool,
    ) -> ParseResult<super::macroblock::Macroblock> {
        let mb_addr = slice.get_next_mb_addr();
        trace!("parse_macroblock addr={}", mb_addr);

        if slice.MbaffFrameFlag() {
            return Err("MBAFF (mb_adaptive_frame_field) is not supported".into());
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
            let skipped = self.parse_mb_skip_flag(slice, mb_addr)?;
            if skipped {
                let motion = super::parser::calculate_motion_b(
                    slice,
                    mb_addr,
                    super::macroblock::BMbType::B_Skip,
                    &[super::macroblock::PartitionInfo::default(); 4],
                    &[super::macroblock::BSubMacroblock::default(); 4],
                );
                let mb = super::macroblock::BMb {
                    mb_type: super::macroblock::BMbType::B_Skip,
                    motion,
                    coded_block_pattern: CodedBlockPattern::new(0, 0),
                    mb_qp_delta: 0,
                    qp: slice.slice_qp_y() as u8,
                    cbf_info: CbfInfo::default(),
                    ..Default::default()
                };
                return Ok(super::macroblock::Macroblock::B(mb));
            }
        }

        let mb_type = if slice.header.slice_type == super::slice::SliceType::I {
            CabacMbType::I(self.parse_mb_type_i(slice, mb_addr)?)
        } else if slice.header.slice_type == super::slice::SliceType::SI {
            return Err("SI slice decoding is not supported".into());
        } else if slice.header.slice_type == super::slice::SliceType::B {
            self.parse_mb_type_b(slice, mb_addr)?
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
                    // PCM payload is read directly from the underlying reader;
                    // push back any CABAC pre-fetched bits first so reader
                    // position matches what CABAC has logically consumed.
                    self.sync_reader_position()?;
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

                let mut mb = super::macroblock::IMb { mb_type: i_type, ..Default::default() };

                // Intra prediction
                if i_type == super::macroblock::IMbType::I_NxN {
                    if slice.pps.transform_8x8_mode_flag {
                        let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                        let ctx_idx_inc = Self::get_ctx_idx_inc_transform_size_8x8_flag(&accessor);


                        let props =
                            get_syntax_element_properties(SyntaxElement::TransformSize8x8Flag);
                        let flag =
                            self.decode_bin((props.ctx_idx_offset as usize) + ctx_idx_inc)? == 1;
                        mb.transform_size_8x8_flag = flag;
                        curr_mb.transform_size_8x8_flag = flag;
                    }

                    // Table 9-34: prev_intra4x4_pred_mode_flag and
                    // prev_intra8x8_pred_mode_flag share ctxIdxOffset 68; the rem_*
                    // syntax elements share ctxIdxOffset 69. Binarization is FL with
                    // 1 bin (flag) / 3 bins (rem), identical between 4x4 and 8x8.
                    let prev_intra_props =
                        get_syntax_element_properties(SyntaxElement::PrevIntra4x4PredModeFlag);
                    let rem_intra_props =
                        get_syntax_element_properties(SyntaxElement::RemIntra4x4PredMode);

                    if mb.transform_size_8x8_flag {
                        for i in 0..4 {
                            let prev_intra_pred_mode_flag =
                                self.decode_bin(prev_intra_props.ctx_idx_offset as usize)? == 1;
                            let prev_mode =
                                super::parser::calc_prev_intra8x8_pred_mode(slice, &mb, mb_addr, i);

                            if prev_intra_pred_mode_flag {
                                mb.rem_intra8x8_pred_mode[i] = prev_mode;
                            } else {
                                let rem_intra_offset = rem_intra_props.ctx_idx_offset as usize;
                                let rem_intra_pred_mode = self.decode_bin(rem_intra_offset)? as u32
                                    | ((self.decode_bin(rem_intra_offset)? as u32) << 1)
                                    | ((self.decode_bin(rem_intra_offset)? as u32) << 2);

                                if rem_intra_pred_mode < (prev_mode as u32) {
                                    mb.rem_intra8x8_pred_mode[i] =
                                        super::macroblock::Intra_8x8_SamplePredMode::try_from(
                                            rem_intra_pred_mode,
                                        )?;
                                } else {
                                    mb.rem_intra8x8_pred_mode[i] =
                                        super::macroblock::Intra_8x8_SamplePredMode::try_from(
                                            rem_intra_pred_mode + 1,
                                        )?;
                                }
                            }
                        }
                    } else {
                        for i in 0..16 {
                            let prev_intra_pred_mode_flag =
                                self.decode_bin(prev_intra_props.ctx_idx_offset as usize)? == 1;
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

                let mut residual = pool.acquire();
                self.parse_residual_cabac(slice, mb_addr, &mut curr_mb, &mut residual)?;
                mb.residual = Some(residual);
                mb.cbf_info = curr_mb.cbf;

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
                    // Section 7.3.5.2: ref_idx_l0 is parsed if num_ref_idx_l0_active_minus1 > 0
                    // In non-MBAFF pictures, mb_field_decoding_flag == field_pic_flag, so the second term is false.
                    if num_ref_idx_l0_active_minus1 > 0 {
                        for i in 0..4 {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx = self.parse_ref_idx_cabac(
                                &accessor,
                                0,
                                num_ref_idx_l0_active_minus1,
                                i,
                            )?;


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
                            let p = super::macroblock::get_4x4luma_block_location(start_blk_idx);
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


                            let mvd_vec = MotionVector { x: mvd_x, y: mvd_y };
                            sub_mbs[i].partitions[j].mvd_l0 = mvd_vec;

                            // Update curr_mb.motion
                            let (w, h) = match sub_mbs[i].sub_mb_type {
                                SubMbType::P_L0_8x8 => (2, 2),
                                SubMbType::P_L0_8x4 => (2, 1),
                                SubMbType::P_L0_4x8 => (1, 2),
                                SubMbType::P_L0_4x4 => (1, 1),
                            };
                            let p = super::macroblock::get_4x4luma_block_location(blk_idx as u8);
                            let start_blk_y = (p.y / 4) as usize;
                            let start_blk_x = (p.x / 4) as usize;
                            for y in 0..h {
                                for x in 0..w {
                                    curr_mb.motion.partitions[start_blk_y + y][start_blk_x + x]
                                        .mvd_l0 = mvd_vec;
                                }
                            }
                        }
                    }
                } else {
                    let num_part = p_type.NumMbPart();
                    let num_ref_idx_l0_active_minus1 = slice.header.num_ref_idx_l0_active_minus1;

                    // Section 7.3.5.1: ref_idx_l0 is parsed if num_ref_idx_l0_active_minus1 > 0
                    // In non-MBAFF pictures, mb_field_decoding_flag == field_pic_flag, so the second term is false.
                    if num_ref_idx_l0_active_minus1 > 0 {
                        for i in 0..num_part {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx = self.parse_ref_idx_cabac(
                                &accessor,
                                0,
                                num_ref_idx_l0_active_minus1,
                                i,
                            )?;

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

                        let mvd_vec = MotionVector { x: mvd_x, y: mvd_y };
                        partitions[i].mvd_l0 = mvd_vec;

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
                                    .mvd_l0 = mvd_vec;
                            }
                        }
                    }
                }

                let cbp = self.parse_coded_block_pattern_cabac(slice, mb_addr, &mut curr_mb)?;
                let mut mb = super::macroblock::PMb {
                    mb_type: p_type,
                    motion: super::parser::calculate_motion(
                        slice,
                        mb_addr,
                        p_type,
                        &partitions,
                        &sub_mbs,
                    ),
                    coded_block_pattern: cbp,
                    mb_qp_delta: 0,
                    qp: slice.slice_qp_y() as u8,
                    transform_size_8x8_flag: false,
                    residual: None,
                    cbf_info: CbfInfo::default(),
                };

                // Spec 7.3.5 macroblock_layer: transform_size_8x8_flag is parsed for
                // non-I_NxN MBs when CodedBlockPatternLuma > 0, PPS allows it, and every
                // sub-MB partition is at least 8x8. For P: sub-partitions can go below
                // 8x8 only when mb_type is P_8x8/P_8x8ref0 and a sub_mb_type has
                // NumSubMbPart > 1.
                let no_sub_mb_part_size_less_than_8x8 = if p_type
                    == super::macroblock::PMbType::P_8x8
                    || p_type == super::macroblock::PMbType::P_8x8ref0
                {
                    sub_mbs.iter().all(|sm| sm.sub_mb_type.NumSubMbPart() == 1)
                } else {
                    true
                };
                if slice.pps.transform_8x8_mode_flag
                    && mb.coded_block_pattern.luma() > 0
                    && no_sub_mb_part_size_less_than_8x8
                {
                    let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                    let ctx_idx_inc = Self::get_ctx_idx_inc_transform_size_8x8_flag(&accessor);
                    let props =
                        get_syntax_element_properties(SyntaxElement::TransformSize8x8Flag);
                    let flag =
                        self.decode_bin((props.ctx_idx_offset as usize) + ctx_idx_inc)? == 1;
                    mb.transform_size_8x8_flag = flag;
                    curr_mb.transform_size_8x8_flag = flag;
                }

                if !mb.coded_block_pattern.is_zero()
                    && mb.mb_type != super::macroblock::PMbType::P_Skip
                {
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }

                let mut residual = pool.acquire();
                self.parse_residual_cabac(slice, mb_addr, &mut curr_mb, &mut residual)?;
                mb.residual = Some(residual);
                mb.cbf_info = curr_mb.cbf;

                Ok(super::macroblock::Macroblock::P(mb))
            }
            CabacMbType::B(b_type) => {
                let mut partitions = [PartitionInfo::default(); 4];
                let mut sub_mbs = [super::macroblock::BSubMacroblock::default(); 4];

                if b_type == super::macroblock::BMbType::B_Direct_16x16 {
                    // No ref_idx or mvd parsed — direct prediction
                } else if b_type == super::macroblock::BMbType::B_8x8 {
                    // Parse sub-macroblock types
                    for i in 0..4 {
                        sub_mbs[i].sub_mb_type = self.parse_sub_mb_type_b(slice, mb_addr)?;
                    }

                    // Set pred_mode in motion grid before ref_idx/mvd parsing
                    // (needed for predModeEqualFlagN within-MB neighbor checks)
                    for i in 0..4 {
                        let mode = sub_mbs[i].sub_mb_type.SubMbPredMode();
                        let start_blk_idx: u8 = match i {
                            0 => 0,
                            1 => 4,
                            2 => 8,
                            3 => 12,
                            _ => 0,
                        };
                        let p = super::macroblock::get_4x4luma_block_location(start_blk_idx);
                        let sy = (p.y / 4) as usize;
                        let sx = (p.x / 4) as usize;
                        for y in 0..2 {
                            for x in 0..2 {
                                curr_mb.motion.partitions[sy + y][sx + x].pred_mode = mode;
                            }
                        }
                    }

                    let num_ref_idx_l0 = slice.header.num_ref_idx_l0_active_minus1;
                    let num_ref_idx_l1 = slice.header.num_ref_idx_l1_active_minus1;

                    // ref_idx_l0 for sub-mbs where mode != Pred_L1 and != Direct
                    for i in 0..4 {
                        let mode = sub_mbs[i].sub_mb_type.SubMbPredMode();
                        if mode != super::macroblock::MbPredictionMode::Pred_L1
                            && mode != super::macroblock::MbPredictionMode::Direct
                            && num_ref_idx_l0 > 0
                        {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx =
                                self.parse_ref_idx_cabac(&accessor, 0, num_ref_idx_l0, i)?;


                            for j in 0..4 {
                                sub_mbs[i].partitions[j].ref_idx_l0 = ref_idx;
                            }
                            let start_blk_idx: u8 = match i {
                                0 => 0,
                                1 => 4,
                                2 => 8,
                                3 => 12,
                                _ => 0,
                            };
                            let p = super::macroblock::get_4x4luma_block_location(start_blk_idx);
                            let sy = (p.y / 4) as usize;
                            let sx = (p.x / 4) as usize;
                            for y in 0..2 {
                                for x in 0..2 {
                                    curr_mb.motion.partitions[sy + y][sx + x].ref_idx_l0 = ref_idx;
                                }
                            }
                        }
                    }

                    // ref_idx_l1
                    for i in 0..4 {
                        let mode = sub_mbs[i].sub_mb_type.SubMbPredMode();
                        if mode != super::macroblock::MbPredictionMode::Pred_L0
                            && mode != super::macroblock::MbPredictionMode::Direct
                            && num_ref_idx_l1 > 0
                        {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx =
                                self.parse_ref_idx_cabac(&accessor, 1, num_ref_idx_l1, i)?;


                            for j in 0..4 {
                                sub_mbs[i].partitions[j].ref_idx_l1 = ref_idx;
                            }
                            let start_blk_idx: u8 = match i {
                                0 => 0,
                                1 => 4,
                                2 => 8,
                                3 => 12,
                                _ => 0,
                            };
                            let p = super::macroblock::get_4x4luma_block_location(start_blk_idx);
                            let sy = (p.y / 4) as usize;
                            let sx = (p.x / 4) as usize;
                            for y in 0..2 {
                                for x in 0..2 {
                                    curr_mb.motion.partitions[sy + y][sx + x].ref_idx_l1 = ref_idx;
                                }
                            }
                        }
                    }

                    // mvd_l0
                    for i in 0..4 {
                        let mode = sub_mbs[i].sub_mb_type.SubMbPredMode();
                        if mode != super::macroblock::MbPredictionMode::Pred_L1
                            && mode != super::macroblock::MbPredictionMode::Direct
                        {
                            let num_sub_part = sub_mbs[i].sub_mb_type.NumSubMbPart();
                            for j in 0..num_sub_part {
                                let p_idx = Self::get_sub_mb_blk_idx_b(sub_mbs[i].sub_mb_type, j);
                                let base_blk_idx: usize = match i {
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


                                let mvd_vec = MotionVector { x: mvd_x, y: mvd_y };
                                sub_mbs[i].partitions[j].mvd_l0 = mvd_vec;

                                let (w, h) = Self::get_sub_mb_grid_size_b(sub_mbs[i].sub_mb_type);
                                let p =
                                    super::macroblock::get_4x4luma_block_location(blk_idx as u8);
                                let sy = (p.y / 4) as usize;
                                let sx = (p.x / 4) as usize;
                                for y in 0..h {
                                    for x in 0..w {
                                        curr_mb.motion.partitions[sy + y][sx + x].mvd_l0 = mvd_vec;
                                    }
                                }
                            }
                        }
                    }

                    // mvd_l1
                    for i in 0..4 {
                        let mode = sub_mbs[i].sub_mb_type.SubMbPredMode();
                        if mode != super::macroblock::MbPredictionMode::Pred_L0
                            && mode != super::macroblock::MbPredictionMode::Direct
                        {
                            let num_sub_part = sub_mbs[i].sub_mb_type.NumSubMbPart();
                            for j in 0..num_sub_part {
                                let p_idx = Self::get_sub_mb_blk_idx_b(sub_mbs[i].sub_mb_type, j);
                                let base_blk_idx: usize = match i {
                                    0 => 0,
                                    1 => 4,
                                    2 => 8,
                                    3 => 12,
                                    _ => 0,
                                };
                                let blk_idx = base_blk_idx + p_idx;

                                let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                                let mvd_x = self.parse_mvd_cabac(&accessor, 1, 0, blk_idx)?;
                                let mvd_y = self.parse_mvd_cabac(&accessor, 1, 1, blk_idx)?;


                                let mvd_vec = MotionVector { x: mvd_x, y: mvd_y };
                                sub_mbs[i].partitions[j].mvd_l1 = mvd_vec;

                                let (w, h) = Self::get_sub_mb_grid_size_b(sub_mbs[i].sub_mb_type);
                                let p =
                                    super::macroblock::get_4x4luma_block_location(blk_idx as u8);
                                let sy = (p.y / 4) as usize;
                                let sx = (p.x / 4) as usize;
                                for y in 0..h {
                                    for x in 0..w {
                                        curr_mb.motion.partitions[sy + y][sx + x].mvd_l1 = mvd_vec;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Non-8x8, non-direct: 16x16, 16x8, 8x16
                    let num_part = b_type.NumMbPart();
                    let num_ref_idx_l0 = slice.header.num_ref_idx_l0_active_minus1;
                    let num_ref_idx_l1 = slice.header.num_ref_idx_l1_active_minus1;
                    // Set pred_mode in motion grid before ref_idx/mvd parsing
                    // (needed for predModeEqualFlagN within-MB neighbor checks)
                    for i in 0..num_part {
                        let mode = b_type.MbPartPredMode(i);
                        let (w, h) = Self::get_mb_part_grid_size_b(b_type);
                        let start_blk_idx = Self::get_mb_part_start_blk_idx_b(b_type, i);
                        let p = super::macroblock::get_4x4luma_block_location(start_blk_idx as u8);
                        let sy = (p.y / 4) as usize;
                        let sx = (p.x / 4) as usize;
                        for y in 0..h {
                            for x in 0..w {
                                curr_mb.motion.partitions[sy + y][sx + x].pred_mode = mode;
                            }
                        }
                    }

                    // ref_idx_l0
                    for i in 0..num_part {
                        let mode = b_type.MbPartPredMode(i);
                        if mode != super::macroblock::MbPredictionMode::Pred_L1
                            && num_ref_idx_l0 > 0
                        {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx =
                                self.parse_ref_idx_cabac(&accessor, 0, num_ref_idx_l0, i)?;

                            partitions[i].ref_idx_l0 = ref_idx;

                            let (w, h) = Self::get_mb_part_grid_size_b(b_type);
                            let start_blk_idx = Self::get_mb_part_start_blk_idx_b(b_type, i);
                            let p =
                                super::macroblock::get_4x4luma_block_location(start_blk_idx as u8);
                            let sy = (p.y / 4) as usize;
                            let sx = (p.x / 4) as usize;
                            for y in 0..h {
                                for x in 0..w {
                                    curr_mb.motion.partitions[sy + y][sx + x].ref_idx_l0 = ref_idx;
                                }
                            }
                        }
                    }

                    // ref_idx_l1
                    for i in 0..num_part {
                        let mode = b_type.MbPartPredMode(i);
                        if mode != super::macroblock::MbPredictionMode::Pred_L0
                            && num_ref_idx_l1 > 0
                        {
                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let ref_idx =
                                self.parse_ref_idx_cabac(&accessor, 1, num_ref_idx_l1, i)?;

                            partitions[i].ref_idx_l1 = ref_idx;

                            let (w, h) = Self::get_mb_part_grid_size_b(b_type);
                            let start_blk_idx = Self::get_mb_part_start_blk_idx_b(b_type, i);
                            let p =
                                super::macroblock::get_4x4luma_block_location(start_blk_idx as u8);
                            let sy = (p.y / 4) as usize;
                            let sx = (p.x / 4) as usize;
                            for y in 0..h {
                                for x in 0..w {
                                    curr_mb.motion.partitions[sy + y][sx + x].ref_idx_l1 = ref_idx;
                                }
                            }
                        }
                    }

                    // mvd_l0
                    for i in 0..num_part {
                        let mode = b_type.MbPartPredMode(i);
                        if mode != super::macroblock::MbPredictionMode::Pred_L1 {
                            let blk_idx = Self::get_mb_part_start_blk_idx_b(b_type, i);

                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let mvd_x = self.parse_mvd_cabac(&accessor, 0, 0, blk_idx)?;
                            let mvd_y = self.parse_mvd_cabac(&accessor, 0, 1, blk_idx)?;

                            let mvd_vec = MotionVector { x: mvd_x, y: mvd_y };
                            partitions[i].mvd_l0 = mvd_vec;

                            let (w, h) = Self::get_mb_part_grid_size_b(b_type);
                            let p = super::macroblock::get_4x4luma_block_location(blk_idx as u8);
                            let sy = (p.y / 4) as usize;
                            let sx = (p.x / 4) as usize;
                            for y in 0..h {
                                for x in 0..w {
                                    curr_mb.motion.partitions[sy + y][sx + x].mvd_l0 = mvd_vec;
                                }
                            }
                        }
                    }

                    // mvd_l1
                    for i in 0..num_part {
                        let mode = b_type.MbPartPredMode(i);
                        if mode != super::macroblock::MbPredictionMode::Pred_L0 {
                            let blk_idx = Self::get_mb_part_start_blk_idx_b(b_type, i);

                            let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                            let mvd_x = self.parse_mvd_cabac(&accessor, 1, 0, blk_idx)?;
                            let mvd_y = self.parse_mvd_cabac(&accessor, 1, 1, blk_idx)?;

                            let mvd_vec = MotionVector { x: mvd_x, y: mvd_y };
                            partitions[i].mvd_l1 = mvd_vec;

                            let (w, h) = Self::get_mb_part_grid_size_b(b_type);
                            let p = super::macroblock::get_4x4luma_block_location(blk_idx as u8);
                            let sy = (p.y / 4) as usize;
                            let sx = (p.x / 4) as usize;
                            for y in 0..h {
                                for x in 0..w {
                                    curr_mb.motion.partitions[sy + y][sx + x].mvd_l1 = mvd_vec;
                                }
                            }
                        }
                    }
                }

                let cbp = self.parse_coded_block_pattern_cabac(slice, mb_addr, &mut curr_mb)?;

                let mut mb = super::macroblock::BMb {
                    mb_type: b_type,
                    motion: super::parser::calculate_motion_b(
                        slice,
                        mb_addr,
                        b_type,
                        &partitions,
                        &sub_mbs,
                    ),
                    coded_block_pattern: cbp,
                    mb_qp_delta: 0,
                    qp: slice.slice_qp_y() as u8,
                    transform_size_8x8_flag: false,
                    residual: None,
                    cbf_info: CbfInfo::default(),
                    sub_mb_types: [
                        sub_mbs[0].sub_mb_type,
                        sub_mbs[1].sub_mb_type,
                        sub_mbs[2].sub_mb_type,
                        sub_mbs[3].sub_mb_type,
                    ],
                };

                // Spec 7.3.5 macroblock_layer: for B MBs transform_size_8x8_flag is
                // parsed when CodedBlockPatternLuma > 0, PPS allows it, every sub-
                // partition is at least 8x8, and (for B_Direct_16x16) direct_8x8_
                // inference_flag is set.
                let no_sub_mb_part_size_less_than_8x8 = if b_type
                    == super::macroblock::BMbType::B_8x8
                {
                    sub_mbs.iter().all(|sm| {
                        if sm.sub_mb_type == super::macroblock::BSubMbType::B_Direct_8x8 {
                            slice.sps.direct_8x8_inference_flag
                        } else {
                            sm.sub_mb_type.NumSubMbPart() == 1
                        }
                    })
                } else {
                    true
                };
                if slice.pps.transform_8x8_mode_flag
                    && mb.coded_block_pattern.luma() > 0
                    && no_sub_mb_part_size_less_than_8x8
                    && (b_type != super::macroblock::BMbType::B_Direct_16x16
                        || slice.sps.direct_8x8_inference_flag)
                {
                    let accessor = NeighborAccessor::new(slice, mb_addr, &curr_mb);
                    let ctx_idx_inc = Self::get_ctx_idx_inc_transform_size_8x8_flag(&accessor);
                    let props =
                        get_syntax_element_properties(SyntaxElement::TransformSize8x8Flag);
                    let flag =
                        self.decode_bin((props.ctx_idx_offset as usize) + ctx_idx_inc)? == 1;
                    mb.transform_size_8x8_flag = flag;
                    curr_mb.transform_size_8x8_flag = flag;
                }

                if !mb.coded_block_pattern.is_zero() {
                    mb.mb_qp_delta = self.parse_mb_qp_delta_cabac(slice, mb_addr)?;
                }

                let mut residual = pool.acquire();
                self.parse_residual_cabac(slice, mb_addr, &mut curr_mb, &mut residual)?;
                mb.residual = Some(residual);
                mb.cbf_info = curr_mb.cbf;

                Ok(super::macroblock::Macroblock::B(mb))
            }
        }
    }

    // 9.3.3.2.2 Renormalization process
    // Instead of looping one bit at a time, compute how many shifts are needed
    // and read that many bits in a single call. `self.range` is always non-zero
    // and < 512 at this point, so `leading_zeros()` is in [23, 31] and the
    // resulting shift is in [0, 8].
    #[inline(always)]
    fn renorm(&mut self) -> ParseResult<()> {
        if self.range < 256 {
            let shift = self.range.leading_zeros() - 23;
            self.range <<= shift;
            let bits = self.read_bits(shift)?;
            self.offset = (self.offset << shift) | bits;
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
    B(super::macroblock::BMbType),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h264::pps::PicParameterSet;
    use crate::h264::slice::{Slice, SliceHeader};
    use crate::h264::sps::SequenceParameterSet;

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

    /// Verify CabacContext's bit buffer returns the same bits in the same
    /// order as a plain `reader.u(1)` stream.
    #[test]
    fn test_bit_buffer_matches_reader() {
        // Mix of byte boundaries and varied bit patterns.
        let data = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xFE, 0xDC];

        // Reference: read 1 bit at a time from the underlying reader.
        let mut ref_reader = BitReader::new(&data);
        let mut ref_bits: Vec<u32> = Vec::new();
        for _ in 0..(data.len() * 8) {
            ref_bits.push(ref_reader.u(1).unwrap());
        }

        // Under test: read through CabacContext's bit buffer, mixing read
        // sizes (9 then 1s then 7 then 3, covering boundary cases).
        let mut reader = BitReader::new(&data);
        let slice = make_dummy_slice();
        let mut ctx = CabacContext {
            reader: &mut reader,
            range: 510,
            offset: 0,
            bit_buf: 0,
            n_bits: 0,
            reader_remaining: 0,
            ctx_table: [0; 1024],
        };
        ctx.init_context_variables(&slice);
        ctx.reader.align();
        ctx.reader_remaining = ctx.reader.remaining();

        let mut got_bits: Vec<u32> = Vec::new();
        let sizes = [9u32, 1, 1, 1, 7, 3, 1, 8];
        for &n in &sizes {
            let v = ctx.read_bits(n).expect("read_bits failed on opening reads");
            for i in (0..n).rev() {
                got_bits.push((v >> i) & 1);
            }
        }
        while got_bits.len() < 70 {
            let before_n = ctx.n_bits;
            let v = ctx.read_bits(1).unwrap_or_else(|e| {
                panic!(
                    "read_bits(1) failed at got_bits.len={}, n_bits_before={}: {}",
                    got_bits.len(),
                    before_n,
                    e,
                );
            });
            got_bits.push(v);
        }

        assert_eq!(
            got_bits,
            ref_bits[..got_bits.len()],
            "bit buffer decoded a different bit sequence than reader.u(1)*N"
        );
    }

    /// `decode_bypass_bits(n)` must produce the same bin sequence as calling
    /// `decode_bypass()` in a loop `n` times — both the returned bits AND
    /// the final `offset` state must match.
    #[test]
    fn test_decode_bypass_bits_matches_loop() {
        let data = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89];

        // Helper to spin up a CabacContext with a primed decoding engine.
        fn make_ctx<'a, 'b>(
            reader: &'a mut BitReader<'b>,
            slice: &Slice,
        ) -> CabacContext<'a, 'b> {
            CabacContext::new(reader, slice).expect("init")
        }

        let slice = make_dummy_slice();

        // Reference: call decode_bypass() n times, packing MSB-first.
        let mut ref_reader = BitReader::new(&data);
        let mut ref_ctx = make_ctx(&mut ref_reader, &slice);
        let n: u32 = 7;
        let mut ref_packed = 0u32;
        for _ in 0..n {
            ref_packed = (ref_packed << 1) | u32::from(ref_ctx.decode_bypass().unwrap());
        }

        // Under test.
        let mut test_reader = BitReader::new(&data);
        let mut test_ctx = make_ctx(&mut test_reader, &slice);
        let got_packed = test_ctx.decode_bypass_bits(n).unwrap();

        assert_eq!(got_packed, ref_packed, "bins differ");
        assert_eq!(
            test_ctx.offset, ref_ctx.offset,
            "offset state diverged between batched and per-bit bypass"
        );
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
    RefIdx(usize),     // list_idx
    MbQpDelta,
    IntraChromaPredMode,
    PrevIntra4x4PredModeFlag,
    RemIntra4x4PredMode,
    MbFieldDecodingFlag,
    CodedBlockPattern,
    CodedBlockFlag(usize),           // ctxBlockCat
    SignificantCoeffFlag(usize),     // ctxBlockCat
    LastSignificantCoeffFlag(usize), // ctxBlockCat
    CoeffAbsLevelMinus1(usize),      // ctxBlockCat
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
    Custom,       // For others
}

pub struct CabacTableEntry {
    pub binarization: BinarizationType,
    pub max_bin_idx_ctx: u32,
    pub ctx_idx_offset: u32,
    // For prefix/suffix types, we might need secondary values
    pub max_bin_idx_ctx_suffix: Option<u32>,
    pub ctx_idx_offset_suffix: Option<u32>,
}

#[inline]
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
        }
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
        SyntaxElement::PrevIntra4x4PredModeFlag | SyntaxElement::MbFieldDecodingFlag => {
            CabacTableEntry {
                binarization: BinarizationType::FL { c_max: 1 },
                max_bin_idx_ctx: 0,
                ctx_idx_offset: if matches!(se, SyntaxElement::MbFieldDecodingFlag) {
                    70
                } else {
                    68
                },
                max_bin_idx_ctx_suffix: None,
                ctx_idx_offset_suffix: None,
            }
        }
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
        SyntaxElement::CodedBlockFlag(cat) => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: *super::tables::CBF_OFFSETS.get(cat).unwrap_or(&85),
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::SignificantCoeffFlag(cat) => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: *super::tables::SIG_COEFF_OFFSETS.get(cat).unwrap_or(&105),
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::LastSignificantCoeffFlag(cat) => CabacTableEntry {
            binarization: BinarizationType::FL { c_max: 1 },
            max_bin_idx_ctx: 0,
            ctx_idx_offset: *super::tables::LAST_SIG_COEFF_OFFSETS.get(cat).unwrap_or(&166),
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
        },
        SyntaxElement::CoeffAbsLevelMinus1(cat) => CabacTableEntry {
            binarization: BinarizationType::UEGk { k: 0, signed_val_flag: false, u_coff: 14 },
            max_bin_idx_ctx: 1,
            ctx_idx_offset: *super::tables::COEFF_ABS_OFFSETS.get(cat).unwrap_or(&227),
            max_bin_idx_ctx_suffix: None,
            ctx_idx_offset_suffix: None,
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
