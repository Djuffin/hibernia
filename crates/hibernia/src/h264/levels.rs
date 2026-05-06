use super::sps::SequenceParameterSet;

/// Compute MaxDpbFrames per spec A.3.1 / Table A-1.
/// Returns the maximum number of frames the DPB can hold based on level limits.
pub fn max_dpb_frames(sps: &SequenceParameterSet) -> usize {
    // Table A-1: MaxDPB in bytes for each level_idc
    let max_dpb_bytes: u64 = match sps.level_idc {
        10 => 152_064,
        11 => {
            if sps.constraint_set3_flag {
                // Level 1b
                152_064
            } else {
                345_600
            }
        }
        12 => 912_384,
        13 => 912_384,
        20 => 912_384,
        21 => 1_824_768,
        22 => 3_110_400,
        30 => 3_110_400,
        31 => 6_912_000,
        32 => 7_864_320,
        40 => 12_582_912,
        41 => 12_582_912,
        42 => 13_369_344,
        50 => 42_393_600,
        51 => 70_778_880,
        52 => 70_778_880,
        // Level 1b encoded as 9
        9 => 152_064,
        _ => 70_778_880, // fallback to max
    };

    // MaxDpbFrames = Min( MaxDPB / ( PicWidthInMbs * FrameHeightInMbs * 384 ), 16 )
    // For frame coding, FrameHeightInMbs = PicHeightInMapUnits
    let frame_size = sps.pic_width_in_mbs() as u64 * sps.pic_height_in_mbs() as u64 * 384;
    std::cmp::min((max_dpb_bytes / frame_size) as usize, 16)
}

/// Annex A Table A-1 frame size limit (MaxFS, in macroblocks) for the SPS's
/// level. Returns the highest known limit (Level 6.2) for unrecognized level
/// codes, so we cap dimensions even when the level is bogus.
pub fn max_frame_size_in_mbs(sps: &SequenceParameterSet) -> u32 {
    match sps.level_idc {
        // Level 1b is signaled either as level_idc=9, or as level_idc=11 with
        // constraint_set3_flag set (when the profile permits, but for the
        // purposes of frame-size limits the cap is identical to level 1).
        9 => 99,
        10 => 99,
        11 if sps.constraint_set3_flag => 99,
        11 => 396,
        12 => 396,
        13 => 396,
        20 => 396,
        21 => 792,
        22 => 1_620,
        30 => 1_620,
        31 => 3_600,
        32 => 5_120,
        40 => 8_192,
        41 => 8_192,
        42 => 8_704,
        50 => 22_080,
        51 => 36_864,
        52 => 36_864,
        60 => 139_264,
        61 => 139_264,
        62 => 696_320,
        // Unknown / future level_idc: fall back to the highest known cap so a
        // hostile bitstream can't bypass MaxFS by picking an unrecognized code.
        _ => 696_320,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sps_with(level_idc: u8, constraint_set3_flag: bool) -> SequenceParameterSet {
        SequenceParameterSet { level_idc, constraint_set3_flag, ..Default::default() }
    }

    #[test]
    fn max_frame_size_table_values() {
        // Spot-check Annex A Table A-1 values across the range.
        assert_eq!(max_frame_size_in_mbs(&sps_with(10, false)), 99);
        assert_eq!(max_frame_size_in_mbs(&sps_with(9, false)), 99); // level 1b alt encoding
        assert_eq!(max_frame_size_in_mbs(&sps_with(11, true)), 99); // level 1b via cs3
        assert_eq!(max_frame_size_in_mbs(&sps_with(11, false)), 396);
        assert_eq!(max_frame_size_in_mbs(&sps_with(31, false)), 3_600);
        assert_eq!(max_frame_size_in_mbs(&sps_with(40, false)), 8_192);
        assert_eq!(max_frame_size_in_mbs(&sps_with(50, false)), 22_080);
        assert_eq!(max_frame_size_in_mbs(&sps_with(51, false)), 36_864);
        assert_eq!(max_frame_size_in_mbs(&sps_with(60, false)), 139_264);
        assert_eq!(max_frame_size_in_mbs(&sps_with(62, false)), 696_320);
    }

    #[test]
    fn max_frame_size_unknown_level_falls_back_to_max() {
        // Hostile bitstream picking a bogus level_idc still gets capped.
        assert_eq!(max_frame_size_in_mbs(&sps_with(255, false)), 696_320);
        assert_eq!(max_frame_size_in_mbs(&sps_with(0, false)), 696_320);
        assert_eq!(max_frame_size_in_mbs(&sps_with(99, false)), 696_320);
    }
}
