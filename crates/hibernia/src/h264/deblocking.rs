use super::decoder::{get_chroma_qp, SliceDeblockParams, VideoFrame};
use super::macroblock::Macroblock;
use super::pps::PicParameterSet;
use super::residual::scan_4x4;
use super::slice::DeblockingFilterIdc;
use super::tables::{MB_HEIGHT, MB_WIDTH};
use super::ColorPlane;

// Boundary Strength (bS) values
const BS_STRONG: u8 = 4;
const BS_INTRA: u8 = 3;
const BS_CODED: u8 = 2;
const BS_MOTION: u8 = 1;
const BS_NONE: u8 = 0;

// Padded to 76 entries: 12 leading (qp_av+offset in -12..-1, clamped to TABLE[0]) +
// 52 spec entries + 12 trailing (52..63, clamped to TABLE[51]). Caller adds
// QP_OFFSET_PAD to map [-12, 63] -> [0, 75]; parser enforces offset in [-12, 12]
// and QP in [0, 51] so the index is always in-bounds.
const QP_OFFSET_PAD: i32 = 12;

// Table 8-16
const ALPHA_TABLE: [u8; 76] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 17, 20,
    22, 25, 28, 32, 36, 40, 45, 50, 56, 63, 71, 80, 90, 101, 113, 127, 144, 162, 182, 203, 226,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

const BETA_TABLE: [u8; 76] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 6, 6, 7, 7, 8, 8,
    9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18,
];

// Table 8-17
const TC0_TABLE: [[u8; 76]; 3] = [
    // bS = 1 (BS_MOTION)
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 7, 8, 9, 10, 11, 13,
        13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    ],
    // bS = 2 (BS_CODED)
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 7, 8, 8, 10, 11, 12, 13, 15, 17,
        17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
    ],
    // bS = 3 (BS_INTRA)
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
        2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 7, 8, 9, 10, 11, 13, 14, 16, 18, 20, 23, 25,
        25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
    ],
];

/// Section 8.7.2.2 -- filter thresholds derived from QP values.
struct FilterThresholds {
    alpha: i32,
    beta: i32,
    index_a: usize,
}

impl FilterThresholds {
    /// Equations 8-453..8-455, Table 8-16: derive alpha, beta, and indexA from the
    /// average QP of the p and q blocks and the slice-level offsets.
    fn from_qp(p_qp: u8, q_qp: u8, alpha_offset: i32, beta_offset: i32) -> Self {
        let qp_av = (p_qp as i32 + q_qp as i32 + 1) >> 1;
        let index_a = (qp_av + alpha_offset + QP_OFFSET_PAD) as usize;
        let index_b = (qp_av + beta_offset + QP_OFFSET_PAD) as usize;
        FilterThresholds {
            alpha: ALPHA_TABLE[index_a] as i32,
            beta: BETA_TABLE[index_b] as i32,
            index_a,
        }
    }
}

/// Picture-wide view consumed by `filter_picture`, besides the
/// `DeblockRecords` gathered while the picture's slices were decoded: the
/// per-slice deblocking parameters and the picture's size.
pub struct PictureDeblockInput<'a> {
    pub slice_deblock: &'a [SliceDeblockParams],
    pub pic_width_in_mbs: usize,
    pub pic_height_in_mbs: usize,
}

// `MbDeblockInfo::flags` bits.
/// The macroblock is coded in an Intra prediction mode (I, including `I_PCM`).
const INTRA: u8 = 1 << 0;
/// `transform_size_8x8_flag` is set for the macroblock.
const TRANSFORM_8X8: u8 = 1 << 1;
/// Every internal edge of the macroblock has bS = 0 (`has_no_internal_edges`).
const NO_INTERNAL_EDGES: u8 = 1 << 2;
/// The macroblock's slice has an empty `RefPicList1`, as P slices do.
const L1_EMPTY: u8 = 1 << 3;
/// No slice decoded the macroblock.
const NOT_DECODED: u8 = 1 << 4;

/// What the deblocking filter reads about one macroblock (Section 8.7).
///
/// Built once per picture from the decoded macroblocks, so that deriving the
/// boundary strengths of an edge (Section 8.7.2.1) reads two compact records
/// rather than the `Macroblock` enum and its boxed residual.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct MbDeblockInfo {
    /// `QP_Y` of the macroblock (`get_qp`), the qPp / qPq of Section 8.7.2.2.
    qp: u8,
    /// `QP_C` of the Cb and Cr planes that corresponds to `qp` (Section 8.5.8).
    qp_c: [u8; 2],
    /// `INTRA`, `TRANSFORM_8X8`, `NO_INTERNAL_EDGES`, `L1_EMPTY`, `NOT_DECODED`.
    flags: u8,
    /// Index of the slice that decoded the macroblock.
    slice_id: u16,
    /// What the boundary strengths depend on, per 4x4 block. Intra
    /// macroblocks leave it zero: their edges get bS 3 or 4 regardless.
    blocks: BlockInfo,
}

/// The per-4x4-block inputs of Section 8.7.2.1 for an inter macroblock, in
/// raster order: 4x4 block (row, col) is index `4 * row + col`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct BlockInfo {
    /// Bit `4 * row + col` is set when the luma transform block containing
    /// 4x4 block (row, col) has non-zero coefficients (`nonzero_blocks`).
    nz: u16,
    /// Picture id (`RefPictureIds`) of each block's list 0 and list 1
    /// reference; 0 when the reference index does not select a picture.
    ref_l0: [u8; 16],
    ref_l1: [u8; 16],
    /// Each block's list 0 and list 1 motion vector as `[x, y]`, as stored in
    /// the macroblock's partitions, also for an unused list.
    mv_l0: [[i16; 2]; 16],
    mv_l1: [[i16; 2]; 16],
}

impl MbDeblockInfo {
    /// Makes this the record of a decoded macroblock of slice `slice_id`,
    /// whose reference picture lists resolve through `ref_ids`.
    /// `chroma_qp_offsets` are the PPS chroma QP index offsets of Cb and Cr.
    /// Writes every field in place: the records live in a picture-sized
    /// array.
    fn fill(
        &mut self,
        mb: &Macroblock,
        slice_id: u16,
        ref_ids: &SliceRefIds,
        chroma_qp_offsets: [i32; 2],
    ) {
        let qp = get_qp(mb);
        self.qp = qp;
        // Section 8.7.2.2: for chroma edges qPp / qPq are the QP_C values of
        // the luma QPs, from Table 8-15.
        self.qp_c = chroma_qp_offsets.map(|offset| get_chroma_qp(i32::from(qp), offset, 0));
        self.slice_id = slice_id;
        let mut flags = if ref_ids.l1_empty { L1_EMPTY } else { 0 };
        let (motion, transform_8x8) = match mb {
            Macroblock::I(m) => {
                flags |= INTRA;
                if m.transform_size_8x8_flag {
                    flags |= TRANSFORM_8X8;
                }
                self.flags = flags;
                self.blocks = BlockInfo::default();
                return;
            }
            Macroblock::PCM(_) => {
                self.flags = flags | INTRA;
                self.blocks = BlockInfo::default();
                return;
            }
            Macroblock::P(m) => (&m.motion, m.transform_size_8x8_flag),
            Macroblock::B(m) => (&m.motion, m.transform_size_8x8_flag),
        };
        if transform_8x8 {
            flags |= TRANSFORM_8X8;
        }
        if has_no_internal_edges(mb) {
            flags |= NO_INTERNAL_EDGES;
        }
        self.flags = flags;
        let blocks = &mut self.blocks;
        blocks.nz = nonzero_blocks(mb);
        for (blk, part) in motion.partitions.as_flattened().iter().enumerate() {
            blocks.ref_l0[blk] = ref_ids.l0[usize::from(part.ref_idx_l0)];
            blocks.ref_l1[blk] = ref_ids.l1[usize::from(part.ref_idx_l1)];
            blocks.mv_l0[blk] = [part.mv_l0.x, part.mv_l0.y];
            blocks.mv_l1[blk] = [part.mv_l1.x, part.mv_l1.y];
        }
    }

    fn is(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}

/// Section 8.7.2.1 compares the reference *pictures* two blocks use, and the
/// blocks may belong to slices with different reference picture lists. Each
/// slice's lists are resolved to POCs at slice-decode time; this numbers the
/// distinct POCs of all lists of the picture from 1, so that the comparison
/// is a byte compare that doesn't depend on either slice's lists. 0 stands
/// for "no picture". Two ids are equal exactly when the POCs they stand for
/// (or their absence) are.
#[derive(Default)]
struct RefPictureIds {
    /// Distinct POCs; the picture id of `pocs[i]` is `i + 1`.
    pocs: Vec<i32>,
}

impl RefPictureIds {
    fn id_of(&mut self, poc: i32) -> u8 {
        let index = self.pocs.iter().position(|&p| p == poc).unwrap_or_else(|| {
            self.pocs.push(poc);
            self.pocs.len() - 1
        });
        // The lists name pictures of the DPB, which holds fewer than 256
        // (max_num_ref_frames and max_dec_frame_buffering are 8-bit), so the
        // ids fit.
        debug_assert!(index < usize::from(u8::MAX), "more than 255 reference pictures");
        u8::try_from(index + 1).unwrap_or(u8::MAX)
    }
}

/// One slice's picture ids, indexed by reference index. Every `u8` index is
/// in range, so the lookup needs no branch: entries past the end of a list
/// are 0, which also covers `u8::MAX`, the marker of an unused list.
struct SliceRefIds {
    slice_id: u16,
    l0: [u8; 256],
    l1: [u8; 256],
    /// Entries of `l0` / `l1` that may be non-zero.
    l0_len: usize,
    l1_len: usize,
    /// The slice's list 1 is empty.
    l1_empty: bool,
}

impl Default for SliceRefIds {
    fn default() -> Self {
        SliceRefIds {
            slice_id: 0,
            l0: [0; 256],
            l1: [0; 256],
            l0_len: 0,
            l1_len: 0,
            l1_empty: true,
        }
    }
}

impl SliceRefIds {
    /// Switches the tables to slice `slice_id`, whose lists hold the
    /// pictures with POCs `l0_pocs` and `l1_pocs`.
    fn load(
        &mut self,
        ref_ids: &mut RefPictureIds,
        slice_id: u16,
        l0_pocs: &[i32],
        l1_pocs: &[i32],
    ) {
        let mut fill = |table: &mut [u8; 256], len: &mut usize, pocs: &[i32]| {
            table[..*len].fill(0);
            *len = pocs.len().min(table.len());
            for (id, &poc) in table.iter_mut().zip(pocs) {
                *id = ref_ids.id_of(poc);
            }
        };
        fill(&mut self.l0, &mut self.l0_len, l0_pocs);
        fill(&mut self.l1, &mut self.l1_len, l1_pocs);
        self.l1_empty = l1_pocs.is_empty();
        self.slice_id = slice_id;
    }
}

/// The `MbDeblockInfo` of every macroblock of the picture being decoded, in
/// raster order, gathered as its slices are decoded; and the storage to
/// gather them, reused from one picture to the next. The records of a 1080p
/// picture take 1.4 MB.
#[derive(Default)]
pub struct DeblockRecords {
    records: Vec<MbDeblockInfo>,
    ref_ids: RefPictureIds,
    slice: SliceRefIds,
    /// The PPS chroma QP index offsets of Cb and Cr.
    chroma_qp_offsets: [i32; 2],
}

impl DeblockRecords {
    /// Starts the records of a picture of `pic_size_in_mbs` macroblocks whose
    /// picture parameter set is `pps`. Macroblocks count as not decoded until
    /// they are recorded.
    pub fn start_picture(&mut self, pic_size_in_mbs: usize, pps: &PicParameterSet) {
        self.records.clear();
        self.records.resize(
            pic_size_in_mbs,
            MbDeblockInfo { flags: NOT_DECODED, ..MbDeblockInfo::default() },
        );
        self.ref_ids.pocs.clear();
        self.chroma_qp_offsets =
            [ColorPlane::Cb, ColorPlane::Cr].map(|plane| pps.get_chroma_qp_index_offset(plane));
    }

    /// Starts slice `slice_id` of the picture, whose reference picture
    /// lists hold the pictures with POCs `l0_pocs` and `l1_pocs`, as
    /// resolved at slice-decode time.
    pub fn start_slice(&mut self, slice_id: u16, l0_pocs: &[i32], l1_pocs: &[i32]) {
        self.slice.load(&mut self.ref_ids, slice_id, l0_pocs, l1_pocs);
    }

    /// Records decoded macroblock `mb` of the slice last started, whose
    /// address is `mb_addr`.
    pub fn record(&mut self, mb_addr: usize, mb: &Macroblock) {
        if let Some(record) = self.records.get_mut(mb_addr) {
            record.fill(mb, self.slice.slice_id, &self.slice, self.chroma_qp_offsets);
        }
    }
}

/// Section 8.7 -- picture-level deblocking pass. Replaces the per-slice
/// `filter_slice` so a multi-slice picture is filtered as one frame and
/// per-MB slice ownership is honoured for `disable_deblocking_filter_idc=2`
/// and for boundary-strength reference comparisons across slice boundaries.
pub fn filter_picture(
    input: &PictureDeblockInput,
    records: &DeblockRecords,
    frame: &mut VideoFrame,
) {
    // disable_deblocking_filter_idc = 1 in every slice leaves every edge of
    // the picture unfiltered.
    if input.slice_deblock.iter().all(|params| params.idc == DeblockingFilterIdc::Off) {
        return;
    }
    let records = records.records.as_slice();
    let width = input.pic_width_in_mbs;
    for mb_y in 0..input.pic_height_in_mbs {
        for mb_x in 0..width {
            let Some(q) = records.get(mb_y * width + mb_x) else { continue };
            let (left, top) = mb_neighbors(records, width, mb_x, mb_y);
            filter_macroblock(input, frame, mb_x, mb_y, q, left, top);
        }
    }
}

/// Section 6.4.9, with every macroblock of the picture available: the
/// macroblocks left of (A) and above (B) the macroblock at (`mb_x`, `mb_y`),
/// or `None` on the picture boundary.
fn mb_neighbors(
    records: &[MbDeblockInfo],
    width: usize,
    mb_x: usize,
    mb_y: usize,
) -> (Option<&MbDeblockInfo>, Option<&MbDeblockInfo>) {
    let mb_addr = mb_y * width + mb_x;
    let left = if mb_x > 0 { records.get(mb_addr - 1) } else { None };
    let top = if mb_y > 0 { records.get(mb_addr - width) } else { None };
    (left, top)
}

/// Section 8.7, steps 1-3 -- Filter all edges of the macroblock at
/// (`mb_x`, `mb_y`). BS values are precomputed once per MB and reused across
/// luma and chroma to avoid redundant derivation (Section 8.7.2.1). `left`
/// and `top` are the neighbours across the macroblock's left and top edges,
/// `None` on the picture boundary.
fn filter_macroblock(
    input: &PictureDeblockInput,
    frame: &mut VideoFrame,
    mb_x: usize,
    mb_y: usize,
    q: &MbDeblockInfo,
    left: Option<&MbDeblockInfo>,
    top: Option<&MbDeblockInfo>,
) {
    if q.is(NOT_DECODED) {
        return;
    }

    // Per Section 8.7, edge filtering parameters come from the slice that
    // contains the q-block (the macroblock on the lower/right side of the
    // edge), which is the current MB for both its left and top edges.
    let q_params = &input.slice_deblock[usize::from(q.slice_id)];
    if q_params.idc == DeblockingFilterIdc::Off {
        return;
    }
    let alpha_offset = q_params.alpha_c0_offset_div2 * 2;
    let beta_offset = q_params.beta_offset_div2 * 2;
    // Section 8.7.2.2: the thresholds of an edge between macroblocks whose
    // QPs are `p_qp` and `q_qp`.
    let thresholds =
        |p_qp: u8, q_qp: u8| FilterThresholds::from_qp(p_qp, q_qp, alpha_offset, beta_offset);

    // Section 8.7, step 2.c / 2.d -- determine filterLeftMbEdgeFlag / filterTopMbEdgeFlag
    let left = left.filter(|p| should_filter_edge(q_params.idc, q, p));
    let top = top.filter(|p| should_filter_edge(q_params.idc, q, p));

    let (bs_vert, bs_horz) = compute_bs_arrays(q, left, top);
    let has_nonzero_bs = |bs: &[u8; 4]| bs[0] | bs[1] | bs[2] | bs[3] != 0;

    // Luma. With the 8x8 transform only internal edge 2 is filtered.
    {
        let mut plane = frame.plane_mut(ColorPlane::Y);
        let stride = plane.cfg.stride;
        let data = plane.data_origin_mut();
        let (x, y) = (mb_x * MB_WIDTH, mb_y * MB_HEIGHT);
        let internal_edges: &[usize] = if q.is(TRANSFORM_8X8) { &[2] } else { &[1, 2, 3] };
        let internal = thresholds(q.qp, q.qp);

        // Section 8.7, step 3.a/3.b -- luma vertical edges, left to right
        if let Some(p) = left {
            if has_nonzero_bs(&bs_vert[0]) {
                let edge = thresholds(p.qp, q.qp);
                filter_luma_edge::<true>(data, stride, x, y, &bs_vert[0], &edge);
            }
        }
        for &edge in internal_edges {
            if has_nonzero_bs(&bs_vert[edge]) {
                filter_luma_edge::<true>(data, stride, x + 4 * edge, y, &bs_vert[edge], &internal);
            }
        }

        // Section 8.7, step 3.c/3.d -- luma horizontal edges, top to bottom
        if let Some(p) = top {
            if has_nonzero_bs(&bs_horz[0]) {
                let edge = thresholds(p.qp, q.qp);
                filter_luma_edge::<false>(data, stride, x, y, &bs_horz[0], &edge);
            }
        }
        for &edge in internal_edges {
            if has_nonzero_bs(&bs_horz[edge]) {
                filter_luma_edge::<false>(data, stride, x, y + 4 * edge, &bs_horz[edge], &internal);
            }
        }
    }

    // Section 8.7, step 3 for chroma (4:2:0), each plane on its own.
    // Chroma edge 0 reuses luma edge 0 BS, chroma edge 1 reuses luma edge 2 BS
    for (i, plane) in [ColorPlane::Cb, ColorPlane::Cr].into_iter().enumerate() {
        let mut plane = frame.plane_mut(plane);
        let stride = plane.cfg.stride;
        let data = plane.data_origin_mut();
        let (x, y) = (mb_x * MB_WIDTH / 2, mb_y * MB_HEIGHT / 2);
        let internal = thresholds(q.qp_c[i], q.qp_c[i]);

        if let Some(p) = left {
            if has_nonzero_bs(&bs_vert[0]) {
                let edge = thresholds(p.qp_c[i], q.qp_c[i]);
                filter_chroma_edge::<true>(data, stride, x, y, &bs_vert[0], &edge);
            }
        }
        if has_nonzero_bs(&bs_vert[2]) {
            filter_chroma_edge::<true>(data, stride, x + 4, y, &bs_vert[2], &internal);
        }
        if let Some(p) = top {
            if has_nonzero_bs(&bs_horz[0]) {
                let edge = thresholds(p.qp_c[i], q.qp_c[i]);
                filter_chroma_edge::<false>(data, stride, x, y, &bs_horz[0], &edge);
            }
        }
        if has_nonzero_bs(&bs_horz[2]) {
            filter_chroma_edge::<false>(data, stride, x, y + 4, &bs_horz[2], &internal);
        }
    }
}

/// Section 8.7, steps 2.c/2.d -- whether the macroblock edge between `q` and
/// its left or top neighbour `p` is filtered (`filterLeftMbEdgeFlag` /
/// `filterTopMbEdgeFlag`). `idc` is `disable_deblocking_filter_idc` of q's
/// slice. Edges on the picture boundary have no `p` and are never filtered.
fn should_filter_edge(idc: DeblockingFilterIdc, q: &MbDeblockInfo, p: &MbDeblockInfo) -> bool {
    if p.is(NOT_DECODED) {
        return false;
    }
    match idc {
        DeblockingFilterIdc::On => true,
        DeblockingFilterIdc::Off => false,
        DeblockingFilterIdc::OnExceptSliceBounds => p.slice_id == q.slice_id,
    }
}

/// Sections 8.7.1/8.7.2 -- Filtering process for a single luma block edge:
/// the vertical (`VERTICAL`) edge left of column `x` along the 16 rows from
/// row `y`, or the horizontal edge above row `y` along the 16 columns from
/// column `x`. `data` is the plane from its visible origin, with rows `stride`
/// apart; `bs_array` holds the bS of each 4-sample segment.
fn filter_luma_edge<const VERTICAL: bool>(
    data: &mut [u8],
    stride: usize,
    x: usize,
    y: usize,
    bs_array: &[u8; 4],
    thresh: &FilterThresholds,
) {
    // ALPHA_TABLE[0..=15] and BETA_TABLE[0..=15] are zero (Table 8-16). When
    // either threshold is zero, every Eq 8-460 comparison `.abs() < threshold`
    // is unconditionally false, so no pixel on this edge can be filtered.
    if thresh.alpha == 0 || thresh.beta == 0 {
        return;
    }
    // The bS < 4 filter's tc0 (Table 8-17) for a segment with bS `bs`.
    let tc0 = |bs: u8| i32::from(TC0_TABLE[usize::from(bs - 1)][thresh.index_a]);

    if VERTICAL {
        // Per row, the samples p3..q3 are the 8 around column `x`.
        let Some(start) = (y * stride + x).checked_sub(4) else { return };
        let Some(region) = data.get_mut(start..) else { return };
        let mut rows = region.chunks_mut(stride);
        // One 4-row segment per bS: its tc0 and the weak/strong dispatch are
        // hoisted out of the per-row loop.
        for &bs in bs_array {
            if bs == BS_NONE {
                rows.nth(3);
                continue;
            }
            let strong = bs >= BS_STRONG;
            let tc0 = if strong { 0 } else { tc0(bs) };
            for row in rows.by_ref().take(4) {
                let Some(samples) = row.first_chunk_mut::<8>() else { return };
                filter_luma_samples(samples, strong, tc0, thresh);
            }
        }
    } else {
        // Rows p3..q3 of the 16 columns: the samples are a column of them.
        let Some(start) = (y * stride + x).checked_sub(4 * stride) else { return };
        let Some(mut rows) = luma_rows(data, start, stride) else { return };
        for (segment, &bs) in bs_array.iter().enumerate() {
            if bs == BS_NONE {
                continue;
            }
            let strong = bs >= BS_STRONG;
            let tc0 = if strong { 0 } else { tc0(bs) };
            for col in 4 * segment..4 * segment + 4 {
                filter_luma_samples(&mut Column { rows: &mut rows, col }, strong, tc0, thresh);
            }
        }
    }
}

/// Sections 8.7.1/8.7.2 -- Filtering process for a single chroma block edge
/// (4:2:0): the vertical (`VERTICAL`) edge left of column `x` along the 8
/// rows from row `y`, or the horizontal edge above row `y` along the 8
/// columns from column `x`; see `filter_luma_edge`. Each bS of `bs_array`
/// covers 2 chroma samples (4 luma samples).
fn filter_chroma_edge<const VERTICAL: bool>(
    data: &mut [u8],
    stride: usize,
    x: usize,
    y: usize,
    bs_array: &[u8; 4],
    thresh: &FilterThresholds,
) {
    // See filter_luma_edge: zero threshold makes Eq 8-460 always false.
    if thresh.alpha == 0 || thresh.beta == 0 {
        return;
    }
    // Section 8.7.2.3 with chromaEdgeFlag = 1.
    // Equation 8-466: tc = tc0 + 1 for chroma.
    let tc = |bs: u8| i32::from(TC0_TABLE[usize::from(bs - 1)][thresh.index_a]) + 1;

    if VERTICAL {
        // Per row, the samples p1, p0, q0, q1 are the 4 around column `x`.
        let Some(start) = (y * stride + x).checked_sub(2) else { return };
        let Some(region) = data.get_mut(start..) else { return };
        let mut rows = region.chunks_mut(stride);
        for &bs in bs_array {
            if bs == BS_NONE {
                rows.nth(1);
                continue;
            }
            let strong = bs >= BS_STRONG;
            let tc = if strong { 0 } else { tc(bs) };
            for row in rows.by_ref().take(2) {
                let Some(samples) = row.first_chunk_mut::<4>() else { return };
                filter_chroma_samples(samples, strong, tc, thresh);
            }
        }
    } else {
        // Rows p1..q1 of the 8 columns: the samples are a column of them.
        let Some(start) = (y * stride + x).checked_sub(2 * stride) else { return };
        let Some(mut rows) = chroma_rows(data, start, stride) else { return };
        for (segment, &bs) in bs_array.iter().enumerate() {
            if bs == BS_NONE {
                continue;
            }
            let strong = bs >= BS_STRONG;
            let tc = if strong { 0 } else { tc(bs) };
            for col in 2 * segment..2 * segment + 2 {
                filter_chroma_samples(&mut Column { rows: &mut rows, col }, strong, tc, thresh);
            }
        }
    }
}

/// The samples of one line across an edge, p side first: p3, p2, p1, p0,
/// q0, q1, q2, q3 for luma, p1, p0, q0, q1 for chroma. `get` and `set` take
/// the index of a sample in that order.
trait EdgeSamples {
    fn get(&self, k: usize) -> i32;
    fn set(&mut self, k: usize, value: u8);
}

/// Across a vertical edge the samples of a line are consecutive in a row.
impl<const N: usize> EdgeSamples for [u8; N] {
    #[inline]
    fn get(&self, k: usize) -> i32 {
        i32::from(self[k])
    }

    #[inline]
    fn set(&mut self, k: usize, value: u8) {
        self[k] = value;
    }
}

/// Across a horizontal edge the samples of a line are a column of the rows
/// on both sides of the edge.
struct Column<'r, 'a, const N: usize, const W: usize> {
    rows: &'r mut [&'a mut [u8; W]; N],
    col: usize,
}

impl<const N: usize, const W: usize> EdgeSamples for Column<'_, '_, N, W> {
    #[inline]
    fn get(&self, k: usize) -> i32 {
        i32::from(self.rows[k][self.col])
    }

    #[inline]
    fn set(&mut self, k: usize, value: u8) {
        self.rows[k][self.col] = value;
    }
}

/// The 16 samples from `start` of the 8 rows p3..q3 around a horizontal
/// luma edge, rows `stride` apart; `None` if they are not all in `data`.
fn luma_rows(data: &mut [u8], start: usize, stride: usize) -> Option<[&mut [u8; 16]; 8]> {
    let mut rows = data.get_mut(start..)?.chunks_mut(stride);
    let mut next = || rows.next().and_then(|row| row.first_chunk_mut::<16>());
    Some([next()?, next()?, next()?, next()?, next()?, next()?, next()?, next()?])
}

/// The 8 samples from `start` of the 4 rows p1..q1 around a horizontal
/// chroma edge; see `luma_rows`.
fn chroma_rows(data: &mut [u8], start: usize, stride: usize) -> Option<[&mut [u8; 8]; 4]> {
    let mut rows = data.get_mut(start..)?.chunks_mut(stride);
    let mut next = || rows.next().and_then(|row| row.first_chunk_mut::<8>());
    Some([next()?, next()?, next()?, next()?])
}

/// Clip1 of Section 5.7 for 8-bit samples.
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn clip1(value: i32) -> u8 {
    // In 0..=255 after the clamp.
    value.clamp(0, 255) as u8
}

/// Sections 8.7.2.3 / 8.7.2.4 -- filters one line of luma samples across an
/// edge whose segment has bS < 4 with `tc0` (Table 8-17), or bS = 4
/// (`strong`).
#[inline]
fn filter_luma_samples(
    samples: &mut impl EdgeSamples,
    strong: bool,
    tc0: i32,
    thresh: &FilterThresholds,
) {
    let (alpha, beta) = (thresh.alpha, thresh.beta);
    let p0 = samples.get(3);
    let q0 = samples.get(4);
    let p1 = samples.get(2);
    let q1 = samples.get(5);

    // Equation 8-460: filter condition
    if (p0 - q0).abs() < alpha && (p1 - p0).abs() < beta && (q1 - q0).abs() < beta {
        let p2 = samples.get(1);
        let q2 = samples.get(6);
        let ap = (p2 - p0).abs();
        let aq = (q2 - q0).abs();
        let ap_lt_beta = ap < beta;
        let aq_lt_beta = aq < beta;

        if strong {
            // Section 8.7.2.4 -- strong filter (bS == 4)
            let small_diff = (p0 - q0).abs() < ((alpha >> 2) + 2); // Eq 8-476

            // p-side: Equations 8-477..8-479 (strong) or 8-480 (weak fallback)
            if ap_lt_beta && small_diff {
                let p3 = samples.get(0);
                samples.set(3, clip1((p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >> 3));
                samples.set(2, clip1((p2 + p1 + p0 + q0 + 2) >> 2));
                samples.set(1, clip1((2 * p3 + 3 * p2 + p1 + p0 + q0 + 4) >> 3));
            } else {
                samples.set(3, clip1((2 * p1 + p0 + q1 + 2) >> 2));
            }

            // q-side: Equations 8-484..8-486 (strong) or 8-487 (weak fallback)
            if aq_lt_beta && small_diff {
                let q3 = samples.get(7);
                samples.set(4, clip1((p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4) >> 3));
                samples.set(5, clip1((p0 + q0 + q1 + q2 + 2) >> 2));
                samples.set(6, clip1((2 * q3 + 3 * q2 + q1 + q0 + p0 + 4) >> 3));
            } else {
                samples.set(4, clip1((2 * q1 + q0 + p1 + 2) >> 2));
            }
        } else {
            // Section 8.7.2.3 -- weak filter (bS < 4)
            let tc = tc0 + i32::from(ap_lt_beta) + i32::from(aq_lt_beta); // Eq 8-465

            let delta = (((q0 - p0) << 2) + (p1 - q1) + 4) >> 3; // Eq 8-467
            let delta_c = delta.clamp(-tc, tc);

            samples.set(3, clip1(p0 + delta_c)); // Eq 8-468: p0'
            samples.set(4, clip1(q0 - delta_c)); // Eq 8-469: q0'

            if ap_lt_beta {
                // Eq 8-470: p1'
                let d = (p2 + ((p0 + q0 + 1) >> 1) - (p1 << 1)) >> 1;
                samples.set(2, clip1(p1 + d.clamp(-tc0, tc0)));
            }
            if aq_lt_beta {
                // Eq 8-472: q1'
                let d = (q2 + ((p0 + q0 + 1) >> 1) - (q1 << 1)) >> 1;
                samples.set(5, clip1(q1 + d.clamp(-tc0, tc0)));
            }
        }
    }
}

/// Sections 8.7.2.3 / 8.7.2.4 with chromaEdgeFlag = 1 -- filters one line of
/// chroma samples across an edge whose segment has bS < 4 with `tc`
/// (Eq. 8-466), or bS = 4 (`strong`).
#[inline]
fn filter_chroma_samples(
    samples: &mut impl EdgeSamples,
    strong: bool,
    tc: i32,
    thresh: &FilterThresholds,
) {
    let (alpha, beta) = (thresh.alpha, thresh.beta);
    let p1 = samples.get(0);
    let p0 = samples.get(1);
    let q0 = samples.get(2);
    let q1 = samples.get(3);

    // Equation 8-460: filter condition
    if (p0 - q0).abs() < alpha && (p1 - p0).abs() < beta && (q1 - q0).abs() < beta {
        let (p0_new, q0_new) = if strong {
            // Section 8.7.2.4 with chromaStyleFilteringFlag = 1.
            // Equations 8-480, 8-487: p0', q0'
            (clip1((2 * p1 + p0 + q1 + 2) >> 2), clip1((2 * q1 + q0 + p1 + 2) >> 2))
        } else {
            // Equation 8-467: delta
            let delta = (((q0 - p0) << 2) + (p1 - q1) + 4) >> 3;
            let delta_c = delta.clamp(-tc, tc);
            // Equations 8-468, 8-469: p0', q0'
            (clip1(p0 + delta_c), clip1(q0 - delta_c))
        };

        samples.set(1, p0_new);
        samples.set(2, q0_new);
    }
}

/// Section 8.7.2.1 -- Precompute boundary strength arrays for all edges of macroblock `q`.
/// Returns `(bs_vert, bs_horz)` where each is `[[u8; 4]; 4]` indexed by `[edge_idx][block_idx]`.
/// Edge 0 is shared with `left` / `top`, which is `None` when that edge isn't filtered.
fn compute_bs_arrays(
    q: &MbDeblockInfo,
    left: Option<&MbDeblockInfo>,
    top: Option<&MbDeblockInfo>,
) -> ([[u8; 4]; 4], [[u8; 4]; 4]) {
    if q.is(INTRA) {
        let mut bs_vert = [[BS_NONE; 4]; 4];
        let mut bs_horz = [[BS_NONE; 4]; 4];
        // External edges (MB boundary): an intra macroblock on either side
        // gives every block bS=BS_STRONG.
        if left.is_some() {
            bs_vert[0] = [BS_STRONG; 4];
        }
        if top.is_some() {
            bs_horz[0] = [BS_STRONG; 4];
        }
        // Internal edges: every internal 4x4 edge has bS=BS_INTRA.
        if q.is(TRANSFORM_8X8) {
            // 8x8 transform: only edge 2 (at the 8-sample boundary)
            bs_vert[2] = [BS_INTRA; 4];
            bs_horz[2] = [BS_INTRA; 4];
        } else {
            for edge in 1..4 {
                bs_vert[edge] = [BS_INTRA; 4];
                bs_horz[edge] = [BS_INTRA; 4];
            }
        }
        return (bs_vert, bs_horz);
    }

    // The internal edges to derive, as a mask of edge indices: with the 8x8
    // transform only edge 2 is filtered, and NO_INTERNAL_EDGES means they all
    // have bS=0 (their BS_NONE init).
    let internal_edges = if q.is(NO_INTERNAL_EDGES) {
        0
    } else if q.is(TRANSFORM_8X8) {
        1 << 2
    } else {
        0b1110
    };
    (
        inter_edge_strengths::<true>(q, left, internal_edges),
        inter_edge_strengths::<false>(q, top, internal_edges),
    )
}

/// Section 8.7.2.1 -- the boundary strengths of the vertical (`VERTICAL`) or
/// horizontal edges of inter macroblock `q`, indexed by `[edge_idx][block_idx]`.
/// `neighbour` is the macroblock across edge 0, `None` when that edge isn't
/// filtered; `internal_edges` has bit `e` set for each internal edge `e` to
/// derive. Edges left out have bS=0.
fn inter_edge_strengths<const VERTICAL: bool>(
    q: &MbDeblockInfo,
    neighbour: Option<&MbDeblockInfo>,
    internal_edges: u8,
) -> [[u8; 4]; 4] {
    let mut bs = [[BS_NONE; 4]; 4];
    let mut edges = internal_edges;
    // The blocks across the macroblock edge; without an inter neighbour the
    // segments of edge 0 are left out, and any blocks will do.
    let mut p_side = q;
    if let Some(p) = neighbour {
        if p.is(INTRA) {
            // Fast path: an intra neighbour gives every segment of the
            // macroblock edge bS=BS_STRONG.
            bs[0] = [BS_STRONG; 4];
        } else {
            edges |= 1;
            p_side = p;
        }
    }
    let p_rules = q.is(L1_EMPTY);
    if edges <= 1 {
        // At most the macroblock edge: its four segments on their own.
        if edges == 1 {
            bs[0] = std::array::from_fn(|b| {
                let (p_blk, q_blk) = if VERTICAL { (4 * b + 3, 4 * b) } else { (12 + b, b) };
                segment_bs(&p_side.blocks, p_blk, &q.blocks, q_blk, p_rules)
            });
        }
        return bs;
    }

    let p_blocks = blocks_across::<VERTICAL>(&q.blocks, &p_side.blocks);
    let segments = segment_strengths(&p_blocks, &q.blocks, p_rules);
    for (edge, edge_bs) in bs.iter_mut().enumerate() {
        if edges & (1 << edge) != 0 {
            // Segment `b` of vertical edge `edge` is block (b, edge); of a
            // horizontal edge, block (edge, b).
            *edge_bs = std::array::from_fn(|b| {
                segments[if VERTICAL { 4 * b + edge } else { 4 * edge + b }]
            });
        }
    }
    bs
}

/// The p sides of the edge segments whose q sides are the blocks of `q`:
/// entry `i` is the 4x4 block left of (`VERTICAL`) or above block `i` of the
/// macroblock -- a block of `q` across an internal edge, and across the
/// macroblock edge, a block of `neighbour` from its last column or row.
fn blocks_across<const VERTICAL: bool>(q: &BlockInfo, neighbour: &BlockInfo) -> BlockInfo {
    fn shift<const VERTICAL: bool, T: Copy>(q: &[T; 16], neighbour: &[T; 16]) -> [T; 16] {
        std::array::from_fn(|i| {
            if VERTICAL {
                if i % 4 == 0 {
                    neighbour[i + 3]
                } else {
                    q[i - 1]
                }
            } else if i < 4 {
                neighbour[i + 12]
            } else {
                q[i - 4]
            }
        })
    }
    let nz = if VERTICAL {
        (q.nz << 1) & 0xEEEE | (neighbour.nz >> 3) & 0x1111
    } else {
        q.nz << 4 | neighbour.nz >> 12
    };
    BlockInfo {
        nz,
        ref_l0: shift::<VERTICAL, _>(&q.ref_l0, &neighbour.ref_l0),
        ref_l1: shift::<VERTICAL, _>(&q.ref_l1, &neighbour.ref_l1),
        mv_l0: shift::<VERTICAL, _>(&q.mv_l0, &neighbour.mv_l0),
        mv_l1: shift::<VERTICAL, _>(&q.mv_l1, &neighbour.mv_l1),
    }
}

/// Section 8.7.2.1 -- bS (2, 1 or 0) of the 16 edge segments between block
/// `i` of `p` and block `i` of `q` (see `blocks_across`), for inter
/// macroblocks. Derived together: `segment_bs` is branch-free.
fn segment_strengths(p: &BlockInfo, q: &BlockInfo, p_rules: bool) -> [u8; 16] {
    if p_rules {
        std::array::from_fn(|i| segment_bs(p, i, q, i, true))
    } else {
        std::array::from_fn(|i| segment_bs(p, i, q, i, false))
    }
}

/// Section 8.7.2.1 -- bS (2, 1 or 0) of the edge segment between 4x4 block
/// `p_blk` of `p` and 4x4 block `q_blk` of `q` (raster indices), blocks of
/// inter macroblocks. `p_rules` is set when q's slice has no list 1.
// Inlined into the edge-0 path too, where the call would outweigh the body.
// `&` rather than `&&` keeps it branch-free, so that `segment_strengths`
// derives its 16 segments together. The p / q names follow the spec's.
#[allow(clippy::inline_always, clippy::needless_bitwise_bool, clippy::similar_names)]
#[inline(always)]
fn segment_bs(p: &BlockInfo, p_blk: usize, q: &BlockInfo, q_blk: usize, p_rules: bool) -> u8 {
    // Motion vectors closer than 4 quarter luma samples in both components.
    let close = |a: &[[i16; 2]; 16], a_blk: usize, b: &[[i16; 2]; 16], b_blk: usize| {
        (a[a_blk][0].abs_diff(b[b_blk][0]) < 4) & (a[a_blk][1].abs_diff(b[b_blk][1]) < 4)
    };
    let (ref_p_l0, ref_q_l0) = (p.ref_l0[p_blk], q.ref_l0[q_blk]);
    let (ref_p_l1, ref_q_l1) = (p.ref_l1[p_blk], q.ref_l1[q_blk]);

    let same_motion = if p_rules {
        // P-slice context for q (refPicList1 is empty in P-slices). If
        // p comes from a B-slice partition that uses BiPred or Pred_L1,
        // only its L0 entry is consulted here -- the q-side P-slice rules
        // don't describe how to interpret p's L1 reference. Acceptable
        // in practice because mixing P and B slices within one picture
        // is uncommon, and uniform-type pictures are unaffected.
        (ref_p_l0 == ref_q_l0) & close(&p.mv_l0, p_blk, &q.mv_l0, q_blk)
    } else {
        // The same pictures with close motion vectors, pairing the lists
        // either directly or crosswise.
        let direct = (ref_p_l0 == ref_q_l0)
            & (ref_p_l1 == ref_q_l1)
            & close(&p.mv_l0, p_blk, &q.mv_l0, q_blk)
            & close(&p.mv_l1, p_blk, &q.mv_l1, q_blk);
        let swap = (ref_p_l0 == ref_q_l1)
            & (ref_p_l1 == ref_q_l0)
            & close(&p.mv_l0, p_blk, &q.mv_l1, q_blk)
            & close(&p.mv_l1, p_blk, &q.mv_l0, q_blk);
        direct | swap
    };

    let coded = (p.nz >> p_blk | q.nz >> q_blk) & 1 != 0;
    if coded {
        BS_CODED
    } else if same_motion {
        BS_NONE
    } else {
        BS_MOTION
    }
}

// True when the MB's internal 4x4 edges are all guaranteed bS=0:
// inter MB with a single 16x16 partition (NumMbPart == 1, which excludes
// B_Direct_16x16 and B_Skip whose motion is derived per 8x8 sub-block) and
// no coded luma coefficients. Saves deriving 24 bS values per such MB.
#[inline]
fn has_no_internal_edges(mb: &Macroblock) -> bool {
    if mb.get_coded_block_pattern().luma() != 0 {
        return false;
    }
    match mb {
        Macroblock::P(m) => m.NumMbPart() == 1,
        Macroblock::B(m) => m.NumMbPart() == 1,
        _ => false,
    }
}

/// The raster mask (bit `4 * row + col`) of the 4x4 blocks whose luma
/// transform block has non-zero coefficients.
fn nonzero_blocks(mb: &Macroblock) -> u16 {
    // Section 8.7.2.1: bS=2 is derived from the transform block containing the
    // sample, whose size depends on transform_size_8x8_flag. For 8x8 transforms
    // the "block" is the enclosing 8x8 group -- its four 4x4 sub-sections share a
    // single coded status for deblocking purposes.
    use super::residual::LumaResidual;
    let Some(res) = mb.get_residual() else { return 0 };
    match &res.luma {
        LumaResidual::Intra16x16 { dc, ac_nc, .. } => {
            raster_mask(|blk_idx| ac_nc[blk_idx] != 0 || dc[blk_idx] != 0)
        }
        LumaResidual::Block8x8 { .. } => {
            let cbp_luma = res.coded_block_pattern.luma();
            raster_mask(|blk_idx| cbp_luma & (1 << (blk_idx / 4)) != 0)
        }
        LumaResidual::Block4x4 { nc, .. } => raster_mask(|blk_idx| nc[blk_idx] != 0),
        LumaResidual::Empty => 0,
    }
}

/// The mask with bit `4 * row + col` set when `coded` holds for 4x4 block
/// (row, col), which it takes by its index in the Z-scan of Section 6.4.3.
#[inline]
fn raster_mask(coded: impl Fn(usize) -> bool) -> u16 {
    let mut mask = 0;
    for row in 0..4 {
        for col in 0..4 {
            mask |= u16::from(coded(scan_4x4(row, col))) << (4 * row + col);
        }
    }
    mask
}

fn get_qp(mb: &Macroblock) -> u8 {
    match mb {
        Macroblock::I(m) => m.qp,
        Macroblock::P(m) => m.qp,
        Macroblock::B(m) => m.qp,
        Macroblock::PCM(m) => m.qp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h264::macroblock::{
        BMb, BMbType, CodedBlockPattern, IMb, IMbType, MbMotion, MbNeighborName, MotionVector, PMb,
        PMbType, PartitionInfo, PcmMb,
    };
    use crate::h264::residual::{LumaLevel8x8, LumaResidual, Residual};

    /// A 4x2 picture (8 MBs in raster order):
    ///
    /// ```text
    ///   +---+---+---+---+
    ///   | 0 | 1 | 2 | 3 |
    ///   +---+---+---+---+
    ///   | 4 | 5 | 6 | 7 |
    ///   +---+---+---+---+
    /// ```
    const W: usize = 4;
    const H: usize = 2;

    fn dummy_mbs() -> Vec<Option<Macroblock>> {
        (0..W * H).map(|_| Some(Macroblock::I(IMb::default()))).collect()
    }

    fn deblock(idc: DeblockingFilterIdc) -> SliceDeblockParams {
        SliceDeblockParams { idc, alpha_c0_offset_div2: 0, beta_offset_div2: 0 }
    }

    /// A decoded picture as the decoder holds it while its slices arrive.
    struct TestPicture<'a> {
        pps: &'a PicParameterSet,
        macroblocks: &'a [Option<Macroblock>],
        mb_slice_id: &'a [u16],
        slice_deblock: &'a [SliceDeblockParams],
        slice_ref_pocs: &'a [(Vec<i32>, Vec<i32>)],
        pic_width_in_mbs: usize,
        pic_height_in_mbs: usize,
    }

    impl TestPicture<'_> {
        fn deblock_input(&self) -> PictureDeblockInput<'_> {
            PictureDeblockInput {
                slice_deblock: self.slice_deblock,
                pic_width_in_mbs: self.pic_width_in_mbs,
                pic_height_in_mbs: self.pic_height_in_mbs,
            }
        }
    }

    /// Gathers the records of `picture` into `records` as the decoder does:
    /// macroblocks in raster order, the slice started when it changes.
    fn gather(picture: &TestPicture, records: &mut DeblockRecords) -> Vec<MbDeblockInfo> {
        let pic_size = picture.pic_width_in_mbs * picture.pic_height_in_mbs;
        records.start_picture(pic_size, picture.pps);
        let mut slice = None;
        for (mb_addr, mb) in picture.macroblocks.iter().enumerate().take(pic_size) {
            let Some(mb) = mb else { continue };
            let slice_id = picture.mb_slice_id[mb_addr];
            if slice != Some(slice_id) {
                let (l0, l1) = &picture.slice_ref_pocs[usize::from(slice_id)];
                records.start_slice(slice_id, l0, l1);
                slice = Some(slice_id);
            }
            records.record(mb_addr, mb);
        }
        records.records.clone()
    }

    fn build_records(picture: &TestPicture) -> Vec<MbDeblockInfo> {
        gather(picture, &mut DeblockRecords::default())
    }

    fn make_input<'a>(
        pps: &'a PicParameterSet,
        macroblocks: &'a [Option<Macroblock>],
        mb_slice_id: &'a [u16],
        slice_deblock: &'a [SliceDeblockParams],
        slice_ref_pocs: &'a [(Vec<i32>, Vec<i32>)],
    ) -> TestPicture<'a> {
        TestPicture {
            pps,
            macroblocks,
            mb_slice_id,
            slice_deblock,
            slice_ref_pocs,
            pic_width_in_mbs: W,
            pic_height_in_mbs: H,
        }
    }

    /// Whether `filter_picture` filters the left (A) or top (B) edge of
    /// macroblock `mb_addr`: the neighbour lookup and `should_filter_edge`,
    /// as `filter_picture` and `filter_macroblock` apply them.
    fn edge_filtered(input: &TestPicture, mb_addr: usize, neighbor: MbNeighborName) -> bool {
        let records = build_records(input);
        let width = input.pic_width_in_mbs;
        let (left, top) = mb_neighbors(&records, width, mb_addr % width, mb_addr / width);
        let p = match neighbor {
            MbNeighborName::A => left,
            MbNeighborName::B => top,
            _ => unreachable!("only the left and top edges are filtered"),
        };
        let q = &records[mb_addr];
        let idc = input.slice_deblock[usize::from(q.slice_id)].idc;
        p.is_some_and(|p| should_filter_edge(idc, q, p))
    }

    #[test]
    fn picture_boundary_edges_never_filtered() {
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::On)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 0 is at the top-left corner -- both A (left) and B (top) are out of picture.
        assert!(!edge_filtered(&input, 0, MbNeighborName::A));
        assert!(!edge_filtered(&input, 0, MbNeighborName::B));
        // MB 4 is at the left edge -- A is out of picture.
        assert!(!edge_filtered(&input, 4, MbNeighborName::A));
        // MB 3 is at the top edge -- B is out of picture.
        assert!(!edge_filtered(&input, 3, MbNeighborName::B));
    }

    #[test]
    fn single_slice_idc_on_filters_internal_edges() {
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::On)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 5 has both an A (MB 4) and B (MB 1) neighbor inside the picture.
        assert!(edge_filtered(&input, 5, MbNeighborName::A));
        assert!(edge_filtered(&input, 5, MbNeighborName::B));
    }

    #[test]
    fn single_slice_idc_off_filters_nothing() {
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::Off)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        assert!(!edge_filtered(&input, 5, MbNeighborName::A));
        assert!(!edge_filtered(&input, 5, MbNeighborName::B));
    }

    #[test]
    fn single_slice_idc_2_behaves_like_idc_on() {
        // OnExceptSliceBounds with one slice has no slice boundaries to skip,
        // so it should filter every internal edge identically to On.
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::OnExceptSliceBounds)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        assert!(edge_filtered(&input, 5, MbNeighborName::A));
        assert!(edge_filtered(&input, 5, MbNeighborName::B));
    }

    #[test]
    fn two_slices_idc_on_still_filters_cross_slice_edges() {
        // disable_deblocking_filter_idc=0 (On): cross-slice edges DO filter.
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        // Top row -> slice 0, bottom row -> slice 1.
        let mb_slice_id = vec![0, 0, 0, 0, 1, 1, 1, 1];
        let slice_deblock = [
            deblock(DeblockingFilterIdc::On),
            deblock(DeblockingFilterIdc::On),
        ];
        let slice_ref_pocs = [(vec![], vec![]), (vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 5 is in slice 1; its B neighbor is MB 1 (slice 0) -- cross-slice.
        // Still filtered because idc=On disregards slice boundaries.
        assert!(edge_filtered(&input, 5, MbNeighborName::B));
        // MB 5's A neighbor is MB 4 (slice 1) -- same slice, also filtered.
        assert!(edge_filtered(&input, 5, MbNeighborName::A));
    }

    #[test]
    fn two_slices_idc_2_suppresses_cross_slice_edges() {
        // disable_deblocking_filter_idc=2: cross-slice edges suppressed,
        // intra-slice edges filtered.
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0, 0, 0, 0, 1, 1, 1, 1];
        let slice_deblock = [
            deblock(DeblockingFilterIdc::OnExceptSliceBounds),
            deblock(DeblockingFilterIdc::OnExceptSliceBounds),
        ];
        let slice_ref_pocs = [(vec![], vec![]), (vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 4: B neighbor is MB 0 (slice 0) -- cross-slice -> suppressed.
        assert!(!edge_filtered(&input, 4, MbNeighborName::B));
        // MB 5: B neighbor is MB 1 (slice 0) -- cross-slice -> suppressed.
        assert!(!edge_filtered(&input, 5, MbNeighborName::B));
        // MB 5: A neighbor is MB 4 (slice 1) -- same slice -> filtered.
        assert!(edge_filtered(&input, 5, MbNeighborName::A));
        // MB 1: A neighbor is MB 0 (slice 0) -- same slice -> filtered.
        assert!(edge_filtered(&input, 1, MbNeighborName::A));
    }

    #[test]
    fn two_slices_with_mixed_idcs_uses_q_block_idc() {
        // Per spec 8.7, the deblock IDC for an edge is taken from the slice
        // containing the q-block (lower/right MB). Verify by giving slice 0
        // idc=Off and slice 1 idc=On, then checking edges where the q is in
        // slice 1.
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0, 0, 0, 0, 1, 1, 1, 1];
        let slice_deblock = [
            deblock(DeblockingFilterIdc::Off),
            deblock(DeblockingFilterIdc::On),
        ];
        let slice_ref_pocs = [(vec![], vec![]), (vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // q = MB 5 (slice 1, idc=On) -> edges filter regardless of p's slice.
        assert!(edge_filtered(&input, 5, MbNeighborName::A));
        assert!(edge_filtered(&input, 5, MbNeighborName::B));
        // q = MB 1 (slice 0, idc=Off) -> no filtering.
        assert!(!edge_filtered(&input, 1, MbNeighborName::A));
    }

    #[test]
    fn edges_next_to_undecoded_macroblocks_are_not_filtered() {
        let pps = PicParameterSet::default();
        let mut mbs = dummy_mbs();
        mbs[4] = None;
        let mb_slice_id = vec![0, 0, 0, 0, u16::MAX, 0, 0, 0];
        let slice_deblock = [deblock(DeblockingFilterIdc::On)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input = make_input(&pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        assert!(build_records(&input)[4].is(NOT_DECODED));
        // MB 5's left neighbour was never decoded.
        assert!(!edge_filtered(&input, 5, MbNeighborName::A));
        assert!(edge_filtered(&input, 5, MbNeighborName::B));
    }

    /// The boundary-strength derivation that `MbDeblockInfo` replaced,
    /// verbatim apart from paths: it read the `Macroblock` enum, the boxed
    /// residual and per-slice POC lists for every edge segment. Kept as the
    /// reference that the record-based derivation must match exactly.
    mod reference {
        use super::TestPicture;
        use crate::h264::deblocking::{BS_CODED, BS_INTRA, BS_MOTION, BS_NONE, BS_STRONG};
        use crate::h264::macroblock::Macroblock;

        pub(super) fn compute_bs_arrays(
            input: &TestPicture,
            mb: &Macroblock,
            q_slice_id: u16,
            left: Option<(&Macroblock, u16)>,
            top: Option<(&Macroblock, u16)>,
            transform_8x8: bool,
        ) -> ([[u8; 4]; 4], [[u8; 4]; 4]) {
            let ref_pocs = |slice_id: u16| {
                let (l0, l1) = &input.slice_ref_pocs[slice_id as usize];
                (l0.as_slice(), l1.as_slice())
            };
            let mut bs_vert = [[BS_NONE; 4]; 4];
            let mut bs_horz = [[BS_NONE; 4]; 4];
            let q_intra = mb.is_intra();

            let (q_l0, q_l1) = ref_pocs(q_slice_id);

            // External edges (MB boundary) -- use neighbor MB as p.
            // Fast path: when either side is intra, every block's bS is BS_STRONG, so
            // we can fill the row directly without 4 enum-dispatch calls into get_bs.
            if let Some((p_mb, p_slice_id)) = left {
                if q_intra || p_mb.is_intra() {
                    bs_vert[0] = [BS_STRONG; 4];
                } else {
                    let (p_l0, p_l1) = ref_pocs(p_slice_id);
                    for b in 0..4 {
                        bs_vert[0][b] = get_bs(mb, p_mb, q_l0, q_l1, p_l0, p_l1, 0, b, true);
                    }
                }
            }
            if let Some((p_mb, p_slice_id)) = top {
                if q_intra || p_mb.is_intra() {
                    bs_horz[0] = [BS_STRONG; 4];
                } else {
                    let (p_l0, p_l1) = ref_pocs(p_slice_id);
                    for b in 0..4 {
                        bs_horz[0][b] = get_bs(mb, p_mb, q_l0, q_l1, p_l0, p_l1, 0, b, false);
                    }
                }
            }

            // Internal edges -- p and q are both within this MB (same slice -> same POCs).
            // Fast paths:
            //   - intra MB: every internal 4x4 edge has bS=BS_INTRA (skips 24 calls)
            //   - 16x16 inter with cbp_luma==0: every internal edge has bS=0; leave
            //     bs_vert/bs_horz at their BS_NONE init
            if q_intra {
                if !transform_8x8 {
                    for edge in 1..4 {
                        bs_vert[edge] = [BS_INTRA; 4];
                        bs_horz[edge] = [BS_INTRA; 4];
                    }
                } else {
                    // 8x8 transform: only edge 2 (at the 8-sample boundary)
                    bs_vert[2] = [BS_INTRA; 4];
                    bs_horz[2] = [BS_INTRA; 4];
                }
            } else if !has_no_internal_edges(mb) {
                if !transform_8x8 {
                    for edge in 1..4 {
                        for b in 0..4 {
                            bs_vert[edge][b] =
                                get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, edge, b, true);
                            bs_horz[edge][b] =
                                get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, edge, b, false);
                        }
                    }
                } else {
                    for b in 0..4 {
                        bs_vert[2][b] = get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, 2, b, true);
                        bs_horz[2][b] = get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, 2, b, false);
                    }
                }
            }

            (bs_vert, bs_horz)
        }

        // True when the MB's internal 4x4 edges are all guaranteed bS=0:
        // inter MB with a single 16x16 partition (NumMbPart == 1, which excludes
        // B_Direct_16x16 and B_Skip whose motion is derived per 8x8 sub-block) and
        // no coded luma coefficients. Saves up to 24 get_bs calls per such MB.
        #[inline]
        fn has_no_internal_edges(mb: &Macroblock) -> bool {
            if mb.get_coded_block_pattern().luma() != 0 {
                return false;
            }
            match mb {
                Macroblock::P(m) => m.NumMbPart() == 1,
                Macroblock::B(m) => m.NumMbPart() == 1,
                _ => false,
            }
        }

        #[inline(always)]
        fn has_nonzero_coeffs(mb: &Macroblock, blk_idx: usize) -> bool {
            // Section 8.7.2.1: bS=2 is derived from the transform block containing the
            // sample, whose size depends on transform_size_8x8_flag. For 8x8 transforms
            // the "block" is the enclosing 8x8 group -- its four 4x4 sub-sections share a
            // single coded status for deblocking purposes.
            use crate::h264::residual::LumaResidual;
            let Some(res) = mb.get_residual() else { return false };
            match &res.luma {
                LumaResidual::Intra16x16 { dc, ac_nc, .. } => {
                    ac_nc[blk_idx] != 0 || dc[blk_idx] != 0
                }
                LumaResidual::Block8x8 { .. } => {
                    let i8x8 = blk_idx / 4;
                    res.coded_block_pattern.luma() & (1 << i8x8) != 0
                }
                LumaResidual::Block4x4 { nc, .. } => nc[blk_idx] != 0,
                LumaResidual::Empty => false,
            }
        }

        #[inline(always)]
        fn get_partition(
            mb: &Macroblock,
            y: usize,
            x: usize,
        ) -> Option<crate::h264::macroblock::PartitionInfo> {
            match mb {
                Macroblock::P(m) => Some(m.motion.partitions[y][x]),
                Macroblock::B(m) => Some(m.motion.partitions[y][x]),
                _ => None,
            }
        }

        // (q_y, q_x, p_y, p_x) for a given edge: outer index is `edge_idx + 4 * is_vertical`,
        // inner index is `block_idx`. The neighbor across an internal edge is `edge_idx - 1`;
        // for `edge_idx == 0` (external edge) it wraps to row/col 3 of the neighboring MB.
        const EDGE_BLOCK_COORDS: [[(usize, usize, usize, usize); 4]; 8] = {
            let mut t = [[(0usize, 0, 0, 0); 4]; 8];
            let mut e = 0;
            while e < 4 {
                let p = if e == 0 { 3 } else { e - 1 };
                let mut b = 0;
                while b < 4 {
                    t[e][b] = (e, b, p, b);
                    t[4 + e][b] = (b, e, b, p);
                    b += 1;
                }
                e += 1;
            }
            t
        };

        #[allow(clippy::too_many_arguments)]
        pub(super) fn get_bs(
            mb_q: &Macroblock,
            mb_p: &Macroblock,
            // Reference POCs for the q-block's slice. Used for ref-list comparison
            // in BS_MOTION derivation; passing POCs (rather than DPB indices) makes
            // cross-slice comparison well-defined when p and q come from different
            // slices that may have different ref lists.
            q_l0_pocs: &[i32],
            q_l1_pocs: &[i32],
            p_l0_pocs: &[i32],
            p_l1_pocs: &[i32],
            // 0 corresponds to the external edge. 1..3 correspond to internal edges.
            edge_idx: usize,
            block_idx: usize,
            is_vertical: bool,
        ) -> u8 {
            let (q_y, q_x, p_y, p_x) =
                EDGE_BLOCK_COORDS[edge_idx + 4 * (is_vertical as usize)][block_idx];

            if mb_p.is_intra() || mb_q.is_intra() {
                if edge_idx == 0 {
                    return BS_STRONG;
                }
                return BS_INTRA;
            }

            let blk_q_idx = crate::h264::residual::scan_4x4(q_y, q_x);
            let blk_p_idx = crate::h264::residual::scan_4x4(p_y, p_x);

            if has_nonzero_coeffs(mb_p, blk_p_idx) || has_nonzero_coeffs(mb_q, blk_q_idx) {
                return BS_CODED;
            }

            let p_part = get_partition(mb_p, p_y, p_x);
            let q_part = get_partition(mb_q, q_y, q_x);

            match (p_part, q_part) {
                (Some(pp), Some(qq)) => {
                    // P-slice context for q (refPicList1 is empty in P-slices). If
                    // p comes from a B-slice partition that uses BiPred or Pred_L1,
                    // only its L0 entry is consulted here -- the q-side P-slice rules
                    // don't describe how to interpret p's L1 reference. Acceptable
                    // in practice because mixing P and B slices within one picture
                    // is uncommon, and uniform-type pictures are unaffected.
                    if q_l1_pocs.is_empty() {
                        let ref_p_l0 = p_l0_pocs.get(pp.ref_idx_l0 as usize).copied();
                        let ref_q_l0 = q_l0_pocs.get(qq.ref_idx_l0 as usize).copied();
                        if ref_p_l0 != ref_q_l0 {
                            return BS_MOTION;
                        }
                        let mv_diff_x = (pp.mv_l0.x as i32 - qq.mv_l0.x as i32).abs();
                        let mv_diff_y = (pp.mv_l0.y as i32 - qq.mv_l0.y as i32).abs();
                        if mv_diff_x >= 4 || mv_diff_y >= 4 {
                            return BS_MOTION;
                        }
                        return BS_NONE;
                    }

                    let ref_p_l0 = p_l0_pocs.get(pp.ref_idx_l0 as usize).copied();
                    let ref_q_l0 = q_l0_pocs.get(qq.ref_idx_l0 as usize).copied();
                    let ref_p_l1 = p_l1_pocs.get(pp.ref_idx_l1 as usize).copied();
                    let ref_q_l1 = q_l1_pocs.get(qq.ref_idx_l1 as usize).copied();

                    let direct_match = ref_p_l0 == ref_q_l0
                        && ref_p_l1 == ref_q_l1
                        && (pp.mv_l0.x as i32 - qq.mv_l0.x as i32).abs() < 4
                        && (pp.mv_l0.y as i32 - qq.mv_l0.y as i32).abs() < 4
                        && (pp.mv_l1.x as i32 - qq.mv_l1.x as i32).abs() < 4
                        && (pp.mv_l1.y as i32 - qq.mv_l1.y as i32).abs() < 4;

                    let swap_match = ref_p_l0 == ref_q_l1
                        && ref_p_l1 == ref_q_l0
                        && (pp.mv_l0.x as i32 - qq.mv_l1.x as i32).abs() < 4
                        && (pp.mv_l0.y as i32 - qq.mv_l1.y as i32).abs() < 4
                        && (pp.mv_l1.x as i32 - qq.mv_l0.x as i32).abs() < 4
                        && (pp.mv_l1.y as i32 - qq.mv_l0.y as i32).abs() < 4;

                    if direct_match || swap_match {
                        BS_NONE
                    } else {
                        BS_MOTION
                    }
                }
                (Some(_), None) | (None, Some(_)) => BS_MOTION,
                (None, None) => BS_NONE,
            }
        }
    }

    /// Deterministic generator for the randomized tests (`SplitMix64`).
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        /// A number in `0..n`, as a `T`.
        fn below<T: TryFrom<u64>>(&mut self, n: u64) -> T
        where
            T::Error: std::fmt::Debug,
        {
            T::try_from(self.next() % n).expect("fits")
        }

        fn pick<T: Copy>(&mut self, items: &[T]) -> T {
            items[self.below::<usize>(items.len() as u64)]
        }
    }

    fn random_partition(rng: &mut Rng) -> PartitionInfo {
        // In-range indices, indices past the end of the test lists, and the
        // unused-list marker.
        const REF_IDX: [u8; 6] = [0, 1, 2, 3, 5, u8::MAX];
        let mut mv = || MotionVector { x: rng.below::<i16>(17) - 8, y: rng.below::<i16>(17) - 8 };
        let (mv_l0, mv_l1) = (mv(), mv());
        PartitionInfo {
            ref_idx_l0: rng.pick(&REF_IDX),
            ref_idx_l1: rng.pick(&REF_IDX),
            mv_l0,
            mv_l1,
            ..PartitionInfo::default()
        }
    }

    /// A motion field that is uniform, uniform per 8x8, a small perturbation
    /// of one partition, or random per 4x4, so that equal and nearly equal
    /// neighbours are common.
    fn random_motion(rng: &mut Rng) -> MbMotion {
        let mut motion = MbMotion::default();
        match rng.below::<u8>(4) {
            0 => {
                let part = random_partition(rng);
                motion.partitions = [[part; 4]; 4];
            }
            1 => {
                for quadrant in 0..4 {
                    let part = random_partition(rng);
                    for blk in 0..4 {
                        let (row, col) = (quadrant / 2 * 2 + blk / 2, quadrant % 2 * 2 + blk % 2);
                        motion.partitions[row][col] = part;
                    }
                }
            }
            2 => {
                let part = random_partition(rng);
                for row in &mut motion.partitions {
                    for blk in row {
                        *blk = part;
                        blk.mv_l0.x += rng.below::<i16>(9) - 4;
                        blk.mv_l1.y += rng.below::<i16>(9) - 4;
                        if rng.below::<u8>(8) == 0 {
                            blk.ref_idx_l1 = random_partition(rng).ref_idx_l1;
                        }
                    }
                }
            }
            _ => {
                for row in &mut motion.partitions {
                    for blk in row {
                        *blk = random_partition(rng);
                    }
                }
            }
        }
        motion
    }

    fn random_residual(rng: &mut Rng) -> Option<Box<Residual>> {
        let mut residual = Box::new(Residual {
            coded_block_pattern: CodedBlockPattern(rng.below(64)),
            ..Residual::default()
        });
        match rng.below::<u8>(5) {
            0 => return None,
            1 => {}
            2 | 3 => {
                let mut nc = [0u8; 16];
                for n in &mut nc {
                    if rng.below::<u8>(3) == 0 {
                        *n = rng.below::<u8>(16) + 1;
                    }
                }
                residual.luma = LumaResidual::Block4x4 { levels: [[0; 16]; 16], nc };
            }
            _ => {
                residual.luma =
                    LumaResidual::Block8x8 { levels: [LumaLevel8x8::default(); 4], nc: [0; 16] };
            }
        }
        Some(residual)
    }

    fn random_mb(rng: &mut Rng) -> Macroblock {
        const P_TYPES: [PMbType; 6] = [
            PMbType::P_L0_16x16,
            PMbType::P_L0_L0_16x8,
            PMbType::P_L0_L0_8x16,
            PMbType::P_8x8,
            PMbType::P_8x8ref0,
            PMbType::P_Skip,
        ];
        let qp = rng.below(52);
        // Coded luma is left out half of the time, which enables the
        // single-partition no-internal-edges fast path.
        let cbp = if rng.below::<u8>(2) == 0 { rng.below::<u8>(4) << 4 } else { rng.below(64) };
        let coded_block_pattern = CodedBlockPattern(cbp);
        let transform_size_8x8_flag = rng.below::<u8>(3) == 0;
        match rng.below::<u8>(10) {
            0 => Macroblock::I(IMb {
                mb_type: IMbType::try_from(rng.below::<u32>(25)).expect("I mb_type"),
                transform_size_8x8_flag,
                coded_block_pattern,
                qp,
                residual: random_residual(rng),
                ..IMb::default()
            }),
            1 => Macroblock::PCM(PcmMb { qp, ..PcmMb::default() }),
            2..=4 => Macroblock::P(PMb {
                mb_type: rng.pick(&P_TYPES),
                motion: random_motion(rng),
                coded_block_pattern,
                qp,
                transform_size_8x8_flag,
                residual: random_residual(rng),
                ..PMb::default()
            }),
            _ => Macroblock::B(BMb {
                mb_type: BMbType::try_from(rng.below::<u32>(24)).expect("B mb_type"),
                motion: random_motion(rng),
                coded_block_pattern,
                qp,
                transform_size_8x8_flag,
                residual: random_residual(rng),
                ..BMb::default()
            }),
        }
    }

    fn transform_8x8(mb: &Macroblock) -> bool {
        match mb {
            Macroblock::I(m) => m.transform_size_8x8_flag,
            Macroblock::P(m) => m.transform_size_8x8_flag,
            Macroblock::B(m) => m.transform_size_8x8_flag,
            Macroblock::PCM(_) => false,
        }
    }

    /// The bS of the 16 segments of `q`'s vertical or horizontal edges as
    /// `inter_edge_strengths` derives them, with `neighbour` across edge 0.
    fn segments(q: &MbDeblockInfo, neighbour: &MbDeblockInfo, vertical: bool) -> [u8; 16] {
        let p_blocks = if vertical {
            blocks_across::<true>(&q.blocks, &neighbour.blocks)
        } else {
            blocks_across::<false>(&q.blocks, &neighbour.blocks)
        };
        segment_strengths(&p_blocks, &q.blocks, q.is(L1_EMPTY))
    }

    /// Raster indices (p, q) of the two 4x4 blocks of segment `b` of edge
    /// `edge`; for edge 0, p is a block of the neighbouring macroblock.
    fn edge_blocks(edge: usize, b: usize, vertical: bool) -> (usize, usize) {
        let p_edge = (edge + 3) % 4;
        if vertical {
            (4 * b + p_edge, 4 * b + edge)
        } else {
            (4 * p_edge + b, 4 * edge + b)
        }
    }

    /// The index of segment `b` of edge `edge` in the result of `segments`.
    fn segment(edge: usize, b: usize, vertical: bool) -> usize {
        if vertical {
            4 * b + edge
        } else {
            4 * edge + b
        }
    }

    /// Checks the whole-macroblock bS arrays of the two macroblocks of
    /// `input`, side by side or one above the other, against the reference,
    /// fast paths included. Returns those of the second one.
    fn check_bs_arrays(
        input: &TestPicture,
        records: &[MbDeblockInfo],
    ) -> ([[u8; 4]; 4], [[u8; 4]; 4]) {
        let [Some(p_mb), Some(q_mb)] = input.macroblocks else { unreachable!("two macroblocks") };
        let (width, height) = (input.pic_width_in_mbs, input.pic_height_in_mbs);
        let neighbour = Some((p_mb, input.mb_slice_id[0]));
        let (left, top) = if width == 2 { (neighbour, None) } else { (None, neighbour) };
        let q_slice_id = input.mb_slice_id[1];
        let expected =
            reference::compute_bs_arrays(input, q_mb, q_slice_id, left, top, transform_8x8(q_mb));
        let (left, top) = mb_neighbors(records, width, width - 1, height - 1);
        let actual = compute_bs_arrays(&records[1], left, top);
        assert_eq!(actual, expected, "{p_mb:?} | {q_mb:?}");

        let p_slice_id = input.mb_slice_id[0];
        let expected =
            reference::compute_bs_arrays(input, p_mb, p_slice_id, None, None, transform_8x8(p_mb));
        assert_eq!(compute_bs_arrays(&records[0], None, None), expected, "{p_mb:?}");
        actual
    }

    /// Two macroblocks, side by side or one above the other, with random
    /// types, residuals, motion and slices: the record-based derivation must
    /// give the bS of the reference `get_bs` on every edge segment between
    /// inter blocks, and the bS arrays of the reference `compute_bs_arrays`.
    #[test]
    fn record_bs_matches_reference_get_bs() {
        let pps = PicParameterSet::default();
        // POC 4 appears twice in slice 0's list 0 and in both slices; slice
        // 1 has no list 1, as a P slice.
        let slice_ref_pocs = [(vec![8, 4, 16, 4], vec![16, 12, 8]), (vec![4, 12, 8], vec![])];
        let slice_deblock = [deblock(DeblockingFilterIdc::On), deblock(DeblockingFilterIdc::On)];
        let mut rng = Rng(0x5EED_DEB1_0C4B_0001);
        let (mut checked, mut by_bs) = (0usize, [0usize; 5]);
        for _ in 0..6000 {
            let mbs = [Some(random_mb(&mut rng)), Some(random_mb(&mut rng))];
            let mb_slice_id = [rng.below::<u16>(2), rng.below::<u16>(2)];
            let [Some(p_mb), Some(q_mb)] = &mbs else { unreachable!() };
            for vertical_edge in [true, false] {
                // MB 1 is right of MB 0 when their shared edge is vertical,
                // below it otherwise.
                let (width, height) = if vertical_edge { (2, 1) } else { (1, 2) };
                let input = TestPicture {
                    pps: &pps,
                    macroblocks: &mbs,
                    mb_slice_id: &mb_slice_id,
                    slice_deblock: &slice_deblock,
                    slice_ref_pocs: &slice_ref_pocs,
                    pic_width_in_mbs: width,
                    pic_height_in_mbs: height,
                };
                let records = build_records(&input);
                let pocs = |mb_addr: usize| {
                    let (l0, l1) = &slice_ref_pocs[usize::from(mb_slice_id[mb_addr])];
                    (l0.as_slice(), l1.as_slice())
                };
                let ((p_l0, p_l1), (q_l0, q_l1)) = (pocs(0), pocs(1));
                let mut check = |actual: u8, expected: u8, what: &dyn Fn() -> String| {
                    assert_eq!(actual, expected, "{}", what());
                    by_bs[usize::from(actual)] += 1;
                    checked += 1;
                };

                // The 4 segments of the macroblock edge, as 16 lanes and one
                // at a time.
                if !p_mb.is_intra() && !q_mb.is_intra() {
                    let (p, q) = (&records[0], &records[1]);
                    let lanes = segments(q, p, vertical_edge);
                    for b in 0..4 {
                        let expected = reference::get_bs(
                            q_mb,
                            p_mb,
                            q_l0,
                            q_l1,
                            p_l0,
                            p_l1,
                            0,
                            b,
                            vertical_edge,
                        );
                        let actual = lanes[segment(0, b, vertical_edge)];
                        check(actual, expected, &|| format!("MB edge {b}: {p_mb:?} | {q_mb:?}"));
                        let (p_blk, q_blk) = edge_blocks(0, b, vertical_edge);
                        let actual = segment_bs(&p.blocks, p_blk, &q.blocks, q_blk, q.is(L1_EMPTY));
                        check(actual, expected, &|| format!("MB edge {b}: {p_mb:?} | {q_mb:?}"));
                    }
                }

                // The 24 segments of each macroblock's internal edges.
                for (mb_addr, mb) in [(0, p_mb), (1, q_mb)] {
                    if mb.is_intra() {
                        continue;
                    }
                    let (l0, l1) = pocs(mb_addr);
                    let record = &records[mb_addr];
                    for vertical in [true, false] {
                        let lanes = segments(record, record, vertical);
                        for edge in 1..4 {
                            for b in 0..4 {
                                let expected =
                                    reference::get_bs(mb, mb, l0, l1, l0, l1, edge, b, vertical);
                                let actual = lanes[segment(edge, b, vertical)];
                                check(actual, expected, &|| format!("edge {edge}, {b}: {mb:?}"));
                                let (p_blk, q_blk) = edge_blocks(edge, b, vertical);
                                let blocks = &record.blocks;
                                let p_rules = record.is(L1_EMPTY);
                                let actual = segment_bs(blocks, p_blk, blocks, q_blk, p_rules);
                                check(actual, expected, &|| format!("edge {edge}, {b}: {mb:?}"));
                            }
                        }
                    }
                }

                // Whole-macroblock arrays, including the intra and other fast
                // paths.
                let (bs_vert, bs_horz) = check_bs_arrays(&input, &records);
                for bs in bs_vert.as_flattened().iter().chain(bs_horz.as_flattened()) {
                    by_bs[usize::from(*bs)] += 1;
                }
            }
        }
        // Every outcome is exercised, each many times.
        assert!(by_bs.iter().all(|&n| n > checked / 100), "bS histogram {by_bs:?}");
    }

    fn inter_mb(residual: Option<Box<Residual>>) -> Macroblock {
        Macroblock::P(PMb { residual, ..PMb::default() })
    }

    /// The record of `mb` as a macroblock of slice `slice_id` of a picture
    /// whose slices have the reference lists `slice_ref_pocs`, the slices
    /// before it decoded first.
    fn record(
        mb: &Macroblock,
        slice_ref_pocs: &[(Vec<i32>, Vec<i32>)],
        slice_id: u16,
    ) -> MbDeblockInfo {
        let mut records = DeblockRecords::default();
        records.start_picture(1, &PicParameterSet::default());
        for (id, (l0, l1)) in (0..=slice_id).zip(slice_ref_pocs) {
            records.start_slice(id, l0, l1);
        }
        records.record(0, mb);
        records.records[0]
    }

    #[test]
    fn record_nz_is_raster_ordered_per_residual_layout() {
        let no_refs = [(vec![], vec![])];
        // 4x4 transform: one bit per 4x4 block with coefficients. Z-scan
        // block 6 is row 1, column 2; block 9 is row 2, column 1.
        let mut nc = [0u8; 16];
        nc[6] = 3;
        nc[9] = 1;
        let residual = Residual {
            luma: LumaResidual::Block4x4 { levels: [[0; 16]; 16], nc },
            ..Residual::default()
        };
        let mb = inter_mb(Some(Box::new(residual)));
        assert_eq!(record(&mb, &no_refs, 0).blocks.nz, 1 << (4 + 2) | 1 << (8 + 1));

        // 8x8 transform: coded_block_pattern bits cover whole quadrants.
        let residual = Residual {
            coded_block_pattern: CodedBlockPattern::new(0, 0b0110),
            luma: LumaResidual::Block8x8 { levels: [LumaLevel8x8::default(); 4], nc: [0; 16] },
            ..Residual::default()
        };
        let mb = inter_mb(Some(Box::new(residual)));
        assert_eq!(
            record(&mb, &no_refs, 0).blocks.nz,
            0b0000_0000_1100_1100 | 0b0011_0011_0000_0000
        );

        // No luma coefficients, or no residual at all.
        assert_eq!(record(&inter_mb(Some(Box::default())), &no_refs, 0).blocks.nz, 0);
        assert_eq!(record(&inter_mb(None), &no_refs, 0).blocks.nz, 0);

        // Intra macroblocks don't need `nz`: their bS doesn't depend on it.
        let residual = Residual {
            luma: LumaResidual::Intra16x16 { dc: [1; 16], ac: [[0; 15]; 16], ac_nc: [1; 16] },
            ..Residual::default()
        };
        let mb = Macroblock::I(IMb { residual: Some(Box::new(residual)), ..IMb::default() });
        let info = record(&mb, &no_refs, 0);
        assert_eq!(info.blocks, BlockInfo::default());
        assert!(info.is(INTRA));
    }

    #[test]
    fn record_resolves_reference_indices_to_picture_ids() {
        let slice_ref_pocs = [(vec![7, 3], vec![3, 9]), (vec![9, 7, 7], vec![])];
        // One id per POC, whichever slice and list it appears in.
        let mut records = DeblockRecords::default();
        records.start_picture(1, &PicParameterSet::default());
        records.start_slice(0, &slice_ref_pocs[0].0, &slice_ref_pocs[0].1);
        assert_eq!(
            (&records.slice.l0[..3], &records.slice.l1[..3]),
            (&[1, 2, 0][..], &[2, 3, 0][..])
        );
        records.start_slice(1, &slice_ref_pocs[1].0, &slice_ref_pocs[1].1);
        assert_eq!((&records.slice.l0[..4], &records.slice.l1[..1]), (&[3, 1, 1, 0][..], &[0][..]));

        let mut motion = MbMotion::default();
        let parts = motion.partitions.as_flattened_mut();
        parts[0].ref_idx_l0 = 1;
        parts[0].ref_idx_l1 = 0;
        parts[5].ref_idx_l0 = u8::MAX;
        parts[5].ref_idx_l1 = 1;
        parts[5].mv_l1 = MotionVector { x: -3, y: 7 };
        parts[15].ref_idx_l0 = 2;
        parts[15].ref_idx_l1 = u8::MAX;
        let mb = Macroblock::B(BMb { motion, ..BMb::default() });

        let info = record(&mb, &slice_ref_pocs, 0);
        let blocks = &info.blocks;
        assert!(!info.is(L1_EMPTY));
        assert_eq!((blocks.ref_l0[0], blocks.ref_l1[0]), (2, 2));
        // u8::MAX marks an unused list; index 2 is past the end of list 0.
        assert_eq!((blocks.ref_l0[5], blocks.ref_l1[5]), (0, 3));
        assert_eq!((blocks.ref_l0[15], blocks.ref_l1[15]), (0, 0));
        assert_eq!(blocks.mv_l1[5], [-3, 7]);

        // Against slice 1's lists: list 1 is empty, so no index resolves.
        let info = record(&mb, &slice_ref_pocs, 1);
        let blocks = &info.blocks;
        assert!(info.is(L1_EMPTY));
        assert_eq!((blocks.ref_l0[0], blocks.ref_l1[0]), (1, 0));
        assert_eq!((blocks.ref_l0[15], blocks.ref_l1[15]), (1, 0));
    }

    #[test]
    fn slice_ref_ids_clear_the_previous_slice() {
        let mut ids = RefPictureIds::default();
        let mut tables = SliceRefIds::default();
        tables.load(&mut ids, 0, &[7, 3, 9], &[5]);
        assert_eq!((&tables.l0[..4], &tables.l1[..2]), (&[1, 2, 3, 0][..], &[4, 0][..]));
        assert!(!tables.l1_empty);
        tables.load(&mut ids, 1, &[9], &[]);
        assert_eq!((&tables.l0[..4], &tables.l1[..2]), (&[3, 0, 0, 0][..], &[0, 0][..]));
        assert!(tables.l1_empty);
        assert!(tables.l0.iter().chain(&tables.l1).skip(1).all(|&id| id == 0));
        assert_eq!(tables.l0[usize::from(u8::MAX)], 0);
    }

    #[test]
    fn record_qp_and_flags() {
        let pps = PicParameterSet {
            chroma_qp_index_offset: -4,
            second_chroma_qp_index_offset: 7,
            ..PicParameterSet::default()
        };
        let mbs = [
            Some(Macroblock::P(PMb {
                mb_type: PMbType::P_L0_16x16,
                qp: 40,
                transform_size_8x8_flag: true,
                ..PMb::default()
            })),
            Some(Macroblock::B(BMb { mb_type: BMbType::B_Skip, qp: 3, ..BMb::default() })),
            Some(Macroblock::PCM(PcmMb::default())),
            Some(Macroblock::I(IMb { qp: 51, ..IMb::default() })),
        ];
        let input = TestPicture {
            pps: &pps,
            macroblocks: &mbs,
            mb_slice_id: &[0, 0, 1, 1],
            slice_deblock: &[deblock(DeblockingFilterIdc::On), deblock(DeblockingFilterIdc::On)],
            slice_ref_pocs: &[(vec![0], vec![]), (vec![], vec![])],
            pic_width_in_mbs: 2,
            pic_height_in_mbs: 2,
        };
        let records = build_records(&input);
        for (info, mb) in records.iter().zip(&mbs) {
            let qp = get_qp(mb.as_ref().expect("decoded"));
            assert_eq!(info.qp, qp);
            assert_eq!(
                info.qp_c,
                [get_chroma_qp(i32::from(qp), -4, 0), get_chroma_qp(i32::from(qp), 7, 0)]
            );
        }
        assert_eq!(records[0].qp_c, [34, 38]);
        assert_eq!(
            records.iter().map(|info| (info.flags, info.slice_id)).collect::<Vec<_>>(),
            [
                (TRANSFORM_8X8 | NO_INTERNAL_EDGES | L1_EMPTY, 0),
                // B_Skip has no macroblock partition, so it takes the full path.
                (L1_EMPTY, 0),
                (INTRA | L1_EMPTY, 1),
                (INTRA | L1_EMPTY, 1),
            ]
        );
    }

    /// The visible samples of the three planes of `frame`, row by row.
    fn visible_samples(frame: &VideoFrame) -> Vec<u8> {
        let mut samples = Vec::new();
        for plane in [ColorPlane::Y, ColorPlane::Cb, ColorPlane::Cr] {
            let plane = frame.plane(plane);
            let (width, height, stride) = (plane.cfg.width, plane.cfg.height, plane.cfg.stride);
            for row in plane.data_origin().chunks(stride).take(height) {
                samples.extend_from_slice(&row[..width]);
            }
        }
        samples
    }

    #[test]
    fn filter_off_in_every_slice_leaves_the_picture_alone() {
        let pps = PicParameterSet::default();
        // 2x2 intra macroblocks, the top row in slice 0 and the bottom row
        // in slice 1, with a step of 10 between the left and right columns
        // of macroblocks: every edge between them has bS 4.
        let mbs: Vec<_> =
            (0..4).map(|_| Some(Macroblock::I(IMb { qp: 40, ..IMb::default() }))).collect();
        let deblocked = |idc: [DeblockingFilterIdc; 2]| {
            let mut frame =
                VideoFrame::alloc_4_2_0(&crate::api::DefaultAllocator, 32, 32).expect("alloc");
            for plane in [ColorPlane::Y, ColorPlane::Cb, ColorPlane::Cr] {
                let mut plane = frame.plane_mut(plane);
                let (width, stride) = (plane.cfg.width, plane.cfg.stride);
                for row in plane.data_origin_mut().chunks_mut(stride) {
                    for (x, sample) in row.iter_mut().take(width).enumerate() {
                        *sample = if x < width / 2 { 60 } else { 70 };
                    }
                }
            }
            let before = visible_samples(&frame);
            let picture = TestPicture {
                pps: &pps,
                macroblocks: &mbs,
                mb_slice_id: &[0, 0, 1, 1],
                slice_deblock: &idc.map(deblock),
                slice_ref_pocs: &[(vec![], vec![]), (vec![], vec![])],
                pic_width_in_mbs: 2,
                pic_height_in_mbs: 2,
            };
            let mut records = DeblockRecords::default();
            gather(&picture, &mut records);
            filter_picture(&picture.deblock_input(), &records, &mut frame);
            (before, visible_samples(&frame))
        };

        let (before, after) = deblocked([DeblockingFilterIdc::Off, DeblockingFilterIdc::Off]);
        assert_eq!(after, before);
        let (before, after) = deblocked([DeblockingFilterIdc::Off, DeblockingFilterIdc::On]);
        assert_ne!(after, before);
        // Only the edges of slice 1, the bottom row of macroblocks, are
        // filtered; across its top edge that reaches 3 rows up (p2).
        assert_eq!(after[..32 * 13], before[..32 * 13]);
        assert_ne!(after[..32 * 16], before[..32 * 16]);
    }

    #[test]
    fn deblock_records_are_reused_without_leftovers() {
        let pps = PicParameterSet::default();
        let mut rng = Rng(0x5C4A_7C11_0000_0003);
        let pictures = [
            (
                (0..8).map(|_| Some(random_mb(&mut rng))).collect::<Vec<_>>(),
                vec![0u16, 0, 0, 1, 1, 1, 2, 2],
                vec![(vec![3, 9, 1], vec![7, 3]), (vec![1], vec![]), (vec![5, 3], vec![3])],
            ),
            (
                (0..2).map(|_| Some(random_mb(&mut rng))).collect(),
                vec![0, 0],
                vec![(vec![2], vec![11])],
            ),
        ];
        let slice_deblock = [DeblockingFilterIdc::On; 3].map(deblock);
        let inputs: Vec<_> = pictures
            .iter()
            .map(|(macroblocks, mb_slice_id, slice_ref_pocs)| TestPicture {
                pps: &pps,
                macroblocks,
                mb_slice_id,
                slice_deblock: &slice_deblock,
                slice_ref_pocs,
                pic_width_in_mbs: macroblocks.len() / 2,
                pic_height_in_mbs: 2,
            })
            .collect();

        let mut records = DeblockRecords::default();
        for input in [&inputs[0], &inputs[1], &inputs[0]] {
            assert_eq!(gather(input, &mut records), build_records(input));
        }
    }
}
