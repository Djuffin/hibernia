# `slice.rs` - H.264 Slice Data

This file defines the data structures and logic for handling H.264 slices. A slice is a sequence of macroblocks that can be decoded independently of other slices in the same picture (to some extent).

## Key Data Structures and Enums

### `SliceType`

An enum representing the type of a slice. The slice type determines how the macroblocks in the slice are coded.

- `P`: A predicted slice, which can use intra-prediction or inter-prediction from a single reference picture list.
- `B`: A bi-predictive slice, which can use intra-prediction or inter-prediction from two reference picture lists.
- `I`: An intra-coded slice, where all macroblocks are intra-predicted.
- `SP`: A switching P-slice, used for switching between different bitstreams.
- `SI`: A switching I-slice, used for switching between different bitstreams.

### `DeblockingFilterIdc`

An enum that controls the behavior of the deblocking filter.

- `On`: The deblocking filter is enabled for all edges.
- `Off`: The deblocking filter is disabled.
- `OnExceptSliceBounds`: The deblocking filter is enabled for all edges except those that are on the slice boundaries.

### `SliceHeader`

A struct that represents the header of a slice. It contains information such as:

- `first_mb_in_slice`: The address of the first macroblock in the slice.
- `slice_type`: The type of the slice.
- `pic_parameter_set_id`: The ID of the PPS used by the slice.
- `frame_num`: The frame number.
- `idr_pic_id`: The ID of the IDR picture, if the slice is part of an IDR picture.
- `slice_qp_delta`: The QP delta for the slice.
- `deblocking_filter_idc`: The deblocking filter IDC.

### `Slice`

A struct that represents a slice. It contains:

- `sps`: The Sequence Parameter Set for the slice.
- `pps`: The Picture Parameter Set for the slice.
- `header`: The slice header.
- `macroblocks`: A `Vec` of the macroblocks in the slice.

## Core Functionality

### `Slice::new`

Creates a new `Slice` with the given SPS, PPS, and header.

### `Slice::MbaffFrameFlag`

A method that returns `true` if the slice is part of a Macroblock-Adaptive Frame-Field (MBAFF) coded frame.

### `Slice::get_mb`

Returns a reference to the macroblock at the given address.

### `Slice::get_mb_neighbor`

Returns a reference to a neighboring macroblock.

### `Slice::has_mb_neighbor`

Checks if a neighboring macroblock exists.

### `Slice::append_mb`

Appends a macroblock to the slice.

### `Slice::get_macroblock_count`

Returns the number of macroblocks in the slice.

### `Slice::get_next_mb_addr`

Returns the address of the next macroblock to be added to the slice.

### `Slice::get_mb_location`

Returns the top-left coordinates of a macroblock given its address.

## Testing

The file includes a `tests` module with a unit test for slice addressing, which verifies the correctness of the macroblock and neighbor access methods.
