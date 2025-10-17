# `mod.rs` - H.264 Module

This file serves as the root of the `h264` module, which contains all the logic for decoding H.264 video streams. It declares the sub-modules of the `h264` module and defines several top-level enums and structs that are used throughout the decoder.

## Sub-modules

- `cavlc`: Context-Adaptive Variable-Length Coding (CAVLC) entropy decoding.
- `decoder`: The main H.264 decoder.
- `macroblock`: Data structures and logic for H.264 macroblocks.
- `nal`: Network Abstraction Layer (NAL) unit handling.
- `parser`: Bitstream parsing logic.
- `pps`: Picture Parameter Set (PPS) data structures.
- `rbsp`: Raw Byte Sequence Payload (RBSP) reading.
- `residual`: Residual data handling.
- `slice`: Slice data structures and logic.
- `sps`: Sequence Parameter Set (SPS) data structures.
- `tables`: Tables and constants used in the H.264 decoding process.

## Key Enums and Structs

### `Profile`

An enum representing the H.264 profiles (e.g., Baseline, Main, High).

### `ChromaFormat`

An enum representing the chroma subsampling format (e.g., YUV420, YUV422, YUV444).

### `ColorPlane`

An enum representing the color planes (Y, Cb, Cr).

### `Point`

A struct representing a 2D point with `x` and `y` coordinates.

### `Size`

A struct representing a size with `width` and `height` dimensions.

### `Rect`

A struct representing a rectangle with an `origin` (`Point`) and a `size` (`Size`).
