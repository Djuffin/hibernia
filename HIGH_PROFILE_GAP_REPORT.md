# Hibernia: High Profile Support Gap Analysis

This report outlines the missing features and required changes to support the **High Profile** (IDC 100) of the H.264 (AVC) specification in the Hibernia decoder.

## 1. Transform and Residual Processing
The most significant addition in High Profile is the support for the 8x8 integer transform, in addition to the standard 4x4 transform used in Baseline/Main profiles.

- **8x8 Integer Transform (Clause 8.5.3, 8.5.13)**: The inverse 8x8 transform is currently not implemented. `src/h264/residual.rs` only contains logic for 4x4 transforms.
- **8x8 Residual Parsing**: CABAC and CAVLC parsing for 8x8 residual blocks are missing.
    - In `src/h264/cabac.rs`, `parse_residual_cabac` returns an error if `transform_size_8x8_flag` is set.
    - Specialized CABAC context derivation for 8x8 blocks (Table 9-43) is unimplemented.
- **Scaling Matrices / Quantization Scaling (Clause 8.5.9)**: Support for custom scaling lists (SPS/PPS) is missing. High Profile allows encoders to specify per-frequency quantization weights. Currently, the decoder assumes a flat scaling list of 16.

## 2. Intra Prediction
- **Intra 8x8 Prediction (Clause 8.3.2)**: This mode is specific to the High Profile and works in conjunction with the 8x8 transform.
    - The 9 prediction modes for Intra 8x8 (Vertical, Horizontal, DC, etc., plus a low-pass filter on reference samples) are not implemented in `src/h264/intra_pred.rs`.
    - `src/h264/parser.rs` returns `FeatureNotSupported` when encountering Intra 8x8 prediction.

## 3. Entropy Coding (CABAC)
High Profile introduces new contexts and binarization requirements for the 8x8 tools.

- **Context Derivation for 8x8 Blocks**: Clause 9.3.3.1.1.9 (Coded Block Flag) and Clause 9.3.3.1.3 (Significant Coeff Flag, etc.) have specific logic for `ctxBlockCat` values related to 8x8 transforms (categories 5, 9, 13, and 6-12 for different chroma formats) that are currently marked as `unimplemented!` in `src/h264/cabac.rs`.

## 4. Syntax Elements and Semantics
Several syntax elements are parsed but their effects are not fully integrated:

- **SPS High Profile Extensions**:
    - `chroma_format_idc`: Support for Monochrome (4:0:0). High Profile allows `chroma_format_idc` in range 0..1.
    - `bit_depth_luma_minus8` / `bit_depth_chroma_minus8`: High Profile (IDC 100) specifically requires these to be 0 (8-bit), but the infrastructure should ideally handle larger values for High 10/4:2:2/4:4:4 profiles.
    - `qpprime_y_zero_transform_bypass_flag`: Lossless coding support.
    - `seq_scaling_matrix_present_flag`: Activation of custom scaling lists.
- **PPS High Profile Extensions**:
    - `transform_8x8_mode_flag`: Enables the 8x8 transform.
    - `pic_scaling_matrix_present_flag`: PPS-level scaling lists.
    - `second_chroma_qp_index_offset`: Allows different QP offsets for Cb and Cr.

## 5. Infrastructure and Performance
- **Bit Depth Abstraction**: The codebase currently assumes 8-bit samples (`u8`). Supporting High 10/High 4:2:2 (which are often lumped into "High Profile" support in practical applications) would require templating or abstracting the sample type.
- **Memory Management**: The DPB size calculation for High Profile (Annex A.3.2) differs slightly from Baseline/Main.

## Summary of Action Items
1.  **Implement 8x8 Inverse Transform** in `src/h264/residual.rs`.
2.  **Implement Intra 8x8 Prediction** in `src/h264/intra_pred.rs`.
3.  **Implement Scaling List derivation and application** in scaling/quantization processes.
4.  **Complete CABAC context logic** for 8x8 blocks in `src/h264/cabac.rs`.
5.  **Add support for Monochrome (4:0:0)** chroma format.
6.  **Refactor residual parsing** to toggle between 4x4 and 8x8 based on `transform_size_8x8_flag`.
