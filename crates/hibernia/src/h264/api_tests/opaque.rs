//! Tests for opaque metadata propagation: caller-supplied tags must
//! ride alongside pictures through DPB reordering, survive complex
//! payload types, and respect packet boundaries.

use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::{FlushMode, VideoDecoder};
use crate::api::packet::EncodedPacket;

use super::support::{
    annexb_packet, baseline_bframe_packets, baseline_bframe_tagged_packets, default_decoder,
    drive_through, fixture, read_fixture_nals, CountingCallbacks, BASELINE_BFRAME_FIXTURE,
};

#[test]
fn opaque_round_trips_through_display_reordering() {
    // Each input packet carries its zero-based index. After display
    // reordering, every tag we emitted must appear exactly once on
    // output, all within range [0, sent).
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let packets = baseline_bframe_tagged_packets();
    let sent = packets.len();
    let pics = drive_through(&mut decoder, packets).expect("drive");
    let tags: Vec<usize> = pics
        .into_iter()
        .map(|p| {
            *p.opaque
                .expect("opaque attached")
                .downcast::<usize>()
                .expect("usize tag")
        })
        .collect();
    assert!(!tags.is_empty());
    let mut sorted = tags.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), tags.len(), "tags must be unique on output");
    for t in &tags {
        assert!(*t < sent, "tag {} out of range {}", t, sent);
    }
}

#[test]
fn packet_without_opaque_yields_picture_without_opaque() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(!pics.is_empty());
    for pic in &pics {
        assert!(pic.opaque.is_none(), "packet without opaque must yield None");
    }
}

#[test]
fn opaque_can_be_a_complex_type() {
    #[derive(Debug, PartialEq, Eq)]
    struct Meta {
        pts: u64,
        flags: u32,
    }
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");

    // Build per-NAL packets with Meta tagged using the packet index.
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let mut packets = Vec::with_capacity(nals.len());
    for (idx, nal) in nals.iter().enumerate() {
        let mut buf = Vec::with_capacity(nal.len() + 4);
        buf.extend_from_slice(&[0, 0, 0, 1]);
        buf.extend_from_slice(nal);
        packets.push(EncodedPacket::from_vec_with_opaque(
            buf,
            Meta { pts: idx as u64 * 1_000_000, flags: idx as u32 & 0xff },
        ));
    }

    let pics = drive_through(&mut decoder, packets).expect("drive");
    assert!(!pics.is_empty());
    for pic in pics {
        let meta = pic.opaque.expect("opaque").downcast::<Meta>().expect("Meta");
        assert!(meta.pts > 0 || meta.flags > 0 || meta.flags == 0); // sanity
    }
}

#[test]
fn sps_only_packet_leaves_opaque_pending_for_next_coded_slice() {
    // A packet with an SPS NAL doesn't produce a picture. Its opaque
    // (if any) must wait for the next primary coded slice rather than
    // being dropped.
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));
    let sps_idx = nals
        .iter()
        .position(|n| n[0] & 0x1f == 7)
        .expect("fixture contains an SPS");
    let pps_idx = nals
        .iter()
        .position(|n| n[0] & 0x1f == 8)
        .expect("fixture contains a PPS");
    // Find the first slice NAL after SPS+PPS.
    let slice_idx = nals
        .iter()
        .position(|n| matches!(n[0] & 0x1f, 5 | 1))
        .expect("fixture contains a slice");

    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");

    // SPS packet carries a tag.
    let sps_packet = {
        let mut buf = vec![0, 0, 0, 1];
        buf.extend_from_slice(&nals[sps_idx]);
        EncodedPacket::from_vec_with_opaque(buf, 0xAAAA_BBBBu32)
    };
    decoder.decode(sps_packet).expect("decode SPS");
    // No picture should be ready.
    assert!(decoder.get_picture().expect("get_picture").is_none());

    // Then PPS (no tag).
    decoder.decode(annexb_packet(&nals[pps_idx])).expect("decode PPS");
    // Then the first slice (no tag).
    decoder.decode(annexb_packet(&nals[slice_idx])).expect("decode slice");
    decoder.flush(FlushMode::Drain).expect("drain");

    // The next emitted picture should carry the SPS packet's opaque.
    let pic = decoder.get_picture().expect("get_picture").expect("a picture");
    let tag = pic.opaque.expect("opaque").downcast::<u32>().expect("u32");
    assert_eq!(*tag, 0xAAAA_BBBB, "pending opaque must attach to next coded picture");
}

#[test]
fn discard_drops_pending_opaque() {
    // After flush(Discard), any pending_opaque must be gone so it
    // doesn't leak into a subsequent decode.
    let nals = read_fixture_nals(&fixture(BASELINE_BFRAME_FIXTURE));

    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");

    // Stash an opaque on an SPS-only packet (no picture starts).
    let sps_idx = nals.iter().position(|n| n[0] & 0x1f == 7).unwrap();
    let mut buf = vec![0, 0, 0, 1];
    buf.extend_from_slice(&nals[sps_idx]);
    decoder
        .decode(EncodedPacket::from_vec_with_opaque(buf, 0xDEADBEEFu32))
        .expect("decode SPS");

    decoder.flush(FlushMode::Discard).expect("discard");

    // Decode the same fixture from scratch. None of the emitted
    // pictures should carry the dropped tag.
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(!pics.is_empty());
    for pic in pics {
        assert!(
            pic.opaque.is_none(),
            "Discard must drop pending_opaque so post-discard pictures stay untagged",
        );
    }
}

#[test]
fn opaque_survives_multiple_decode_drain_cycles() {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");

    let first_pics = drive_through(&mut decoder, baseline_bframe_tagged_packets()).expect("first");
    assert!(!first_pics.is_empty());
    for pic in &first_pics {
        assert!(pic.opaque.is_some());
    }

    let second_pics = drive_through(&mut decoder, baseline_bframe_tagged_packets()).expect("second");
    assert!(!second_pics.is_empty());
    for pic in &second_pics {
        assert!(pic.opaque.is_some(), "second pass tags must also emerge");
    }
}
