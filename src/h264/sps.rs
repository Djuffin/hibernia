use super::{tables, ChromaFormat, Profile};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HdrParameters {}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct VuiParameters {
    pub aspect_ratio_info_present_flag: bool,
    pub aspect_ratio_idc: u8,
    pub sar_width: u16,
    pub sar_height: u16,

    pub overscan_info_present_flag: bool,
    pub overscan_appropriate_flag: bool,

    pub video_signal_type_present_flag: bool,
    pub video_format: u8,
    pub video_full_range_flag: bool,
    pub color_description_present_flag: bool,
    pub color_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,

    pub chroma_loc_info_present_flag: bool,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,

    pub timing_info_present_flag: bool,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub fixed_frame_rate_flag: bool,

    pub nal_hrd_parameters: Option<HdrParameters>,
    pub vcl_hrd_parameters: Option<HdrParameters>,
    pub low_delay_hrd_flag: bool,

    pub pic_struct_present_flag: bool,

    pub bitstream_restriction_flag: bool,
    pub motion_vectors_over_pic_boundaries_flag: bool,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_mb_denom: u8,
    pub log2_max_mv_length_horizontal: u8,
    pub log2_max_mv_length_vertical: u8,
    pub max_num_reorder_frames: u8,
    pub max_dec_frame_buffering: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FrameCrop {
    pub top: u32,
    pub left: u32,
    pub right: u32,
    pub bottom: u32,
}

// Section 7.4.2.1.1 Sequence parameter set data semantics
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SequenceParameterSet {
    pub profile: Profile,
    pub constraint_set0_flag: bool,
    pub constraint_set1_flag: bool,
    pub constraint_set2_flag: bool,
    pub constraint_set3_flag: bool,
    pub constraint_set4_flag: bool,
    pub constraint_set5_flag: bool,
    pub level_idc: u8,
    pub seq_parameter_set_id: u8,
    pub chroma_format_idc: ChromaFormat,
    pub separate_color_plane_flag: bool,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub qpprime_y_zero_transform_bypass_flag: bool,
    pub seq_scaling_matrix_present_flag: bool,

    pub log2_max_frame_num_minus4: u8,

    pub pic_order_cnt_type: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub delta_pic_order_always_zero_flag: bool,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub offset_for_ref_frame: Vec<i32>,

    pub max_num_ref_frames: u8,
    pub gaps_in_frame_num_value_allowed_flag: bool,
    pub pic_width_in_mbs_minus1: u16,
    pub pic_height_in_map_units_minus1: u16,

    // False specifies that coded pictures of the coded video sequence may either be coded fields or
    // coded frames. True specifies that every coded picture of the coded video sequence is a coded
    // frame containing only frame macroblocks.
    pub frame_mbs_only_flag: bool,

    // False specifies no switching between frame and field macroblocks within a picture.
    // True specifies the possible use of switching between frame and field macroblocks within frames.
    pub mb_adaptive_frame_field_flag: bool,
    pub direct_8x8_inference_flag: bool,

    pub frame_cropping: Option<FrameCrop>,
    pub vui_parameters: Option<VuiParameters>,
}

#[allow(non_snake_case)]
impl SequenceParameterSet {
    pub fn bits_in_frame_num(&self) -> u8 {
        self.log2_max_frame_num_minus4 + 4
    }

    pub fn bits_in_max_pic_order_cnt(&self) -> u8 {
        self.log2_max_pic_order_cnt_lsb_minus4 + 4
    }

    pub fn ChromaArrayType(&self) -> ChromaFormat {
        if self.separate_color_plane_flag {
            ChromaFormat::Monochrome
        } else {
            self.chroma_format_idc
        }
    }

    pub fn pic_width_in_mbs(&self) -> usize {
        self.pic_width_in_mbs_minus1 as usize + 1
    }

    pub fn pic_width(&self) -> usize {
        self.pic_width_in_mbs() * tables::MB_WIDTH
    }

    pub fn pic_hight_in_mbs(&self) -> usize {
        self.pic_height_in_map_units_minus1 as usize + 1
    }

    pub fn pic_hight(&self) -> usize {
        self.pic_hight_in_mbs() * tables::MB_HEIGHT
    }

    pub fn pic_size_in_mbs(&self) -> usize {
        self.pic_hight_in_mbs() * self.pic_width_in_mbs()
    }
}
