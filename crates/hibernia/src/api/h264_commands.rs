//! H.264-specific commands dispatched through
//! [`crate::api::VideoDecoder::control`].

/// Replace the decoder's parameter-set tables at runtime. Same blob
/// format as [`crate::api::H264Config::extradata`]; entries with
/// matching parameter-set IDs overwrite the previous ones.
///
/// ```ignore
/// let mut cmd = H264SetExtradata { data: avcc_blob };
/// decoder.control(&mut cmd)?;
/// ```
#[derive(Debug, Clone)]
pub struct H264SetExtradata {
    pub data: Vec<u8>,
}
