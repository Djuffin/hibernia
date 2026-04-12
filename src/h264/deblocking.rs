use super::decoder::{get_chroma_qp, VideoFrame};
use super::macroblock::{
    get_neighbor_mbs, IMb, Macroblock, MbAddr, MbNeighborName, MbPredictionMode,
};
use super::slice::{DeblockingFilterIdc, Slice};
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
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 7, 8, 8, 10, 11, 12, 13, 15, 17,
    ],
    // bS = 3 (BS_INTRA)
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
        2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 7, 8, 9, 10, 11, 13, 14, 16, 18, 20, 23, 25,
    ],
];

/// Section 8.7 — Deblocking filter process
pub fn filter_slice(slice: &mut Slice, frame: &mut VideoFrame) {
    if slice.header.deblocking_filter_idc == DeblockingFilterIdc::Off {
        return;
    }

    let mb_width = slice.sps.pic_width_in_mbs();
    let mb_height = (slice.sps.pic_height_in_map_units_minus1 + 1) as usize;
    let total_mbs = mb_width * mb_height;

    for mb_addr in 0..total_mbs {
        filter_macroblock(slice, frame, mb_addr as u32);
    }
}

/// Section 8.7, steps 2–4 — Filter all edges of a single macroblock.
/// BS values are precomputed once per MB and reused across luma and chroma
/// to avoid redundant derivation (Section 8.7.2.1).
fn filter_macroblock(slice: &Slice, frame: &mut VideoFrame, mb_addr: MbAddr) {
    let mb = match slice.get_mb(mb_addr) {
        Some(mb) => mb,
        None => return,
    };

    let mb_xy = slice.get_mb_location(mb_addr);
    let q_qp = get_qp(mb);

    // Section 8.7, step 2.c / 2.d — determine filterLeftMbEdgeFlag / filterTopMbEdgeFlag
    let filter_left = should_filter_left_edge(slice, mb_addr);
    let filter_top = should_filter_top_edge(slice, mb_addr);

    let transform_8x8 = match mb {
        Macroblock::I(m) => m.transform_size_8x8_flag,
        Macroblock::P(m) => m.transform_size_8x8_flag,
        _ => false,
    };

    // Section 8.7, step 2.a / 2.b — locate neighbor macroblocks A (left) and B (top)
    let left_info: Option<(&Macroblock, u8)> = if filter_left {
        slice
            .get_mb_neighbor(mb_addr, MbNeighborName::A)
            .map(|p_mb| (p_mb, get_qp(p_mb)))
    } else {
        None
    };

    let top_info: Option<(&Macroblock, u8)> = if filter_top {
        slice
            .get_mb_neighbor(mb_addr, MbNeighborName::B)
            .map(|p_mb| (p_mb, get_qp(p_mb)))
    } else {
        None
    };

    // Section 8.7.2.1 — precompute boundary strength for all edges: [edge_idx][block_idx]
    let mut bs_vert = [[BS_NONE; 4]; 4];
    let mut bs_horz = [[BS_NONE; 4]; 4];

    if let Some((p_mb, _)) = left_info {
        for b in 0..4 {
            bs_vert[0][b] = get_bs(slice, mb, p_mb, 0, b, true);
        }
    }
    if let Some((p_mb, _)) = top_info {
        for b in 0..4 {
            bs_horz[0][b] = get_bs(slice, mb, p_mb, 0, b, false);
        }
    }
    if !transform_8x8 {
        for edge in 1..4 {
            for b in 0..4 {
                bs_vert[edge][b] = get_bs(slice, mb, mb, edge, b, true);
                bs_horz[edge][b] = get_bs(slice, mb, mb, edge, b, false);
            }
        }
    } else {
        for b in 0..4 {
            bs_vert[2][b] = get_bs(slice, mb, mb, 2, b, true);
            bs_horz[2][b] = get_bs(slice, mb, mb, 2, b, false);
        }
    }

    let has_nonzero_bs = |bs: &[u8; 4]| bs[0] | bs[1] | bs[2] | bs[3] != 0;

    // Section 8.7, step 3 — luma vertical edges
    if let Some((_, p_qp)) = left_info {
        if has_nonzero_bs(&bs_vert[0]) {
            filter_luma_edge(slice, frame, mb_xy, 0, true, &bs_vert[0], p_qp, q_qp);
        }
    }
    if !transform_8x8 {
        for edge in 1..4 {
            if has_nonzero_bs(&bs_vert[edge]) {
                filter_luma_edge(slice, frame, mb_xy, edge, true, &bs_vert[edge], q_qp, q_qp);
            }
        }
    } else if has_nonzero_bs(&bs_vert[2]) {
        filter_luma_edge(slice, frame, mb_xy, 2, true, &bs_vert[2], q_qp, q_qp);
    }

    // Section 8.7, step 4 — luma horizontal edges
    if let Some((_, p_qp)) = top_info {
        if has_nonzero_bs(&bs_horz[0]) {
            filter_luma_edge(slice, frame, mb_xy, 0, false, &bs_horz[0], p_qp, q_qp);
        }
    }
    if !transform_8x8 {
        for edge in 1..4 {
            if has_nonzero_bs(&bs_horz[edge]) {
                filter_luma_edge(slice, frame, mb_xy, edge, false, &bs_horz[edge], q_qp, q_qp);
            }
        }
    } else if has_nonzero_bs(&bs_horz[2]) {
        filter_luma_edge(slice, frame, mb_xy, 2, false, &bs_horz[2], q_qp, q_qp);
    }

    // Section 8.7, steps 3–4 for chroma (4:2:0)
    // Chroma edge 0 reuses luma edge 0 BS, chroma edge 1 reuses luma edge 2 BS
    if let Some((_, p_qp)) = left_info {
        if has_nonzero_bs(&bs_vert[0]) {
            filter_chroma_edge(slice, frame, mb_xy, 0, true, &bs_vert[0], p_qp, q_qp);
        }
    }
    if has_nonzero_bs(&bs_vert[2]) {
        filter_chroma_edge(slice, frame, mb_xy, 1, true, &bs_vert[2], q_qp, q_qp);
    }
    if let Some((_, p_qp)) = top_info {
        if has_nonzero_bs(&bs_horz[0]) {
            filter_chroma_edge(slice, frame, mb_xy, 0, false, &bs_horz[0], p_qp, q_qp);
        }
    }
    if has_nonzero_bs(&bs_horz[2]) {
        filter_chroma_edge(slice, frame, mb_xy, 1, false, &bs_horz[2], q_qp, q_qp);
    }
}

fn should_filter_left_edge(slice: &Slice, mb_addr: MbAddr) -> bool {
    let mb_x = (mb_addr as usize) % slice.sps.pic_width_in_mbs();
    if mb_x == 0 {
        return false;
    }
    let disable_idc = slice.header.deblocking_filter_idc;
    if disable_idc == DeblockingFilterIdc::Off {
        return false;
    }
    if disable_idc == DeblockingFilterIdc::OnExceptSliceBounds {
        // Section 8.7, Step 2.c:
        // If disable_deblocking_filter_idc is equal to 2 and the macroblock mbAddrA is not available.

        if !slice.has_mb_neighbor(mb_addr, MbNeighborName::A) {
            return false;
        }

        let neighbor_a_addr = get_neighbor_mbs(
            slice.sps.pic_width_in_mbs() as u32,
            slice.header.first_mb_in_slice,
            mb_addr,
            MbNeighborName::A,
        );

        // Check if neighbor is in the current slice
        if let Some(addr) = neighbor_a_addr {
            if slice.get_mb(addr).is_none() {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn should_filter_top_edge(slice: &Slice, mb_addr: MbAddr) -> bool {
    let mb_y = (mb_addr as usize) / slice.sps.pic_width_in_mbs();
    if mb_y == 0 {
        return false;
    }
    let disable_idc = slice.header.deblocking_filter_idc;
    if disable_idc == DeblockingFilterIdc::Off {
        return false;
    }
    if disable_idc == DeblockingFilterIdc::OnExceptSliceBounds {
        // Section 8.7, Step 2.d:
        // If disable_deblocking_filter_idc is equal to 2 and the macroblock mbAddrB is not available.

        if !slice.has_mb_neighbor(mb_addr, MbNeighborName::B) {
            return false;
        }
        let neighbor_b_addr = get_neighbor_mbs(
            slice.sps.pic_width_in_mbs() as u32,
            slice.header.first_mb_in_slice,
            mb_addr,
            MbNeighborName::B,
        );

        // Check if neighbor is in the current slice
        if let Some(addr) = neighbor_b_addr {
            if slice.get_mb(addr).is_none() {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

/// Section 8.7.2 — Filtering process for a single luma block edge.
fn filter_luma_edge(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_xy: Point,
    edge_idx: usize,
    is_vertical: bool,
    bs_array: &[u8; 4],
    p_qp: u8,
    q_qp: u8,
) {
    let plane = &mut frame.planes[ColorPlane::Y as usize];
    let stride = plane.cfg.stride;
    let data = plane.data_origin_mut();

    // Section 8.7.2.2 — threshold derivation
    // Equation 8-453: qPav, Equation 8-454: indexA, Equation 8-455: indexB
    let qp_av = (p_qp as i32 + q_qp as i32 + 1) >> 1;
    let index_a = (qp_av + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51) as usize;
    let index_b = (qp_av + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51) as usize;
    // Table 8-16: alpha' and beta' lookup
    let alpha = ALPHA_TABLE[index_a] as i32;
    let beta = BETA_TABLE[index_b] as i32;

    // step: stride between consecutive samples along the edge
    // d: stride perpendicular to the edge (offset direction for p/q neighbors)
    let (step, d, base_idx) = if is_vertical {
        (
            stride,
            1usize,
            mb_xy.y as usize * stride + mb_xy.x as usize + edge_idx * 4,
        )
    } else {
        (
            1,
            stride,
            (mb_xy.y as usize + edge_idx * 4) * stride + mb_xy.x as usize,
        )
    };

    let mut q0_idx = base_idx;

    for k in 0..16 {
        let bs = bs_array[k / 4];
        if bs == BS_NONE {
            q0_idx += step;
            continue;
        }

        let p0_idx = q0_idx - d;
        let p1_idx = q0_idx - 2 * d;
        let q1_idx = q0_idx + d;

        let p0 = data[p0_idx] as i32;
        let q0 = data[q0_idx] as i32;
        let p1 = data[p1_idx] as i32;
        let q1 = data[q1_idx] as i32;

        // Equation 8-460: filter condition
        if (p0 - q0).abs() < alpha && (p1 - p0).abs() < beta && (q1 - q0).abs() < beta {
            let p2_idx = q0_idx - 3 * d;
            let q2_idx = q0_idx + 2 * d;
            let p2 = data[p2_idx] as i32;
            let q2 = data[q2_idx] as i32;

            if bs < BS_STRONG {
                // Section 8.7.2.3 — filtering process for edges with bS < 4
                // Table 8-17: tc0 lookup
                let tc0 = TC0_TABLE[(bs - 1) as usize][index_a];
                let mut tc = tc0 as i32;
                let ap = (p2 - p0).abs();
                let aq = (q2 - q0).abs();
                // Equation 8-465: tc adjustment
                if ap < beta {
                    tc += 1;
                }
                if aq < beta {
                    tc += 1;
                }

                // Equation 8-467: delta
                let delta = (((q0 - p0) << 2) + (p1 - q1) + 4) >> 3;
                let delta_c = delta.clamp(-tc, tc);

                // Equations 8-468, 8-469: p0', q0'
                data[p0_idx] = (p0 + delta_c).clamp(0, 255) as u8;
                data[q0_idx] = (q0 - delta_c).clamp(0, 255) as u8;

                if ap < beta {
                    // Equation 8-470: p1'
                    let delta_p1 = (p2 + ((p0 + q0 + 1) >> 1) - (p1 << 1)) >> 1;
                    data[p1_idx] =
                        (p1 + delta_p1.clamp(-(tc0 as i32), tc0 as i32)).clamp(0, 255) as u8;
                }
                if aq < beta {
                    // Equation 8-472: q1'
                    let delta_q1 = (q2 + ((p0 + q0 + 1) >> 1) - (q1 << 1)) >> 1;
                    data[q1_idx] =
                        (q1 + delta_q1.clamp(-(tc0 as i32), tc0 as i32)).clamp(0, 255) as u8;
                }
            } else {
                // Section 8.7.2.4 — filtering process for edges with bS equal to 4
                let ap = (p2 - p0).abs();
                let aq = (q2 - q0).abs();
                // Equation 8-476: chromaEdgeFlag == 0 condition
                let small_diff = (p0 - q0).abs() < ((alpha >> 2) + 2);

                if ap < beta && small_diff {
                    let p3 = data[q0_idx - 4 * d] as i32;
                    // Equations 8-477, 8-478, 8-479: p0', p1', p2'
                    data[p0_idx] =
                        ((p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >> 3).clamp(0, 255) as u8;
                    data[p1_idx] = ((p2 + p1 + p0 + q0 + 2) >> 2).clamp(0, 255) as u8;
                    data[p2_idx] =
                        ((2 * p3 + 3 * p2 + p1 + p0 + q0 + 4) >> 3).clamp(0, 255) as u8;
                } else {
                    // Equation 8-480: p0' (weak fallback)
                    data[p0_idx] = ((2 * p1 + p0 + q1 + 2) >> 2).clamp(0, 255) as u8;
                }

                if aq < beta && small_diff {
                    let q3 = data[q0_idx + 3 * d] as i32;
                    // Equations 8-484, 8-485, 8-486: q0', q1', q2'
                    data[q0_idx] =
                        ((p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4) >> 3).clamp(0, 255) as u8;
                    data[q1_idx] = ((p0 + q0 + q1 + q2 + 2) >> 2).clamp(0, 255) as u8;
                    data[q2_idx] =
                        ((2 * q3 + 3 * q2 + q1 + q0 + p0 + 4) >> 3).clamp(0, 255) as u8;
                } else {
                    // Equation 8-487: q0' (weak fallback)
                    data[q0_idx] = ((2 * q1 + q0 + p1 + 2) >> 2).clamp(0, 255) as u8;
                }
            }
        }

        q0_idx += step;
    }
}

/// Section 8.7.2 — Filtering process for a single chroma block edge (4:2:0).
fn filter_chroma_edge(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_xy: Point,
    edge_idx: usize,
    is_vertical: bool,
    bs_array: &[u8; 4],
    p_qp: u8,
    q_qp: u8,
) {
    let chroma_shift_x = 1u32; // 4:2:0
    let chroma_shift_y = 1u32;

    struct ChromaParams {
        alpha: i32,
        beta: i32,
        index_a: usize,
    }

    // Section 8.7.2.2 — chroma threshold derivation using QPc from Table 8-15
    let get_chroma_params = |plane_idx: ColorPlane| -> ChromaParams {
        let qp_index_offset = slice.pps.get_chroma_qp_index_offset(plane_idx);
        let qp_p_c = get_chroma_qp(p_qp as i32, qp_index_offset, 0);
        let qp_q_c = get_chroma_qp(q_qp as i32, qp_index_offset, 0);
        // Equation 8-453 (chroma variant)
        let qp_av_c = (qp_p_c + qp_q_c + 1) >> 1;
        // Equations 8-454, 8-455
        let index_a =
            (qp_av_c + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51) as usize;
        let index_b =
            (qp_av_c + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51) as usize;
        ChromaParams {
            alpha: ALPHA_TABLE[index_a] as i32,
            beta: BETA_TABLE[index_b] as i32,
            index_a,
        }
    };

    let params_cb = get_chroma_params(ColorPlane::Cb);
    let params_cr = get_chroma_params(ColorPlane::Cr);
    let chroma_params = [params_cb, params_cr];

    // Process each chroma plane with incremental indexing
    for (pidx, &plane_idx) in [ColorPlane::Cb, ColorPlane::Cr].iter().enumerate() {
        let alpha = chroma_params[pidx].alpha;
        let beta = chroma_params[pidx].beta;
        let index_a = chroma_params[pidx].index_a;

        let plane = &mut frame.planes[plane_idx as usize];
        let stride = plane.cfg.stride;
        let data = plane.data_origin_mut();

        let (step, d, base_idx) = if is_vertical {
            let abs_x = (mb_xy.x >> chroma_shift_x) as usize + edge_idx * 4;
            let abs_y = (mb_xy.y >> chroma_shift_y) as usize;
            (stride, 1usize, abs_y * stride + abs_x)
        } else {
            let abs_x = (mb_xy.x >> chroma_shift_x) as usize;
            let abs_y = (mb_xy.y >> chroma_shift_y) as usize + edge_idx * 4;
            (1, stride, abs_y * stride + abs_x)
        };

        let mut q0_idx = base_idx;

        for k in 0..8 {
            let bs = bs_array[k / 2];
            if bs == BS_NONE {
                q0_idx += step;
                continue;
            }

            let p0_idx = q0_idx - d;
            let p1_idx = q0_idx - 2 * d;
            let q1_idx = q0_idx + d;

            let p0 = data[p0_idx] as i32;
            let q0 = data[q0_idx] as i32;
            let p1 = data[p1_idx] as i32;
            let q1 = data[q1_idx] as i32;

            // Equation 8-460: filter condition
            if (p0 - q0).abs() < alpha && (p1 - p0).abs() < beta && (q1 - q0).abs() < beta {
                let (p0_new, q0_new) = if bs < BS_STRONG {
                    // Section 8.7.2.3 with chromaEdgeFlag = 1
                    let tc0 = TC0_TABLE[(bs - 1) as usize][index_a];
                    // Equation 8-466: tc = tc0 + 1 for chroma
                    let tc = tc0 as i32 + 1;
                    // Equation 8-467: delta
                    let delta = (((q0 - p0) << 2) + (p1 - q1) + 4) >> 3;
                    let delta_c = delta.clamp(-tc, tc);
                    // Equations 8-468, 8-469: p0', q0'
                    ((p0 + delta_c).clamp(0, 255) as u8, (q0 - delta_c).clamp(0, 255) as u8)
                } else {
                    // Section 8.7.2.4 with chromaStyleFilteringFlag = 1
                    // Equations 8-480, 8-487: p0', q0'
                    (
                        ((2 * p1 + p0 + q1 + 2) >> 2).clamp(0, 255) as u8,
                        ((2 * q1 + q0 + p1 + 2) >> 2).clamp(0, 255) as u8,
                    )
                };

                data[p0_idx] = p0_new;
                data[q0_idx] = q0_new;
            }

            q0_idx += step;
        }
    }
}

fn get_bs(
    slice: &Slice,
    mb_q: &Macroblock,
    mb_p: &Macroblock,
    // 0 corresponds to the external edge. 1..3 correspond to internal edges.
    edge_idx: usize,
    block_idx: usize,
    is_vertical: bool,
) -> u8 {
    // Section 8.7.2.1 Derivation process for the luma content dependent boundary filtering strength
    // p samples are in mb_p, q samples are in mb_q

    // Determine 4x4 block indices for p and q.
    // Luma 4x4 blocks are indexed 0..15.

    let (blk_q_idx, blk_p_idx) = if is_vertical {
        // Vertical edge
        let y_blk = block_idx;
        let q_blk_x = edge_idx;
        let q_idx = super::residual::scan_4x4(y_blk, q_blk_x);

        if edge_idx == 0 {
            // p is in MB P, rightmost column (x=3)
            let p_idx = super::residual::scan_4x4(y_blk, 3);
            (q_idx, p_idx)
        } else {
            // p is in MB Q (internal)
            let p_idx = super::residual::scan_4x4(y_blk, edge_idx - 1);
            (q_idx, p_idx)
        }
    } else {
        // Horizontal edge
        let x_blk = block_idx;
        let q_blk_y = edge_idx;
        let q_idx = super::residual::scan_4x4(q_blk_y, x_blk);

        if edge_idx == 0 {
            // p is in MB P, bottom row (y=3)
            let p_idx = super::residual::scan_4x4(3, x_blk);
            (q_idx, p_idx)
        } else {
            let p_idx = super::residual::scan_4x4(edge_idx - 1, x_blk);
            (q_idx, p_idx)
        }
    };

    // Section 8.7.2.1: mixedModeEdgeFlag derivation (MBAFF not yet supported)
    let mixed_mode_edge_flag = false;

    // Condition 1: Intra coding
    if mb_p.is_intra() || mb_q.is_intra() {
        // If edge is a macroblock edge ...
        if edge_idx == 0 {
            return BS_STRONG;
        }
        return BS_INTRA;
    }

    // Condition 2: Non-zero transform coefficients
    // Clause 8.7.2.1: check if the 4x4 luma transform block contains non-zero transform coefficient levels.

    if has_nonzero_coeffs(mb_p, blk_p_idx as u8) || has_nonzero_coeffs(mb_q, blk_q_idx as u8) {
        return BS_CODED;
    }

    // Condition 3: Motion vectors / Reference frames
    if mixed_mode_edge_flag || check_motion_discontinuity(slice, mb_p, blk_p_idx, mb_q, blk_q_idx) {
        return BS_MOTION;
    }

    BS_NONE
}

fn get_qp(mb: &Macroblock) -> u8 {
    match mb {
        Macroblock::I(m) => m.qp,
        Macroblock::P(m) => m.qp,
        Macroblock::B(m) => m.qp,
        Macroblock::PCM(m) => m.qp,
    }
}

/// Clause 8.7.2.1 — "the 4x4 luma transform block … contains non-zero transform coefficient levels."
/// Uses precomputed non-zero coefficient counts (_nc fields) instead of iterating arrays.
fn has_nonzero_coeffs(mb: &Macroblock, blk_idx: u8) -> bool {

    match mb {
        Macroblock::I(m) => {
            if let Some(res) = &m.residual {
                if m.MbPartPredMode(0) == MbPredictionMode::Intra_16x16 {
                    res.ac_level16x16_nc[blk_idx as usize] != 0
                        || res.dc_level16x16[blk_idx as usize] != 0
                } else {
                    res.luma_level4x4_nc[blk_idx as usize] != 0
                }
            } else {
                false
            }
        }
        Macroblock::P(m) => {
            if let Some(res) = &m.residual {
                res.luma_level4x4_nc[blk_idx as usize] != 0
            } else {
                false
            }
        }
        Macroblock::B(m) => {
            if let Some(res) = &m.residual {
                res.luma_level4x4_nc[blk_idx as usize] != 0
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Section 8.7.2.1, condition 3 — motion vector / reference frame discontinuity check.
fn check_motion_discontinuity(
    slice: &Slice,
    mb_p: &Macroblock,
    blk_p_idx: usize,
    mb_q: &Macroblock,
    blk_q_idx: usize,
) -> bool {

    let get_part = |mb: &Macroblock, idx: usize| -> Option<super::macroblock::PartitionInfo> {
        let motion = match mb {
            Macroblock::P(pmb) => &pmb.motion,
            Macroblock::B(bmb) => &bmb.motion,
            _ => return None, // Intra/PCM have no motion
        };
        let (y, x) = super::residual::unscan_4x4(idx);
        Some(motion.partitions[y][x])
    };

    let p_part = get_part(mb_p, blk_p_idx);
    let q_part = get_part(mb_q, blk_q_idx);

    match (p_part, q_part) {
        (Some(pp), Some(qq)) => {
            // §8.7.2.1: For B slices, check both L0 and L1 references and MVs
            let ref_p_l0 = slice.ref_pic_list0.get(pp.ref_idx_l0 as usize).copied();
            let ref_q_l0 = slice.ref_pic_list0.get(qq.ref_idx_l0 as usize).copied();
            let ref_p_l1 = slice.ref_pic_list1.get(pp.ref_idx_l1 as usize).copied();
            let ref_q_l1 = slice.ref_pic_list1.get(qq.ref_idx_l1 as usize).copied();

            let uses_bipred = !slice.ref_pic_list1.is_empty();

            if !uses_bipred {
                // P-slice or B-slice with only L0: simple check
                if ref_p_l0 != ref_q_l0 {
                    return true;
                }
                let mv_diff_x = (pp.mv_l0.x as i32 - qq.mv_l0.x as i32).abs();
                let mv_diff_y = (pp.mv_l0.y as i32 - qq.mv_l0.y as i32).abs();
                return mv_diff_x >= 4 || mv_diff_y >= 4;
            }

            // B-slice: check if refs and MVs match in either direct or swapped order
            let mv_close = |a: super::macroblock::MotionVector, b: super::macroblock::MotionVector| -> bool {
                (a.x as i32 - b.x as i32).abs() < 4 && (a.y as i32 - b.y as i32).abs() < 4
            };

            // Direct order: L0p==L0q && L1p==L1q && MVs close
            let direct_match = ref_p_l0 == ref_q_l0
                && ref_p_l1 == ref_q_l1
                && mv_close(pp.mv_l0, qq.mv_l0)
                && mv_close(pp.mv_l1, qq.mv_l1);

            // Swapped order: L0p==L1q && L1p==L0q && MVs close (swapped)
            let swap_match = ref_p_l0 == ref_q_l1
                && ref_p_l1 == ref_q_l0
                && mv_close(pp.mv_l0, qq.mv_l1)
                && mv_close(pp.mv_l1, qq.mv_l0);

            !(direct_match || swap_match)
        }
        (Some(_), None) | (None, Some(_)) => true,
        (None, None) => false,
    }
}
