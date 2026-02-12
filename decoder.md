# Decoder API Redesign

This document outlines the proposed redesign of the H.264 Decoder API in `hibernia`. The goal is to make the API more idiomatic, convenient, performant, and compatible with existing codebases while enabling streaming use cases.

## 1. Current State Analysis

### 1.1 Structure
The current implementation relies on a single `Decoder` struct (`src/h264/decoder.rs`) which handles:
- **Stream Parsing**: Extracting NAL units from a raw byte stream (Annex B).
- **NAL Parsing**: Decoding NAL unit headers and payloads.
- **State Management**: Maintaining DPB (Decoded Picture Buffer), SPS, PPS.
- **Output Management**: Storing decoded frames in an internal buffer (`output_frames`).

### 1.2 Limitations
- **Memory Usage**: The `decode(&[u8])` method requires the entire encoded stream to be in memory. It uses `BitReader` which wraps a slice.
- **Implicit State**: Output frames are stored internally. The user must manually call `get_frame_buffer()` to access them and `clear_frame_buffer()` to release memory. This is error-prone and stateful.
- **Tight Coupling**: The NAL unit extraction logic is embedded within the `decode` loop, making it difficult to use the decoder with a custom stream source or network packet iterator.
- **Performance**: `remove_emulation_if_needed` allocates a new `Vec<u8>` for every NAL unit payload, even if no emulation bytes are present. This adds unnecessary overhead.

## 2. Design Goals

- **Idiomatic Rust**: Utilize `Iterator` for stream processing and `Result` for error handling. Adhere to standard traits like `Read` and `BufRead`.
- **Convenient**: Simplify the API for common use cases (e.g., file decoding, frame iteration) while offering low-level control for advanced users.
- **Performant**: Minimize memory allocations and copies. Support zero-copy parsing where possible (e.g., using `Cow` or on-the-fly emulation removal).
- **Streaming Support**: Enable decoding of streams (e.g., network sockets, large files) without loading the entire content into memory.
- **Compatibility**: Provide a migration path for existing code (e.g., `main.rs`) and maintain high-level behavior.

## 3. Proposed Architecture

The redesign splits the monolithic `Decoder` into two distinct layers:
1.  **Transport Layer (`NalParser`)**: Handles the extraction of NAL units from a byte stream.
2.  **Decoding Layer (`H264Decoder`)**: Handles the decoding of individual NAL units and frame management.

### 3.1. Transport Layer: `NalParser`

A new `NalParser` struct will be introduced to handle Annex B stream parsing. It will implement `Iterator` and yield `Result<NalUnit>`.

```rust
pub enum NalUnit<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}

pub struct NalParser<R> {
    reader: R,
    buffer: Vec<u8>,
}

impl<R: BufRead> Iterator for NalParser<R> {
    type Item = Result<Vec<u8>, std::io::Error>; // or Cow<'a, [u8]> if possible

    fn next(&mut self) -> Option<Self::Item> {
        // Implementation to find start codes (0x000001 or 0x00000001)
        // and extract NAL units.
    }
}
```

### 3.2. Decoding Layer: `H264Decoder`

The `Decoder` struct will be refactored (or wrapped) to focus solely on decoding logic. It will accept NAL units and return decoded frames.

```rust
pub struct Decoder {
    // Internal state (DPB, Context, etc.)
}

impl Decoder {
    pub fn new() -> Self;

    /// Decodes a single NAL unit.
    /// Returns a list of frames that are ready to be output (e.g. from DPB).
    pub fn decode_nal(&mut self, nal_data: &[u8]) -> Result<Vec<VideoFrame>, DecodingError>;

    /// Flushes the decoder, returning any remaining frames in the DPB.
    pub fn flush(&mut self) -> Result<Vec<VideoFrame>, DecodingError>;
}
```

### 3.3. High-Level API (Stream Decoder)

To maintain convenience and support the most common use case (decoding a stream/file), we can provide a high-level wrapper or helper.

```rust
pub struct StreamDecoder<R> {
    parser: NalParser<R>,
    decoder: Decoder,
}

impl<R: BufRead> StreamDecoder<R> {
    pub fn new(reader: R) -> Self;
}

impl<R: BufRead> Iterator for StreamDecoder<R> {
    type Item = Result<VideoFrame, DecodingError>;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. Get next NAL from parser.
        // 2. Feed to decoder.
        // 3. Return frames (buffering if multiple frames returned).
    }
}
```

## 4. Detailed Refactoring Plan

### Phase 1: NAL Parser Extraction
1.  **Create `src/h264/nal_parser.rs`**: Implement `NalParser` that takes a `BufRead` (or `Read` and buffers internally) and yields `Vec<u8>` containing NAL units (including headers, stripping start codes).
2.  **Test `NalParser`**: Ensure it correctly handles various start code formats and edge cases (split across reads).

### Phase 2: Decoder Core Refactoring
1.  **Refactor `Decoder::decode`**:
    - Rename current `decode` to `decode_nal` (or similar).
    - Remove the loop that searches for start codes. It should assume the input `&[u8]` is a single NAL unit.
    - Change return type from `()` to `Vec<VideoFrame>`.
    - Instead of `self.output_frames.extend(...)`, return the frames directly.
    - Remove `output_frames` field from `Decoder`.
2.  **Handle Emulation Prevention**:
    - Ideally, `decode_nal` should handle emulation prevention removal.
    - Current `remove_emulation_if_needed` returns `Vec<u8>`. Optimize this to return `Cow<[u8]>` to avoid allocation when not needed.

### Phase 3: High-Level API & Migration
1.  **Implement `StreamDecoder`**: Combine `NalParser` and `Decoder`.
2.  **Update `src/main.rs`**:
    - Replace the manual file reading and buffer management with `StreamDecoder`.
    - Iterate over frames and write to Y4M.
3.  **Update Tests**:
    - Refactor `tests/` to use the new API.
    - Ensure `test_decoding_against_gold` still passes.

### Phase 4: Optimization (Optional/Future)
1.  **Zero-Copy Parsing**:
    - Modify `BitReader` / `RbspReader` to handle emulation prevention on-the-fly (iterator over bits/bytes skipping `0x03`).
    - This would allow `decode_nal` to take `&[u8]` directly from the memory-mapped file or network buffer without an intermediate `Vec<u8>` allocation for the payload.

## 5. Compatibility Note
The proposed `StreamDecoder` can serve as a drop-in replacement for the logic currently in `main.rs`. The existing `Decoder` struct can be kept (with the new `decode_nal` method) for users who need fine-grained control over NAL unit processing.

## 6. Example Usage (New API)

```rust
use std::fs::File;
use std::io::BufReader;
use hibernia::h264::StreamDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("video.h264")?;
    let reader = BufReader::new(file);

    let decoder = StreamDecoder::new(reader);

    for frame_result in decoder {
        let frame = frame_result?;
        // Process frame (e.g., save to file, display)
    }

    Ok(())
}
```
