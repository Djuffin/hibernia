use super::color::{ColorSpace, PixelFormat};
use super::config::Codec;

/// Active stream geometric and color format parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamFormat {
    /// The active video codec.
    pub codec: Codec,
    /// Coded width of the picture in pixels (includes stride/macroblock alignment).
    pub coded_width: usize,
    /// Coded height of the picture in pixels (includes stride/macroblock alignment).
    pub coded_height: usize,
    /// Crop offset from the left edge for display.
    pub crop_left: usize,
    /// Crop offset from the top edge for display.
    pub crop_top: usize,
    /// Visible width of the picture in pixels.
    pub display_width: usize,
    /// Visible height of the picture in pixels.
    pub display_height: usize,
    /// Active color space of the stream, if specified.
    pub color_space: Option<ColorSpace>,
    /// Pixel format of the decoded pictures.
    pub pixel_format: PixelFormat,
    /// Bits per pixel component (typically 8 for standard, 10 or 12 for HDR).
    pub bit_depth: u8,
}
