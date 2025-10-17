# `main.rs` - Hibernia H.264 Decoder Executable

This file is the main entry point for the Hibernia H.264 decoder executable. It handles command-line argument parsing, file I/O, and the overall decoding workflow.

## Modules

- `diag`: Diagnostic logging.
- `h264`: The main H.264 decoder library.
- `y4m_cmp`: YUV4MPEG2 (.y4m) frame comparison utility.

## Core Functionality

### `main`

The `main` function orchestrates the decoding process:

1.  **Initialization**: Initializes the diagnostic logger using `diag::init`.
2.  **Argument Parsing**: Parses command-line arguments to get the input H.264 bitstream file. It defaults to "data/NL1_Sony_D.jsv" if no file is provided.
3.  **File Reading**: Reads the input H.264 bitstream file into a byte buffer.
4.  **Decoding**: Creates a new `h264::decoder::Decoder` instance and calls its `decode` method with the input buffer.
5.  **Output**:
    -   Retrieves the decoded frame from the decoder's frame buffer.
    -   Writes the decoded frame to a YUV4MPEG2 (.y4m) file named "output.y4m".
6.  **Comparison (Optional)**:
    -   If a reference YUV4MPEG2 file named "data/NL1_Sony_D.y4m" exists, it reads this file.
    -   It then compares the decoded frame with the reference frame using the `y4m_cmp::compare_frames` function.
    -   The comparison result is printed to the console.

## Usage

The executable can be run from the command line, optionally providing a path to an H.264 bitstream file:

```bash
cargo run -- [path/to/video.h264]
```

If no path is provided, it will attempt to decode the default file. The decoded output will be saved as `output.y4m`.
