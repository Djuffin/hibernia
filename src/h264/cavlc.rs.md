# `cavlc.rs` - Context-Adaptive Variable-Length Coding (CAVLC)

This file is intended to contain the implementation of the Context-Adaptive Variable-Length Coding (CAVLC) entropy coding scheme, as specified in the H.264 standard.

**Note:** The current implementation is a placeholder and does not yet contain the full CAVLC logic. The `parse_residual_block` function is a stub that will be implemented in the future.

## Core Functionality (Planned)

### `parse_residual_block`

This function will be responsible for parsing a residual block that has been coded using CAVLC. It will read the variable-length codes from the bitstream and reconstruct the quantized transform coefficients.

The function will take the following arguments:
- `input`: A mutable reference to a `BitReader`.
- `levels`: A mutable slice to store the reconstructed coefficient levels.
- `nc`: The number of non-zero coefficients, which is determined by the context of the block (i.e., the `nC` value of its neighbors).

The function will return the total number of non-zero coefficients in the block.
