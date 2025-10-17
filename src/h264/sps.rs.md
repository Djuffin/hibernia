# `sps.rs` - Sequence Parameter Set (SPS)

This file defines the data structures related to the H.264 Sequence Parameter Set (SPS). The SPS contains high-level information about the video sequence that remains constant for a group of consecutive pictures.

## Key Data Structures

### `HdrParameters`

A placeholder struct for High Dynamic Range (HDR) parameters.

### `VuiParameters`

A struct that represents the Video Usability Information (VUI) parameters. VUI provides optional information that can be used by a display device to enhance the presentation of the video. It includes parameters such as:

- Aspect ratio
- Color information (primaries, transfer characteristics, matrix coefficients)
- Timing information

### `FrameCrop`

A struct that specifies the cropping rectangle for the output frame.

### `SequenceParameterSet`

A struct that represents a Sequence Parameter Set. It contains the following fields:

- `profile`: The H.264 profile (e.g., Baseline, Main, High).
- `level_idc`: The level of the profile.
- `seq_parameter_set_id`: The ID of the SPS.
- `chroma_format_idc`: The chroma subsampling format.
- `log2_max_frame_num_minus4`: The maximum frame number.
- `pic_order_cnt_type`: The picture order count type.
- `max_num_ref_frames`: The maximum number of reference frames.
- `pic_width_in_mbs_minus1`: The width of the picture in macroblocks.
- `pic_height_in_map_units_minus1`: The height of the picture in macroblocks.
- `frame_mbs_only_flag`: A flag that indicates whether the sequence contains only frames (no fields).
- `frame_cropping`: An `Option<FrameCrop>` that contains the frame cropping parameters, if present.
- `vui_parameters`: An `Option<VuiParameters>` that contains the VUI parameters, if present.

## Core Functionality

### `SequenceParameterSet` Methods

The `SequenceParameterSet` struct provides several helper methods for calculating derived values:

- `bits_in_frame_num`: Returns the number of bits used to represent the frame number.
- `bits_in_max_pic_order_cnt`: Returns the number of bits used to represent the picture order count.
- `ChromaArrayType`: Returns the chroma array type.
- `pic_width_in_mbs`: Returns the width of the picture in macroblocks.
- `pic_width`: Returns the width of the picture in pixels.
- `pic_hight_in_mbs`: Returns the height of the picture in macroblocks.
- `pic_hight`: Returns the height of the picture in pixels.
- `pic_size_in_mbs`: Returns the total number of macroblocks in a picture.
