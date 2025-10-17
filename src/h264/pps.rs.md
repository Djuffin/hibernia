# `pps.rs` - Picture Parameter Set (PPS)

This file defines the data structures related to the H.264 Picture Parameter Set (PPS). The PPS contains information that applies to one or more pictures in a video sequence.

## Key Data Structures

### `SliceRect`

A struct representing a rectangular region of a slice, used for defining slice groups.

### `SliceGroupChangeType`

An enum representing the type of change for a slice group, used in flexible macroblock ordering (FMO).

### `SliceGroup`

An enum representing a slice group, which is a mechanism for partitioning a picture into multiple slices. This is used in FMO.

### `PicParameterSet`

A struct that represents a Picture Parameter Set. It contains the following fields:

- `pic_parameter_set_id`: The ID of the PPS.
- `seq_parameter_set_id`: The ID of the SPS that this PPS refers to.
- `entropy_coding_mode_flag`: A flag that selects the entropy decoding method (CAVLC or CABAC).
- `bottom_field_pic_order_in_frame_present_flag`: A flag related to picture order count for interlaced video.
- `slice_group`: An `Option<SliceGroup>` that contains information about slice groups, if they are used.
- `num_ref_idx_l0_default_active_minus1` and `num_ref_idx_l1_default_active_minus1`: The default number of reference pictures for L0 and L1 prediction.
- `weighted_pred_flag`: A flag that enables weighted prediction.
- `weighted_bipred_idc`: The type of weighted prediction to be used for B-slices.
- `pic_init_qp_minus26`: The initial quantization parameter (QP).
- `pic_init_qs_minus26`: The initial quantization parameter for SP/SI slices.
- `chroma_qp_index_offset`: An offset for the chroma QP.
- `deblocking_filter_control_present_flag`: A flag that indicates whether deblocking filter control parameters are present in the slice header.
- `constrained_intra_pred_flag`: A flag that enables constrained intra prediction.
- `redundant_pic_cnt_present_flag`: A flag that indicates whether a redundant picture count is present.
- `transform_8x8_mode_flag`: A flag that enables the use of 8x8 transforms.
- `second_chroma_qp_index_offset`: A second offset for the chroma QP.
