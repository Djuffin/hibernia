use std::cmp::{max, min};

use v_frame::frame::Frame;
use v_frame::plane::PlaneOffset;

use crate::h264::macroblock::{
    get_4x4chroma_block_location, get_4x4luma_block_location, Macroblock, MbAddr, MbNeighborName,
    MbPredictionMode,
};
use crate::h264::slice::{DeblockingFilterIdc, Slice};
use crate::h264::{tables, ColorPlane, Point};

type VideoFrame = Frame<u8>;

// Table 8-16 – Derivation of offset dependent threshold alpha’(0) and beta’(0)
const ALPHA_TABLE: [u8; 52] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 17, 20,
    22, 25, 28, 32, 36, 40, 45, 50, 56, 63, 71, 80, 90, 101, 113, 127, 144, 162, 182, 203, 226,
    255, 255,
];

const BETA_TABLE: [u8; 52] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 6, 6, 7, 7, 8, 8,
    9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18,
];

// Table 8-17 – Value of tc0
// Column 0 for bS=1, Column 1 for bS=2, Column 2 for bS=3
const TC0_TABLE: [[u8; 3]; 52] = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 1],
    [0, 0, 1],
    [0, 0, 1],
    [0, 0, 1],
    [0, 1, 1],
    [0, 1, 1],
    [1, 1, 1],
    [1, 1, 1],
    [1, 1, 1],
    [1, 1, 1],
    [1, 1, 2],
    [1, 1, 2],
    [1, 1, 2],
    [1, 1, 2],
    [1, 2, 3],
    [1, 2, 3],
    [2, 2, 3],
    [2, 2, 4],
    [2, 3, 4],
    [2, 3, 4],
    [3, 3, 5],
    [3, 4, 6],
    [3, 4, 6],
    [4, 5, 7],
    [4, 5, 8],
    [4, 6, 9],
    [5, 7, 10],
    [5, 8, 11],
    [6, 9, 13],
    [7, 10, 14],
    [8, 11, 16],
    [9, 13, 18],
    [10, 14, 20],
    [11, 16, 23],
    [13, 19, 25],
];

pub fn apply_deblocking_filter(slice: &Slice, frame: &mut VideoFrame) {
    if slice.header.deblocking_filter_idc == DeblockingFilterIdc::Off {
        return;
    }

    let pic_width_in_mbs = slice.sps.pic_width_in_mbs() as u32;
    let pic_height_in_mbs = slice.sps.pic_height_in_map_units_minus1 + 1;

    for mb_addr in slice.header.first_mb_in_slice
        ..(slice.header.first_mb_in_slice + slice.get_macroblock_count() as u32)
    {
        let mb_loc = slice.get_mb_location(mb_addr);

        // Luma vertical edges
        // 4 edges: 0, 1, 2, 3
        for edge in 0..4 {
            filter_luma_edge_vertical(slice, frame, mb_addr, mb_loc, edge);
        }

        // Luma horizontal edges
        // 4 edges: 0, 1, 2, 3
        for edge in 0..4 {
            filter_luma_edge_horizontal(slice, frame, mb_addr, mb_loc, edge);
        }

        // Chroma vertical edges
        // 2 edges: 0, 1
        for edge in 0..2 {
            filter_chroma_edge_vertical(slice, frame, mb_addr, mb_loc, edge);
        }

        // Chroma horizontal edges
        // 2 edges: 0, 1
        for edge in 0..2 {
            filter_chroma_edge_horizontal(slice, frame, mb_addr, mb_loc, edge);
        }
    }
}

// 8.7.2.1 Derivation process for the luma content of each boundary strength
fn get_boundary_strength_luma(
    slice: &Slice,
    curr_mb_addr: MbAddr,
    neighbor_mb_addr: Option<MbAddr>,
    curr_blk_idx: u8,
    neighbor_blk_idx: u8,
    vertical_edge: bool,
) -> u8 {
    let curr_mb = slice.get_mb(curr_mb_addr).unwrap();
    let neighbor_mb = if let Some(addr) = neighbor_mb_addr {
        if let Some(mb) = slice.get_mb(addr) {
            mb
        } else {
            // Neighbor not available (e.g. outside slice/picture)
            // If neighbor is not available, BS is 0.
            return 0;
        }
    } else {
        return 0;
    };

    let curr_is_intra = curr_mb.is_intra();
    let neighbor_is_intra = neighbor_mb.is_intra();

    if curr_is_intra || neighbor_is_intra {
        // Condition 1: Intra coded
        // If the boundary is a macroblock boundary
        let is_mb_boundary = if vertical_edge {
            curr_mb_addr != neighbor_mb_addr.unwrap()
        } else {
            curr_mb_addr != neighbor_mb_addr.unwrap()
        };

        if is_mb_boundary {
            return 4;
        } else {
            return 3;
        }
    }

    // Condition 2: Non-zero transform coefficients
    let curr_has_coeffs = check_non_zero_coeffs(curr_mb, curr_blk_idx);
    let neighbor_has_coeffs = check_non_zero_coeffs(neighbor_mb, neighbor_blk_idx);

    if curr_has_coeffs || neighbor_has_coeffs {
        return 2;
    }

    // Condition 3: Motion vectors / Reference frames
    // different reference picture or difference in motion vector components >= 4
    let curr_motion = curr_mb.get_motion_info();
    let neighbor_motion = neighbor_mb.get_motion_info();

    // Map 4x4 block index to 4x4 grid coordinates (0..3, 0..3)
    let curr_p = get_4x4luma_block_location(curr_blk_idx);
    let curr_grid_x = (curr_p.x / 4) as usize;
    let curr_grid_y = (curr_p.y / 4) as usize;

    let neighbor_p = get_4x4luma_block_location(neighbor_blk_idx);
    let neighbor_grid_x = (neighbor_p.x / 4) as usize;
    let neighbor_grid_y = (neighbor_p.y / 4) as usize;

    let curr_part = curr_motion.partitions[curr_grid_y][curr_grid_x];
    let neighbor_part = neighbor_motion.partitions[neighbor_grid_y][neighbor_grid_x];

    if curr_part.ref_idx_l0 != neighbor_part.ref_idx_l0 {
        return 1;
    }

    if (curr_part.mv_l0.x as i32 - neighbor_part.mv_l0.x as i32).abs() >= 4
        || (curr_part.mv_l0.y as i32 - neighbor_part.mv_l0.y as i32).abs() >= 4
    {
        return 1;
    }

    0
}

fn check_non_zero_coeffs(mb: &Macroblock, blk_idx: u8) -> bool {
    match mb {
        Macroblock::I(m) => {
            if let Some(res) = &m.residual {
                res.get_nc(blk_idx, ColorPlane::Y) > 0
            } else {
                false
            }
        }
        Macroblock::P(m) => {
            if let Some(res) = &m.residual {
                res.get_nc(blk_idx, ColorPlane::Y) > 0
            } else {
                false
            }
        }
        Macroblock::PCM(_) => false,
    }
}

fn filter_luma_edge_vertical(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_addr: MbAddr,
    mb_loc: Point,
    edge: u32,
) {
    let mb_x = mb_loc.x;
    let mb_y = mb_loc.y;
    let x_offset = edge * 4;

    for k in 0..4 {
        let y_offset = k * 4;
        let blk_y = y_offset;
        let blk_x = x_offset;

        // Current block index
        let curr_p = Point { x: blk_x, y: blk_y };
        let curr_blk_idx = crate::h264::macroblock::get_4x4luma_block_index(curr_p);

        // Neighbor block
        let (neighbor_blk_idx, neighbor_mb_addr) = if edge == 0 {
            // Left MB
            let nb_addr_opt = crate::h264::macroblock::get_neighbor_mbs(
                slice.sps.pic_width_in_mbs() as u32,
                slice.header.first_mb_in_slice,
                mb_addr,
                MbNeighborName::A,
            );
            if let Some(addr) = nb_addr_opt {
                 if slice.get_mb(addr).is_some() {
                     let (idx, _) = crate::h264::macroblock::get_4x4luma_block_neighbor(
                        curr_blk_idx,
                        MbNeighborName::A,
                     );
                     (idx, Some(addr))
                 } else {
                     (0, None)
                 }
            } else {
                (0, None)
            }
        } else {
            // Internal neighbor
            // (blk_x - 4, blk_y)
            let neighbor_p = Point { x: blk_x - 4, y: blk_y };
            let idx = crate::h264::macroblock::get_4x4luma_block_index(neighbor_p);
            (idx, Some(mb_addr))
        };

        let bs = get_boundary_strength_luma(
            slice,
            mb_addr,
            neighbor_mb_addr,
            curr_blk_idx,
            neighbor_blk_idx,
            true,
        );

        if bs == 0 {
            continue;
        }

        // Get QP values
        let curr_mb = slice.get_mb(mb_addr).unwrap();
        let get_qp = |mb: &Macroblock| -> i32 {
            if let Macroblock::PCM(_) = mb {
                0
            } else {
                (mb.get_mb_qp_delta() + slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta).clamp(0, 51)
            }
        };

        let qp_p = if let Some(addr) = neighbor_mb_addr {
            if let Some(mb) = slice.get_mb(addr) {
                get_qp(mb)
            } else {
                get_qp(curr_mb)
            }
        } else {
            get_qp(curr_mb)
        };
        let qp_q = get_qp(curr_mb);
        let qp_av = (qp_p + qp_q + 1) >> 1;

        // Apply filter for 4 lines
        let index_a = (qp_av + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51);
        let index_b = (qp_av + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51);

        let alpha = ALPHA_TABLE[index_a as usize];
        let beta = BETA_TABLE[index_b as usize];

        // Access pixels
        let plane = &mut frame.planes[0];
        // We need to modify 4 rows.
        for i in 0..4 {
             let y = (mb_y + blk_y + i) as isize;
             let x = (mb_x + blk_x) as isize;

             // Using mut_slice to access rows because row_mut is not available
             // But we need random access to a row.
             // We can get a slice starting at (0, y) and take the row.
             let mut plane_slice = plane.mut_slice(PlaneOffset { x: 0, y });
             let row = &mut plane_slice[0]; // First row of the slice, which is row y.

             let p0 = row[(x - 1) as usize];
             let p1 = row[(x - 2) as usize];
             let p2 = row[(x - 3) as usize];
             let q0 = row[x as usize];
             let q1 = row[(x + 1) as usize];
             let q2 = row[(x + 2) as usize];

             // Check filter condition
             // |p0 - q0| < alpha && |p1 - p0| < beta && |q1 - q0| < beta
             let dp0q0 = (p0 as i32 - q0 as i32).abs();
             let dp1p0 = (p1 as i32 - p0 as i32).abs();
             let dq1q0 = (q1 as i32 - q0 as i32).abs();

             if dp0q0 < alpha as i32 && dp1p0 < beta as i32 && dq1q0 < beta as i32 {
                 // Table 8-17:
                 // bS=1 -> column 0
                 // bS=2 -> column 1
                 // bS=3 -> column 2
                 // bS=4 -> column 2 (if weak filter applied)
                 let tc0_idx = match bs {
                     1 => 0,
                     2 => 1,
                     _ => 2,
                 };
                 let tc0 = TC0_TABLE[index_a as usize][tc0_idx] as i32;

                 // Calculate p2 and q2 diffs for both cases
                 let dp2p0 = (p2 as i32 - p0 as i32).abs();
                 let dq2q0 = (q2 as i32 - q0 as i32).abs();

                 if bs == 4 {
                     // Strong filter
                     let alpha_prime = (alpha as i32 >> 2) + 2;
                     let p_strong = dp2p0 < beta as i32 && dp0q0 < alpha_prime;
                     let q_strong = dq2q0 < beta as i32 && dp0q0 < alpha_prime;

                     if p_strong {
                         let p3 = row[(x - 4) as usize];
                         row[(x - 1) as usize] = ((p2 as i32 + 2 * p1 as i32 + 2 * p0 as i32 + 2 * q0 as i32 + q1 as i32 + 4) >> 3).clamp(0, 255) as u8;
                         row[(x - 2) as usize] = ((p2 as i32 + p1 as i32 + p0 as i32 + q0 as i32 + 2) >> 2).clamp(0, 255) as u8;
                         row[(x - 3) as usize] = ((2 * p3 as i32 + 3 * p2 as i32 + p1 as i32 + p0 as i32 + q0 as i32 + 4) >> 3).clamp(0, 255) as u8;
                     } else {
                         // Weak filter for p
                         // p0' = p0 + clip(...)
                         // "For the calculation of the filtered sample values p0' ... the corresponding value of tc is set equal to tc0 + 1" (8.7.2.3 for Chroma? No, 8.7.2.2 Luma for bS=4, weak)
                         // Wait, 8.7.2.2 says if bS=4 and !strong:
                         // p0' = (2*p1 + p0 + q1 + 2) >> 2
                         // No clipping with tc here?
                         // "The filtered samples p0', p1', p2' are derived ... as follows: p0' = (2*p1 + p0 + q1 + 2) >> 2"
                         let val = (2 * p1 as i32 + p0 as i32 + q1 as i32 + 2) >> 2;
                         row[(x - 1) as usize] = val.clamp(0, 255) as u8;
                     }

                     if q_strong {
                         let q3 = row[(x + 3) as usize];
                         row[x as usize] = ((p1 as i32 + 2 * p0 as i32 + 2 * q0 as i32 + 2 * q1 as i32 + q2 as i32 + 4) >> 3).clamp(0, 255) as u8;
                         row[(x + 1) as usize] = ((p0 as i32 + q0 as i32 + q1 as i32 + q2 as i32 + 2) >> 2).clamp(0, 255) as u8;
                         row[(x + 2) as usize] = ((2 * q3 as i32 + 3 * q2 as i32 + q1 as i32 + q0 as i32 + p0 as i32 + 4) >> 3).clamp(0, 255) as u8;
                     } else {
                         // Weak filter for q (Bs=4)
                         let val = (2 * q1 as i32 + q0 as i32 + p1 as i32 + 2) >> 2;
                         row[x as usize] = val.clamp(0, 255) as u8;
                     }

                 } else {
                     // Bs < 4 (Weak filter)
                     // tc = tc0 + ((|p2 - p0| < beta) ? 1 : 0) + ((|q2 - q0| < beta) ? 1 : 0);
                     let mut tc = tc0;
                     let ap = dp2p0 < beta as i32;
                     let aq = dq2q0 < beta as i32;
                     if ap { tc += 1; }
                     if aq { tc += 1; }

                     let delta = (((q0 as i32 - p0 as i32) << 2) + (p1 as i32 - q1 as i32) + 4) >> 3;
                     let delta_clipped = delta.clamp(-tc, tc);

                     let p0_new = (p0 as i32 + delta_clipped).clamp(0, 255) as u8;
                     let q0_new = (q0 as i32 - delta_clipped).clamp(0, 255) as u8;

                     row[(x - 1) as usize] = p0_new;
                     row[x as usize] = q0_new;

                     if ap {
                         let delta_p = (p2 as i32 + ((p0 as i32 + q0 as i32 + 1) >> 1) - 2 * p1 as i32) >> 1;
                         let delta_p_clipped = delta_p.clamp(-tc0, tc0);
                         row[(x - 2) as usize] = (p1 as i32 + delta_p_clipped).clamp(0, 255) as u8;
                     }

                     if aq {
                         let delta_q = (q2 as i32 + ((p0 as i32 + q0 as i32 + 1) >> 1) - 2 * q1 as i32) >> 1;
                         let delta_q_clipped = delta_q.clamp(-tc0, tc0);
                         row[(x + 1) as usize] = (q1 as i32 + delta_q_clipped).clamp(0, 255) as u8;
                     }
                 }
             }
        }
    }
}

fn filter_luma_edge_horizontal(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_addr: MbAddr,
    mb_loc: Point,
    edge: u32,
) {
    let mb_x = mb_loc.x;
    let mb_y = mb_loc.y;
    let y_offset = edge * 4;

    for k in 0..4 {
        let x_offset = k * 4;
        let blk_y = y_offset;
        let blk_x = x_offset;

        let curr_p = Point { x: blk_x, y: blk_y };
        let curr_blk_idx = crate::h264::macroblock::get_4x4luma_block_index(curr_p);

        let (neighbor_blk_idx, neighbor_mb_addr) = if edge == 0 {
            // Top MB
            if let Some(nb_addr) = slice.get_mb_neighbor(mb_addr, MbNeighborName::B) {
                // Address of B neighbor.
                let nb_addr_val = crate::h264::macroblock::get_neighbor_mbs(
                    slice.sps.pic_width_in_mbs() as u32,
                    slice.header.first_mb_in_slice,
                    mb_addr,
                    MbNeighborName::B,
                );
                // The block index in top MB adjacent to current is on bottom row.
                let (idx, _) = crate::h264::macroblock::get_4x4luma_block_neighbor(
                    curr_blk_idx,
                    MbNeighborName::B,
                );
                (idx, nb_addr_val)
            } else {
                (0, None)
            }
        } else {
            // Internal neighbor (blk_x, blk_y - 4)
             let neighbor_p = Point { x: blk_x, y: blk_y - 4 };
             let idx = crate::h264::macroblock::get_4x4luma_block_index(neighbor_p);
             (idx, Some(mb_addr))
        };

        let bs = get_boundary_strength_luma(
            slice,
            mb_addr,
            neighbor_mb_addr,
            curr_blk_idx,
            neighbor_blk_idx,
            false,
        );

        if bs == 0 {
            continue;
        }

        let curr_mb = slice.get_mb(mb_addr).unwrap();
        let get_qp = |mb: &Macroblock| -> i32 {
            if let Macroblock::PCM(_) = mb {
                0
            } else {
                (mb.get_mb_qp_delta() + slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta).clamp(0, 51)
            }
        };

        let qp_p = if let Some(addr) = neighbor_mb_addr {
            if let Some(mb) = slice.get_mb(addr) {
                get_qp(mb)
            } else {
                get_qp(curr_mb)
            }
        } else {
            get_qp(curr_mb)
        };
        let qp_q = get_qp(curr_mb);
        let qp_av = (qp_p + qp_q + 1) >> 1;

        let index_a = (qp_av + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51);
        let index_b = (qp_av + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51);

        let alpha = ALPHA_TABLE[index_a as usize];
        let beta = BETA_TABLE[index_b as usize];

        let plane = &mut frame.planes[0];
        // Vertical filtering (modifying 4 columns)
        for i in 0..4 {
             let x = (mb_x + blk_x + i) as isize;
             let y = (mb_y + blk_y) as isize;

             // Samples p0..p3 (top), q0..q3 (bottom)
             // p0 is at y-1
             let p0 = plane.p(x as usize, (y - 1) as usize);
             let p1 = plane.p(x as usize, (y - 2) as usize);
             let p2 = plane.p(x as usize, (y - 3) as usize);
             let q0 = plane.p(x as usize, y as usize);
             let q1 = plane.p(x as usize, (y + 1) as usize);
             let q2 = plane.p(x as usize, (y + 2) as usize);

             let dp0q0 = (p0 as i32 - q0 as i32).abs();
             let dp1p0 = (p1 as i32 - p0 as i32).abs();
             let dq1q0 = (q1 as i32 - q0 as i32).abs();

             if dp0q0 < alpha as i32 && dp1p0 < beta as i32 && dq1q0 < beta as i32 {
                 let tc0_idx = match bs {
                     1 => 0,
                     2 => 1,
                     _ => 2,
                 };
                 let tc0 = TC0_TABLE[index_a as usize][tc0_idx] as i32;

                 let dp2p0 = (p2 as i32 - p0 as i32).abs();
                 let dq2q0 = (q2 as i32 - q0 as i32).abs();

                 if bs == 4 {
                     let alpha_prime = (alpha as i32 >> 2) + 2;
                     let p_strong = dp2p0 < beta as i32 && dp0q0 < alpha_prime;
                     let q_strong = dq2q0 < beta as i32 && dp0q0 < alpha_prime;

                     if p_strong {
                         if y < 4 { continue; }
                         let p3 = plane.p(x as usize, (y - 4) as usize);
                         plane.mut_slice(PlaneOffset{x: x, y: y-1})[0][0] = ((p2 as i32 + 2 * p1 as i32 + 2 * p0 as i32 + 2 * q0 as i32 + q1 as i32 + 4) >> 3).clamp(0, 255) as u8;
                         plane.mut_slice(PlaneOffset{x: x, y: y-2})[0][0] = ((p2 as i32 + p1 as i32 + p0 as i32 + q0 as i32 + 2) >> 2).clamp(0, 255) as u8;
                         plane.mut_slice(PlaneOffset{x: x, y: y-3})[0][0] = ((2 * p3 as i32 + 3 * p2 as i32 + p1 as i32 + p0 as i32 + q0 as i32 + 4) >> 3).clamp(0, 255) as u8;
                     } else {
                         let val = (2 * p1 as i32 + p0 as i32 + q1 as i32 + 2) >> 2;
                         plane.mut_slice(PlaneOffset{x: x, y: y-1})[0][0] = val.clamp(0, 255) as u8;
                     }

                     if q_strong {
                         let q3 = plane.p(x as usize, (y + 3) as usize);
                         plane.mut_slice(PlaneOffset{x: x, y: y})[0][0] = ((p1 as i32 + 2 * p0 as i32 + 2 * q0 as i32 + 2 * q1 as i32 + q2 as i32 + 4) >> 3).clamp(0, 255) as u8;
                         plane.mut_slice(PlaneOffset{x: x, y: y+1})[0][0] = ((p0 as i32 + q0 as i32 + q1 as i32 + q2 as i32 + 2) >> 2).clamp(0, 255) as u8;
                         plane.mut_slice(PlaneOffset{x: x, y: y+2})[0][0] = ((2 * q3 as i32 + 3 * q2 as i32 + q1 as i32 + q0 as i32 + p0 as i32 + 4) >> 3).clamp(0, 255) as u8;
                     } else {
                         let val = (2 * q1 as i32 + q0 as i32 + p1 as i32 + 2) >> 2;
                         plane.mut_slice(PlaneOffset{x: x, y: y})[0][0] = val.clamp(0, 255) as u8;
                     }

                 } else {
                     let mut tc = tc0;
                     let ap = dp2p0 < beta as i32;
                     let aq = dq2q0 < beta as i32;
                     if ap { tc += 1; }
                     if aq { tc += 1; }

                     let delta = (((q0 as i32 - p0 as i32) << 2) + (p1 as i32 - q1 as i32) + 4) >> 3;
                     let delta_clipped = delta.clamp(-tc, tc);

                     plane.mut_slice(PlaneOffset{x: x, y: y-1})[0][0] = (p0 as i32 + delta_clipped).clamp(0, 255) as u8;
                     plane.mut_slice(PlaneOffset{x: x, y: y})[0][0] = (q0 as i32 - delta_clipped).clamp(0, 255) as u8;

                     if ap {
                         let delta_p = (p2 as i32 + ((p0 as i32 + q0 as i32 + 1) >> 1) - 2 * p1 as i32) >> 1;
                         let delta_p_clipped = delta_p.clamp(-tc0, tc0);
                         plane.mut_slice(PlaneOffset{x: x, y: y-2})[0][0] = (p1 as i32 + delta_p_clipped).clamp(0, 255) as u8;
                     }

                     if aq {
                         let delta_q = (q2 as i32 + ((p0 as i32 + q0 as i32 + 1) >> 1) - 2 * q1 as i32) >> 1;
                         let delta_q_clipped = delta_q.clamp(-tc0, tc0);
                         plane.mut_slice(PlaneOffset{x: x, y: y+1})[0][0] = (q1 as i32 + delta_q_clipped).clamp(0, 255) as u8;
                     }
                 }
             }
        }
    }
}

// For Chroma, BS is calculated similar to Luma?
// 8.7.2.3: "The value of the boundary strength Bs is determined ... as specified in subclause 8.7.2.1."
// It refers to Luma BS.
// "The process specified in this subclause is invoked for each chroma edge ... corresponding to the luma edge..."
// So Chroma uses the SAME BS as the corresponding Luma edge.
// Chroma edge 0 corresponds to Luma edge 0.
// Chroma edge 1 corresponds to Luma edge 2 (because chroma is subsampled 4:2:0).
// 8.7.1:
// "For each macroblock, the vertical edges are filtered first...
//  a) Vertical edge filtering
//     - Luma: edges 0, 1, 2, 3
//     - Chroma: edges 0, 1
//  b) Horizontal edge filtering
//     - Luma: edges 0, 1, 2, 3
//     - Chroma: edges 0, 1"
//
// Chroma edge 0 (x=0) matches Luma edge 0 (x=0).
// Chroma edge 1 (x=4) matches Luma edge 2 (x=8). (Since chroma 4x4 covers luma 8x8).
// So when filtering chroma edge `c_edge` (0 or 1), we need BS from luma edge `c_edge * 2`.

fn filter_chroma_edge_vertical(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_addr: MbAddr,
    mb_loc: Point,
    c_edge: u32,
) {
    let mb_x_chroma = mb_loc.x / 2;
    let mb_y_chroma = mb_loc.y / 2;
    let x_offset = c_edge * 4;
    let luma_edge = c_edge * 2;

    // We filter Cb then Cr.
    for plane_idx in 1..=2 {
        let plane = &mut frame.planes[plane_idx];

        // Loop over 4 blocks vertically?
        // Chroma block is 8x8 pixels (for 4:2:0 MB). Composed of 4 4x4 blocks.
        // Vertical edges are at x=0, x=4.
        // Height is 8 pixels.
        // So we iterate k=0..1 (0..3? No, height is 8, so 2 4x4 blocks).

        for k in 0..2 {
            let y_offset = k * 4;
            // Get BS from corresponding Luma block.
            // Luma block vertical position:
            // Chroma block (k) corresponds to Luma block (k*2).
            // Actually, we need to check BS for each 4x4 block line?
            // BS is derived per 4x4 block boundary.
            // Luma has 4 blocks vertically. Chroma has 2 blocks vertically.
            // "The value of the boundary strength Bs is determined ... for the luma edge."
            // Which luma edge?
            // 8.7.2.3: "Let Bs be the boundary strength ... for the luma edge ... corresponding to the chroma edge."
            // Since chroma resolution is half, one chroma 4x4 block height corresponds to 2 luma 4x4 blocks height.
            // But 8.7.2.1 derives BS for "a luma edge".
            // It seems we should use the BS of the corresponding luma 4x4 block.
            // If chroma 4x4 block covers luma rows 0..7, there are 2 luma blocks (rows 0..3, 4..7).
            // Do they always have same BS? Not necessarily.
            // If they differ, do we filter top half with one BS and bottom half with another?
            // Yes. The filtering is defined per sample (or line).
            // Usually implementations loop over 4x4 chroma blocks.
            // For each 4x4 chroma block (height 4), we map to a luma block.
            // Chroma row 0..3 corresponds to Luma row 0..7 (actually 0,2,4,6?). No, spatial correspondence.
            // Luma 0..3 -> Chroma 0..1 ? No, 4:2:0 means 2x2 luma = 1x1 chroma.
            // So 4x4 chroma = 8x8 luma.
            // So 4 lines of chroma correspond to 8 lines of luma.
            // So 1 chroma block corresponds to 2x2 luma blocks.
            // Vertical edge of chroma corresponds to vertical edge of luma.
            // Chroma lines 0, 1, 2, 3.
            // Line 0 corresponds to Luma line 0, 1.
            // Line 1 corresponds to Luma line 2, 3.
            // The BS is defined on 4x4 grid.
            // Luma block at (0,0) covers lines 0..3.
            // Chroma block at (0,0) covers lines 0..3.
            // Wait, chroma block at (0,0) covers spatial area of luma (0,0) to (7,7).
            // So chroma line 0 is luma line 0 and 1.
            // Does the BS change every 2 luma lines? No, every 4.
            // So for chroma lines 0 and 1, they correspond to luma block row 0 (lines 0..3).
            // Chroma lines 2 and 3 correspond to luma block row 1 (lines 4..7)?
            // No.
            // Chroma 0 -> Luma 0,1.
            // Chroma 1 -> Luma 2,3.
            // Chroma 2 -> Luma 4,5.
            // Chroma 3 -> Luma 6,7.
            // Luma block 0 is lines 0..3. Luma block 2 (below it) is lines 4..7.
            // So Chroma lines 0,1 use BS from Luma block 0.
            // Chroma lines 2,3 use BS from Luma block 2.

            // So we split loop k into sub-parts?
            // Or simpler: Iterate over each chroma line 0..3?
            // And fetch BS for that line.

            // Actually, spec 8.7.1 says:
            // "Deblocking filter process for chroma ... edges 0, 1"
            // "For each chroma edge ... edges are filtered ... 8 lines? No, MbHeightC / 2 ?"
            // For YUV420, MbHeightC = 8.
            // Edges are vertical. Length is 8.
            // We process 4x4 blocks.
            // So k=0 (top 4 lines), k=1 (bottom 4 lines).

            // For k=0 (lines 0..3):
            // Luma lines 0..7.
            // This covers Luma block (0,0) and (0,1) [vertical index].
            // Wait, Luma block (0,0) is lines 0..3. Luma block (0,1) is lines 4..7.
            // So Chroma lines 0,1 map to Luma block (0,0).
            // Chroma lines 2,3 map to Luma block (0,1).
            // So BS can change inside a 4x4 chroma block!

            // Let's implement per 2 lines.

            let blk_y_chroma = k * 4;
            let blk_x_chroma = x_offset;

            // Lines 0, 1
            {
                 let luma_blk_y = blk_y_chroma * 2; // 0 -> 0. 4 -> 8.
                 let luma_blk_x = blk_x_chroma * 2; // 0 -> 0. 4 -> 8.

                 // Get BS for Luma block at (luma_blk_x, luma_blk_y)
                 // Need neighbor info.
                 // We can reuse `get_boundary_strength_luma`.
                 // Need to reconstruct block indices.
                 let curr_p = Point { x: luma_blk_x, y: luma_blk_y };
                 let curr_blk_idx = crate::h264::macroblock::get_4x4luma_block_index(curr_p);

                 let (neighbor_blk_idx, neighbor_mb_addr) = if luma_edge == 0 {
                    if let Some(nb_addr) = slice.get_mb_neighbor(mb_addr, MbNeighborName::A) {
                         let nb_addr_val = crate::h264::macroblock::get_neighbor_mbs(
                             slice.sps.pic_width_in_mbs() as u32,
                             slice.header.first_mb_in_slice,
                             mb_addr,
                             MbNeighborName::A,
                         );
                        let (idx, _) = crate::h264::macroblock::get_4x4luma_block_neighbor(
                            curr_blk_idx,
                            MbNeighborName::A,
                        );
                        (idx, nb_addr_val)
                    } else { (0, None) }
                 } else {
                     let neighbor_p = Point { x: luma_blk_x - 4, y: luma_blk_y };
                     let idx = crate::h264::macroblock::get_4x4luma_block_index(neighbor_p);
                     (idx, Some(mb_addr))
                 };

                 let bs = get_boundary_strength_luma(
                    slice, mb_addr, neighbor_mb_addr, curr_blk_idx, neighbor_blk_idx, true
                 );

                 if bs > 0 {
                     apply_chroma_filter_vertical(slice, plane, mb_x_chroma, mb_y_chroma, blk_x_chroma, blk_y_chroma, 0, 2, bs, mb_addr, neighbor_mb_addr);
                 }
            }

            // Lines 2, 3
            {
                 let luma_blk_y = (blk_y_chroma * 2) + 4; // 0 -> 4. 4 -> 12.
                 let luma_blk_x = blk_x_chroma * 2;

                 let curr_p = Point { x: luma_blk_x, y: luma_blk_y };
                 let curr_blk_idx = crate::h264::macroblock::get_4x4luma_block_index(curr_p);

                 let (neighbor_blk_idx, neighbor_mb_addr) = if luma_edge == 0 {
                    if let Some(nb_addr) = slice.get_mb_neighbor(mb_addr, MbNeighborName::A) {
                         let nb_addr_val = crate::h264::macroblock::get_neighbor_mbs(
                             slice.sps.pic_width_in_mbs() as u32,
                             slice.header.first_mb_in_slice,
                             mb_addr,
                             MbNeighborName::A,
                         );
                        let (idx, _) = crate::h264::macroblock::get_4x4luma_block_neighbor(
                            curr_blk_idx,
                            MbNeighborName::A,
                        );
                        (idx, nb_addr_val)
                    } else { (0, None) }
                 } else {
                     let neighbor_p = Point { x: luma_blk_x - 4, y: luma_blk_y };
                     let idx = crate::h264::macroblock::get_4x4luma_block_index(neighbor_p);
                     (idx, Some(mb_addr))
                 };

                 let bs = get_boundary_strength_luma(
                    slice, mb_addr, neighbor_mb_addr, curr_blk_idx, neighbor_blk_idx, true
                 );

                 if bs > 0 {
                     apply_chroma_filter_vertical(slice, plane, mb_x_chroma, mb_y_chroma, blk_x_chroma, blk_y_chroma, 2, 2, bs, mb_addr, neighbor_mb_addr);
                 }
            }
        }
    }
}

fn apply_chroma_filter_vertical(
    slice: &Slice,
    plane: &mut v_frame::plane::Plane<u8>,
    mb_x: u32, mb_y: u32,
    blk_x: u32, blk_y: u32,
    start_line: u32, num_lines: u32,
    bs: u8,
    curr_mb_addr: MbAddr,
    neighbor_mb_addr: Option<MbAddr>,
) {
    let curr_mb = slice.get_mb(curr_mb_addr).unwrap();
    let get_qp = |mb: &Macroblock| -> i32 {
        if let Macroblock::PCM(_) = mb {
            0
        } else {
            let qp_y = (mb.get_mb_qp_delta() + slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta).clamp(0, 51);
            crate::h264::decoder::get_chroma_qp(qp_y, slice.pps.chroma_qp_index_offset, 0)
        }
    };

    let qp_p = if let Some(addr) = neighbor_mb_addr {
        if let Some(mb) = slice.get_mb(addr) {
            get_qp(mb)
        } else {
             get_qp(curr_mb)
        }
    } else {
        get_qp(curr_mb)
    };
    let qp_q = get_qp(curr_mb);
    let qp_av = (qp_p + qp_q + 1) >> 1;

    let index_a = (qp_av + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51);
    let index_b = (qp_av + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51);
    let alpha = ALPHA_TABLE[index_a as usize];
    let beta = BETA_TABLE[index_b as usize];

    // For Chroma, if Bs > 3 (i.e. Bs=4), we use TC0 from table.
    // Spec 8.7.2.3:
    // tc = tc0 + 1 if Bs == 4.
    // tc = tc0     if Bs < 4.

    // TC0 index logic is same as luma?
    // "tc0 is determined ... as specified in ... using indexA"
    let tc0_idx = match bs {
        1 => 0,
        2 => 1,
        _ => 2,
    };
    let tc0 = TC0_TABLE[index_a as usize][tc0_idx] as i32;
    let tc = tc0 + (if bs == 4 { 1 } else { 0 });

    for i in start_line..(start_line + num_lines) {
        let x = (mb_x + blk_x) as isize;
        let y = (mb_y + blk_y + i) as isize;

        let mut plane_slice = plane.mut_slice(PlaneOffset { x: 0, y: y });
        let row = &mut plane_slice[0];

        let p0 = row[(x - 1) as usize];
        let p1 = row[(x - 2) as usize];
        let q0 = row[x as usize];
        let q1 = row[(x + 1) as usize];

        let dp0q0 = (p0 as i32 - q0 as i32).abs();
        let dp1p0 = (p1 as i32 - p0 as i32).abs();
        let dq1q0 = (q1 as i32 - q0 as i32).abs();

        if dp0q0 < alpha as i32 && dp1p0 < beta as i32 && dq1q0 < beta as i32 {
            if bs == 4 {
                row[(x - 1) as usize] = ((2 * p1 as i32 + p0 as i32 + q1 as i32 + 2) >> 2).clamp(0, 255) as u8;
                row[x as usize] = ((2 * q1 as i32 + q0 as i32 + p1 as i32 + 2) >> 2).clamp(0, 255) as u8;
            } else {
                let delta = (((q0 as i32 - p0 as i32) << 2) + (p1 as i32 - q1 as i32) + 4) >> 3;
                let delta_clipped = delta.clamp(-tc, tc);

                row[(x - 1) as usize] = (p0 as i32 + delta_clipped).clamp(0, 255) as u8;
                row[x as usize] = (q0 as i32 - delta_clipped).clamp(0, 255) as u8;
            }
        }
    }
}

fn filter_chroma_edge_horizontal(
    slice: &Slice,
    frame: &mut VideoFrame,
    mb_addr: MbAddr,
    mb_loc: Point,
    c_edge: u32,
) {
    let mb_x_chroma = mb_loc.x / 2;
    let mb_y_chroma = mb_loc.y / 2;
    let y_offset = c_edge * 4;
    let luma_edge = c_edge * 2;

    for plane_idx in 1..=2 {
        let plane = &mut frame.planes[plane_idx];

        for k in 0..2 {
            let x_offset = k * 4;
            let blk_y_chroma = y_offset;
            let blk_x_chroma = x_offset;

            // Sub-block 0 (cols 0, 1) -> Luma row 0, 1 -> Luma BS from block (x, y)
            {
                 let luma_blk_y = blk_y_chroma * 2;
                 let luma_blk_x = blk_x_chroma * 2;

                 let curr_p = Point { x: luma_blk_x, y: luma_blk_y };
                 let curr_blk_idx = crate::h264::macroblock::get_4x4luma_block_index(curr_p);

                 let (neighbor_blk_idx, neighbor_mb_addr) = if luma_edge == 0 {
                    if let Some(nb_addr) = slice.get_mb_neighbor(mb_addr, MbNeighborName::B) {
                         let nb_addr_val = crate::h264::macroblock::get_neighbor_mbs(
                             slice.sps.pic_width_in_mbs() as u32,
                             slice.header.first_mb_in_slice,
                             mb_addr,
                             MbNeighborName::B,
                         );
                        let (idx, _) = crate::h264::macroblock::get_4x4luma_block_neighbor(
                            curr_blk_idx,
                            MbNeighborName::B,
                        );
                        (idx, nb_addr_val)
                    } else { (0, None) }
                 } else {
                     let neighbor_p = Point { x: luma_blk_x, y: luma_blk_y - 4 };
                     let idx = crate::h264::macroblock::get_4x4luma_block_index(neighbor_p);
                     (idx, Some(mb_addr))
                 };

                 let bs = get_boundary_strength_luma(
                    slice, mb_addr, neighbor_mb_addr, curr_blk_idx, neighbor_blk_idx, false
                 );

                 if bs > 0 {
                     apply_chroma_filter_horizontal(slice, plane, mb_x_chroma, mb_y_chroma, blk_x_chroma, blk_y_chroma, 0, 2, bs, mb_addr, neighbor_mb_addr);
                 }
            }

            // Sub-block 1 (cols 2, 3)
            {
                 let luma_blk_y = blk_y_chroma * 2;
                 let luma_blk_x = (blk_x_chroma * 2) + 4;

                 let curr_p = Point { x: luma_blk_x, y: luma_blk_y };
                 let curr_blk_idx = crate::h264::macroblock::get_4x4luma_block_index(curr_p);

                 let (neighbor_blk_idx, neighbor_mb_addr) = if luma_edge == 0 {
                    if let Some(nb_addr) = slice.get_mb_neighbor(mb_addr, MbNeighborName::B) {
                         let nb_addr_val = crate::h264::macroblock::get_neighbor_mbs(
                             slice.sps.pic_width_in_mbs() as u32,
                             slice.header.first_mb_in_slice,
                             mb_addr,
                             MbNeighborName::B,
                         );
                        let (idx, _) = crate::h264::macroblock::get_4x4luma_block_neighbor(
                            curr_blk_idx,
                            MbNeighborName::B,
                        );
                        (idx, nb_addr_val)
                    } else { (0, None) }
                 } else {
                     let neighbor_p = Point { x: luma_blk_x, y: luma_blk_y - 4 };
                     let idx = crate::h264::macroblock::get_4x4luma_block_index(neighbor_p);
                     (idx, Some(mb_addr))
                 };

                 let bs = get_boundary_strength_luma(
                    slice, mb_addr, neighbor_mb_addr, curr_blk_idx, neighbor_blk_idx, false
                 );

                 if bs > 0 {
                     apply_chroma_filter_horizontal(slice, plane, mb_x_chroma, mb_y_chroma, blk_x_chroma, blk_y_chroma, 2, 2, bs, mb_addr, neighbor_mb_addr);
                 }
            }
        }
    }
}

fn apply_chroma_filter_horizontal(
    slice: &Slice,
    plane: &mut v_frame::plane::Plane<u8>,
    mb_x: u32, mb_y: u32,
    blk_x: u32, blk_y: u32,
    start_col: u32, num_cols: u32,
    bs: u8,
    curr_mb_addr: MbAddr,
    neighbor_mb_addr: Option<MbAddr>,
) {
    let curr_mb = slice.get_mb(curr_mb_addr).unwrap();
    let get_qp = |mb: &Macroblock| -> i32 {
        if let Macroblock::PCM(_) = mb {
            0
        } else {
            let qp_y = (mb.get_mb_qp_delta() + slice.pps.pic_init_qp_minus26 + 26 + slice.header.slice_qp_delta).clamp(0, 51);
            crate::h264::decoder::get_chroma_qp(qp_y, slice.pps.chroma_qp_index_offset, 0)
        }
    };

    let qp_p = if let Some(addr) = neighbor_mb_addr {
        if let Some(mb) = slice.get_mb(addr) {
            get_qp(mb)
        } else {
             get_qp(curr_mb)
        }
    } else {
        get_qp(curr_mb)
    };
    let qp_q = get_qp(curr_mb);
    let qp_av = (qp_p + qp_q + 1) >> 1;

    let index_a = (qp_av + slice.header.slice_alpha_c0_offset_div2 * 2).clamp(0, 51);
    let index_b = (qp_av + slice.header.slice_beta_offset_div2 * 2).clamp(0, 51);
    let alpha = ALPHA_TABLE[index_a as usize];
    let beta = BETA_TABLE[index_b as usize];

    let tc0_idx = match bs {
        1 => 0,
        2 => 1,
        _ => 2,
    };
    let tc0 = TC0_TABLE[index_a as usize][tc0_idx] as i32;
    let tc = tc0 + (if bs == 4 { 1 } else { 0 });

    for i in start_col..(start_col + num_cols) {
        let x = (mb_x + blk_x + i) as isize;
        let y = (mb_y + blk_y) as isize;

        let p0 = plane.p(x as usize, (y - 1) as usize);
        let p1 = plane.p(x as usize, (y - 2) as usize);
        let q0 = plane.p(x as usize, y as usize);
        let q1 = plane.p(x as usize, (y + 1) as usize);

        let dp0q0 = (p0 as i32 - q0 as i32).abs();
        let dp1p0 = (p1 as i32 - p0 as i32).abs();
        let dq1q0 = (q1 as i32 - q0 as i32).abs();

        if dp0q0 < alpha as i32 && dp1p0 < beta as i32 && dq1q0 < beta as i32 {
            if bs == 4 {
                plane.mut_slice(PlaneOffset{x: x, y: y-1})[0][0] = ((2 * p1 as i32 + p0 as i32 + q1 as i32 + 2) >> 2).clamp(0, 255) as u8;
                plane.mut_slice(PlaneOffset{x: x, y: y})[0][0] = ((2 * q1 as i32 + q0 as i32 + p1 as i32 + 2) >> 2).clamp(0, 255) as u8;
            } else {
                let delta = (((q0 as i32 - p0 as i32) << 2) + (p1 as i32 - q1 as i32) + 4) >> 3;
                let delta_clipped = delta.clamp(-tc, tc);

                plane.mut_slice(PlaneOffset{x: x, y: y-1})[0][0] = (p0 as i32 + delta_clipped).clamp(0, 255) as u8;
                plane.mut_slice(PlaneOffset{x: x, y: y})[0][0] = (q0 as i32 - delta_clipped).clamp(0, 255) as u8;
            }
        }
    }
}
