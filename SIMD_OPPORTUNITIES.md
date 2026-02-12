# SIMD Optimization Opportunities in Hibernia

This document identifies areas in the Hibernia H.264 decoder where SIMD (Single Instruction, Multiple Data) optimizations can be applied to improve performance. These opportunities are categorized by their estimated impact and implementation complexity.

## 1. High Impact / Easy to Implement

### Saturated Residual Addition
- **Location:** `src/h264/decoder.rs`
- **Functions:** `render_luma_inter_prediction`, `render_chroma_inter_prediction`, `render_luma_4x4_intra_prediction`, `render_luma_16x16_intra_prediction`, `render_chroma_intra_prediction`
- **Description:** After prediction, residual coefficients are added to the predicted samples and clamped to the range [0, 255].
- **Why SIMD:** This is a classic SIMD operation (saturated addition). A 4x4 block (16 pixels) can be processed in a single SIMD instruction (e.g., using 128-bit registers).

### Inverse Transforms & Hadamard Transform
- **Location:** `src/h264/residual.rs`
- **Functions:** `transform_4x4`, `transform_dc`, `transform_chroma_dc`
- **Description:** These functions perform fixed-size (4x4 or 2x2) matrix-like operations using only additions, subtractions, and shifts.
- **Why SIMD:** The logic is highly regular and operates on small, contiguous blocks. SIMD can process rows or columns in parallel.

### Level Scaling
- **Location:** `src/h264/residual.rs`
- **Functions:** `level_scale_4x4_block`, `dc_scale_4x4_block`, `dc_scale_2x2_block`
- **Description:** Element-wise multiplication and shifting of transform coefficients.
- **Why SIMD:** This is a simple loop over 16 elements (4x4 block) that can be easily vectorized.

### Quarter-pel Averaging
- **Location:** `src/h264/inter_pred.rs`
- **Functions:** `interpolate_luma` (cases for quarter-pel positions)
- **Description:** Averaging two samples: `(a + b + 1) >> 1`.
- **Why SIMD:** Operates on entire blocks of pixels. SIMD "average" instructions (like `vpavgb` in x86) can process 16 or 32 pixels at once.

---

## 2. High Impact / Medium Complexity

### Luma 6-tap Filter
- **Location:** `src/h264/inter_pred.rs`
- **Functions:** `filter_6tap`, `filter_6tap_and_clip`, `filter_6tap_vertical_and_clip`
- **Description:** Applies a 6-tap FIR filter `[1, -5, 20, 20, -5, 1]` to interpolate half-pel positions.
- **Why SIMD:** This is the most computationally intensive part of inter-prediction.
    - **Horizontal:** Can be optimized by loading multiple overlapping windows into SIMD registers.
    - **Vertical:** Multiple columns can be processed in parallel.

### Deblocking Filter (Edge Filtering)
- **Location:** `src/h264/deblocking.rs`
- **Functions:** `filter_luma_edge`, `filter_chroma_edge`
- **Description:** Filters 16 (luma) or 8 (chroma) samples along a macroblock or block edge.
- **Why SIMD:** Each line of samples perpendicular to the edge is filtered independently. SIMD can process all 16 or 8 lines along the edge simultaneously.

---

## 3. Moderate Impact / Medium Complexity

### Intra Plane Prediction
- **Location:** `src/h264/decoder.rs`
- **Functions:** `render_luma_16x16_intra_prediction` (Plane mode), `render_chroma_intra_prediction` (Plane mode)
- **Description:** Generates a linear gradient across a 16x16 or 8x8 block: `value = (a + b * (x - 7) + c * (y - 7) + 16) >> 5`.
- **Why SIMD:** The linear nature allows for efficient calculation using SIMD by adding incremental values (`b` for horizontal, `c` for vertical) to a base register.

### Chroma Bilinear Interpolation
- **Location:** `src/h264/inter_pred.rs`
- **Functions:** `interpolate_chroma`
- **Description:** 2D bilinear interpolation using weights derived from fractional motion vectors.
- **Why SIMD:** Similar to luma quarter-pel averaging but with variable weights. Can be vectorized by processing blocks of pixels.
