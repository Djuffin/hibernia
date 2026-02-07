# Decoded Picture Buffer (DPB) in Hibernia

This document explains the implementation, usage, and limitations of the Decoded Picture Buffer (DPB) in the Hibernia H.264 decoder.

## Overview

The Decoded Picture Buffer (DPB) is a critical component of an H.264 decoder. It serves two primary purposes:
1. **Reference Storage:** It holds decoded pictures that are used as references for inter-prediction in future frames (P and B slices).
2. **Picture Reordering:** It holds decoded pictures to be output in the correct display order (Picture Order Count or POC), which may differ from the decoding order.

In Hibernia, the DPB logic is primarily contained in `src/h264/dpb.rs`.

## Key Structures

### `DpbMarking`
Defines the reference status of a picture:
- `UnusedForReference`: The picture is no longer needed as a reference.
- `UsedForShortTermReference`: The picture is a short-term reference, identified by its frame number.
- `UsedForLongTermReference(u32)`: The picture is a long-term reference, identified by a `long_term_frame_idx`.

### `DpbPicture`
Represents a single picture in the DPB. It contains:
- `picture`: The actual `Picture` struct containing the `VideoFrame` (pixel data).
- `marking`: The current `DpbMarking` status.
- `needed_for_output`: A boolean indicating if the picture still needs to be sent to the display/output buffer.
- Metadata such as `is_idr` and `structure` (Frame/TopField/BottomField).

### `DecodedPictureBuffer`
The main manager for the buffer. It contains:
- `pictures`: A `Vec<DpbPicture>` storing the pictures.
- `max_size`: The maximum number of frames the buffer can hold (derived from `max_dec_frame_buffering` in VUI).

## How It Works

### Storage and Bumping (Annex C.4.5.3)
When a new picture is decoded, it is added to the DPB using `store_picture`. If the DPB is full (`pictures.len() >= max_size`), the "bumping" process is triggered:
1. It searches for pictures that are marked as `UnusedForReference` but are still `needed_for_output`.
2. These pictures are removed from the DPB and returned as output.
3. The new picture is then pushed into the buffer.

### Reference Marking (Section 8.2.5)
The `mark_references` method implements the processes that change the status of reference pictures:
- **IDR Marking:** When an IDR slice is encountered, all existing pictures are marked as `UnusedForReference`.
- **Sliding Window:** If the number of reference frames exceeds `max_num_ref_frames`, the oldest short-term reference is marked as `UnusedForReference`.
- **Adaptive Memory Control:** Uses explicit commands (`MemoryManagementControlOperation`) from the bitstream to mark specific pictures as unused, assign long-term indices, or clear the whole buffer.

### Reference List Construction (Section 8.2.4)
Before decoding a slice, the `Decoder` uses the DPB to build the reference picture lists (e.g., `ref_pic_list0`):
1. **Initialization:** Short-term references are sorted by descending `PicNum` (for P slices), and long-term references are sorted by ascending `LongTermPicNum`.
2. **Modification:** The list can be further reordered based on `RefPicListModification` commands in the slice header.

## Usage in the Codebase

- **Initialization:** Created in `Decoder::new()` with a default size of 1.
- **Picture Storage:** In `Decoder::decode()`, a `DpbPicture` is created and stored right after parsing the slice header. The `VideoFrame` is pre-allocated.
- **Decoding:** In `Decoder::process_slice()`, the current frame (at the end of DPB) is filled with decoded macroblocks. Inter-prediction logic fetches reference frames from the DPB using indices in `ref_pic_list0`.
- **Marking:** After a slice is fully processed, `dpb.mark_references()` is called to update the status of pictures based on the slice header and NAL unit type.

## Potential Problems and Limitations

1. **Fixed DPB Size:** Currently, `Decoder::new()` initializes the DPB with a `max_size` of 1. Analysis of `Decoder::decode()` shows that this size is **not updated** when a Sequence Parameter Set (SPS) is parsed. This will cause decoding failures or incorrect output for bitstreams that require more than one reference frame.
2. **Lack of B-Slice Support:** The reference list construction logic (`construct_ref_pic_list0`) is only implemented for List 0 (used by P-slices). There is no implementation for List 1, which is required for B-slices.
3. **Incomplete POC Support:** `Decoder::calculate_poc` contains a `TODO` for POC Type 1. While Type 0 and Type 2 are implemented, streams using Type 1 will not work correctly.
4. **Interlaced Video:** There are several `TODO` comments regarding interlaced video (fields). While `DpbPictureStructure` exists, the actual decoding process (`parse_slice_header`, `calculate_poc`) does not yet support fields.
5. **Memory Overhead:** Each `DpbPicture` owns a full `VideoFrame`. In H.264, the DPB can hold up to 16 frames. For 4K video, this represents a significant amount of memory. A more optimized approach might involve a frame pool to reuse allocations.
6. **Error Handling:** Some operations in the DPB use `unwrap()` or `panic!`, which could lead to crashes on malformed bitstreams.
