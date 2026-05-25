//! Out-of-band H.264 parameter-set delivery.
//!
//! Two on-the-wire shapes show up in containers and signaling protocols:
//!
//! - **avcC** (ISO/IEC 14496-15 AVCDecoderConfigurationRecord): the
//!   `avcC` box in MP4, the `CodecPrivate` in MKV / WebM. Structured
//!   record starting with `configurationVersion = 1`. Carries one or
//!   more SPS NALs, one or more PPS NALs, and `lengthSizeMinusOne`
//!   which dictates the byte width of the length prefix on each
//!   sample's NAL.
//!
//! - **Annex-B SPS+PPS**: a concatenation of SPS and PPS NAL units
//!   with start codes (`0x000001` or `0x00000001`) between them. Some
//!   demuxers emit this shape into `extradata`; FFmpeg's
//!   `ff_h264_decode_extradata` accepts it as a fallback.
//!
//! [`parse_extradata`] sniffs which shape it has (avcC iff
//! `bytes[0] == 1`) and returns a uniform `ParsedExtradata` carrying
//! the constituent NAL byte arrays plus, for avcC, the implied
//! length-prefix size.

use crate::api::bitstream::AnnexBSplitter;
use crate::api::callbacks::DecoderError;

use super::nal::NalUnitType;

/// Parsed contents of an ISO/IEC 14496-15 AVCDecoderConfigurationRecord.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Avcc {
    /// NAL length-prefix size in bytes (1, 2, or 4). Value 3 is
    /// reserved by the standard; we reject it.
    pub length_size: usize,
    /// SPS NAL byte arrays, in record order.
    pub sps_nals: Vec<Vec<u8>>,
    /// PPS NAL byte arrays, in record order.
    pub pps_nals: Vec<Vec<u8>>,
}

/// Outcome of sniffing an extradata blob.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedExtradata {
    /// SPS+PPS NALs to feed through the regular NAL pipeline, in
    /// record order (SPSes first, then PPSes for avcC; whatever
    /// AnnexBSplitter yielded for the Annex-B fallback).
    pub nals: Vec<Vec<u8>>,
    /// `Some(size)` when avcC's `lengthSizeMinusOne` is authoritative;
    /// `None` for Annex-B-form extradata (no length size guidance).
    pub length_size: Option<usize>,
}

/// Sniff and parse an extradata blob.
///
/// Recognizes avcC by `bytes[0] == 1`. Otherwise treats the input as
/// a concatenation of Annex-B-framed SPS+PPS NALs. An empty input
/// returns an empty `ParsedExtradata` and is not an error.
pub(crate) fn parse_extradata(bytes: &[u8]) -> Result<ParsedExtradata, DecoderError> {
    if bytes.is_empty() {
        return Ok(ParsedExtradata { nals: Vec::new(), length_size: None });
    }
    if bytes[0] == 1 {
        let avcc = parse_avcc(bytes)?;
        let mut nals = avcc.sps_nals;
        nals.extend(avcc.pps_nals);
        return Ok(ParsedExtradata { nals, length_size: Some(avcc.length_size) });
    }
    // Annex-B fallback: concatenated SPS+PPS NALs with start codes.
    let nals: Vec<Vec<u8>> =
        AnnexBSplitter::new(bytes).filter(|n| !n.is_empty()).map(|n| n.to_vec()).collect();
    Ok(ParsedExtradata { nals, length_size: None })
}

/// Parse an avcC blob. Returns the constituent NAL byte arrays and
/// the NAL length-prefix size implied by `lengthSizeMinusOne`.
///
/// Trailing bytes after the PPS list (High-profile chroma/bit-depth
/// extension) are intentionally ignored: the inner SPS is the
/// authoritative source for that information, and we don't currently
/// surface anything in those fields.
pub(crate) fn parse_avcc(bytes: &[u8]) -> Result<Avcc, DecoderError> {
    if bytes.len() < 7 {
        return Err(DecoderError::MisformedData(format!(
            "avcC: truncated header ({} bytes, need >= 7)",
            bytes.len(),
        )));
    }
    if bytes[0] != 1 {
        return Err(DecoderError::MisformedData(format!(
            "avcC: unsupported configurationVersion {}",
            bytes[0],
        )));
    }
    // bytes[1..4] = AVCProfileIndication, profile_compatibility,
    // AVCLevelIndication. The inner SPS is authoritative; we don't
    // cross-check here.

    let length_size_minus_one = bytes[4] & 0x03;
    if length_size_minus_one == 2 {
        return Err(DecoderError::MisformedData(
            "avcC: reserved lengthSizeMinusOne value 2".into(),
        ));
    }
    let length_size = (length_size_minus_one as usize) + 1;

    let num_of_sps = (bytes[5] & 0x1F) as usize;
    let mut off = 6;
    let mut sps_nals = Vec::with_capacity(num_of_sps);
    for i in 0..num_of_sps {
        let nal = read_length_prefixed_nal(bytes, &mut off, 2, &format!("avcC SPS[{i}]"))?;
        validate_nal_type(&nal, NalUnitType::SeqParameterSet, &format!("avcC SPS[{i}]"))?;
        sps_nals.push(nal);
    }

    if off >= bytes.len() {
        return Err(DecoderError::MisformedData("avcC: truncated before PPS count".into()));
    }
    let num_of_pps = bytes[off] as usize;
    off += 1;

    let mut pps_nals = Vec::with_capacity(num_of_pps);
    for i in 0..num_of_pps {
        let nal = read_length_prefixed_nal(bytes, &mut off, 2, &format!("avcC PPS[{i}]"))?;
        validate_nal_type(&nal, NalUnitType::PicParameterSet, &format!("avcC PPS[{i}]"))?;
        pps_nals.push(nal);
    }

    // Trailing bytes (the High-profile sps_ext / chroma_format / bit_depth_*
    // section) are accepted and ignored.

    Ok(Avcc { length_size, sps_nals, pps_nals })
}

fn read_length_prefixed_nal(
    bytes: &[u8],
    off: &mut usize,
    length_size: usize,
    context: &str,
) -> Result<Vec<u8>, DecoderError> {
    if *off + length_size > bytes.len() {
        return Err(DecoderError::MisformedData(format!(
            "{context}: truncated length prefix",
        )));
    }
    let mut len = 0usize;
    for i in 0..length_size {
        len = (len << 8) | bytes[*off + i] as usize;
    }
    *off += length_size;
    if *off + len > bytes.len() {
        return Err(DecoderError::MisformedData(format!(
            "{context}: payload runs past buffer ({} bytes declared, {} available)",
            len,
            bytes.len() - *off,
        )));
    }
    let nal = bytes[*off..*off + len].to_vec();
    *off += len;
    Ok(nal)
}

fn validate_nal_type(
    nal: &[u8],
    expected: NalUnitType,
    context: &str,
) -> Result<(), DecoderError> {
    if nal.is_empty() {
        return Err(DecoderError::MisformedData(format!("{context}: empty NAL")));
    }
    let nal_type = nal[0] & 0x1F;
    let expected_u8 = expected as u8;
    if nal_type != expected_u8 {
        return Err(DecoderError::MisformedData(format!(
            "{context}: expected NAL type {expected_u8}, got {nal_type}",
        )));
    }
    Ok(())
}

/// Build an avcC blob (ISO/IEC 14496-15
/// AVCDecoderConfigurationRecord) from a list of SPS NAL byte arrays
/// and PPS NAL byte arrays, with the given NAL length-prefix size in
/// bytes (1, 2, or 4 — 3 is reserved by the standard and rejected).
///
/// The first SPS supplies the profile / compatibility / level bytes
/// in the record header. With an empty `sps_list`, those fields are
/// zero — useful for tests but won't decode anything real.
pub(crate) fn build_avcc(
    sps_list: &[Vec<u8>],
    pps_list: &[Vec<u8>],
    length_size: u8,
) -> Vec<u8> {
    assert!(
        matches!(length_size, 1 | 2 | 4),
        "length_size must be 1, 2, or 4 (got {length_size})",
    );
    assert!(sps_list.len() < 32, "avcC supports at most 31 SPS entries");
    assert!(pps_list.len() <= 255, "avcC supports at most 255 PPS entries");

    let (profile, compat, level) = sps_list
        .first()
        .filter(|sps| sps.len() >= 4)
        .map(|sps| (sps[1], sps[2], sps[3]))
        .unwrap_or((0, 0, 0));

    let mut out = Vec::new();
    out.push(0x01); // configurationVersion
    out.push(profile);
    out.push(compat);
    out.push(level);
    out.push(0xFC | ((length_size - 1) & 0x03));
    out.push(0xE0 | (sps_list.len() as u8 & 0x1F));
    for sps in sps_list {
        push_u16_be(&mut out, sps.len() as u16);
        out.extend_from_slice(sps);
    }
    out.push(pps_list.len() as u8);
    for pps in pps_list {
        push_u16_be(&mut out, pps.len() as u16);
        out.extend_from_slice(pps);
    }
    out
}

fn push_u16_be(out: &mut Vec<u8>, v: u16) {
    out.push((v >> 8) as u8);
    out.push((v & 0xFF) as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal NAL payloads accepted by the structural parser: the
    // first byte's low 5 bits encode the NAL unit type. Anything past
    // that byte is opaque to parse_avcc (we don't decode SPS / PPS
    // contents at this layer).
    //
    // 0x67 = forbidden_zero_bit=0 | nal_ref_idc=3 | nal_unit_type=7 (SPS)
    // 0x68 = forbidden_zero_bit=0 | nal_ref_idc=3 | nal_unit_type=8 (PPS)
    const SPS_NAL: &[u8] = &[0x67, 0x42, 0x00, 0x1E, 0xAB, 0xCD];
    const SPS_NAL_2: &[u8] = &[0x67, 0x4D, 0x40, 0x29, 0x11, 0x22];
    const PPS_NAL: &[u8] = &[0x68, 0xEE, 0x12];
    const PPS_NAL_2: &[u8] = &[0x68, 0xFA, 0x34, 0x56];

    fn sample_avcc(sps_list: &[&[u8]], pps_list: &[&[u8]], length_size: u8) -> Vec<u8> {
        let sps_vec: Vec<Vec<u8>> = sps_list.iter().map(|s| s.to_vec()).collect();
        let pps_vec: Vec<Vec<u8>> = pps_list.iter().map(|p| p.to_vec()).collect();
        build_avcc(&sps_vec, &pps_vec, length_size)
    }

    #[test]
    fn parses_well_formed_avcc_one_sps_one_pps() {
        let bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        let avcc = parse_avcc(&bytes).expect("parse");
        assert_eq!(avcc.length_size, 4);
        assert_eq!(avcc.sps_nals, vec![SPS_NAL.to_vec()]);
        assert_eq!(avcc.pps_nals, vec![PPS_NAL.to_vec()]);
    }

    #[test]
    fn parses_multiple_sps_and_pps() {
        let bytes = sample_avcc(&[SPS_NAL, SPS_NAL_2], &[PPS_NAL, PPS_NAL_2], 4);
        let avcc = parse_avcc(&bytes).expect("parse");
        assert_eq!(avcc.sps_nals.len(), 2);
        assert_eq!(avcc.pps_nals.len(), 2);
        assert_eq!(avcc.sps_nals[1], SPS_NAL_2);
        assert_eq!(avcc.pps_nals[1], PPS_NAL_2);
    }

    #[test]
    fn honors_length_size_one_byte() {
        let bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 1);
        let avcc = parse_avcc(&bytes).expect("parse");
        assert_eq!(avcc.length_size, 1);
    }

    #[test]
    fn honors_length_size_two_byte() {
        let bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 2);
        let avcc = parse_avcc(&bytes).expect("parse");
        assert_eq!(avcc.length_size, 2);
    }

    #[test]
    fn rejects_reserved_length_size_minus_one_equal_two() {
        // Hand-craft an avcC with lengthSizeMinusOne=2.
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        bytes[4] = 0xFE; // 0b1111_1110 -> lengthSizeMinusOne=2
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_non_one_configuration_version() {
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        bytes[0] = 0x02;
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_input_shorter_than_header() {
        let bytes = vec![0x01, 0x42, 0x00, 0x1E];
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_truncated_sps_length_prefix() {
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        bytes.truncate(7); // header (6) + 1 of the 2-byte SPS length
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_truncated_sps_payload() {
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        // header (6) + SPS length prefix (2) + half of the SPS bytes
        bytes.truncate(6 + 2 + SPS_NAL.len() / 2);
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_truncated_before_pps_count() {
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        // Cut right after the SPS payload (before numOfPPS).
        bytes.truncate(6 + 2 + SPS_NAL.len());
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_truncated_pps_length_prefix() {
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        // Cut after numOfPPS but before the PPS length is complete.
        bytes.truncate(6 + 2 + SPS_NAL.len() + 1 + 1);
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_truncated_pps_payload() {
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        bytes.truncate(6 + 2 + SPS_NAL.len() + 1 + 2 + PPS_NAL.len() / 2);
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_non_sps_in_sps_slot() {
        let pps_in_sps_slot: &[u8] = PPS_NAL; // NAL type 8 in SPS slot
        let bytes = sample_avcc(&[pps_in_sps_slot], &[PPS_NAL], 4);
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn rejects_non_pps_in_pps_slot() {
        let sps_in_pps_slot: &[u8] = SPS_NAL; // NAL type 7 in PPS slot
        let bytes = sample_avcc(&[SPS_NAL], &[sps_in_pps_slot], 4);
        let err = parse_avcc(&bytes).unwrap_err();
        assert!(matches!(err, DecoderError::MisformedData(_)));
    }

    #[test]
    fn accepts_trailing_bytes_after_pps_list() {
        // High-profile avcC has additional fields (sps_ext etc.) after
        // the PPS list. The parser must ignore them.
        let mut bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        bytes.extend_from_slice(&[0xFC, 0x01, 0xFE, 0x00]); // chroma_format_idc etc.
        let avcc = parse_avcc(&bytes).expect("parse");
        assert_eq!(avcc.length_size, 4);
        assert_eq!(avcc.sps_nals.len(), 1);
        assert_eq!(avcc.pps_nals.len(), 1);
    }

    #[test]
    fn parse_extradata_empty_input_yields_no_nals() {
        let parsed = parse_extradata(&[]).expect("parse");
        assert!(parsed.nals.is_empty());
        assert_eq!(parsed.length_size, None);
    }

    #[test]
    fn parse_extradata_dispatches_avcc_on_first_byte_one() {
        let bytes = sample_avcc(&[SPS_NAL], &[PPS_NAL], 4);
        let parsed = parse_extradata(&bytes).expect("parse");
        assert_eq!(parsed.length_size, Some(4));
        assert_eq!(parsed.nals.len(), 2);
        assert_eq!(parsed.nals[0], SPS_NAL);
        assert_eq!(parsed.nals[1], PPS_NAL);
    }

    #[test]
    fn parse_extradata_annexb_form_with_long_start_codes() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[0, 0, 0, 1]);
        bytes.extend_from_slice(SPS_NAL);
        bytes.extend_from_slice(&[0, 0, 0, 1]);
        bytes.extend_from_slice(PPS_NAL);
        let parsed = parse_extradata(&bytes).expect("parse");
        assert_eq!(parsed.length_size, None);
        assert_eq!(parsed.nals, vec![SPS_NAL.to_vec(), PPS_NAL.to_vec()]);
    }

    #[test]
    fn parse_extradata_annexb_form_with_short_start_codes() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[0, 0, 1]);
        bytes.extend_from_slice(SPS_NAL);
        bytes.extend_from_slice(&[0, 0, 1]);
        bytes.extend_from_slice(PPS_NAL);
        let parsed = parse_extradata(&bytes).expect("parse");
        assert_eq!(parsed.length_size, None);
        assert_eq!(parsed.nals, vec![SPS_NAL.to_vec(), PPS_NAL.to_vec()]);
    }

    #[test]
    fn build_avcc_emits_expected_header_bytes() {
        let bytes = build_avcc(&[SPS_NAL.to_vec()], &[PPS_NAL.to_vec()], 4);
        assert_eq!(bytes[0], 0x01);            // configurationVersion
        assert_eq!(bytes[1], SPS_NAL[1]);      // profile
        assert_eq!(bytes[2], SPS_NAL[2]);      // compat
        assert_eq!(bytes[3], SPS_NAL[3]);      // level
        assert_eq!(bytes[4], 0xFF);            // lengthSize=4 -> minusOne=3
        assert_eq!(bytes[5], 0xE1);            // numOfSPS=1
    }

    #[test]
    fn build_avcc_round_trips_through_parse_avcc() {
        let sps = vec![SPS_NAL.to_vec(), SPS_NAL_2.to_vec()];
        let pps = vec![PPS_NAL.to_vec(), PPS_NAL_2.to_vec()];
        let bytes = build_avcc(&sps, &pps, 2);
        let parsed = parse_avcc(&bytes).expect("parse");
        assert_eq!(parsed.length_size, 2);
        assert_eq!(parsed.sps_nals, sps);
        assert_eq!(parsed.pps_nals, pps);
    }

    #[test]
    fn build_avcc_with_empty_sps_list_still_parses() {
        let bytes = build_avcc(&[], &[PPS_NAL.to_vec()], 4);
        let parsed = parse_avcc(&bytes).expect("parse");
        assert!(parsed.sps_nals.is_empty());
        assert_eq!(parsed.pps_nals.len(), 1);
    }

    #[test]
    #[should_panic(expected = "length_size must be 1, 2, or 4")]
    fn build_avcc_rejects_length_size_three() {
        let _ = build_avcc(&[SPS_NAL.to_vec()], &[PPS_NAL.to_vec()], 3);
    }
}
