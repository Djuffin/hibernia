# Implementation Plan: CABAC Support for Hibernia

This document outlines the step-by-step plan to add Context-Adaptive Binary Arithmetic Coding (CABAC) parsing support to the Hibernia H.264 decoder. This feature is required to support Main and High profiles (as indicated by `entropy_coding_mode_flag = 1`).

## 1. Context & Goal

The primary goal is to implement the **CABAC Parsing Process** as defined in **ITU-T H.264 Specification Section 9.3**.
Currently, `src/h264/parser.rs` asserts failure if `pps.entropy_coding_mode_flag` is true. We need to replace this assertion with a fully functional arithmetic decoding engine and context-adaptive parsing logic.

## 2. Step-by-Step Implementation Phases

### Phase 1: Infrastructure & Arithmetic Engine (COMPLETED)
**Goal:** Create the core arithmetic decoding engine and state management.

1.  **Create `src/h264/cabac.rs`**:
    *   Define a `CabacContext` struct to hold the state of the arithmetic decoder.
        *   `codIRange` (current interval range, initialized to 510).
        *   `codIOffset` (current offset/value, read from bitstream).
        *   `ctx_table` (state of all context models, likely `Vec<u8>` or fixed array).
    *   Implement initialization logic (`init_context_variables` - Spec 9.3.1.1):
        *   Depends on `SliceQpY` and `cabac_init_idc`.
        *   Requires initializing the `pStateIdx` and `valMPS` for all context indices (0 to ~1023).
    *   Implement the **Arithmetic Decoding Engine** (Spec 9.3.3.2):
        *   `decode_bin(ctxIdx)`: The core function to decode a single bin using a context model.
        *   `decode_bypass()`: For bins with equal probability (bypass mode).
        *   `decode_terminate()`: To check for the end of the slice (Bin string termination).
        *   `renorm()`: Renormalization process to keep precision and read new bits from the stream.

2.  **Add Lookup Tables (`src/h264/tables.rs` or `src/h264/cabac_tables.rs`)**:
    *   Initialization tables for context variables (Spec Table 9-11 to 9-23).
    *   State transition tables for probability estimation (Spec Table 9-35, `transIdxLPS` and `transIdxMPS`).
    *   Range table `rangeTabLPS` (Spec Table 9-33).

### Phase 2: Binarization & Context Selection (COMPLETED)
**Goal:** Implement helpers to map syntax elements to/from bin strings and select context indices (`ctxIdx`).

1.  **Implement `CabacContext` methods for Binarization (Spec 9.3.2)**:
    *   *Note: In a decoder, we perform "Reverse Binarization" (matching bins to values).*
    *   `parse_unary_bin(ctx_base, max_bins)`: For syntax elements like `ref_idx`.
    *   `parse_truncated_unary_bin(...)`: For `intra_chroma_pred_mode`, etc.
    *   `parse_ueg_k(...)`: For `mvd` (Motion Vector Difference) and `coeff_abs_level_minus1`.

2.  **Implement `ctxIdx` Derivation (Spec 9.3.3.1)**:
    *   Implement `get_ctx_idx_inc(...)` functions. These require checking neighboring macroblocks (A and B).
    *   **Action**: Extend `Slice` or `Macroblock` traits to expose helper methods for CABAC neighbor availability (checking `mb_skip_flag`, etc., which differs slightly from CAVLC neighbors).

### Phase 3: Parsing Macroblock Layer (COMPLETED)
**Goal:** Implement parsing of macroblock types and prediction modes using CABAC.

1.  **Add `parse_macroblock_cabac` to `src/h264/cabac.rs`**:
    *   This will mirror `parser::parse_macroblock` but use the `CabacContext` instead of `BitReader`.
2.  **Implement Syntax Element Parsers**:
    *   `mb_skip_flag`: Special handling to infer `P_Skip` types.
    *   `mb_type` (I, P, B): Uses a specific binarization tree depending on slice type.
    *   `mb_pred` (Intra prediction modes): `prev_intra4x4_pred_mode_flag`, `rem_intra4x4_pred_mode`.
    *   `sub_mb_pred` (Inter prediction modes): `sub_mb_type`.

### Phase 4: Parsing Prediction Data & Residuals (COMPLETED)
**Goal:** Parse motion vectors and transform coefficients.

1.  **Implement Prediction Parsers**:
    *   `mvd` (Motion Vector Difference): Uses UEG0 binarization with specific context selection based on vector components.
    *   `ref_idx`: Uses Truncated Unary binarization.

2.  **Implement Residual Parsers (Spec 9.3.3.1.3)**:
    *   `coded_block_pattern`: Unique binarization involving luma/chroma bits.
    *   `mb_qp_delta`: Parsed only if `CBP != 0` or Intra16x16.
    *   `residual_block_cabac`:
        *   `coded_block_flag`: Determines if a block has non-zero coefficients.
        *   `significant_coeff_flag` & `last_significant_coeff_flag`: Map the position of coefficients.
        *   `coeff_abs_level_minus1`: Decodes the value of coefficients.
        *   *Note*: This replaces `cavlc::parse_residual_block`.

### Phase 5: Integration (COMPLETED)
**Goal:** Wire the new CABAC parser into the main decoding loop.

1.  **Update `src/h264/mod.rs`**:
    *   Add `pub mod cabac;`.
2.  **Update `src/h264/parser.rs`**:
    *   In `parse_slice_data`:
        *   Check `slice.pps.entropy_coding_mode_flag`.
        *   If `true`:
            *   Initialize `CabacContext` (which aligns the `BitReader` and reads initial bytes).
            *   Enter a new loop calling `cabac::parse_macroblock_cabac`.
            *   Handle the `end_of_slice_flag` (parsed via `decode_terminate()`).
        *   If `false`: Keep existing CAVLC loop.

### Phase 6: Testing
1.  **Unit Tests**: Create unit tests in `src/h264/cabac.rs` for individual binarization processes (e.g., feeding a known bitstream and checking decoded values).
2.  **Integration Tests**: Run `e2e_tests.rs` with Main/High profile samples (which use CABAC) to verify the integration.

## 3. Proposed File Structure Changes

### `src/h264/cabac.rs` (New File)
Structure draft:
```rust
pub struct CabacContext<'a> {
    reader: &'a mut BitReader<'b>,
    range: u32,
    offset: u32,
    // Storage for context models (approx 400-1000 uint8s depending on spec)
    ctx_table: Vec<u8>,
}

impl<'a> CabacContext<'a> {
    pub fn new(reader: &mut BitReader, slice: &Slice) -> Self { ... }
    pub fn decode_bin(&mut self, ctx_idx: usize) -> u8 { ... }
    pub fn decode_bypass(&mut self) -> u8 { ... }
    pub fn decode_terminate(&mut self) -> bool { ... }

    // High-level parsing methods
    pub fn parse_mb_type(&mut self, slice_type: SliceType) -> MbType { ... }
    pub fn parse_residual(&mut self, mb: &mut Macroblock) { ... }
}

pub fn parse_macroblock_cabac(input: &mut BitReader, slice: &mut Slice) -> ParseResult<Macroblock> {
    // 1. Init engine if start of slice
    // 2. Parse MB_SKIP_FLAG
    // 3. Parse MB_TYPE
    // 4. ...
}
```

### `src/h264/tables.rs`
*   Add `CABAC_INIT_IDC` tables.
*   Add `RANGE_TAB_LPS`.
*   Add `TRANS_IDX_LPS` / `TRANS_IDX_MPS`.

### `src/h264/parser.rs`
Update `parse_slice_data`:
```rust
pub fn parse_slice_data(...) {
    if slice.pps.entropy_coding_mode_flag {
         // Initialize CABAC
         // Loop over macroblocks using parse_macroblock_cabac
    } else {
         // Existing CAVLC loop
    }
}
```
