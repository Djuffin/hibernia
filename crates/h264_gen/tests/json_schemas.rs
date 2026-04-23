use hibernia::h264::pps::{PicParameterSet, SliceGroup, SliceGroupChangeType, SliceRect};
use hibernia::h264::slice::{
    DeblockingFilterIdc, DecRefPicMarking, MemoryManagementControlOperation, PredWeightTable,
    RefPicListModification, RefPicListModifications, SliceHeader, SliceType, WeightingFactors,
};
use hibernia::h264::sps::{FrameCrop, SequenceParameterSet, VuiParameters};
use hibernia::h264::{ChromaFormat, ColorPlane, Profile};

#[test]
fn test_sps_json_deserialization() {
    let sps = SequenceParameterSet {
        profile: Profile::High,
        constraint_set0_flag: true,
        constraint_set1_flag: false,
        constraint_set2_flag: true,
        constraint_set3_flag: false,
        constraint_set4_flag: true,
        constraint_set5_flag: false,
        level_idc: 42,
        seq_parameter_set_id: 1,
        chroma_format_idc: ChromaFormat::YUV422,
        separate_color_plane_flag: true,
        bit_depth_luma_minus8: 2,
        bit_depth_chroma_minus8: 2,
        qpprime_y_zero_transform_bypass_flag: true,
        seq_scaling_matrix_present_flag: false,
        log2_max_frame_num_minus4: 4,
        pic_order_cnt_type: 1,
        log2_max_pic_order_cnt_lsb_minus4: 5,
        delta_pic_order_always_zero_flag: true,
        offset_for_non_ref_pic: -1,
        offset_for_top_to_bottom_field: 2,
        offset_for_ref_frame: vec![1, -2, 3],
        max_num_ref_frames: 4,
        gaps_in_frame_num_value_allowed_flag: true,
        pic_width_in_mbs_minus1: 119,
        pic_height_in_map_units_minus1: 67,
        frame_mbs_only_flag: false,
        mb_adaptive_frame_field_flag: true,
        direct_8x8_inference_flag: true,
        frame_cropping: Some(FrameCrop { top: 1, left: 2, right: 3, bottom: 4 }),
        vui_parameters: Some(VuiParameters {
            aspect_ratio_info_present_flag: true,
            aspect_ratio_idc: 255,
            sar_width: 16,
            sar_height: 9,
            overscan_info_present_flag: true,
            overscan_appropriate_flag: true,
            video_signal_type_present_flag: true,
            video_format: 5,
            video_full_range_flag: true,
            color_description_present_flag: true,
            color_primaries: 1,
            transfer_characteristics: 2,
            matrix_coefficients: 3,
            chroma_loc_info_present_flag: true,
            chroma_sample_loc_type_top_field: 1,
            chroma_sample_loc_type_bottom_field: 2,
            timing_info_present_flag: true,
            num_units_in_tick: 1000,
            time_scale: 60000,
            fixed_frame_rate_flag: true,
            nal_hrd_parameters: None,
            vcl_hrd_parameters: None,
            low_delay_hrd_flag: false,
            pic_struct_present_flag: true,
            bitstream_restriction_flag: true,
            motion_vectors_over_pic_boundaries_flag: true,
            max_bytes_per_pic_denom: 0,
            max_bits_per_mb_denom: 0,
            log2_max_mv_length_horizontal: 10,
            log2_max_mv_length_vertical: 10,
            max_num_reorder_frames: 2,
            max_dec_frame_buffering: 3,
        }),
    };

    let json = serde_json::to_string(&sps).expect("Failed to serialize SPS");
    let sps_deserialized: SequenceParameterSet =
        serde_json::from_str(&json).expect("Failed to deserialize SPS");

    assert_eq!(sps, sps_deserialized);
}

#[test]
fn test_pps_json_deserialization() {
    let pps = PicParameterSet {
        pic_parameter_set_id: 2,
        seq_parameter_set_id: 1,
        entropy_coding_mode_flag: true,
        bottom_field_pic_order_in_frame_present_flag: true,
        slice_group: Some(SliceGroup::Foreground {
            rectangles: vec![
                SliceRect { top_left: 0, bottom_right: 10 },
                SliceRect { top_left: 11, bottom_right: 20 },
            ],
        }),
        num_ref_idx_l0_default_active_minus1: 3,
        num_ref_idx_l1_default_active_minus1: 4,
        weighted_pred_flag: true,
        weighted_bipred_idc: 2,
        pic_init_qp_minus26: -5,
        pic_init_qs_minus26: 3,
        chroma_qp_index_offset: 2,
        deblocking_filter_control_present_flag: true,
        constrained_intra_pred_flag: true,
        redundant_pic_cnt_present_flag: true,
        transform_8x8_mode_flag: true,
        second_chroma_qp_index_offset: -2,
    };

    let json = serde_json::to_string(&pps).expect("Failed to serialize PPS");
    let pps_deserialized: PicParameterSet =
        serde_json::from_str(&json).expect("Failed to deserialize PPS");

    assert_eq!(pps, pps_deserialized);
}

#[test]
fn test_slice_header_json_deserialization() {
    let header = SliceHeader {
        first_mb_in_slice: 42,
        slice_type: SliceType::B,
        pic_parameter_set_id: 2,
        color_plane: Some(ColorPlane::Cb),
        frame_num: 15,
        field_pic_flag: true,
        bottom_field_flag: Some(true),
        idr_pic_id: Some(3),
        pic_order_cnt_lsb: Some(128),
        delta_pic_order_cnt_bottom: Some(-10),
        delta_pic_order_cnt: [5, -5],
        redundant_pic_cnt: Some(1),
        direct_spatial_mv_pred_flag: Some(true),
        num_ref_idx_l0_active_minus1: 2,
        num_ref_idx_l1_active_minus1: 3,
        ref_pic_list_modification: RefPicListModifications {
            list0: vec![RefPicListModification::RemapShortTermNegative(5)],
            list1: vec![RefPicListModification::RemapLongTerm(2)],
        },
        pred_weight_table: Some(PredWeightTable {
            luma_log2_weight_denom: 2,
            chroma_log2_weight_denom: 3,
            list0: vec![WeightingFactors {
                luma_weight: 4,
                luma_offset: 1,
                chroma_weights: [5, 6],
                chroma_offsets: [-1, -2],
            }],
            list1: vec![WeightingFactors {
                luma_weight: 8,
                luma_offset: -2,
                chroma_weights: [2, 3],
                chroma_offsets: [1, 2],
            }],
        }),
        dec_ref_pic_marking: Some(DecRefPicMarking {
            no_output_of_prior_pics_flag: Some(false),
            long_term_reference_flag: Some(true),
            adaptive_ref_pic_marking_mode_flag: Some(true),
            memory_management_operations: vec![
                MemoryManagementControlOperation::MarkShortTermUnused {
                    difference_of_pic_nums_minus1: 3,
                },
                MemoryManagementControlOperation::MarkAllUnused,
            ],
        }),
        cabac_init_idc: 1,
        slice_qp_delta: -3,
        sp_for_switch_flag: Some(false),
        slice_qs_delta: Some(2),
        deblocking_filter_idc: DeblockingFilterIdc::OnExceptSliceBounds,
        slice_alpha_c0_offset_div2: -1,
        slice_beta_offset_div2: 1,
        slice_group_change_cycle: Some(4),
    };

    let json = serde_json::to_string(&header).expect("Failed to serialize SliceHeader");
    let header_deserialized: SliceHeader =
        serde_json::from_str(&json).expect("Failed to deserialize SliceHeader");

    assert_eq!(header, header_deserialized);
}

#[test]
fn test_sps_from_raw_json() {
    let raw_json = r#"{
        "profile": "High",
        "constraint_set0_flag": true,
        "constraint_set1_flag": false,
        "constraint_set2_flag": true,
        "constraint_set3_flag": false,
        "constraint_set4_flag": false,
        "constraint_set5_flag": false,
        "level_idc": 31,
        "seq_parameter_set_id": 0,
        "chroma_format_idc": "YUV420",
        "separate_color_plane_flag": false,
        "bit_depth_luma_minus8": 0,
        "bit_depth_chroma_minus8": 0,
        "qpprime_y_zero_transform_bypass_flag": false,
        "seq_scaling_matrix_present_flag": false,
        "log2_max_frame_num_minus4": 4,
        "pic_order_cnt_type": 0,
        "log2_max_pic_order_cnt_lsb_minus4": 2,
        "delta_pic_order_always_zero_flag": false,
        "offset_for_non_ref_pic": 0,
        "offset_for_top_to_bottom_field": 0,
        "offset_for_ref_frame": [],
        "max_num_ref_frames": 2,
        "gaps_in_frame_num_value_allowed_flag": false,
        "pic_width_in_mbs_minus1": 39,
        "pic_height_in_map_units_minus1": 29,
        "frame_mbs_only_flag": true,
        "mb_adaptive_frame_field_flag": false,
        "direct_8x8_inference_flag": true,
        "frame_cropping": null,
        "vui_parameters": null
    }"#;

    let sps: SequenceParameterSet =
        serde_json::from_str(raw_json).expect("Failed to parse raw SPS JSON");
    assert_eq!(sps.profile, Profile::High);
    assert_eq!(sps.level_idc, 31);
    assert_eq!(sps.seq_parameter_set_id, 0);
    assert_eq!(sps.pic_width_in_mbs_minus1, 39);
    assert_eq!(sps.pic_height_in_map_units_minus1, 29);
    assert!(sps.frame_mbs_only_flag);
}

#[test]
fn test_pps_from_raw_json() {
    let raw_json = r#"{
        "pic_parameter_set_id": 0,
        "seq_parameter_set_id": 0,
        "entropy_coding_mode_flag": true,
        "bottom_field_pic_order_in_frame_present_flag": false,
        "slice_group": null,
        "num_ref_idx_l0_default_active_minus1": 0,
        "num_ref_idx_l1_default_active_minus1": 0,
        "weighted_pred_flag": false,
        "weighted_bipred_idc": 0,
        "pic_init_qp_minus26": 0,
        "pic_init_qs_minus26: 0,
        "chroma_qp_index_offset": 0,
        "deblocking_filter_control_present_flag": true,
        "constrained_intra_pred_flag": false,
        "redundant_pic_cnt_present_flag": false,
        "transform_8x8_mode_flag": true,
        "second_chroma_qp_index_offset": 0
    }"#;

    let pps: PicParameterSet = serde_json::from_str(
        raw_json.replace("\"pic_init_qs_minus26: 0,", "\"pic_init_qs_minus26\": 0,").as_str(),
    )
    .expect("Failed to parse raw PPS JSON");
    assert_eq!(pps.pic_parameter_set_id, 0);
    assert!(pps.entropy_coding_mode_flag);
    assert!(pps.deblocking_filter_control_present_flag);
    assert!(pps.transform_8x8_mode_flag);
}

#[test]
fn test_slice_header_from_raw_json() {
    let raw_json = r#"{
        "first_mb_in_slice": 0,
        "slice_type": "I",
        "pic_parameter_set_id": 0,
        "color_plane": null,
        "frame_num": 0,
        "field_pic_flag": false,
        "bottom_field_flag": null,
        "idr_pic_id": 0,
        "pic_order_cnt_lsb": 0,
        "delta_pic_order_cnt_bottom": null,
        "delta_pic_order_cnt": [0, 0],
        "redundant_pic_cnt": null,
        "direct_spatial_mv_pred_flag": null,
        "num_ref_idx_l0_active_minus1": 0,
        "num_ref_idx_l1_active_minus1": 0,
        "ref_pic_list_modification": {
            "list0": [
                { "RemapShortTermNegative": 5 },
                { "RemapShortTermPositive": 2 },
                { "RemapLongTerm": 10 }
            ],
            "list1": [
                { "RemapShortTermPositive": 1 },
                { "RemapLongTerm": 3 }
            ]
        },
        "pred_weight_table": null,
        "dec_ref_pic_marking": null,
        "cabac_init_idc": 0,
        "slice_qp_delta": 24,
        "sp_for_switch_flag": null,
        "slice_qs_delta": null,
        "deblocking_filter_idc": "On",
        "slice_alpha_c0_offset_div2": 0,
        "slice_beta_offset_div2": 0,
        "slice_group_change_cycle": null
    }"#;

    let header: SliceHeader =
        serde_json::from_str(raw_json).expect("Failed to parse raw SliceHeader JSON");
    assert_eq!(header.first_mb_in_slice, 0);
    assert_eq!(header.slice_type, SliceType::I);
    assert_eq!(header.slice_qp_delta, 24);
    assert_eq!(header.deblocking_filter_idc, DeblockingFilterIdc::On);

    assert_eq!(header.ref_pic_list_modification.list0.len(), 3);
    assert_eq!(
        header.ref_pic_list_modification.list0[0],
        RefPicListModification::RemapShortTermNegative(5)
    );
    assert_eq!(
        header.ref_pic_list_modification.list0[1],
        RefPicListModification::RemapShortTermPositive(2)
    );
    assert_eq!(
        header.ref_pic_list_modification.list0[2],
        RefPicListModification::RemapLongTerm(10)
    );

    assert_eq!(header.ref_pic_list_modification.list1.len(), 2);
    assert_eq!(
        header.ref_pic_list_modification.list1[0],
        RefPicListModification::RemapShortTermPositive(1)
    );
    assert_eq!(header.ref_pic_list_modification.list1[1], RefPicListModification::RemapLongTerm(3));
}
