# `tables_9-7and8.rs` - CAVLC `level_prefix` and `level_suffix` Tables

This file contains a static table used in the Context-Adaptive Variable-Length Coding (CAVLC) decoding process. This table combines information from **Table 9-7** and **Table 9-8** in the ITU-T H.264 specification.

## Purpose

This table is used to decode the `level_prefix` and `level_suffix` syntax elements in CAVLC. These elements are used to reconstruct the absolute value of a transform coefficient's level.

- **Table 9-7**: Defines the codes for the `level_prefix`. The `level_prefix` determines the most significant bits of the coefficient's level.
- **Table 9-8**: Defines the length of the `level_suffix`, which contains the least significant bits of the level.

This combined table provides a mapping from the decoded `level_prefix` to the corresponding VLC code and the length of the `level_suffix`.

## Structure

The file contains a single static array of tuples. Each tuple has the following structure:

```rust
(level_prefix, (code_for_suffixLength_0, code_for_suffixLength_1, ...))
```

- `level_prefix`: The decoded value of `level_prefix`.
- `code_for_suffixLength_n`: A tuple `(code, length)` representing the VLC code for a specific `level_suffix` length.
  - `code`: The binary code.
  - `length`: The length of the code in bits.

This table is used by the CAVLC parser to decode the level of each non-zero coefficient in a residual block.
