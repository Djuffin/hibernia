/// Color primaries (ISO/IEC 23091-2 section 8.1).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ColorPrimaries {
    #[default]
    Unspecified,
    Bt709,
    /// BT.601 525-line (NTSC).
    Smpte170m,
    /// BT.601 625-line (PAL/SECAM).
    Bt470bg,
    Bt2020,
    /// Display P3 (SMPTE EG 432-1, D65 white point).
    Smpte432,
}

/// Opto-electronic transfer characteristic (ISO/IEC 23091-2 section 8.2).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TransferCharacteristics {
    #[default]
    Unspecified,
    Bt709,
    Bt601,
    Smpte240,
    Linear,
    Srgb,
    Bt2020_10,
    Bt2020_12,
    /// SMPTE ST 2084 / HDR10.
    SmptePq,
    /// ARIB STD-B67 / HLG.
    AribStdB67,
}

/// YUV-to-RGB matrix (ISO/IEC 23091-2 section 8.3).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MatrixCoefficients {
    #[default]
    Unspecified,
    /// RGB / GBR -- no YUV conversion.
    Identity,
    Bt709,
    Bt601,
    Smpte240,
    /// BT.2020 non-constant luminance.
    Bt2020Ncl,
    /// BT.2020 constant luminance.
    Bt2020Cl,
}

/// Sample range. When the bitstream does not signal range, decoders
/// default to `Limited`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ColorRange {
    #[default]
    Limited,
    Full,
}

/// Full color signaling for a stream. Primaries, transfer, and matrix
/// are orthogonal in the spec and codecs may carry them independently
/// (H.264/HEVC/AV1 VUI). Legacy codecs that carry only a combined
/// label (VP9's 3-bit field) map onto this struct via a fixed table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ColorSpace {
    pub primaries: ColorPrimaries,
    pub transfer: TransferCharacteristics,
    pub matrix: MatrixCoefficients,
    pub range: ColorRange,
}

/// Pixel memory layout and chroma subsampling format.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// Planar YUV 4:2:0 (e.g., I420: Y plane, then U plane, then V plane).
    I420,
    /// Semi-planar YUV 4:2:0 (e.g., NV12: Y plane, then interleaved UV plane).
    NV12,
    /// Planar YUV 4:2:2 (e.g., I422).
    I422,
    /// Planar YUV 4:4:4 (e.g., I444).
    I444,
    /// Planar YUV 4:2:0 with alpha plane (I420 + A).
    I420A,
    /// Single Y plane (grayscale).
    Monochrome,
}

/// Plane channels inside a video frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoPlane {
    /// Luma plane.
    Y,
    /// Chroma Cb plane (planar formats only).
    U,
    /// Chroma Cr plane (planar formats only).
    V,
    /// Interleaved Cb/Cr plane (semi-planar formats like NV12).
    UV,
    /// Transparency plane.
    Alpha,
}
