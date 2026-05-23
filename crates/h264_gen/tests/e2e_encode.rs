use h264_gen::nal_writer::create_annex_b_nal_unit;
use h264_gen::rbsp_writer::RbspWriter;
use h264_gen::writer::{write_pps, write_slice_header, write_sps};
use hibernia::api::{
    create_decoder, Codec, DecodedPicture, DecoderConfig, DefaultAllocator, EncodedPacket,
    FlushMode, StreamFormat, VideoDecoderCallbacks, VideoPlane,
};
use hibernia::h264::nal::{NalHeader, NalUnitType};
use hibernia::h264::nal_parser::NalParser;
use hibernia::h264::pps::PicParameterSet;
use hibernia::h264::slice::{DeblockingFilterIdc, SliceHeader, SliceType};
use hibernia::h264::sps::SequenceParameterSet;
use hibernia::h264::{ChromaFormat, Profile};
use std::io::Cursor;
use std::sync::Arc;

struct NoopCallbacks;

impl VideoDecoderCallbacks for NoopCallbacks {
    fn on_picture_available(&self) {}
    fn on_format_changed(&self, _format: StreamFormat) {}
}

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
        &sps_writer.into_inner(),
    ));

    // 2. Write PPS
    let mut pps_writer = RbspWriter::new();
    write_pps(&pps, &mut pps_writer).unwrap();
    bitstream.extend(create_annex_b_nal_unit(
        &NalHeader { nal_ref_idc: 3, nal_unit_type: NalUnitType::PicParameterSet },
        &pps_writer.into_inner(),
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
    idr_header.dec_ref_pic_marking = Some(hibernia::h264::slice::DecRefPicMarking {
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
        for _ in 0..256 {
            idr_writer.u(8, 100).unwrap();
        } // Luma
        for _ in 0..64 {
            idr_writer.u(8, 101).unwrap();
        } // Cb
        for _ in 0..64 {
            idr_writer.u(8, 102).unwrap();
        } // Cr
    }
    idr_writer.rbsp_trailing_bits().unwrap();
    bitstream.extend(create_annex_b_nal_unit(
        &NalHeader { nal_ref_idc: 3, nal_unit_type: NalUnitType::IDRSlice },
        &idr_writer.into_inner(),
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
        p_header.dec_ref_pic_marking = Some(hibernia::h264::slice::DecRefPicMarking {
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
    let mut decoder = create_decoder(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        Arc::new(NoopCallbacks),
    )
    .expect("create_decoder");

    let mut frames_decoded = 0;

    let check_frame = |pic: &DecodedPicture, frames_decoded: usize, is_flush: bool| {
        let msg = if is_flush {
            format!("in flushed frame {}", frames_decoded)
        } else {
            format!("in frame {}", frames_decoded)
        };

        let y = pic.frame.plane(VideoPlane::Y).expect("Y");
        let u = pic.frame.plane(VideoPlane::U).expect("U");
        let v = pic.frame.plane(VideoPlane::V).expect("V");

        assert_eq!(y.width, 256, "Width mismatch {}", msg);
        assert_eq!(y.height, 256, "Height mismatch {}", msg);
        assert_eq!(u.width, 128, "Cb width mismatch {}", msg);
        assert_eq!(u.height, 128, "Cb height mismatch {}", msg);
        assert_eq!(v.width, 128, "Cr width mismatch {}", msg);
        assert_eq!(v.height, 128, "Cr height mismatch {}", msg);

        for row in 0..256 {
            let base = row * y.stride;
            for col in 0..256 {
                assert_eq!(
                    y.data[base + col],
                    100,
                    "Luma mismatch at {}x{} {}",
                    col,
                    row,
                    msg
                );
            }
        }

        for row in 0..128 {
            let u_base = row * u.stride;
            let v_base = row * v.stride;
            for col in 0..128 {
                assert_eq!(u.data[u_base + col], 101, "Cb mismatch at {}x{} {}", col, row, msg);
                assert_eq!(v.data[v_base + col], 102, "Cr mismatch at {}x{} {}", col, row, msg);
            }
        }
    };

    for nal_result in nal_parser {
        let nal = nal_result.unwrap();
        let mut buf = Vec::with_capacity(nal.len() + 4);
        buf.extend_from_slice(&[0, 0, 0, 1]);
        buf.extend_from_slice(&nal);
        decoder.decode(EncodedPacket::from_vec(buf)).unwrap();

        while let Some(pic) = decoder.get_picture().unwrap() {
            frames_decoded += 1;
            check_frame(&pic, frames_decoded, false);
        }
    }

    decoder.flush(FlushMode::Drain).unwrap();
    while let Some(pic) = decoder.get_picture().unwrap() {
        frames_decoded += 1;
        check_frame(&pic, frames_decoded, true);
    }

    assert_eq!(frames_decoded, 5);
}
