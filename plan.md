# B Slice Implementation Plan

## Context

This H.264 decoder currently supports I and P slices only. B slices (bidirectional prediction) are a core part of the standard and are used by virtually every real-world H.264 stream for compression efficiency. The decoder already has significant scaffolding (SliceType::B, slice header parsing, ref_pic_list1 field, CABAC SyntaxElement variants) but all runtime paths panic or silently skip B-specific logic. This plan covers the complete implementation, organized into phases that can be built and tested incrementally.

### Design Decision: Extend PartitionInfo + New BMb Struct

- **Extend `PartitionInfo` in place** with L1 fields. This avoids duplicating the entire motion grid infrastructure. P-slice code is unaffected since new fields default to zero and are never read.
- **Create a parallel `BMb` struct** (like `PMb`) because B macroblocks have a completely different type system (23 types vs P's 6, 13 sub-mb types vs 4). The `Macroblock` enum gains a `B(BMb)` variant.

---

## Phase 1: Data Structures and Enums

**Goal:** Define all B-slice types and extend shared structures. Zero behavioral change — all existing tests pass.

### 1.1 Extend `PartitionInfo` — `macroblock.rs:414-424`

Add L1 fields and a per-block prediction mode:

```rust
pub struct PartitionInfo {
    pub pred_mode: MbPredictionMode,  // NEW — which lists are active
    pub ref_idx_l0: u8,
    pub mv_l0: MotionVector,
    pub mvd_l0: MotionVector,
    pub ref_idx_l1: u8,              // NEW
    pub mv_l1: MotionVector,         // NEW
    pub mvd_l1: MotionVector,        // NEW
}
```

Size grows from ~12 to ~28 bytes. All existing construction sites use `PartitionInfo::default()` or named fields — add the new fields with zero defaults. Places that explicitly construct PartitionInfo values (e.g. `predict_mv_l0`'s `get_vals` closure at `parser.rs:1134`, `fill()` calls in `calculate_motion`) need updating.

### 1.2 Add `MbPredictionMode` variants — `macroblock.rs:241`

Add `BiPred` and `Direct` to the enum. Existing matches use `_` wildcards or list known variants, so this is safe.

### 1.3 Define `BMbType` enum — `macroblock.rs` (new, after `PMbType`)

H.264 Table 7-14 — 23 macroblock types for B slices:

```
B_Direct_16x16 = 0,
B_L0_16x16 = 1, B_L1_16x16 = 2, B_Bi_16x16 = 3,
B_L0_L0_16x8 = 4, B_L0_L0_8x16 = 5,
B_L1_L1_16x8 = 6, B_L1_L1_8x16 = 7,
B_L0_L1_16x8 = 8, B_L0_L1_8x16 = 9,
B_L1_L0_16x8 = 10, B_L1_L0_8x16 = 11,
B_L0_Bi_16x8 = 12, B_L0_Bi_8x16 = 13,
B_L1_Bi_16x8 = 14, B_L1_Bi_8x16 = 15,
B_Bi_L0_16x8 = 16, B_Bi_L0_8x16 = 17,
B_Bi_L1_16x8 = 18, B_Bi_L1_8x16 = 19,
B_Bi_Bi_16x8 = 20, B_Bi_Bi_8x16 = 21,
B_8x8 = 22,
B_Skip (not in table, sentinel for skipped MBs)
```

Implement `TryFrom<u32>`, `NumMbPart()`, and `MbPartPredMode(partition: usize)` per Table 7-14.

### 1.4 Define `BSubMbType` enum — `macroblock.rs` (new)

H.264 Table 7-18 — 13 sub-macroblock types for B slices:

```
B_Direct_8x8=0, B_L0_8x8=1, B_L1_8x8=2, B_Bi_8x8=3,
B_L0_8x4=4, B_L0_4x8=5, B_L1_8x4=6, B_L1_4x8=7,
B_Bi_8x4=8, B_Bi_4x8=9, B_L0_4x4=10, B_L1_4x4=11, B_Bi_4x4=12
```

Implement `TryFrom<u32>`, `NumSubMbPart()`, `SubMbPredMode()`, `SubMbPartWidth()`, `SubMbPartHeight()`.

### 1.5 Define `BSubMacroblock` struct — `macroblock.rs`

Same layout as `SubMacroblock` but with `BSubMbType`.

### 1.6 Define `BMb` struct — `macroblock.rs`

```rust
pub struct BMb {
    pub mb_type: BMbType,
    pub motion: MbMotion,
    pub coded_block_pattern: CodedBlockPattern,
    pub mb_qp_delta: i32,
    pub qp: u8,
    pub transform_size_8x8_flag: bool,
    pub residual: Option<Box<Residual>>,
    pub cbf_info: CbfInfo,
}
```

### 1.7 Add `Macroblock::B(BMb)` variant — `macroblock.rs:484`

### 1.8 Update all `Macroblock` match arms — `macroblock.rs:533-620`

Every method needs a `Macroblock::B(mb) =>` arm:
- `MbPartPredMode()` — delegate to `BMb`
- `is_skipped()` — true for `B_Skip`
- `is_direct()` — true for `B_Direct_16x16`
- `get_nc()`, `get_coded_block_pattern()`, `set_residual()`, `get_motion_info()`, `set_qp()`, `get_cbf_info()` — mirror the P arms

### 1.9 Add `CabacMbType::B(BMbType)` — `cabac.rs:2151`

Update all matches on `CabacMbType`:
- `parse_residual_cabac` (line 1229): `B(_) => Pred_L0` (residual parsing is list-independent)
- Various `is_intra` / `is_skipped` checks in `NeighborAccessor`

---

## Phase 2: Reference Picture List 1 Construction

**Goal:** Build ref_pic_list1 for B slices. Spec §8.2.4.2.3 and §8.2.4.3.

### 2.1 `initialize_ref_pic_list0_b()` — `decoder.rs` (new)

For B slices, list 0 initialization differs from P (§8.2.4.2.3 vs §8.2.4.2.1):
1. Short-term refs with `POC <= current_POC`, sorted **descending** by POC
2. Short-term refs with `POC > current_POC`, sorted **ascending** by POC
3. Long-term refs, sorted ascending by LongTermPicNum

The current `initialize_ref_pic_list0()` (decoder.rs:465) sorts by frame_num — that's the P-slice algorithm.

### 2.2 `initialize_ref_pic_list1()` — `decoder.rs` (new)

Per §8.2.4.2.3:
1. Short-term refs with `POC > current_POC`, sorted ascending by POC
2. Short-term refs with `POC <= current_POC`, sorted descending by POC
3. Long-term refs, sorted ascending
4. **Special case:** if list1 == list0 and len > 1, swap first two entries

### 2.3 `construct_ref_pic_list1()` — `decoder.rs` (new)

Same pattern as `construct_ref_pic_list0()`: initialize → modify → truncate to `num_ref_idx_l1_active_minus1 + 1`.

### 2.4 `modify_ref_pic_list1()` — `decoder.rs` (new)

Identical logic to `modify_ref_pic_list0()` (decoder.rs:504) but operates on `list1` modifications. Reuses `place_picture_in_list()`, `find_short_term_in_dpb()`, `find_long_term_in_dpb()`.

### 2.5 Update `construct_ref_pic_list0()` — `decoder.rs:443`

Branch on `slice.header.slice_type == SliceType::B` to call `initialize_ref_pic_list0_b()` instead of the P-slice `initialize_ref_pic_list0()`.

### 2.6 Update `process_slice()` — `decoder.rs:287`

After `construct_ref_pic_list0`, add:
```rust
if slice.header.slice_type == SliceType::B {
    self.construct_ref_pic_list1(slice)?;
}
```

---

## Phase 3: CAVLC B Slice Parsing

**Goal:** Parse B macroblock syntax via Exp-Golomb (non-CABAC path).

### 3.1 Update `parse_macroblock()` — `parser.rs:984`

Add B-slice branch before the P fallthrough:
- `mb_type` 0–22 → `BMbType::try_from(mb_type_val)`
- `mb_type` 23–48 → `IMbType::try_from(mb_type_val - 23)` (intra MB in B slice)

### 3.2 Implement `parse_b_macroblock()` — `parser.rs` (new)

Mirrors `parse_p_macroblock()` (parser.rs:1321) but handles all four prediction modes per partition. Parsing order per §7.3.5.1:

1. If `B_8x8`: parse 4 sub-macroblock types (`BSubMbType`)
2. ref_idx_l0 for each partition where mode ∉ {Pred_L1, Direct}
3. ref_idx_l1 for each partition where mode ∉ {Pred_L0, Direct}
4. mvd_l0 for each partition where mode ∉ {Pred_L1, Direct}
5. mvd_l1 for each partition where mode ∉ {Pred_L0, Direct}

For `B_Direct_16x16`: no ref_idx or mvd parsed — motion derived in Phase 6.

---

## Phase 4: CABAC B Slice Parsing

**Goal:** CABAC binarization and context derivation for B-slice syntax elements.

### 4.1 `MbSkipFlagB` context — `cabac.rs:1210`

Replace `unimplemented!()`. The ctxIdxInc derivation is identical to `MbSkipFlagP` (condTermFlagA + condTermFlagB from neighbors). The ctxIdxOffset (24 vs 11) is already handled by the `SyntaxElement` → table entry mapping. Just dispatch to the same neighbor-checking code.

### 4.2 B skip MB construction — `cabac.rs:1764`

Replace `unimplemented!("B-slice mb_skip_flag parsing")`. Construct a `BMb` with `B_Skip` type and default motion (direct prediction fills it in during Phase 6).

### 4.3 `parse_mb_type_b()` — `cabac.rs` (new)

Table 9-37 binarization tree (ctxIdxOffset 27 prefix, 32 suffix for intra):
- Bin 0=0 → `B_Direct_16x16`
- Bin 0=1, 1=0, 2=0 → `B_L0_16x16`
- Bin 0=1, 1=0, 2=1 → `B_L1_16x16`
- Bin 0=1, 1=1, 2=0, 3=0, 4=0 → `B_Bi_16x16`
- … (20 more types via deeper bins)
- Prefix `111110` + I-type suffix → Intra MB in B slice

Context increments per Table 9-39: binIdx 0 uses condTermFlagA+condTermFlagB (0-2), binIdx 1 → 3, binIdx 2 → 4 or 5, etc.

### 4.4 `MbTypeB` context — `cabac.rs:1210`

Implement ctxIdxInc per Table 9-39 for MbTypeB.

### 4.5 `parse_sub_mb_type_b()` — `cabac.rs` (new)

Table 9-38, ctxIdxOffset 36, 13 sub-mb types.

### 4.6 `SubMbTypeB` context — `cabac.rs:1210`

Implement ctxIdxInc per Table 9-39 for SubMbTypeB.

### 4.7 B mb_type dispatch — `cabac.rs:1768`

Add `SliceType::B` branch calling `parse_mb_type_b()`.

### 4.8 B partition parsing — `cabac.rs:1920`

Replace `unimplemented!("B-slice mb_type and partition parsing")`. Parse ref_idx and mvd for L0/L1 based on per-partition prediction mode. The existing `parse_ref_idx_cabac()` and `parse_mvd_cabac()` already accept `list_idx`, so they work for both L0 and L1.

### 4.9 Fix `get_ref_idx()` L1 — `cabac.rs:120,132`

Replace the four `unimplemented!()` calls:
- Current MB (line 118-120): read `p_info.ref_idx_l1` when `list_idx == 1`
- Neighbor MB (lines 123-133): add `Macroblock::B(bmb)` arm, read L1 from motion grid

### 4.10 Fix `get_mvd()` L1 — `cabac.rs:158,169`

Same pattern: read `mvd_l1` when `list_idx == 1`, add B variant arm.

---

## Phase 5: Motion Vector Prediction for B Slices

**Goal:** MVP derivation for L1 and B-slice motion calculation.

### 5.1 Refactor `predict_mv_l0` into generic helper — `parser.rs:1055-1174`

Extract a generic `predict_mv()` that takes accessor functions:

```rust
fn predict_mv(
    slice, mb_addr, part_x, part_y, part_w, part_h, ref_idx,
    current_mb_motion,
    get_ref_idx: impl Fn(&PartitionInfo) -> u8,
    get_mv: impl Fn(&PartitionInfo) -> MotionVector,
) -> MotionVector
```

Then `predict_mv_l0` = `predict_mv(..., |p| p.ref_idx_l0, |p| p.mv_l0)` and `predict_mv_l1` = `predict_mv(..., |p| p.ref_idx_l1, |p| p.mv_l1)`.

`get_motion_at_coord()` (parser.rs:1004) already returns `PartitionInfo` with both L0/L1 fields — no change needed.

### 5.2 Implement `calculate_motion_b()` — `parser.rs` (new)

Analogous to `calculate_motion()` (parser.rs:1176-1320). Per partition:
- **Pred_L0**: call `predict_mv_l0`, set L0 fields only
- **Pred_L1**: call `predict_mv_l1`, set L1 fields only
- **BiPred**: call both, set both
- **Direct**: delegate to direct prediction (Phase 6)
- Set `partition.pred_mode` on each 4×4 block

---

## Phase 6: Direct Prediction (Spec §8.4.1.2)

**Goal:** Spatial and temporal direct prediction. The most spec-intensive phase.

### 6.1 Spatial Direct (§8.4.1.2.2) — new function

Used when `direct_spatial_mv_pred_flag == true` (common case). Algorithm:
1. From spatial neighbors A, B, C/D, derive mvL0 and mvL1 independently using median prediction
2. Derive refIdxL0 and refIdxL1 as minimum of neighbors' ref indices
3. Determine pred_mode: BiPred if both valid, Pred_L0 or Pred_L1 if only one, skip if neither
4. Derive `directZeroFlag` for zero-MV cases

### 6.2 Temporal Direct (§8.4.1.2.3) — new function

Used when `direct_spatial_mv_pred_flag == false`. Algorithm:
1. Colocated picture = refPicList1[0]
2. For each 4×4 block, get colocated partition's ref_idx and MV
3. Scale by POC distance: `td = POC(colRef) - POC(col)`, `tb = POC(cur) - POC(colRef)`
4. `mvL0 = scale(mvCol, tb, td)`, `mvL1 = mvL0 - mvCol`

**Requires:** Storing per-MB motion data in reference pictures. Add `motion_field: Option<Vec<MbMotion>>` to `Picture` struct (`decoder.rs:33`). Populate after decoding each reference picture's slice. Memory: ~450 bytes/MB × MBs-in-picture.

**Strategy:** Implement spatial direct first (no motion field needed). Temporal direct as a follow-up.

### 6.3 Direct motion dispatcher — new function

```rust
fn derive_direct_motion(slice, mb_addr, references) -> MbMotion {
    if slice.header.direct_spatial_mv_pred_flag.unwrap_or(true) {
        derive_spatial_direct(...)
    } else {
        derive_temporal_direct(...)
    }
}
```

### 6.4 Update `Macroblock::is_direct()` — `macroblock.rs:554`

Return true for `Macroblock::B(mb)` when `mb.mb_type == BMbType::B_Direct_16x16`.

---

## Phase 7: Bidirectional Motion Compensation

**Goal:** Render B macroblock pixels.

### 7.1 `render_luma_inter_prediction_b()` — `decoder.rs` (new)

For each 4×4 block, check `partition.pred_mode`:
- **Pred_L0**: interpolate from `ref_pic_list0[ref_idx_l0]` using `mv_l0` (same as P)
- **Pred_L1**: interpolate from `ref_pic_list1[ref_idx_l1]` using `mv_l1`
- **BiPred**: interpolate from both, average: `pred = (pred_l0 + pred_l1 + 1) >> 1` (§8.4.2.2.1)
- **Direct**: uses whichever mode direct prediction derived

Reuses existing `interpolate_luma()` from `inter_pred.rs` — it takes a reference plane and MV, fully generic.

### 7.2 `render_chroma_inter_prediction_b()` — `decoder.rs` (new)

Same pattern using `interpolate_chroma()`.

### 7.3 Weighted prediction (deferrable)

Three modes via `pps.weighted_bipred_idc`:
- 0: Default (simple average) — implement this first
- 1: Explicit weights from `PredWeightTable` (already parsed)
- 2: Implicit weights from POC distances

### 7.4 Decoder dispatch — `decoder.rs:300`

Add `Macroblock::B(block) =>` arm in the MB rendering loop, following the same QP → residual → render → chroma pattern as `Macroblock::P`.

---

## Phase 8: Deblocking Filter Update

**Goal:** Correct boundary strength for B macroblocks.

### 8.1 `has_nonzero_coeffs()` — `deblocking.rs:645`
Add `Macroblock::B(m)` arm.

### 8.2 `get_qp()` — `deblocking.rs:637`
Add `Macroblock::B(m) => m.qp`.

### 8.3 `check_motion_discontinuity()` — `deblocking.rs:676`
The `get_part` closure returns `None` for non-P MBs. Add `Macroblock::B(bmb)` support. For B slices (§8.7.2.1), bS derivation must check both L0 and L1 references and MVs. Initial conservative implementation: if pred_mode, ref indices, or MVs differ on **either** list → `BS_MOTION`.

---

## Phase 9: Integration Testing

- **Test vectors:** IPB, IBBBP, IBBPBBP sequences, both CAVLC and CABAC
- **Regression:** All existing tests in `e2e_tests.rs` pass unchanged
- **DPB output order:** B frames output in POC order via existing bumping (dpb.rs:101)

---

## Dependency Graph

```
Phase 1 (data structures)
  ├── Phase 2 (ref pic list 1)      ── can proceed in parallel ──┐
  ├── Phase 3 (CAVLC parsing)       ── can proceed in parallel ──┤
  ├── Phase 4 (CABAC parsing)       ── can proceed in parallel ──┤
  └── Phase 5 (MV prediction)       ── can proceed in parallel ──┤
       └── Phase 6 (direct pred)    ── depends on Phase 5 ───────┤
            └── Phase 7 (BiPred MC) ── depends on Phases 5,6 ────┤
  └── Phase 8 (deblocking)          ── can proceed in parallel ──┘
                                                                  │
                                              Phase 9 (integration)
```

## Critical Files

| File | Changes |
|------|---------|
| `src/h264/macroblock.rs` | BMbType, BSubMbType, BMb, PartitionInfo extension, Macroblock::B variant, all match arms |
| `src/h264/parser.rs` | parse_macroblock B branch, parse_b_macroblock, calculate_motion_b, predict_mv refactor |
| `src/h264/cabac.rs` | CabacMbType::B, parse_mb_type_b, parse_sub_mb_type_b, context derivation, L1 get_ref_idx/get_mvd |
| `src/h264/decoder.rs` | construct_ref_pic_list1, render_*_b, MB dispatch, motion_field storage |
| `src/h264/deblocking.rs` | B variant in get_part/get_qp/has_nonzero_coeffs/check_motion_discontinuity |

## Verification

1. After Phase 1: `cargo test` — all existing tests pass, no behavioral change
2. After Phase 2: unit test ref_pic_list1 ordering with a mock DPB
3. After Phases 3-4: parse B-slice bitstreams without decoding (verify no panics)
4. After Phase 7: decode B-slice test vectors, compare output frames against reference decoder (e.g., ffmpeg)
5. After Phase 8: visual inspection of deblocking artifacts on B-slice boundaries
6. Final: full regression on all test vectors in `data/`
