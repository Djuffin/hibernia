use crate::h264::nal::{NalHeader, NalUnitType};
use crate::h264::nal_writer::create_annex_b_nal_unit;
use crate::h264::pps::PicParameterSet;
use crate::h264::rbsp_writer::RbspWriter;
use crate::h264::slice::{DeblockingFilterIdc, SliceHeader, SliceType};
use crate::h264::sps::SequenceParameterSet;
use crate::h264::writer::{write_pps, write_slice_header, write_sps};
use crate::h264::{ChromaFormat, Profile};

use crate::h264::decoder::Decoder;
use crate::h264::nal_parser::NalParser;
use std::io::Cursor;

#[test]
fn test_generate_and_decode_video() {
    let mut sps = SequenceParameterSet::default();
    sps.profile = Profile::High;
    sps.level_idc = 40;
    sps.seq_parameter_set_id = 0;
    sps.chroma_format_idc = ChromaFormat::YUV420;
    sps.pic_width_in_mbs_minus1 = 15;
    sps.pic_height_in_map_units_minus1 = 15;
    sps.max_num_ref_frames = 1;
    sps.frame_mbs_only_flag = true;
    sps.direct_8x8_inference_flag = true;
    sps.pic_order_cnt_type = 0;
    sps.log2_max_frame_num_minus4 = 0;
    sps.log2_max_pic_order_cnt_lsb_minus4 = 0;

    let mut pps = PicParameterSet::default();
    pps.pic_parameter_set_id = 0;
    pps.seq_parameter_set_id = 0;
    pps.entropy_coding_mode_flag = false; // CAVLC

    let mut bitstream = Vec::new();

    // 1. Write SPS
    let mut sps_writer = RbspWriter::new();
    write_sps(&sps, &mut sps_writer).unwrap();
    bitstream.extend(create_annex_b_nal_unit(
        &NalHeader { nal_ref_idc: 3, nal_unit_type: NalUnitType::SeqParameterSet }, 
        &sps_writer.into_inner()
    ));

    // 2. Write PPS
    let mut pps_writer = RbspWriter::new();
    write_pps(&pps, &mut pps_writer).unwrap();
    bitstream.extend(create_annex_b_nal_unit(
        &NalHeader { nal_ref_idc: 3, nal_unit_type: NalUnitType::PicParameterSet }, 
        &pps_writer.into_inner()
    ));

    // 3. Write IDR Frame (I_PCM for all MBs)
    let mut idr_header = SliceHeader::default();
    idr_header.first_mb_in_slice = 0;
    idr_header.slice_type = SliceType::I;
    idr_header.pic_parameter_set_id = 0;
    idr_header.frame_num = 0;
    idr_header.idr_pic_id = Some(0);
    idr_header.pic_order_cnt_lsb = Some(0);
    idr_header.deblocking_filter_idc = DeblockingFilterIdc::Off;
    idr_header.dec_ref_pic_marking = Some(crate::h264::slice::DecRefPicMarking {
        no_output_of_prior_pics_flag: Some(false),
        long_term_reference_flag: Some(false),
        adaptive_ref_pic_marking_mode_flag: None,
        memory_management_operations: vec![],
    });
    
    let mut idr_writer = RbspWriter::new();
    write_slice_header(&idr_header, &sps, &pps, true, &mut idr_writer).unwrap();
    for _ in 0..256 {
        idr_writer.ue(25).unwrap(); // I_PCM mb_type
        idr_writer.align().unwrap();
        for _ in 0..256 { idr_writer.u(8, 100).unwrap(); } // Luma
        for _ in 0..64 { idr_writer.u(8, 101).unwrap(); }  // Cb
        for _ in 0..64 { idr_writer.u(8, 102).unwrap(); } // Cr
    }
    idr_writer.rbsp_trailing_bits().unwrap();
    bitstream.extend(create_annex_b_nal_unit(
        &NalHeader { nal_ref_idc: 3, nal_unit_type: NalUnitType::IDRSlice }, 
        &idr_writer.into_inner()
    ));

    // 4. Write 4 P-Frames (Skipping all MBs)
    for frame_idx in 1..=4 {
        let mut p_header = SliceHeader::default();
        p_header.first_mb_in_slice = 0;
        p_header.slice_type = SliceType::P;
        p_header.pic_parameter_set_id = 0;
        p_header.frame_num = frame_idx;
        p_header.pic_order_cnt_lsb = Some(frame_idx as u32 * 2);
        p_header.deblocking_filter_idc = DeblockingFilterIdc::Off;
        p_header.num_ref_idx_l0_active_minus1 = 0;
        p_header.num_ref_idx_l1_active_minus1 = 0;
        p_header.dec_ref_pic_marking = Some(crate::h264::slice::DecRefPicMarking {
            no_output_of_prior_pics_flag: None,
            long_term_reference_flag: None,
            adaptive_ref_pic_marking_mode_flag: Some(false),
            memory_management_operations: vec![],
        });

        let mut p_writer = RbspWriter::new();
        write_slice_header(&p_header, &sps, &pps, false, &mut p_writer).unwrap();
        p_writer.ue(256).unwrap(); // mb_skip_run = 256
        p_writer.rbsp_trailing_bits().unwrap();
        bitstream.extend(create_annex_b_nal_unit(
            &NalHeader { nal_ref_idc: 2, nal_unit_type: NalUnitType::NonIDRSlice },
            &p_writer.into_inner(),
        ));
    }

    let cursor = Cursor::new(bitstream);
    let nal_parser = NalParser::new(cursor);
    let mut decoder = Decoder::new();

    let mut frames_decoded = 0;
    for nal_result in nal_parser {
        let nal_data = nal_result.unwrap();
        decoder.decode(&nal_data).unwrap();

        while let Some(frame) = decoder.retrieve_frame() {
            frames_decoded += 1;

            let y_plane = &frame.planes[0];
            let u_plane = &frame.planes[1];
            let v_plane = &frame.planes[2];

            let y_idx = y_plane.cfg.yorigin * y_plane.cfg.stride + y_plane.cfg.xorigin;
            let u_idx = u_plane.cfg.yorigin * u_plane.cfg.stride + u_plane.cfg.xorigin;
            let v_idx = v_plane.cfg.yorigin * v_plane.cfg.stride + v_plane.cfg.xorigin;

            assert_eq!(y_plane.data[y_idx], 100, "Luma mismatch in frame {}", frames_decoded);
            assert_eq!(u_plane.data[u_idx], 101, "Cb mismatch in frame {}", frames_decoded);
            assert_eq!(v_plane.data[v_idx], 102, "Cr mismatch in frame {}", frames_decoded);
        }
    }

    decoder.flush().unwrap();
    while let Some(frame) = decoder.retrieve_frame() {
        frames_decoded += 1;
        let y_plane = &frame.planes[0];
        let u_plane = &frame.planes[1];
        let v_plane = &frame.planes[2];

        let y_idx = y_plane.cfg.yorigin * y_plane.cfg.stride + y_plane.cfg.xorigin;
        let u_idx = u_plane.cfg.yorigin * u_plane.cfg.stride + u_plane.cfg.xorigin;
        let v_idx = v_plane.cfg.yorigin * v_plane.cfg.stride + v_plane.cfg.xorigin;

        assert_eq!(y_plane.data[y_idx], 100, "Luma mismatch in flushed frame {}", frames_decoded);
        assert_eq!(u_plane.data[u_idx], 101, "Cb mismatch in flushed frame {}", frames_decoded);
        assert_eq!(v_plane.data[v_idx], 102, "Cr mismatch in flushed frame {}", frames_decoded);
    }

    assert_eq!(frames_decoded, 5);
}
