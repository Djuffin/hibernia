# `tables.rs` - H.264 Tables and Constants

This file serves as a central repository for various tables and constants used throughout the H.264 decoder, as defined in the ITU-T H.264 specification.

## Constants

- `MB_WIDTH`: The width of a macroblock in pixels (16).
- `MB_HEIGHT`: The height of a macroblock in pixels (16).
- `BIT_DEPTH`: The bit depth of the video samples (8).

## Scaling Lists

- `DEFAULT_SCALING_LIST_4X4_INTRA`: The default 4x4 scaling list for intra-coded blocks.
- `DEFAULT_SCALING_LIST_4X4_INTER`: The default 4x4 scaling list for inter-coded blocks.

Scaling lists are used to apply custom quantization scaling to different frequency components of the residual data.

## Coded Block Pattern (CBP) Mapping

- `CODE_NUM_TO_INTRA_CODED_BLOCK_PATTERN`: A table that maps a `codeNum` from the bitstream to a `CodedBlockPattern` for intra-coded macroblocks.
- `code_num_to_intra_coded_block_pattern`: A function that performs the lookup in the `CODE_NUM_TO_INTRA_CODED_BLOCK_PATTERN` table.

## Macroblock Type Mapping

- `mb_type_to_coded_block_pattern`: A function that maps an `IMbType` to its corresponding `CodedBlockPattern` for Intra_16x16 macroblocks.
- `mb_type_to_16x16_pred_mode`: A function that maps an `IMbType` to its corresponding `Intra_16x16_SamplePredMode` for Intra_16x16 macroblocks.

## CAVLC Tables

This file also includes and re-exports the static tables used for Context-Adaptive Variable-Length Coding (CAVLC) from other files:

- `TABLE95`: From `table_9-5.rs` (for `TotalCoeff` and `T1s`).
- `TABLE9_7AND8`: From `tables_9-7and8.rs` (for `level_prefix` and `level_suffix`).
- `TABLE9_9A`: From `table_9-9_a.rs` (for `run_before`).
- `TABLE9_9B`: From `table_9-9_b.rs` (for `run_before`).
- `TABLE9_10`: From `table_9-10.rs` (for `total_zeros`).
