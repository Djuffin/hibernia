# `decoder.rs` - H.264 Decoder Implementation

This file contains the core implementation of the H.264 video decoder. It defines the main `Decoder` struct and its associated methods for parsing and decoding H.264 bitstreams.

## Key Data Structures

### `DecodingError`

An enum representing possible errors that can occur during the decoding process.

- `MisformedData(String)`: Indicates that the input bitstream is not well-formed.
- `OutOfRange(String)`: Indicates that a value in the bitstream is outside the valid range.
- `Wtf`: A catch-all for unexpected errors.

### `DecoderContext`

A struct that holds the state of the decoder, including the Sequence Parameter Sets (SPS) and Picture Parameter Sets (PPS) that have been parsed from the bitstream.

- `sps`: A `Vec` of `sps::SequenceParameterSet`.
- `pps`: A `Vec` of `pps::PicParameterSet`.

### `Decoder`

The main decoder struct.

- `context`: A `DecoderContext` instance.
- `frame_buffer`: An `Option<VideoFrame>` that holds the decoded video frame.

## Core Functionality

### `Decoder::new()`

Creates a new `Decoder` instance with default values.

### `Decoder::decode(&mut self, data: &[u8])`

The main decoding loop. It takes a byte slice containing the H.264 bitstream and processes it NAL unit by NAL unit.

- It parses the NAL unit header to determine the type of NAL unit.
- It handles different NAL unit types, including:
    - `SeqParameterSet`: Parses the SPS and stores it in the `DecoderContext`.
    - `PicParameterSet`: Parses the PPS and stores it in the `DecoderContext`.
    - `IDRSlice` and `NonIDRSlice`: Parses the slice header and slice data, and then processes the slice to reconstruct the video frame.

### `Decoder::process_slice(&mut self, slice: &mut Slice)`

Processes a single slice of a video frame.

- It iterates through the macroblocks in the slice.
- For each macroblock, it determines the prediction mode (e.g., Intra_4x4, Intra_16x16) and reconstructs the pixel values.
- It handles both luma and chroma components.

## Intra Prediction Functions

This file contains several functions for performing intra prediction, which is a key part of H.264 decoding for I-frames. These functions reconstruct a block of pixels based on the values of neighboring, already-decoded pixels.

### `render_luma_4x4_intra_prediction`

Handles Intra_4x4 prediction for luma samples. It implements the various prediction modes defined in the H.264 specification (e.g., Vertical, Horizontal, DC, Diagonal Down-Left, etc.).

### `render_luma_16x16_intra_prediction`

Handles Intra_16x16 prediction for luma samples. It implements the four prediction modes for this block size (Vertical, Horizontal, DC, Plane).

### `render_chroma_intra_prediction`

Handles intra prediction for chroma samples. It implements the four prediction modes for chroma (DC, Horizontal, Vertical, Plane).

## Helper Functions and Structs

### `Surroundings4x4`

A helper struct for Intra_4x4 prediction that stores the neighboring pixel values for a 4x4 block.

### `point_to_plain_offset`

A utility function to convert a `Point` struct to a `PlaneOffset` struct.

## Overall Flow

1. The `decode` method is called with the H.264 bitstream.
2. The `decode` method parses NAL units.
3. When an SPS or PPS is encountered, it is parsed and stored in the `DecoderContext`.
4. When a slice NAL unit is encountered, the slice header and data are parsed.
5. The `process_slice` method is called to reconstruct the frame.
6. `process_slice` iterates through macroblocks, calling the appropriate intra prediction function (`render_luma_4x4_intra_prediction`, `render_luma_16x16_intra_prediction`, `render_chroma_intra_prediction`) to reconstruct the pixel data.
7. The reconstructed pixel data is written to the `frame_buffer`.
