//! H.264-specific commands dispatched through
//! [`crate::api::VideoDecoder::control`].

/// Replace or refresh the decoder's parameter-set tables at runtime.
/// The blob format matches [`crate::api::H264Config::extradata`]: an
/// avcC record (`bytes[0] == 1`) or a concatenation of Annex-B-framed
/// SPS+PPS NALs.
///
/// Typical use:
///
/// - **Fragmented MP4 sample-entry change.** When a new fragment
///   introduces a different `avcC`, send a `H264SetExtradata` with
///   the new blob before feeding any of the fragment's samples.
///
/// - **WebRTC SDP renegotiation.** When `sprop-parameter-sets`
///   changes, parse the base64 into NAL bytes and send via this
///   command.
///
/// - **Delayed parameter-set arrival.** When the parameter sets
///   weren't available at construction time but show up later through
///   a side channel.
///
/// Send it via:
///
/// ```ignore
/// let mut cmd = H264SetExtradata { data: avcc_blob };
/// decoder.control(&mut cmd)?;
/// ```
///
/// The decoder parses the blob, feeds the contained SPS/PPS NALs
/// through the normal parsing pipeline (which overwrites any
/// previously stored entries with the same parameter-set ID), and
/// updates `nal_length_size` from `lengthSizeMinusOne` if the blob is
/// avcC-shaped.
///
/// Malformed input returns `DecoderError::MisformedData` (or another
/// specific variant if SPS validation rejects the contents).
#[derive(Debug, Clone)]
pub struct H264SetExtradata {
    pub data: Vec<u8>,
}
