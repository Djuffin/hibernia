# `y4m_cmp.rs` - YUV4MPEG2 (.y4m) Frame Comparison

This file provides a utility for comparing two YUV4MPEG2 (.y4m) frames. It is used for testing and verification purposes to ensure that the output of the Hibernia decoder matches a known-good reference frame.

## Core Functionality

### `compare_plane`

Compares a single plane (Y, U, or V) of two frames. It iterates through the pixels of the plane and returns `Some((x, y, actual_value, expected_value))` if a mismatch is found, where `(x, y)` are the coordinates of the mismatched pixel. If the planes are identical, it returns `None`.

### `compare_frames`

Compares two complete `y4m::Frame`s. It calls `compare_plane` for each of the Y, U, and V planes and aggregates the results into a string.

The function returns a string that describes any mismatches found. If the frames are identical, it returns an empty string.

## Usage

The `compare_frames` function is called from `main.rs` to compare the decoded frame with a reference frame. The result of the comparison is then printed to the console. This provides a quick and easy way to verify the correctness of the decoder's output.
