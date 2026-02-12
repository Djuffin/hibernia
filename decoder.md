# Decoder API Redesign

This document outlines the proposed redesign of the H.264 Decoder API in `hibernia`. The goal is to make the API more idiomatic, convenient, performant, and compatible with existing codebases.

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
- **Compatibility**: Provide a migration path for existing code (e.g., `main.rs`) and maintain high-level behavior.

## 3. Proposed Architecture

The redesign splits the monolithic `Decoder` into two distinct layers:
1.  **Transport Layer (`NalParser`)**: Handles the extraction of NAL units from a byte stream.
2.  **Decoding Layer (`Decoder`)**: Handles the decoding of individual NAL units and frame management.

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

### 3.2. Decoding Layer: `Decoder`

The `Decoder` struct will be refactored to focus solely on decoding logic. It will accept NAL units and allow decoupled retrieval of decoded frames.

```rust
pub struct Decoder {
    // Internal state (DPB, Context, etc.)
}

impl Decoder {
    pub fn new() -> Self;

    /// Decodes a single NAL unit.
    /// This method updates the internal state of the decoder but does not return frames directly.
    /// Frames are pushed to an internal queue and can be retrieved using `retrieve_frame`.
    pub fn decode(&mut self, nal_data: &[u8]) -> Result<(), DecodingError>;

    /// Retrieves the next available frame from the decoder's output queue.
    /// Returns `Some(VideoFrame)` if a frame is available, or `None` if the queue is empty.
    pub fn retrieve_frame(&mut self) -> Option<VideoFrame>;

    /// Flushes the decoder, forcing any remaining frames in the DPB to be output.
    /// This should be called at the end of the stream.
    /// Call `retrieve_frame` repeatedly after flushing until it returns `None`.
    pub fn flush(&mut self) -> Result<(), DecodingError>;
}
```

## 4. Detailed Refactoring Plan

### Phase 1: NAL Parser Extraction
1.  **Create `src/h264/nal_parser.rs`**: Implement `NalParser` that takes a `BufRead` (or `Read` and buffers internally) and yields `Vec<u8>` containing NAL units (including headers, stripping start codes).
2.  **Test `NalParser`**: Ensure it correctly handles various start code formats and edge cases (split across reads).

### Phase 2: Decoder Core Refactoring
1.  **Refactor `Decoder::decode`**:
    - Rename current `decode` to `decode_nal` (or simply `decode` as proposed).
    - Remove the loop that searches for start codes. It should assume the input `&[u8]` is a single NAL unit.
    - Decouple output: instead of accumulating in `output_frames` and waiting for `get_frame_buffer`, implement an internal queue (e.g., `VecDeque`) and expose `retrieve_frame` to pop from it.
2.  **Handle Emulation Prevention**:
    - Ideally, `decode` should handle emulation prevention removal.
    - Current `remove_emulation_if_needed` returns `Vec<u8>`. Optimize this to return `Cow<[u8]>` to avoid allocation when not needed.

### Phase 3: Migration
1.  **Update `src/main.rs`**:
    - Use `NalParser` to read the file.
    - Iterate over NALs and feed them to `Decoder::decode`.
    - Retrieve frames using `Decoder::retrieve_frame` and write to Y4M.
2.  **Update Tests**:
    - Refactor `tests/` to use the new API.
    - Ensure `test_decoding_against_gold` still passes.

## 5. Example Usage (New API)

```rust
use std::fs::File;
use std::io::BufReader;
use hibernia::h264::{Decoder, NalParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("video.h264")?;
    let reader = BufReader::new(file);
    let parser = NalParser::new(reader);
    let mut decoder = Decoder::new();

    for nal_result in parser {
        let nal = nal_result?;
        decoder.decode(&nal)?;

        while let Some(frame) = decoder.retrieve_frame() {
             // Process frame (e.g., save to file, display)
        }
    }

    decoder.flush()?;
    while let Some(frame) = decoder.retrieve_frame() {
         // Process remaining frames
    }

    Ok(())
}
```
