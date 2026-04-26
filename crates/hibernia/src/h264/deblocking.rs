use super::decoder::{get_chroma_qp, SliceDeblockParams, VideoFrame};
use super::macroblock::{
    get_neighbor_mbs, IMb, Macroblock, MbAddr, MbNeighborName, MbPredictionMode,
};
use super::pps::PicParameterSet;
use super::slice::DeblockingFilterIdc;
use super::sps::SequenceParameterSet;
use super::tables::{MB_HEIGHT, MB_WIDTH};
use super::{ColorPlane, Point};

// Boundary Strength (bS) values
const BS_STRONG: u8 = 4;
const BS_INTRA: u8 = 3;
const BS_CODED: u8 = 2;
const BS_MOTION: u8 = 1;
const BS_NONE: u8 = 0;

// Table 8-16
const ALPHA_TABLE: [u8; 52] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 17, 20,
    22, 25, 28, 32, 36, 40, 45, 50, 56, 63, 71, 80, 90, 101, 113, 127, 144, 162, 182, 203, 226,
    255, 255,
];

const BETA_TABLE: [u8; 52] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 6, 6, 7, 7, 8, 8,
    9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18,
];

// Table 8-17
const TC0_TABLE: [[u8; 52]; 3] = [
    // bS = 1 (BS_MOTION)
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 7, 8, 9, 10, 11, 13,
    ],
    // bS = 2 (BS_CODED)
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 7, 8, 8, 10, 11, 12, 13, 15, 17,
    ],
    // bS = 3 (BS_INTRA)
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
        2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 7, 8, 9, 10, 11, 13, 14, 16, 18, 20, 23, 25,
    ],
];

/// Section 8.7.2.2 — filter thresholds derived from QP values.
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
        let index_a = (qp_av + alpha_offset).clamp(0, 51) as usize;
        let index_b = (qp_av + beta_offset).clamp(0, 51) as usize;
        FilterThresholds {
            alpha: ALPHA_TABLE[index_a] as i32,
            beta: BETA_TABLE[index_b] as i32,
            index_a,
        }
    }
}

/// Picture-wide view consumed by `filter_picture`. Holds references to
/// `CurrentPicture`'s decoded macroblocks, per-MB slice ownership, and the
/// per-slice deblocking parameters and reference-list POCs accumulated during
/// slice processing.
pub struct PictureDeblockInput<'a> {
    pub sps: &'a SequenceParameterSet,
    pub pps: &'a PicParameterSet,
    pub macroblocks: &'a [Option<Macroblock>],
    pub mb_slice_id: &'a [u16],
    pub slice_deblock: &'a [SliceDeblockParams],
    pub slice_ref_pocs: &'a [(Vec<i32>, Vec<i32>)],
    pub pic_width_in_mbs: usize,
    pub pic_height_in_mbs: usize,
}

impl PictureDeblockInput<'_> {
    fn get_mb(&self, mb_addr: MbAddr) -> Option<&Macroblock> {
        self.macroblocks.get(mb_addr as usize).and_then(|m| m.as_ref())
    }

    fn slice_id(&self, mb_addr: MbAddr) -> u16 {
        self.mb_slice_id[mb_addr as usize]
    }

    fn deblock_params(&self, mb_addr: MbAddr) -> &SliceDeblockParams {
        &self.slice_deblock[self.slice_id(mb_addr) as usize]
    }

    fn ref_pocs(&self, slice_id: u16) -> (&[i32], &[i32]) {
        let (l0, l1) = &self.slice_ref_pocs[slice_id as usize];
        (l0.as_slice(), l1.as_slice())
    }

    fn neighbor_addr(&self, mb_addr: MbAddr, neighbor: MbNeighborName) -> Option<MbAddr> {
        get_neighbor_mbs(self.pic_width_in_mbs as u32, 0, mb_addr, neighbor)
    }

    fn mb_xy(&self, mb_addr: MbAddr) -> Point {
        let width_in_mbs = self.pic_width_in_mbs as u32;
        let x = mb_addr % width_in_mbs * (MB_WIDTH as u32);
        let y = mb_addr / width_in_mbs * (MB_HEIGHT as u32);
        Point { x, y }
    }
}

/// Section 8.7 — picture-level deblocking pass. Replaces the per-slice
/// `filter_slice` so a multi-slice picture is filtered as one frame and
/// per-MB slice ownership is honoured for `disable_deblocking_filter_idc=2`
/// and for boundary-strength reference comparisons across slice boundaries.
pub fn filter_picture(input: &PictureDeblockInput, frame: &mut VideoFrame) {
    let total_mbs = input.pic_width_in_mbs * input.pic_height_in_mbs;
    for mb_addr in 0..total_mbs {
        filter_macroblock(input, frame, mb_addr as MbAddr);
    }
}

/// Section 8.7, steps 1–3 — Filter all edges of a single macroblock.
/// BS values are precomputed once per MB and reused across luma and chroma
/// to avoid redundant derivation (Section 8.7.2.1).
fn filter_macroblock(input: &PictureDeblockInput, frame: &mut VideoFrame, mb_addr: MbAddr) {
    let mb = match input.get_mb(mb_addr) {
        Some(mb) => mb,
        None => return,
    };

    // Per Section 8.7, edge filtering parameters come from the slice that
    // contains the q-block (the macroblock on the lower/right side of the
    // edge), which is the current MB for both its left and top edges.
    let q_params = input.deblock_params(mb_addr);
    if q_params.idc == DeblockingFilterIdc::Off {
        return;
    }
    let alpha_offset = q_params.alpha_c0_offset_div2 * 2;
    let beta_offset = q_params.beta_offset_div2 * 2;

    let q_slice_id = input.slice_id(mb_addr);
    let mb_xy = input.mb_xy(mb_addr);
    let q_qp = get_qp(mb);

    // Section 8.7, step 2.c / 2.d — determine filterLeftMbEdgeFlag / filterTopMbEdgeFlag
    let filter_left = should_filter_edge(input, mb_addr, MbNeighborName::A);
    let filter_top = should_filter_edge(input, mb_addr, MbNeighborName::B);

    let transform_8x8 = match mb {
        Macroblock::I(m) => m.transform_size_8x8_flag,
        Macroblock::P(m) => m.transform_size_8x8_flag,
        Macroblock::B(m) => m.transform_size_8x8_flag,
        Macroblock::PCM(_) => false,
    };

    // Section 8.7, step 1 — locate neighbor macroblocks A (left) and B (top)
    let left_info: Option<(&Macroblock, u8, u16)> = if filter_left {
        input.neighbor_addr(mb_addr, MbNeighborName::A).and_then(|addr| {
            input.get_mb(addr).map(|p_mb| (p_mb, get_qp(p_mb), input.slice_id(addr)))
        })
    } else {
        None
    };

    let top_info: Option<(&Macroblock, u8, u16)> = if filter_top {
        input.neighbor_addr(mb_addr, MbNeighborName::B).and_then(|addr| {
            input.get_mb(addr).map(|p_mb| (p_mb, get_qp(p_mb), input.slice_id(addr)))
        })
    } else {
        None
    };

    let (bs_vert, bs_horz) = compute_bs_arrays(
        input,
        mb,
        q_slice_id,
        left_info.map(|(m, _, sid)| (m, sid)),
        top_info.map(|(m, _, sid)| (m, sid)),
        transform_8x8,
    );

    let has_nonzero_bs = |bs: &[u8; 4]| bs[0] | bs[1] | bs[2] | bs[3] != 0;

    // Section 8.7, step 3.a/3.b — luma vertical edges
    if let Some((_, p_qp, _)) = left_info {
        if has_nonzero_bs(&bs_vert[0]) {
            filter_luma_edge(
                frame,
                mb_xy,
                0,
                true,
                &bs_vert[0],
                p_qp,
                q_qp,
                alpha_offset,
                beta_offset,
            );
        }
    }
    if !transform_8x8 {
        for edge in 1..4 {
            if has_nonzero_bs(&bs_vert[edge]) {
                filter_luma_edge(
                    frame,
                    mb_xy,
                    edge,
                    true,
                    &bs_vert[edge],
                    q_qp,
                    q_qp,
                    alpha_offset,
                    beta_offset,
                );
            }
        }
    } else if has_nonzero_bs(&bs_vert[2]) {
        filter_luma_edge(
            frame,
            mb_xy,
            2,
            true,
            &bs_vert[2],
            q_qp,
            q_qp,
            alpha_offset,
            beta_offset,
        );
    }

    // Section 8.7, step 3.c/3.d — luma horizontal edges
    if let Some((_, p_qp, _)) = top_info {
        if has_nonzero_bs(&bs_horz[0]) {
            filter_luma_edge(
                frame,
                mb_xy,
                0,
                false,
                &bs_horz[0],
                p_qp,
                q_qp,
                alpha_offset,
                beta_offset,
            );
        }
    }
    if !transform_8x8 {
        for edge in 1..4 {
            if has_nonzero_bs(&bs_horz[edge]) {
                filter_luma_edge(
                    frame,
                    mb_xy,
                    edge,
                    false,
                    &bs_horz[edge],
                    q_qp,
                    q_qp,
                    alpha_offset,
                    beta_offset,
                );
            }
        }
    } else if has_nonzero_bs(&bs_horz[2]) {
        filter_luma_edge(
            frame,
            mb_xy,
            2,
            false,
            &bs_horz[2],
            q_qp,
            q_qp,
            alpha_offset,
            beta_offset,
        );
    }

    // Section 8.7, step 3 for chroma (4:2:0)
    // Chroma edge 0 reuses luma edge 0 BS, chroma edge 1 reuses luma edge 2 BS
    if let Some((_, p_qp, _)) = left_info {
        if has_nonzero_bs(&bs_vert[0]) {
            filter_chroma_edge(
                input.pps,
                frame,
                mb_xy,
                0,
                true,
                &bs_vert[0],
                p_qp,
                q_qp,
                alpha_offset,
                beta_offset,
            );
        }
    }
    if has_nonzero_bs(&bs_vert[2]) {
        filter_chroma_edge(
            input.pps,
            frame,
            mb_xy,
            1,
            true,
            &bs_vert[2],
            q_qp,
            q_qp,
            alpha_offset,
            beta_offset,
        );
    }
    if let Some((_, p_qp, _)) = top_info {
        if has_nonzero_bs(&bs_horz[0]) {
            filter_chroma_edge(
                input.pps,
                frame,
                mb_xy,
                0,
                false,
                &bs_horz[0],
                p_qp,
                q_qp,
                alpha_offset,
                beta_offset,
            );
        }
    }
    if has_nonzero_bs(&bs_horz[2]) {
        filter_chroma_edge(
            input.pps,
            frame,
            mb_xy,
            1,
            false,
            &bs_horz[2],
            q_qp,
            q_qp,
            alpha_offset,
            beta_offset,
        );
    }
}

/// Section 8.7, steps 2.c/2.d — determine whether to filter an MB boundary edge.
/// `neighbor` is `MbNeighborName::A` for the left edge, `MbNeighborName::B` for the top edge.
fn should_filter_edge(
    input: &PictureDeblockInput,
    mb_addr: MbAddr,
    neighbor: MbNeighborName,
) -> bool {
    let mb_width = input.pic_width_in_mbs;
    let is_at_boundary = match neighbor {
        MbNeighborName::A => (mb_addr as usize) % mb_width == 0,
        MbNeighborName::B => (mb_addr as usize) / mb_width == 0,
        _ => return false,
    };
    if is_at_boundary {
        return false;
    }

    let disable_idc = input.deblock_params(mb_addr).idc;
    if disable_idc == DeblockingFilterIdc::Off {
        return false;
    }
    if disable_idc == DeblockingFilterIdc::OnExceptSliceBounds {
        let neighbor_addr = match input.neighbor_addr(mb_addr, neighbor) {
            Some(addr) if input.get_mb(addr).is_some() => addr,
            _ => return false,
        };
        if input.slice_id(mb_addr) != input.slice_id(neighbor_addr) {
            return false;
        }
    }
    true
}

/// Sections 8.7.1/8.7.2 — Filtering process for a single luma block edge.
fn filter_luma_edge(
    frame: &mut VideoFrame,
    mb_xy: Point,
    edge_idx: usize,
    is_vertical: bool,
    bs_array: &[u8; 4],
    p_qp: u8,
    q_qp: u8,
    alpha_offset: i32,
    beta_offset: i32,
) {
    let plane = &mut frame.planes[ColorPlane::Y as usize];
    let stride = plane.cfg.stride;
    let data = plane.data_origin_mut();

    let thresh = FilterThresholds::from_qp(p_qp, q_qp, alpha_offset, beta_offset);
    let alpha = thresh.alpha;
    let beta = thresh.beta;
    let index_a = thresh.index_a;

    // edge_step: distance between consecutive samples along the edge
    // perp_step: distance from q0 toward p0 (perpendicular to the edge)
    let (edge_step, perp_step, base_idx) = if is_vertical {
        (stride, 1usize, mb_xy.y as usize * stride + mb_xy.x as usize + edge_idx * 4)
    } else {
        (1, stride, (mb_xy.y as usize + edge_idx * 4) * stride + mb_xy.x as usize)
    };

    // Pre-compute perpendicular step multiples. The 8-sample perpendicular
    // window for one pixel covers offsets [0..=7*perp_step] — laid out as
    // p3, p2, p1, p0, q0, q1, q2, q3 at strides perp_step apart.
    let s1 = perp_step;
    let s2 = perp_step * 2;
    let s3 = perp_step * 3;
    let s4 = perp_step * 4;

    let mut q0_idx = base_idx;

    // Restructure as 4 4-pixel blocks: bs lookup, tc0 derivation, and the
    // weak/strong dispatch run once per block instead of once per pixel.
    for blk in 0..4 {
        let bs = bs_array[blk];
        if bs == BS_NONE {
            q0_idx += 4 * edge_step;
            continue;
        }

        let strong = bs >= BS_STRONG;
        let tc0 = if !strong {
            TC0_TABLE[(bs - 1) as usize][index_a] as i32 // Table 8-17
        } else {
            0
        };

        for _ in 0..4 {
            // Slice the 8-sample perpendicular window once per pixel — one
            // pair of bounds checks for the whole window. All in-window reads
            // and writes below use offsets `N * perp_step` for N in 0..=7,
            // which the optimizer can prove are < `win.len() = 7*perp_step+1`
            // and elide the per-access check.
            let win = &mut data[q0_idx - s4..q0_idx + s3 + 1];

            // Layout within `win`:
            //   off=0      → p3        off=s4     → q0
            //   off=s1     → p2        off=s4+s1  → q1
            //   off=s2     → p1        off=s4+s2  → q2
            //   off=s3     → p0        off=s4+s3  → q3
            let p0 = win[s3] as i32;
            let q0 = win[s4] as i32;
            let p1 = win[s2] as i32;
            let q1 = win[s4 + s1] as i32;

            // Equation 8-460: filter condition
            if (p0 - q0).abs() < alpha
                && (p1 - p0).abs() < beta
                && (q1 - q0).abs() < beta
            {
                let p2 = win[s1] as i32;
                let q2 = win[s4 + s2] as i32;
                let ap = (p2 - p0).abs();
                let aq = (q2 - q0).abs();
                let ap_lt_beta = ap < beta;
                let aq_lt_beta = aq < beta;

                if !strong {
                    // Section 8.7.2.3 — weak filter (bS < 4)
                    let tc = tc0 + ap_lt_beta as i32 + aq_lt_beta as i32; // Eq 8-465

                    let delta = (((q0 - p0) << 2) + (p1 - q1) + 4) >> 3; // Eq 8-467
                    let delta_c = delta.clamp(-tc, tc);

                    win[s3] = (p0 + delta_c).clamp(0, 255) as u8; // Eq 8-468: p0'
                    win[s4] = (q0 - delta_c).clamp(0, 255) as u8; // Eq 8-469: q0'

                    if ap_lt_beta {
                        // Eq 8-470: p1'
                        let d = (p2 + ((p0 + q0 + 1) >> 1) - (p1 << 1)) >> 1;
                        win[s2] = (p1 + d.clamp(-tc0, tc0)).clamp(0, 255) as u8;
                    }
                    if aq_lt_beta {
                        // Eq 8-472: q1'
                        let d = (q2 + ((p0 + q0 + 1) >> 1) - (q1 << 1)) >> 1;
                        win[s4 + s1] = (q1 + d.clamp(-tc0, tc0)).clamp(0, 255) as u8;
                    }
                } else {
                    // Section 8.7.2.4 — strong filter (bS == 4)
                    let small_diff = (p0 - q0).abs() < ((alpha >> 2) + 2); // Eq 8-476

                    // p-side: Equations 8-477..8-479 (strong) or 8-480 (weak fallback)
                    if ap_lt_beta && small_diff {
                        let p3 = win[0] as i32;
                        win[s3] = ((p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >> 3)
                            .clamp(0, 255) as u8;
                        win[s2] = ((p2 + p1 + p0 + q0 + 2) >> 2).clamp(0, 255) as u8;
                        win[s1] = ((2 * p3 + 3 * p2 + p1 + p0 + q0 + 4) >> 3)
                            .clamp(0, 255) as u8;
                    } else {
                        win[s3] = ((2 * p1 + p0 + q1 + 2) >> 2).clamp(0, 255) as u8;
                    }

                    // q-side: Equations 8-484..8-486 (strong) or 8-487 (weak fallback)
                    if aq_lt_beta && small_diff {
                        let q3 = win[s4 + s3] as i32;
                        win[s4] = ((p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4) >> 3)
                            .clamp(0, 255) as u8;
                        win[s4 + s1] =
                            ((p0 + q0 + q1 + q2 + 2) >> 2).clamp(0, 255) as u8;
                        win[s4 + s2] = ((2 * q3 + 3 * q2 + q1 + q0 + p0 + 4) >> 3)
                            .clamp(0, 255) as u8;
                    } else {
                        win[s4] = ((2 * q1 + q0 + p1 + 2) >> 2).clamp(0, 255) as u8;
                    }
                }
            }

            q0_idx += edge_step;
        }
    }
}

/// Sections 8.7.1/8.7.2 — Filtering process for a single chroma block edge (4:2:0).
#[allow(clippy::too_many_arguments)]
fn filter_chroma_edge(
    pps: &PicParameterSet,
    frame: &mut VideoFrame,
    mb_xy: Point,
    edge_idx: usize,
    is_vertical: bool,
    bs_array: &[u8; 4],
    p_qp: u8,
    q_qp: u8,
    alpha_offset: i32,
    beta_offset: i32,
) {
    let chroma_shift_x = 1u32; // 4:2:0
    let chroma_shift_y = 1u32;

    // Section 8.7.2.2 — chroma threshold derivation using QPc from Table 8-15
    let chroma_thresh = [ColorPlane::Cb, ColorPlane::Cr].map(|plane_idx| {
        let qp_index_offset = pps.get_chroma_qp_index_offset(plane_idx);
        let qp_p_c = get_chroma_qp(p_qp as i32, qp_index_offset, 0) as u8;
        let qp_q_c = get_chroma_qp(q_qp as i32, qp_index_offset, 0) as u8;
        FilterThresholds::from_qp(qp_p_c, qp_q_c, alpha_offset, beta_offset)
    });

    for (pidx, &plane_idx) in [ColorPlane::Cb, ColorPlane::Cr].iter().enumerate() {
        let alpha = chroma_thresh[pidx].alpha;
        let beta = chroma_thresh[pidx].beta;
        let index_a = chroma_thresh[pidx].index_a;

        let plane = &mut frame.planes[plane_idx as usize];
        let stride = plane.cfg.stride;
        let data = plane.data_origin_mut();

        let (edge_step, perp_step, base_idx) = if is_vertical {
            let abs_x = (mb_xy.x >> chroma_shift_x) as usize + edge_idx * 4;
            let abs_y = (mb_xy.y >> chroma_shift_y) as usize;
            (stride, 1usize, abs_y * stride + abs_x)
        } else {
            let abs_x = (mb_xy.x >> chroma_shift_x) as usize;
            let abs_y = (mb_xy.y >> chroma_shift_y) as usize + edge_idx * 4;
            (1, stride, abs_y * stride + abs_x)
        };

        let s1 = perp_step;
        let s2 = perp_step * 2;

        let mut q0_idx = base_idx;

        // Restructure as 4 2-pixel blocks. Each chroma 4:2:0 edge has 8 samples
        // and the bs array has 4 entries (one per 2 chroma samples = 4 luma).
        for blk in 0..4 {
            let bs = bs_array[blk];
            if bs == BS_NONE {
                q0_idx += 2 * edge_step;
                continue;
            }

            let strong = bs >= BS_STRONG;
            // Section 8.7.2.3 with chromaEdgeFlag = 1.
            // Equation 8-466: tc = tc0 + 1 for chroma.
            let tc = if !strong {
                TC0_TABLE[(bs - 1) as usize][index_a] as i32 + 1
            } else {
                0
            };

            for _ in 0..2 {
                // Per-pixel 4-sample perpendicular window: p1, p0, q0, q1 at
                // strides perp_step. One pair of bounds checks for the whole
                // window; the constant-stride offsets below LLVM can prove
                // safe and elide.
                let win = &mut data[q0_idx - s2..q0_idx + s1 + 1];
                // Layout: win[0]=p1, win[s1]=p0, win[s2]=q0, win[s2+s1]=q1
                let p0 = win[s1] as i32;
                let q0 = win[s2] as i32;
                let p1 = win[0] as i32;
                let q1 = win[s2 + s1] as i32;

                // Equation 8-460: filter condition
                if (p0 - q0).abs() < alpha
                    && (p1 - p0).abs() < beta
                    && (q1 - q0).abs() < beta
                {
                    let (p0_new, q0_new) = if !strong {
                        // Equation 8-467: delta
                        let delta = (((q0 - p0) << 2) + (p1 - q1) + 4) >> 3;
                        let delta_c = delta.clamp(-tc, tc);
                        // Equations 8-468, 8-469: p0', q0'
                        (
                            (p0 + delta_c).clamp(0, 255) as u8,
                            (q0 - delta_c).clamp(0, 255) as u8,
                        )
                    } else {
                        // Section 8.7.2.4 with chromaStyleFilteringFlag = 1.
                        // Equations 8-480, 8-487: p0', q0'
                        (
                            ((2 * p1 + p0 + q1 + 2) >> 2).clamp(0, 255) as u8,
                            ((2 * q1 + q0 + p1 + 2) >> 2).clamp(0, 255) as u8,
                        )
                    };

                    win[s1] = p0_new;
                    win[s2] = q0_new;
                }

                q0_idx += edge_step;
            }
        }
    }
}

/// Section 8.7.2.1 — Precompute boundary strength arrays for all edges of a macroblock.
/// Returns `(bs_vert, bs_horz)` where each is `[[u8; 4]; 4]` indexed by `[edge_idx][block_idx]`.
fn compute_bs_arrays(
    input: &PictureDeblockInput,
    mb: &Macroblock,
    q_slice_id: u16,
    left: Option<(&Macroblock, u16)>,
    top: Option<(&Macroblock, u16)>,
    transform_8x8: bool,
) -> ([[u8; 4]; 4], [[u8; 4]; 4]) {
    let mut bs_vert = [[BS_NONE; 4]; 4];
    let mut bs_horz = [[BS_NONE; 4]; 4];

    let (q_l0, q_l1) = input.ref_pocs(q_slice_id);

    // External edges (MB boundary) — use neighbor MB as p
    if let Some((p_mb, p_slice_id)) = left {
        let (p_l0, p_l1) = input.ref_pocs(p_slice_id);
        for b in 0..4 {
            bs_vert[0][b] = get_bs(mb, p_mb, q_l0, q_l1, p_l0, p_l1, 0, b, true);
        }
    }
    if let Some((p_mb, p_slice_id)) = top {
        let (p_l0, p_l1) = input.ref_pocs(p_slice_id);
        for b in 0..4 {
            bs_horz[0][b] = get_bs(mb, p_mb, q_l0, q_l1, p_l0, p_l1, 0, b, false);
        }
    }

    // Internal edges — p and q are both within this MB (same slice → same POCs).
    if !transform_8x8 {
        for edge in 1..4 {
            for b in 0..4 {
                bs_vert[edge][b] = get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, edge, b, true);
                bs_horz[edge][b] = get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, edge, b, false);
            }
        }
    } else {
        // 8x8 transform: only edge 2 (at the 8-sample boundary)
        for b in 0..4 {
            bs_vert[2][b] = get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, 2, b, true);
            bs_horz[2][b] = get_bs(mb, mb, q_l0, q_l1, q_l0, q_l1, 2, b, false);
        }
    }

    (bs_vert, bs_horz)
}

#[inline(always)]
fn has_nonzero_coeffs(mb: &Macroblock, blk_idx: usize) -> bool {
    // Section 8.7.2.1: bS=2 is derived from the transform block containing the
    // sample, whose size depends on transform_size_8x8_flag. For 8x8 transforms
    // the "block" is the enclosing 8x8 group — its four 4x4 sub-sections share a
    // single coded status for deblocking purposes.
    use super::residual::LumaResidual;
    let Some(res) = mb.get_residual() else { return false };
    match &res.luma {
        LumaResidual::Intra16x16 { dc, ac_nc, .. } => ac_nc[blk_idx] != 0 || dc[blk_idx] != 0,
        LumaResidual::Block8x8 { levels, .. } => {
            let i8x8 = blk_idx / 4;
            levels[i8x8].0.iter().any(|&v| v != 0)
        }
        LumaResidual::Block4x4 { nc, .. } => nc[blk_idx] != 0,
        LumaResidual::Empty => false,
    }
}

#[inline(always)]
fn get_partition(mb: &Macroblock, y: usize, x: usize) -> Option<super::macroblock::PartitionInfo> {
    match mb {
        Macroblock::P(m) => Some(m.motion.partitions[y][x]),
        Macroblock::B(m) => Some(m.motion.partitions[y][x]),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn get_bs(
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
    let (q_y, q_x, p_y, p_x) = if is_vertical {
        if edge_idx == 0 {
            (block_idx, edge_idx, block_idx, 3)
        } else {
            (block_idx, edge_idx, block_idx, edge_idx - 1)
        }
    } else if edge_idx == 0 {
        (edge_idx, block_idx, 3, block_idx)
    } else {
        (edge_idx, block_idx, edge_idx - 1, block_idx)
    };

    if mb_p.is_intra() || mb_q.is_intra() {
        if edge_idx == 0 {
            return BS_STRONG;
        }
        return BS_INTRA;
    }

    let blk_q_idx = super::residual::scan_4x4(q_y, q_x);
    let blk_p_idx = super::residual::scan_4x4(p_y, p_x);

    if has_nonzero_coeffs(mb_p, blk_p_idx) || has_nonzero_coeffs(mb_q, blk_q_idx) {
        return BS_CODED;
    }

    let p_part = get_partition(mb_p, p_y, p_x);
    let q_part = get_partition(mb_q, q_y, q_x);

    match (p_part, q_part) {
        (Some(pp), Some(qq)) => {
            // P-slice context for q (refPicList1 is empty in P-slices). If
            // p comes from a B-slice partition that uses BiPred or Pred_L1,
            // only its L0 entry is consulted here — the q-side P-slice rules
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
    use crate::h264::macroblock::IMb;

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

    fn make_input<'a>(
        sps: &'a SequenceParameterSet,
        pps: &'a PicParameterSet,
        macroblocks: &'a [Option<Macroblock>],
        mb_slice_id: &'a [u16],
        slice_deblock: &'a [SliceDeblockParams],
        slice_ref_pocs: &'a [(Vec<i32>, Vec<i32>)],
    ) -> PictureDeblockInput<'a> {
        PictureDeblockInput {
            sps,
            pps,
            macroblocks,
            mb_slice_id,
            slice_deblock,
            slice_ref_pocs,
            pic_width_in_mbs: W,
            pic_height_in_mbs: H,
        }
    }

    #[test]
    fn picture_boundary_edges_never_filtered() {
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::On)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input =
            make_input(&sps, &pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 0 is at the top-left corner — both A (left) and B (top) are out of picture.
        assert!(!should_filter_edge(&input, 0, MbNeighborName::A));
        assert!(!should_filter_edge(&input, 0, MbNeighborName::B));
        // MB 4 is at the left edge — A is out of picture.
        assert!(!should_filter_edge(&input, 4, MbNeighborName::A));
        // MB 3 is at the top edge — B is out of picture.
        assert!(!should_filter_edge(&input, 3, MbNeighborName::B));
    }

    #[test]
    fn single_slice_idc_on_filters_internal_edges() {
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::On)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input =
            make_input(&sps, &pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 5 has both an A (MB 4) and B (MB 1) neighbor inside the picture.
        assert!(should_filter_edge(&input, 5, MbNeighborName::A));
        assert!(should_filter_edge(&input, 5, MbNeighborName::B));
    }

    #[test]
    fn single_slice_idc_off_filters_nothing() {
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::Off)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input =
            make_input(&sps, &pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        assert!(!should_filter_edge(&input, 5, MbNeighborName::A));
        assert!(!should_filter_edge(&input, 5, MbNeighborName::B));
    }

    #[test]
    fn single_slice_idc_2_behaves_like_idc_on() {
        // OnExceptSliceBounds with one slice has no slice boundaries to skip,
        // so it should filter every internal edge identically to On.
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0u16; W * H];
        let slice_deblock = [deblock(DeblockingFilterIdc::OnExceptSliceBounds)];
        let slice_ref_pocs = [(vec![], vec![])];
        let input =
            make_input(&sps, &pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        assert!(should_filter_edge(&input, 5, MbNeighborName::A));
        assert!(should_filter_edge(&input, 5, MbNeighborName::B));
    }

    #[test]
    fn two_slices_idc_on_still_filters_cross_slice_edges() {
        // disable_deblocking_filter_idc=0 (On): cross-slice edges DO filter.
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        // Top row → slice 0, bottom row → slice 1.
        let mb_slice_id = vec![0, 0, 0, 0, 1, 1, 1, 1];
        let slice_deblock = [
            deblock(DeblockingFilterIdc::On),
            deblock(DeblockingFilterIdc::On),
        ];
        let slice_ref_pocs = [(vec![], vec![]), (vec![], vec![])];
        let input =
            make_input(&sps, &pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 5 is in slice 1; its B neighbor is MB 1 (slice 0) — cross-slice.
        // Still filtered because idc=On disregards slice boundaries.
        assert!(should_filter_edge(&input, 5, MbNeighborName::B));
        // MB 5's A neighbor is MB 4 (slice 1) — same slice, also filtered.
        assert!(should_filter_edge(&input, 5, MbNeighborName::A));
    }

    #[test]
    fn two_slices_idc_2_suppresses_cross_slice_edges() {
        // disable_deblocking_filter_idc=2: cross-slice edges suppressed,
        // intra-slice edges filtered.
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0, 0, 0, 0, 1, 1, 1, 1];
        let slice_deblock = [
            deblock(DeblockingFilterIdc::OnExceptSliceBounds),
            deblock(DeblockingFilterIdc::OnExceptSliceBounds),
        ];
        let slice_ref_pocs = [(vec![], vec![]), (vec![], vec![])];
        let input =
            make_input(&sps, &pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // MB 4: B neighbor is MB 0 (slice 0) — cross-slice → suppressed.
        assert!(!should_filter_edge(&input, 4, MbNeighborName::B));
        // MB 5: B neighbor is MB 1 (slice 0) — cross-slice → suppressed.
        assert!(!should_filter_edge(&input, 5, MbNeighborName::B));
        // MB 5: A neighbor is MB 4 (slice 1) — same slice → filtered.
        assert!(should_filter_edge(&input, 5, MbNeighborName::A));
        // MB 1: A neighbor is MB 0 (slice 0) — same slice → filtered.
        assert!(should_filter_edge(&input, 1, MbNeighborName::A));
    }

    #[test]
    fn two_slices_with_mixed_idcs_uses_q_block_idc() {
        // Per spec 8.7, the deblock IDC for an edge is taken from the slice
        // containing the q-block (lower/right MB). Verify by giving slice 0
        // idc=Off and slice 1 idc=On, then checking edges where the q is in
        // slice 1.
        let sps = SequenceParameterSet::default();
        let pps = PicParameterSet::default();
        let mbs = dummy_mbs();
        let mb_slice_id = vec![0, 0, 0, 0, 1, 1, 1, 1];
        let slice_deblock = [
            deblock(DeblockingFilterIdc::Off),
            deblock(DeblockingFilterIdc::On),
        ];
        let slice_ref_pocs = [(vec![], vec![]), (vec![], vec![])];
        let input =
            make_input(&sps, &pps, &mbs, &mb_slice_id, &slice_deblock, &slice_ref_pocs);

        // q = MB 5 (slice 1, idc=On) → edges filter regardless of p's slice.
        assert!(should_filter_edge(&input, 5, MbNeighborName::A));
        assert!(should_filter_edge(&input, 5, MbNeighborName::B));
        // q = MB 1 (slice 0, idc=Off) → no filtering.
        assert!(!should_filter_edge(&input, 1, MbNeighborName::A));
    }
}
