//! Map an H.264 `SequenceParameterSet` plus VUI signaling into the
//! codec-agnostic [`crate::api::StreamFormat`].

use crate::api::color::{
    ColorPrimaries, ColorRange, ColorSpace, MatrixCoefficients, PixelFormat,
    TransferCharacteristics,
};
use crate::api::config::Codec;
use crate::api::format::StreamFormat;

use super::sps::SequenceParameterSet;

/// Build a `StreamFormat` from an active SPS.
///
/// Parsing-time validation (chroma format, frame-mbs-only, etc.)
/// already happened inside the H.264 decoder; this function only
/// translates already-validated fields into the codec-agnostic shape.
pub fn stream_format_from_sps(sps: &SequenceParameterSet) -> StreamFormat {
    let crop = sps.crop_dimensions();
    let color_space = sps.vui_parameters.as_ref().and_then(|vui| {
        if !vui.video_signal_type_present_flag {
            return None;
        }
        let range = if vui.video_full_range_flag { ColorRange::Full } else { ColorRange::Limited };
        if !vui.color_description_present_flag {
            return Some(ColorSpace {
                primaries: ColorPrimaries::Unspecified,
                transfer: TransferCharacteristics::Unspecified,
                matrix: MatrixCoefficients::Unspecified,
                range,
            });
        }
        Some(ColorSpace {
            primaries: map_primaries(vui.color_primaries),
            transfer: map_transfer(vui.transfer_characteristics),
            matrix: map_matrix(vui.matrix_coefficients),
            range,
        })
    });

    StreamFormat {
        codec: Codec::H264,
        coded_width: sps.pic_width(),
        coded_height: sps.pic_height(),
        crop_left: crop.crop_left,
        crop_top: crop.crop_top,
        display_width: crop.display_width,
        display_height: crop.display_height,
        color_space,
        pixel_format: PixelFormat::I420,
        bit_depth: 8 + sps.bit_depth_luma_minus8,
    }
}

/// ISO/IEC 14496-10 Table E-3.
fn map_primaries(code: u8) -> ColorPrimaries {
    match code {
        1 => ColorPrimaries::Bt709,
        5 => ColorPrimaries::Bt470bg,
        6 | 7 => ColorPrimaries::Smpte170m,
        9 => ColorPrimaries::Bt2020,
        12 => ColorPrimaries::Smpte432,
        _ => ColorPrimaries::Unspecified,
    }
}

/// ISO/IEC 14496-10 Table E-4.
fn map_transfer(code: u8) -> TransferCharacteristics {
    match code {
        1 => TransferCharacteristics::Bt709,
        6 => TransferCharacteristics::Bt601,
        7 => TransferCharacteristics::Smpte240,
        8 => TransferCharacteristics::Linear,
        13 => TransferCharacteristics::Srgb,
        14 => TransferCharacteristics::Bt2020_10,
        15 => TransferCharacteristics::Bt2020_12,
        16 => TransferCharacteristics::SmptePq,
        18 => TransferCharacteristics::AribStdB67,
        _ => TransferCharacteristics::Unspecified,
    }
}

/// ISO/IEC 14496-10 Table E-5.
fn map_matrix(code: u8) -> MatrixCoefficients {
    match code {
        0 => MatrixCoefficients::Identity,
        1 => MatrixCoefficients::Bt709,
        6 | 5 => MatrixCoefficients::Bt601,
        7 => MatrixCoefficients::Smpte240,
        9 => MatrixCoefficients::Bt2020Ncl,
        10 => MatrixCoefficients::Bt2020Cl,
        _ => MatrixCoefficients::Unspecified,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h264::sps::{FrameCrop, SequenceParameterSet, VuiParameters};
    use crate::h264::{ChromaFormat, Profile};

    fn base_sps() -> SequenceParameterSet {
        SequenceParameterSet {
            profile: Profile::Baseline,
            chroma_format_idc: ChromaFormat::YUV420,
            // 320x240 visible at 16-px MB alignment -> 20x15 MBs = 320x240
            pic_width_in_mbs_minus1: 19,
            pic_height_in_map_units_minus1: 14,
            frame_mbs_only_flag: true,
            ..Default::default()
        }
    }

    #[test]
    fn coded_geometry_matches_sps() {
        let sps = base_sps();
        let fmt = stream_format_from_sps(&sps);
        assert_eq!(fmt.codec, Codec::H264);
        assert_eq!(fmt.coded_width, 320);
        assert_eq!(fmt.coded_height, 240);
        assert_eq!(fmt.display_width, 320);
        assert_eq!(fmt.display_height, 240);
        assert_eq!(fmt.pixel_format, PixelFormat::I420);
        assert_eq!(fmt.bit_depth, 8);
        assert!(fmt.color_space.is_none());
    }

    #[test]
    fn crop_propagates() {
        let mut sps = base_sps();
        sps.frame_cropping = Some(FrameCrop { left: 0, right: 4, top: 0, bottom: 4 });
        let fmt = stream_format_from_sps(&sps);
        // crop_unit = 2 (4:2:0, frame-only) so 4 -> 8 pixels per side.
        assert_eq!(fmt.display_width, 320 - 8);
        assert_eq!(fmt.display_height, 240 - 8);
    }

    #[test]
    fn vui_color_signaling_maps_to_full_struct() {
        let mut sps = base_sps();
        sps.vui_parameters = Some(VuiParameters {
            video_signal_type_present_flag: true,
            video_full_range_flag: true,
            color_description_present_flag: true,
            color_primaries: 1,         // BT.709
            transfer_characteristics: 16, // SMPTE PQ
            matrix_coefficients: 9,     // BT.2020 NCL
            ..Default::default()
        });
        let fmt = stream_format_from_sps(&sps);
        let cs = fmt.color_space.expect("color_space");
        assert_eq!(cs.primaries, ColorPrimaries::Bt709);
        assert_eq!(cs.transfer, TransferCharacteristics::SmptePq);
        assert_eq!(cs.matrix, MatrixCoefficients::Bt2020Ncl);
        assert_eq!(cs.range, ColorRange::Full);
    }

    #[test]
    fn vui_without_color_description_returns_unspecified_color_space() {
        let mut sps = base_sps();
        sps.vui_parameters = Some(VuiParameters {
            video_signal_type_present_flag: true,
            video_full_range_flag: false,
            color_description_present_flag: false,
            ..Default::default()
        });
        let fmt = stream_format_from_sps(&sps);
        let cs = fmt.color_space.expect("color_space");
        assert_eq!(cs.primaries, ColorPrimaries::Unspecified);
        assert_eq!(cs.range, ColorRange::Limited);
    }
}
