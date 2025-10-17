# `parser.rs` - H.264 Bitstream Parser

This file contains the functions responsible for parsing the H.264 bitstream, including Network Abstraction Layer (NAL) units, Sequence Parameter Sets (SPS), Picture Parameter Sets (PPS), slice headers, and macroblock data.

## Key Data Structures and Types

### `BitReader<'a>`

A type alias for `rbsp::RbspReader<'a>`, which is a custom bitstream reader for reading Raw Byte Sequence Payload (RBSP) data.

### `ParseResult<T>`

A type alias for `rbsp::ParseResult<T>`, which is a `Result` type for parsing operations.

## Core Parsing Functions

### `parse_nal_header`

Parses the NAL unit header, which includes the `nal_ref_idc` and `nal_unit_type`.

### `parse_sps`

Parses a Sequence Parameter Set (SPS) NAL unit. The SPS contains high-level information about the video sequence, such as profile, level, resolution, and frame rate.

### `parse_pps`

Parses a Picture Parameter Set (PPS) NAL unit. The PPS contains information that applies to one or more pictures in the sequence, such as entropy coding mode and quantization parameters.

### `parse_slice_header`

Parses the slice header, which contains information about a single slice, such as the slice type, frame number, and picture parameter set ID.

### `parse_slice_data`

Parses the slice data, which contains the coded macroblocks for a slice. It iterates through the macroblocks in the slice and calls `parse_macroblock` for each one.

### `parse_macroblock`

Parses a single macroblock. It determines the macroblock type and then parses the corresponding data, including prediction modes, coded block pattern, and residual data.

### `parse_residual`

Parses the residual data for a macroblock. The residual is the difference between the predicted and actual pixel values, and it is transform-coded and quantized. This function calls `parse_residual_block` (from the `cavlc` module) to parse the transform coefficients.

## Helper Macros and Functions

### `cast_or_error!`

A macro for safely casting a value from one type to another, returning an error if the cast fails.

### `expect_value!`

A macro for reading a value from the bitstream and checking if it matches an expected value.

### `read_value!`

A macro for reading a value of a specific type (e.g., unsigned integer, signed integer, flag) from the bitstream.

### `rbsp_trailing_bits`

A function that checks for the RBSP trailing bits, which are used to mark the end of an RBSP.

### `more_rbsp_data`

A function that checks if there is more data to be read in the current RBSP.

### `count_bytes_till_start_code`

A function that counts the number of bytes until the next start code prefix (`0x000001` or `0x00000001`).

### `remove_emulation_if_needed`

A function that removes emulation prevention bytes (`0x03`) from the bitstream. Emulation prevention bytes are inserted to prevent start code prefixes from appearing accidentally in the bitstream.

### `calc_prev_intra4x4_pred_mode`

A function that calculates the predicted Intra_4x4 prediction mode for a block based on the modes of its neighbors.

## Testing

The file includes a `tests` module with unit tests for parsing SPS, PPS, slice headers, and NAL headers, as well as for the helper functions. These tests use sample bitstream data to verify the correctness of the parsing logic.
