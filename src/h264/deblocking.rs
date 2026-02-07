use super::decoder::{get_chroma_qp, VideoFrame};
use super::macroblock::{
    get_neighbor_mbs, IMb, Macroblock, MbAddr, MbNeighborName, MbPredictionMode,
};
use super::slice::{DeblockingFilterIdc, Slice, SliceType};
use super::tables::{MB_HEIGHT, MB_WIDTH};
use super::{ColorPlane, Point};
use std::cmp::{max, min};
use v_frame::plane::Plane;

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
    // bS = 1
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 7, 8, 9, 10, 11,
    ],
    // bS = 2
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 7, 8, 8, 10, 11, 12, 13, 15,
    ],
    // bS = 3
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
        2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 7, 8, 9, 10, 11, 13, 14, 16, 18, 20, 23, 25,
    ],
];

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

fn filter_macroblock(slice: &Slice, frame: &mut VideoFrame, mb_addr: MbAddr) {
    let mb = match slice.get_mb(mb_addr) {
        Some(mb) => mb,
        None => return,
    };

    let mb_xy = slice.get_mb_location(mb_addr);

    let filter_left_mb_edge_flag = should_filter_left_edge(slice, mb_addr);
    let filter_top_mb_edge_flag = should_filter_top_edge(slice, mb_addr);

    // Luma vertical edges
    // Edge 0 (Left MB boundary)
    if filter_left_mb_edge_flag {
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 0, true);
    }
    // Internal vertical edges 1, 2, 3
    // If transform_8x8 is enabled, we skip edges 1 and 3.
    let transform_8x8 = match mb {
        Macroblock::I(m) => m.transform_size_8x8_flag,
        Macroblock::P(m) => m.transform_size_8x8_flag,
        _ => false,
    };

    if !transform_8x8 {
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 1, true);
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 2, true);
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 3, true);
    } else {
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 2, true);
    }

    // Luma horizontal edges
    // Edge 0 (Top MB boundary)
    if filter_top_mb_edge_flag {
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 0, false);
    }
    // Internal horizontal edges 1, 2, 3
    if !transform_8x8 {
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 1, false);
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 2, false);
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 3, false);
    } else {
        filter_luma_edge(slice, frame, mb_addr, mb_xy, 2, false);
    }

    // Chroma (4:2:0 assumed)
    // Vertical
    if filter_left_mb_edge_flag {
        filter_chroma_edge(slice, frame, mb_addr, mb_xy, 0, true);
    }
    filter_chroma_edge(slice, frame, mb_addr, mb_xy, 1, true);

    // Horizontal
    if filter_top_mb_edge_flag {
        filter_chroma_edge(slice, frame, mb_addr, mb_xy, 0, false);
    }
    filter_chroma_edge(slice, frame, mb_addr, mb_xy, 1, false);
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

fn filter_luma_edge(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_addr: MbAddr,
    mb_xy: Point,
    // 0 corresponds to the external edge (MB boundary).
    // 1, 2, 3 correspond to internal edges at 4, 8, 12 offsets respectively.
    edge_idx: usize,
    is_vertical: bool,
) {
    let q_mb = slice.get_mb(mb_addr).unwrap();
    let q_qp = get_qp(q_mb);

    // Determine p0 location and P MB
    let (p_mb_addr, p_mb, p_qp) = if edge_idx == 0 {
        // External edge
        let neighbor_name = if is_vertical { MbNeighborName::A } else { MbNeighborName::B };
        match slice.get_mb_neighbor(mb_addr, neighbor_name) {
            Some(mb) => {
                let addr = get_neighbor_mbs(
                    slice.sps.pic_width_in_mbs() as u32,
                    slice.header.first_mb_in_slice,
                    mb_addr,
                    neighbor_name,
                )
                .unwrap();
                (addr, mb, get_qp(mb))
            }
            None => return,
        }
    } else {
        // Internal edge
        (mb_addr, q_mb, q_qp)
    };

    let plane = &mut frame.planes[ColorPlane::Y as usize];
    let stride = plane.cfg.stride;
    let data = plane.data_origin_mut();

    let qp_av = (p_qp as i32 + q_qp as i32 + 1) >> 1;
    let index_a = (qp_av + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51) as usize;
    let index_b = (qp_av + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51) as usize;

    let alpha = ALPHA_TABLE[index_a];
    let beta = BETA_TABLE[index_b];

    // Filter 16 samples (length of edge)
    for k in 0..16 {
        let (abs_x_q, abs_y_q) = if is_vertical {
            (mb_xy.x + edge_idx as u32 * 4, mb_xy.y + k as u32)
        } else {
            (mb_xy.x + k as u32, mb_xy.y + edge_idx as u32 * 4)
        };

        // Determine Boundary Strength (bS)
        let bs = get_bs(slice, mb_addr, p_mb_addr, edge_idx, k, is_vertical);
        if bs == 0 {
            continue;
        }

        // Load samples p3..p0, q0..q3
        let mut samples = [0u8; 8];
        if is_vertical {
            let row_off = abs_y_q as usize * stride;
            let x_off = abs_x_q as usize;
            samples.copy_from_slice(&data[row_off + x_off - 4..row_off + x_off + 4]);
        } else {
            let y_off = abs_y_q as usize * stride;
            let x_off = abs_x_q as usize;
            for i in 0..4 {
                samples[3 - i] = data[y_off - (i + 1) * stride + x_off];
                samples[4 + i] = data[y_off + i * stride + x_off];
            }
        }

        let (p0, q0) = (samples[3], samples[4]);

        // Check filter condition
        if (p0 as i32 - q0 as i32).abs() < alpha as i32
            && (samples[2] as i32 - p0 as i32).abs() < beta as i32
            && (samples[5] as i32 - q0 as i32).abs() < beta as i32
        {
            let (q0_idx, p0_idx, p1_idx, q1_idx, p2_idx, q2_idx) = if is_vertical {
                let row_off = abs_y_q as usize * stride;
                let x_off = abs_x_q as usize;
                (
                    row_off + x_off,
                    row_off + x_off - 1,
                    row_off + x_off - 2,
                    row_off + x_off + 1,
                    row_off + x_off - 3,
                    row_off + x_off + 2,
                )
            } else {
                let y_off = abs_y_q as usize * stride;
                let x_off = abs_x_q as usize;
                (
                    y_off + x_off,
                    y_off - stride + x_off,
                    y_off - 2 * stride + x_off,
                    y_off + stride + x_off,
                    y_off - 3 * stride + x_off,
                    y_off + 2 * stride + x_off,
                )
            };

            if bs < 4 {
                // Calculate tc0
                let tc0 = TC0_TABLE[(bs - 1) as usize][index_a];
                let mut tc = tc0 as i32;

                let ap = (samples[1] as i32 - p0 as i32).abs();
                let aq = (samples[6] as i32 - q0 as i32).abs();

                if ap < beta as i32 {
                    tc += 1;
                }
                if aq < beta as i32 {
                    tc += 1;
                }

                // Weak filtering
                let delta =
                    (((q0 as i32 - p0 as i32) << 2) + (samples[2] as i32 - samples[5] as i32) + 4)
                        >> 3;
                let delta_c = delta.clamp(-tc, tc);

                let p0_new = (p0 as i32 + delta_c).clamp(0, 255) as u8;
                let q0_new = (q0 as i32 - delta_c).clamp(0, 255) as u8;

                // Write back
                data[p0_idx] = p0_new;
                data[q0_idx] = q0_new;

                // Filter p1 (Section 8.7.2.3)
                if ap < beta as i32 {
                    let p2 = samples[1] as i32;
                    let p1 = samples[2] as i32;
                    let delta_p1 = (p2 + ((p0 as i32 + q0 as i32 + 1) >> 1) - (p1 << 1)) >> 1;
                    let p1_new =
                        (p1 + delta_p1.clamp(-(tc0 as i32), tc0 as i32)).clamp(0, 255) as u8;
                    data[p1_idx] = p1_new;
                }

                if aq < beta as i32 {
                    let q2 = samples[6] as i32;
                    let q1 = samples[5] as i32;
                    let delta_q1 = (q2 + ((p0 as i32 + q0 as i32 + 1) >> 1) - (q1 << 1)) >> 1;
                    let q1_new =
                        (q1 + delta_q1.clamp(-(tc0 as i32), tc0 as i32)).clamp(0, 255) as u8;
                    data[q1_idx] = q1_new;
                }
            } else {
                // Strong filtering (bs == 4)
                let ap = (samples[1] as i32 - p0 as i32).abs();
                let aq = (samples[6] as i32 - q0 as i32).abs();

                let small_diff = (p0 as i32 - q0 as i32).abs() < ((alpha as i32 >> 2) + 2);

                if ap < beta as i32 && small_diff {
                    // Filter p0, p1, p2
                    let p2 = samples[1] as i32;
                    let p1 = samples[2] as i32;
                    let p0 = samples[3] as i32;
                    let q0 = samples[4] as i32;
                    let q1 = samples[5] as i32;

                    data[p0_idx] = ((p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >> 3).clamp(0, 255) as u8;
                    data[p1_idx] = ((p2 + p1 + p0 + q0 + 2) >> 2).clamp(0, 255) as u8;
                    data[p2_idx] = ((2 * samples[0] as i32 + 3 * p2 + p1 + p0 + q0 + 4) >> 3).clamp(0, 255) as u8;
                } else {
                    // Weak filter p0 only (same as bs < 4 but with tc0=0)
                    let p1 = samples[2] as i32;
                    let p0 = samples[3] as i32;
                    let q1 = samples[5] as i32;
                    data[p0_idx] = ((2 * p1 + p0 + q1 + 2) >> 2).clamp(0, 255) as u8;
                }

                if aq < beta as i32 && small_diff {
                    let q2 = samples[6] as i32;
                    let q1 = samples[5] as i32;
                    let q0 = samples[4] as i32;
                    let p0 = samples[3] as i32;
                    let p1 = samples[2] as i32;

                    data[q0_idx] = ((p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4) >> 3).clamp(0, 255) as u8;
                    data[q1_idx] = ((p0 + q0 + q1 + q2 + 2) >> 2).clamp(0, 255) as u8;
                    data[q2_idx] = ((2 * samples[7] as i32 + 3 * q2 + q1 + q0 + p0 + 4) >> 3).clamp(0, 255) as u8;
                } else {
                    let q1 = samples[5] as i32;
                    let q0 = samples[4] as i32;
                    let p1 = samples[2] as i32;
                    data[q0_idx] = ((2 * q1 + q0 + p1 + 2) >> 2).clamp(0, 255) as u8;
                }
            }
        }
    }
}

fn filter_chroma_edge(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_addr: MbAddr,
    mb_xy: Point,
    // 0 corresponds to the external edge (MB boundary).
    // 1 corresponds to the internal edge (center of MB) for 4:2:0.
    edge_idx: usize,
    is_vertical: bool,
) {
    let q_mb = slice.get_mb(mb_addr).unwrap();
    let q_qp = get_qp(q_mb);

    // Determine P MB
    let (p_mb_addr, p_mb, p_qp) = if edge_idx == 0 {
        let neighbor_name = if is_vertical { MbNeighborName::A } else { MbNeighborName::B };
        match slice.get_mb_neighbor(mb_addr, neighbor_name) {
            Some(mb) => {
                let addr = get_neighbor_mbs(
                    slice.sps.pic_width_in_mbs() as u32,
                    slice.header.first_mb_in_slice,
                    mb_addr,
                    neighbor_name,
                )
                .unwrap();
                (addr, mb, get_qp(mb))
            }
            None => return,
        }
    } else {
        (mb_addr, q_mb, q_qp)
    };

    let luma_edge_idx = edge_idx * 2;
    let chroma_shift_x = 1; // 4:2:0
    let chroma_shift_y = 1;

    for k in 0..8 {
        let (x_q_c, y_q_c) = if is_vertical {
            (edge_idx as u32 * 4, k as u32)
        } else {
            (k as u32, edge_idx as u32 * 4)
        };

        // Map to luma coordinates for BS calculation
        let luma_k = k * 2;

        let bs = get_bs(slice, mb_addr, p_mb_addr, luma_edge_idx, luma_k as usize, is_vertical);
        if bs == 0 {
            continue;
        }

        // Filtering for both Cb and Cr
        for plane_idx in [ColorPlane::Cb, ColorPlane::Cr] {
            let qp_index_offset = slice.pps.get_chroma_qp_index_offset(plane_idx);

            let qp_p_c = get_chroma_qp(p_qp as i32, qp_index_offset, 0);
            let qp_q_c = get_chroma_qp(q_qp as i32, qp_index_offset, 0);
            let qp_av_c = (qp_p_c + qp_q_c + 1) >> 1;

            let index_a =
                (qp_av_c + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51) as usize;
            let index_b =
                (qp_av_c + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51) as usize;

            let alpha = ALPHA_TABLE[index_a];
            let beta = BETA_TABLE[index_b];

            let plane = &mut frame.planes[plane_idx as usize];

            let abs_x_q = (mb_xy.x >> chroma_shift_x) + x_q_c;
            let abs_y_q = (mb_xy.y >> chroma_shift_y) + y_q_c;

            let stride = plane.cfg.stride;
            let data = plane.data_origin_mut();

            let (q0_idx, p0_idx, p1_idx, q1_idx) = if is_vertical {
                let row_off = abs_y_q as usize * stride;
                let x_off = abs_x_q as usize;
                (row_off + x_off, row_off + x_off - 1, row_off + x_off - 2, row_off + x_off + 1)
            } else {
                let y_off = abs_y_q as usize * stride;
                let x_off = abs_x_q as usize;
                (
                    y_off + x_off,
                    y_off - stride + x_off,
                    y_off - 2 * stride + x_off,
                    y_off + stride + x_off,
                )
            };

            let p0 = data[p0_idx];
            let q0 = data[q0_idx];
            let p1 = data[p1_idx];
            let q1 = data[q1_idx];

            if (p0 as i32 - q0 as i32).abs() < alpha as i32
                && (p1 as i32 - p0 as i32).abs() < beta as i32
                && (q1 as i32 - q0 as i32).abs() < beta as i32
            {
                let (p0_new, q0_new) = if bs < 4 {
                    let tc0 = TC0_TABLE[(bs - 1) as usize][index_a];
                    let tc = tc0 as i32 + 1; // Chroma always adds 1 to tc0

                    let delta = (((q0 as i32 - p0 as i32) << 2) + (p1 as i32 - q1 as i32) + 4) >> 3;
                    let delta_c = delta.clamp(-tc, tc);

                    let p0_new = (p0 as i32 + delta_c).clamp(0, 255) as u8;
                    let q0_new = (q0 as i32 - delta_c).clamp(0, 255) as u8;
                    (p0_new, q0_new)
                } else {
                    // bS == 4
                    // 8.7.2.4, chromaStyleFilteringFlag = 1
                    let p0_new = ((2 * (p1 as i32) + (p0 as i32) + (q1 as i32) + 2) >> 2)
                        .clamp(0, 255) as u8;
                    let q0_new = ((2 * (q1 as i32) + (q0 as i32) + (p1 as i32) + 2) >> 2)
                        .clamp(0, 255) as u8;
                    (p0_new, q0_new)
                };

                data[p0_idx] = p0_new;
                data[q0_idx] = q0_new;
            }
        }
    }
}

fn get_bs(
    slice: &Slice,
    mb_q_addr: MbAddr,
    mb_p_addr: MbAddr,
    // 0 corresponds to the external edge. 1..3 correspond to internal edges.
    edge_idx: usize,
    k: usize,
    is_vertical: bool,
) -> u8 {
    // Section 8.7.2.1 Derivation process for the luma content dependent boundary filtering strength
    // p samples are in mb_p, q samples are in mb_q

    // We need to identify the 4x4 blocks containing p0 and q0.
    // k is sample index along the edge (0..15).
    // edge_idx is the block boundary index (0..3).
    // is_vertical: true -> vertical edge.

    // Determine 4x4 block indices for p and q.
    // Luma 4x4 blocks are indexed 0..15.

    let (blk_q_idx, blk_p_idx) = if is_vertical {
        // Vertical edge
        // q is at (edge_idx * 4, k) relative to MB Q
        // p is at (edge_idx * 4 - 1, k) relative to MB Q, which is (3, k) in a 4x4 block at (edge_idx-1)
        // If edge_idx=0, p is in MB P (rightmost column)

        let y_blk = k / 4;
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
        // q is at (k, edge_idx * 4)
        let x_blk = k / 4;
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

    let mb_p = slice.get_mb(mb_p_addr).unwrap();
    let mb_q = slice.get_mb(mb_q_addr).unwrap();

    // Condition 1: Intra coding
    if mb_p.is_intra() || mb_q.is_intra() {
        // If edge is a macroblock edge ...
        if edge_idx == 0 {
            return 4;
        }
        return 3;
    }

    // Condition 2: Non-zero transform coefficients
    // Clause 8.7.2.1: check if the 4x4 luma transform block contains non-zero transform coefficient levels.

    if has_nonzero_coeffs(mb_p, blk_p_idx as u8) || has_nonzero_coeffs(mb_q, blk_q_idx as u8) {
        return 2;
    }

    // Condition 3: Motion vectors / Reference frames
    if check_motion_discontinuity(slice, mb_p, blk_p_idx, mb_q, blk_q_idx) {
        return 1;
    }

    0
}

fn get_qp(mb: &Macroblock) -> u8 {
    match mb {
        Macroblock::I(m) => m.qp,
        Macroblock::P(m) => m.qp,
        Macroblock::PCM(m) => m.qp,
    }
}

fn has_nonzero_coeffs(mb: &Macroblock, blk_idx: u8) -> bool {
    // Check if the residual block has any non-zero coefficients.
    // This implements the check "contains non-zero transform coefficient levels" from Clause 8.7.2.1.

    match mb {
        Macroblock::I(m) => {
            if let Some(res) = &m.residual {
                // Check specific 4x4 block in luma_level4x4
                if m.MbPartPredMode(0) == MbPredictionMode::Intra_16x16 {
                    // For Intra_16x16, check both AC (in 4x4 block) and the corresponding DC coefficient.
                    let has_ac = res.ac_level16x16[blk_idx as usize].iter().any(|&x| x != 0);
                    let has_dc = res.dc_level16x16[blk_idx as usize] != 0;
                    has_ac || has_dc
                } else {
                    res.luma_level4x4[blk_idx as usize].iter().any(|&x| x != 0)
                }
            } else {
                false
            }
        }
        Macroblock::P(m) => {
            if let Some(res) = &m.residual {
                res.luma_level4x4[blk_idx as usize].iter().any(|&x| x != 0)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn check_motion_discontinuity(
    slice: &Slice,
    mb_p: &Macroblock,
    blk_p_idx: usize,
    mb_q: &Macroblock,
    blk_q_idx: usize,
) -> bool {
    // Only P slices with L0 are fully supported currently.
    // Get PartitionInfo for the 4x4 blocks.

    let get_part = |mb: &Macroblock, idx: usize| -> Option<super::macroblock::PartitionInfo> {
        match mb {
            Macroblock::P(pmb) => {
                let (y, x) = super::residual::unscan_4x4(idx);
                Some(pmb.motion.partitions[y][x])
            }
            _ => None, // Intra/PCM have no motion
        }
    };

    let p_part = get_part(mb_p, blk_p_idx);
    let q_part = get_part(mb_q, blk_q_idx);

    match (p_part, q_part) {
        (Some(pp), Some(qq)) => {
            // Check ref index
            let ref_p = slice.ref_pic_list0.get(pp.ref_idx_l0 as usize);
            let ref_q = slice.ref_pic_list0.get(qq.ref_idx_l0 as usize);

            if ref_p != ref_q {
                return true;
            }
            // Check MV difference >= 4 (quarter pel units)
            let mv_diff_x = (pp.mv_l0.x as i32 - qq.mv_l0.x as i32).abs();
            let mv_diff_y = (pp.mv_l0.y as i32 - qq.mv_l0.y as i32).abs();

            if mv_diff_x >= 4 || mv_diff_y >= 4 {
                return true;
            }
            false
        }
        // One Inter, one Intra/PCM.
        // (Intra is usually handled in Condition 1, but this covers structural mismatches).
        (Some(_), None) | (None, Some(_)) => true,
        // Both invalid/missing.
        (None, None) => false,
    }
}
