# Hibernia Codebase Review

This document summarizes the review of the Hibernia codebase, focusing on idiomatic Rust, code clarity, unused code, and performance.

## 1. Non-idiomatic Rust

*   **Unprofessional Error Naming:** `DecodingError::Wtf` in `src/h264/decoder.rs` is non-idiomatic and unprofessional.
*   **Widespread Lint Suppression:** The project-wide suppression of lints like `non_snake_case`, `unused_variables`, and `clippy::needless_range_loop` in `src/lib.rs` and `src/main.rs` hides potential issues and encourages non-standard coding styles. In particular, `non_snake_case` is used to allow methods like `MbPartPredMode` and `NumMbPart` to follow spec naming instead of Rust conventions.
*   **Macro-heavy Parsing:** The `read_value!`, `expect_value!`, and `cast_or_error!` macros in `src/h264/parser.rs` use hidden `return Err(...)` statements. While they reduce boilerplate, they make control flow harder to follow and deviate from standard Rust error handling patterns (like `?` operator).
*   **Typographical Errors:**
    *   `is_chrome_subsampled` instead of `is_chroma_subsampled` in `src/h264/mod.rs` and `src/h264/parser.rs`.
    *   `point_to_plain_offset` instead of `point_to_plane_offset` in `src/h264/decoder.rs`.
*   **Naming Conventions:** Some methods use `PascalCase` (e.g., `NumSubMbPart` in `src/h264/macroblock.rs`) which is unconventional for Rust methods.
*   **Inefficient `remaining()`:** `RbspReader::remaining()` in `src/h264/rbsp.rs` clones the reader and seeks to the end of the bitstream on every call. This is non-idiomatic for a performance-sensitive reader.

## 2. Hard to Follow Regions

*   **Motion Vector Prediction:** `predict_mv_l0` in `src/h264/parser.rs` is a large, dense function with many special cases that are hard to verify without deep spec knowledge.
*   **Intra Prediction Rendering:** `render_luma_4x4_intra_prediction` in `src/h264/decoder.rs` contains a massive `match` statement for all 9 prediction modes, making it difficult to maintain.
*   **Complex Parsing Logic:** `remove_emulation_if_needed` in `src/h264/parser.rs` has intricate logic to avoid allocations that is somewhat opaque.
*   **Large Source Files:** `decoder.rs` and `parser.rs` are very large and handle many different aspects of H.264 decoding, which could be better modularized.

## 3. Unneeded Variables and Functions

*   **Redundant QP Storage:** In `Decoder::process_slice` (in `src/h264/decoder.rs`), macroblock QPs are first collected into a `mb_qps` vector and then applied to macroblocks in a second loop. This could be done in a single pass.
*   **TODOs in Code:** Several `todo!()` calls remain in the codebase (e.g., for 8x8 transforms, interlaced video, and high profiles), which will cause panics if those code paths are hit.
*   **Duplicated Logic:** `main.rs` and its tests share a significant amount of boilerplate code for Y4M encoding.

## 4. Obvious Performance Issues

*   **CAVLC Table Lookups:** `src/h264/cavlc.rs` uses linear searches through large tables (`TABLE95`, `TABLE9_7AND8`, etc.) for every coefficient token and run. This is a major bottleneck. These should be replaced with Tries, pre-computed lookup tables, or more efficient search algorithms.
*   **Redundant Interpolation:** In `src/h264/inter_pred.rs`, `interpolate_quarter_pel` re-calculates half-pel positions multiple times for different quarter-pel positions instead of caching or reusing them.
*   **Excessive Reader Operations:** As noted in section 1, `RbspReader::remaining()` is extremely expensive. Frequent calls to this in parsing loops will significantly slow down decoding.
*   **Unnecessary Allocations:** `remove_emulation_if_needed` always returns a `Vec<u8>`, resulting in an allocation even when no emulation bytes are present (though it tries to return an empty Vec in some cases, the caller still handles it).
*   **Inner Loop Overheads:** In `deblocking.rs` and `inter_pred.rs`, plane slices and row iterators are fetched inside the innermost loops. While `v_frame` is designed to be efficient, constant coordinate mapping and boundary checking in the inner loops add up.
*   **Deblocking Scan:** `has_nonzero_coeffs` in `src/h264/deblocking.rs` performs a linear scan `any(|&x| x != 0)` over 16 coefficients every time it checks for boundary strength. Keeping a "non-zero" flag during residual parsing would be much faster.
