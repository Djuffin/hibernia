//! H.264-specific commands dispatched through
//! [`crate::api::VideoDecoder::control`].

/// Replace the decoder's parameter-set tables at runtime. `data` is
/// either an ISO/IEC 14496-15 `AVCDecoderConfigurationRecord` (a.k.a.
/// avcC) or a concatenation of Annex-B-framed SPS+PPS NALs -- the
/// same blob format as [`crate::api::H264Config::extradata`].
/// Entries with matching parameter-set IDs overwrite the previous
/// ones.
///
/// ```ignore
/// let mut cmd = H264SetExtradata { data: avcc_blob };
/// decoder.control(&mut cmd)?;
/// ```
#[derive(Debug, Clone)]
pub struct H264SetExtradata {
    pub data: Vec<u8>,
}
