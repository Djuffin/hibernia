use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::nal_writer::create_annex_b_nal_unit;
use crate::rbsp_writer::RbspWriter;
use crate::writer::{write_pps, write_slice_header, write_sps};
use hibernia::h264::nal::{NalHeader, NalUnitType};
use hibernia::h264::pps::PicParameterSet;
use hibernia::h264::slice::SliceHeader;
use hibernia::h264::sps::SequenceParameterSet;

pub type BitstreamConfig = Vec<NalUnitDescriptor>;

/// Descriptor for a single NAL unit to be generated in the bitstream.
#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
pub struct NalUnitDescriptor {
    pub nal_ref_idc: u8,
    pub nal_unit_type: NalUnitType,
    #[serde(flatten)]
    pub payload: NalPayload,
}

/// Payload of a NAL unit in the generator configuration.
#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
pub enum NalPayload {
    SPS(SequenceParameterSet),
    PPS(PicParameterSet),
    Slice(SliceConfig),
    RawHex(String),
}

/// Configuration for generating a slice NAL unit.
#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
pub struct SliceConfig {
    pub header: SliceHeader,
    pub writer_sps: Option<SequenceParameterSet>,
    pub writer_pps: Option<PicParameterSet>,
    pub data: Vec<SliceDataConfig>,
}

/// Configuration for a chunk of slice data, containing macroblocks or raw hex data.
#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(untagged)]
pub enum SliceDataConfig {
    Macroblock(MacroblockConfig),
    RawHex(String),
}

/// Configuration for generating a specific type of macroblock.
#[derive(Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(tag = "type")]
pub enum MacroblockConfig {
    #[serde(rename = "I_PCM")]
    IPCM { count: usize, luma: u8, cb: u8, cr: u8 },
    #[serde(rename = "P_Skip")]
    PSkip { count: usize },
}

pub fn generate_bitstream(config: &BitstreamConfig) -> Result<Vec<u8>, String> {
    let mut bitstream = Vec::new();
    let mut active_sps = HashMap::new();
    let mut active_pps = HashMap::new();

    for desc in config {
        let header = NalHeader { nal_ref_idc: desc.nal_ref_idc, nal_unit_type: desc.nal_unit_type };

        match &desc.payload {
            NalPayload::SPS(sps) => {
                active_sps.insert(sps.seq_parameter_set_id, sps.clone());
                let mut writer = RbspWriter::new();
                write_sps(sps, &mut writer)?;
                bitstream.extend(create_annex_b_nal_unit(&header, &writer.into_inner()));
            }
            NalPayload::PPS(pps) => {
                active_pps.insert(pps.pic_parameter_set_id, pps.clone());
                let mut writer = RbspWriter::new();
                write_pps(pps, &mut writer)?;
                bitstream.extend(create_annex_b_nal_unit(&header, &writer.into_inner()));
            }
            NalPayload::Slice(slice_cfg) => {
                let pps_id = slice_cfg.header.pic_parameter_set_id;
                let pps = slice_cfg.writer_pps.as_ref().or_else(|| active_pps.get(&pps_id));
                let pps = pps.ok_or_else(|| {
                    format!("PPS {} not found and no writer_pps provided", pps_id)
                })?;

                let sps_id = pps.seq_parameter_set_id;
                let sps = slice_cfg.writer_sps.as_ref().or_else(|| active_sps.get(&sps_id));
                let sps = sps.ok_or_else(|| {
                    format!("SPS {} not found and no writer_sps provided", sps_id)
                })?;

                let mut writer = RbspWriter::new();
                let is_idr = desc.nal_unit_type == NalUnitType::IDRSlice;
                write_slice_header(&slice_cfg.header, sps, pps, is_idr, &mut writer)?;

                for data_chunk in &slice_cfg.data {
                    match data_chunk {
                        SliceDataConfig::Macroblock(MacroblockConfig::IPCM {
                            count,
                            luma,
                            cb,
                            cr,
                        }) => {
                            for _ in 0..*count {
                                writer.ue(25)?; // I_PCM mb_type
                                writer.align()?;
                                for _ in 0..256 {
                                    writer.u(8, *luma as u32)?;
                                }
                                for _ in 0..64 {
                                    writer.u(8, *cb as u32)?;
                                }
                                for _ in 0..64 {
                                    writer.u(8, *cr as u32)?;
                                }
                            }
                        }
                        SliceDataConfig::Macroblock(MacroblockConfig::PSkip { count }) => {
                            writer.ue(*count as u32)?;
                        }
                        SliceDataConfig::RawHex(hex_str) => {
                            if !writer.is_aligned() {
                                writer.align()?;
                            }
                            let bytes =
                                decode_hex(hex_str).map_err(|e| format!("Invalid hex: {}", e))?;
                            for b in bytes {
                                writer.u(8, b as u32)?;
                            }
                        }
                    }
                }

                writer.rbsp_trailing_bits()?;
                bitstream.extend(create_annex_b_nal_unit(&header, &writer.into_inner()));
            }
            NalPayload::RawHex(hex_str) => {
                let bytes = decode_hex(hex_str).map_err(|e| format!("Invalid hex: {}", e))?;
                bitstream.extend(create_annex_b_nal_unit(&header, &bytes));
            }
        }
    }

    Ok(bitstream)
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err("Hex string length must be even".to_string());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_bitstream() {
        let json = r#"[
            {
                "nal_ref_idc": 3,
                "nal_unit_type": "SeqParameterSet",
                "SPS": {
                    "profile": "High",
                    "constraint_set0_flag": false,
                    "constraint_set1_flag": false,
                    "constraint_set2_flag": false,
                    "constraint_set3_flag": false,
                    "constraint_set4_flag": false,
                    "constraint_set5_flag": false,
                    "level_idc": 40,
                    "seq_parameter_set_id": 0,
                    "chroma_format_idc": "YUV420",
                    "separate_color_plane_flag": false,
                    "bit_depth_luma_minus8": 0,
                    "bit_depth_chroma_minus8": 0,
                    "qpprime_y_zero_transform_bypass_flag": false,
                    "seq_scaling_matrix_present_flag": false,
                    "log2_max_frame_num_minus4": 0,
                    "pic_order_cnt_type": 0,
                    "log2_max_pic_order_cnt_lsb_minus4": 0,
                    "delta_pic_order_always_zero_flag": false,
                    "offset_for_non_ref_pic": 0,
                    "offset_for_top_to_bottom_field": 0,
                    "offset_for_ref_frame": [],
                    "max_num_ref_frames": 1,
                    "gaps_in_frame_num_value_allowed_flag": false,
                    "pic_width_in_mbs_minus1": 15,
                    "pic_height_in_map_units_minus1": 15,
                    "frame_mbs_only_flag": true,
                    "mb_adaptive_frame_field_flag": false,
                    "direct_8x8_inference_flag": true,
                    "frame_cropping": null,
                    "vui_parameters": null
                }
            },
            {
                "nal_ref_idc": 3,
                "nal_unit_type": "PicParameterSet",
                "PPS": {
                    "pic_parameter_set_id": 0,
                    "seq_parameter_set_id": 0,
                    "entropy_coding_mode_flag": false,
                    "bottom_field_pic_order_in_frame_present_flag": false,
                    "slice_group": null,
                    "num_ref_idx_l0_default_active_minus1": 0,
                    "num_ref_idx_l1_default_active_minus1": 0,
                    "weighted_pred_flag": false,
                    "weighted_bipred_idc": 0,
                    "pic_init_qp_minus26": 0,
                    "pic_init_qs_minus26": 0,
                    "chroma_qp_index_offset": 0,
                    "deblocking_filter_control_present_flag": false,
                    "constrained_intra_pred_flag": false,
                    "redundant_pic_cnt_present_flag": false,
                    "transform_8x8_mode_flag": false,
                    "second_chroma_qp_index_offset": 0
                }
            },
            {
                "nal_ref_idc": 3,
                "nal_unit_type": "IDRSlice",
                "Slice": {
                    "header": {
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
                            "list0": [],
                            "list1": []
                        },
                        "pred_weight_table": null,
                        "dec_ref_pic_marking": {
                            "no_output_of_prior_pics_flag": false,
                            "long_term_reference_flag": false,
                            "adaptive_ref_pic_marking_mode_flag": null,
                            "memory_management_operations": []
                        },
                        "cabac_init_idc": 0,
                        "slice_qp_delta": 0,
                        "sp_for_switch_flag": null,
                        "slice_qs_delta": null,
                        "deblocking_filter_idc": "Off",
                        "slice_alpha_c0_offset_div2": 0,
                        "slice_beta_offset_div2": 0,
                        "slice_group_change_cycle": null
                    },
                    "writer_sps": null,
                    "writer_pps": null,
                    "data": [
                        {
                            "type": "I_PCM",
                            "count": 256,
                            "luma": 100,
                            "cb": 101,
                            "cr": 102
                        }
                    ]
                }
            }
        ]"#;

        let config: BitstreamConfig = serde_json::from_str(json).expect("Failed to parse JSON");
        let bitstream = generate_bitstream(&config).expect("Failed to generate bitstream");

        // Should contain 3 NAL units, so 3 start codes.
        let start_codes = bitstream.windows(4).filter(|w| *w == [0, 0, 0, 1]).count();
        assert_eq!(start_codes, 3);

        // Output shouldn't be empty
        assert!(!bitstream.is_empty());

        // Decode the generated bitstream
        let mut decoder = hibernia::h264::decoder::Decoder::new();
        let cursor = std::io::Cursor::new(bitstream);
        let nal_parser = hibernia::h264::nal_parser::NalParser::new(cursor);

        let mut frames_decoded = 0;

        let mut check_frame = |frame: &hibernia::h264::decoder::VideoFrame| {
            frames_decoded += 1;
            let y_plane = &frame.planes[0];
            let u_plane = &frame.planes[1];
            let v_plane = &frame.planes[2];

            assert_eq!(y_plane.cfg.width, 256);
            assert_eq!(y_plane.cfg.height, 256);

            for y in 0..256 {
                let row_start =
                    (y_plane.cfg.yorigin + y) * y_plane.cfg.stride + y_plane.cfg.xorigin;
                for x in 0..256 {
                    assert_eq!(y_plane.data[row_start + x], 100);
                }
            }

            for y in 0..128 {
                let u_row_start =
                    (u_plane.cfg.yorigin + y) * u_plane.cfg.stride + u_plane.cfg.xorigin;
                let v_row_start =
                    (v_plane.cfg.yorigin + y) * v_plane.cfg.stride + v_plane.cfg.xorigin;
                for x in 0..128 {
                    assert_eq!(u_plane.data[u_row_start + x], 101);
                    assert_eq!(v_plane.data[v_row_start + x], 102);
                }
            }
        };

        for nal_result in nal_parser {
            let nal_data = nal_result.unwrap();
            decoder.decode(&nal_data).unwrap();
            while let Some(pic) = decoder.retrieve_picture() {
                check_frame(&pic.frame);
            }
        }

        decoder.flush().unwrap();
        while let Some(pic) = decoder.retrieve_picture() {
            check_frame(&pic.frame);
        }

        assert_eq!(frames_decoded, 1);
    }

    #[test]
    fn test_decode_hex() {
        assert_eq!(decode_hex("AABBCC").unwrap(), vec![0xAA, 0xBB, 0xCC]);
        assert_eq!(decode_hex("00010203").unwrap(), vec![0x00, 0x01, 0x02, 0x03]);
        assert!(decode_hex("AAB").is_err());
        assert!(decode_hex("GG").is_err());
    }
}
