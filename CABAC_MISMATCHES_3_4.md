# Detailed CABAC Mismatch Report (Items #3 and #4)

This report details the discrepancies between the current CABAC implementation in `src/h264/cabac.rs` and the H.264 specification for the Main Profile.

---

## Mismatch #3: Reference Index (`ref_idx`) Parsing Condition

### Specification Reference
- **Section 7.3.5.1 (Macroblock prediction syntax)**
- **Section 7.3.5.2 (Sub-macroblock prediction syntax)**

The specification defines the condition for parsing `ref_idx_l0` as:
`if( ( num_ref_idx_l0_active_minus1 > 0 || mb_field_decoding_flag != field_pic_flag ) && ... )`

### Implementation Issue
In `src/h264/cabac.rs` (specifically within `parse_macroblock` and sub-macroblock parsing blocks, e.g., lines 986 and 1083), the implementation uses:
```rust
if num_ref_idx_l0_active_minus1 > 0 || slice.header.field_pic_flag { ... }
```

### Technical Impact
1. **Standard Field Pictures:** For a standard field picture (`field_pic_flag == 1`), `mb_field_decoding_flag` is inferred to be 1. Thus, `mb_field_decoding_flag != field_pic_flag` is **false**. The spec requirement reduces to `num_ref_idx_l0_active_minus1 > 0`.
2. **Implementation Deviation:** The current code's `|| field_pic_flag` branch forces the decoder to always try to parse `ref_idx` in field pictures.
3. **Failure Case:** If a field picture bitstream has only one reference field active (`num_ref_idx_l0_active_minus1 == 0`), the reference index is inferred as 0 and not present in the bits. The current implementation will attempt to read non-existent bits, leading to bitstream de-synchronization and decoding failure.
4. **MBAFF Context:** The spec's logic (`mb_field_decoding_flag != field_pic_flag`) is specifically intended to handle MBAFF frame pictures where a field-coded macroblock has access to twice as many reference fields, even if the picture-level `num_ref_idx` is low.

---

## Mismatch #4: Neighbor Mode Logic (`predModeEqualFlagN`)

### Specification Reference
- **Section 9.3.3.1.1.6 (ctxIdxInc for ref_idx)**
- **Section 9.3.3.1.1.7 (ctxIdxInc for mvd)**

The selection of a context model (probability state) for Inter data depends on the prediction mode of the neighboring macroblocks (A and B). This is governed by the `predModeEqualFlagN` variable.

### Implementation Issue
The current implementation of `get_ctx_idx_inc_ref_idx` (line 527) and `get_ctx_idx_inc_mvd` (line 560) contains the following comment:
`// predModeEqualFlagN: P slices always Pred_L0 unless B slice logic (not impl)`

The code effectively treats every Inter-coded neighbor as a "match" for the current list.

### Technical Impact
1. **Main Profile B-Slices:** In the Main Profile, B-slices are standard. A neighbor might be `Pred_L1` only.
2. **Context Model Selection:** According to the spec, if we are decoding data for **List 0** and the neighbor uses only **List 1**, `predModeEqualFlagN` must be 0, which sets `condTermFlagN` to 0.
3. **Implementation Deviation:** By ignoring this check and assuming a match, the decoder calculates an incorrect `ctxIdxInc`.
4. **Arithmetic Failure:** Selecting the wrong context model means the arithmetic engine uses an incorrect probability distribution to decode the bin. This results in incorrect values for motion vectors or reference indices, which typically causes the decoder to crash or produce visual artifacts shortly after.
5. **Skipped Blocks:** The implementation lacks logic to handle `B_Skip` and `B_Direct_16x16` neighbors correctly for these context derivations.
