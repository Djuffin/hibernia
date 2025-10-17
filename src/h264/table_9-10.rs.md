# `table_9-10.rs` - CAVLC `total_zeros` Table

This file contains a static table used in the Context-Adaptive Variable-Length Coding (CAVLC) decoding process. Specifically, this table corresponds to **Table 9-10** in the ITU-T H.264 specification.

## Purpose

This table is used to decode the `total_zeros` syntax element in CAVLC. `total_zeros` represents the number of zero-valued coefficients before the last non-zero coefficient in a block.

The table is indexed by `TotalCoeff` (the total number of non-zero coefficients in the block). Each entry in the table provides a set of variable-length codes for different values of `total_zeros`.

## Structure

The file contains a single static array of tuples. Each tuple has the following structure:

```rust
(TotalCoeff, (code_for_total_zeros_1, ...))
```

- `TotalCoeff`: The total number of non-zero coefficients.
- `code_for_total_zeros_n`: A tuple `(code, length)` representing the VLC code for a specific `total_zeros` value.
  - `code`: The binary code.
  - `length`: The length of the code in bits.

This table is used by the CAVLC parser to look up the appropriate VLC table for decoding the `total_zeros` value based on the already-decoded `TotalCoeff`.
