# `table_9-9_a.rs` - CAVLC `run_before` Table (Part A)

This file contains a static table used in the Context-Adaptive Variable-Length Coding (CAVLC) decoding process. This table corresponds to the first part of **Table 9-9** in the ITU-T H.264 specification.

## Purpose

This table is used to decode the `run_before` syntax element in CAVLC. `run_before` represents the number of zero-valued coefficients that occur before each non-zero coefficient in a block.

This specific table is used when the number of zeros left to be coded (`zerosLeft`) is between 1 and 3.

## Structure

The file contains a single static array of tuples. Each tuple has the following structure:

```rust
(run_before, (code_for_zerosLeft_1, code_for_zerosLeft_2, code_for_zerosLeft_3))
```

- `run_before`: The decoded value of `run_before`.
- `code_for_zerosLeft_n`: A tuple `(code, length)` representing the VLC code for a specific `zerosLeft` value.
  - `code`: The binary code.
  - `length`: The length of the code in bits.

This table is one of several tables used for decoding `run_before`, and the selection of the correct table depends on the value of `zerosLeft`.
