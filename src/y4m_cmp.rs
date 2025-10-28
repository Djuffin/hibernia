use v_frame::frame;

use crate::h264::tables::MB_WIDTH;

use std::fmt::format;
use std::fs;
use std::io;
use std::io::Error;

fn compare_plane(
    width: usize,
    height: usize,
    actual: &[u8],
    expected: &[u8],
) -> Option<(usize, usize, u8, u8)> {
    let mut idx = 0;
    assert!(actual.len() == width * height);
    assert!(expected.len() == width * height);
    for y in 0..height {
        for x in 0..width {
            if actual[idx] != expected[idx] {
                return Some((x, y, actual[idx], expected[idx]));
            }
            idx += 1;
        }
    }
    None
}

pub fn compare_frames(
    width: usize,
    height: usize,
    actual: &y4m::Frame,
    expected: &y4m::Frame,
) -> String {
    let mut result = String::new();
    let actual_y = actual.get_y_plane();
    let expected_y = expected.get_y_plane();
    if let Some((x, y, a, e)) = compare_plane(width, height, actual_y, expected_y) {
        let width_in_mb = width / MB_WIDTH;
        let mb_idx = x / MB_WIDTH + y / MB_WIDTH * width_in_mb;
        result.push_str(&format!("Y-plane mismatch at {x},{y} (MB:{mb_idx}) : {a} != {e}\n"));
    }
    let chroma_mb_width = MB_WIDTH / 2;
    let actual_u = actual.get_u_plane();
    let expected_u = expected.get_u_plane();
    if let Some((x, y, a, e)) = compare_plane(width / 2, height / 2, actual_u, expected_u) {
        let width_in_mb = width / 2 / chroma_mb_width;
        let mb_idx = x / chroma_mb_width + (y / chroma_mb_width) * width_in_mb;
        result.push_str(&format!("U-plane mismatch at {x},{y} (MB:{mb_idx}) : {a} != {e}\n"));
    }
    let actual_v = actual.get_v_plane();
    let expected_v = expected.get_v_plane();
    if let Some((x, y, a, e)) = compare_plane(width / 2, height / 2, actual_v, expected_v) {
        let width_in_mb = width / 2 / chroma_mb_width;
        let mb_idx = x / chroma_mb_width + (y / chroma_mb_width) * width_in_mb;
        result.push_str(&format!(
            "V-plane mismatch at {x},{y} (MB:{mb_idx}) width_in_mb:{width_in_mb} : {a} != {e}\n"
        ));
    }

    result
}

pub fn compare_files(actual_filename: &str, expected_filename: &str) -> Result<(), String> {
    fn stringify(x: Error) -> String {
        format!("error code: {x}")
    }
    fn y4m_stringify(x: y4m::Error) -> String {
        format!("error code: {x}")
    }

    let expected_file = fs::File::open(expected_filename).map_err(stringify)?;
    let expected_reader = io::BufReader::new(expected_file);
    let mut expected_decoder = y4m::Decoder::new(expected_reader).map_err(y4m_stringify)?;

    let actual_file = fs::File::open(actual_filename).map_err(stringify)?;
    let actual_reader = io::BufReader::new(actual_file);
    let mut actual_decoder = y4m::Decoder::new(actual_reader).map_err(y4m_stringify)?;

    let expected_h = expected_decoder.get_height();
    let expected_w = expected_decoder.get_width();
    let actual_h = actual_decoder.get_height();
    let actual_w = actual_decoder.get_width();
    if (expected_w, expected_h) != (actual_w, actual_h) {
        return Err(format!("Unexpected size of frames. {actual_w}x{actual_h} vs expected {expected_w}x{expected_h}"));
    }

    let mut frame_idx = 0;
    while let (Ok(actual_frame), Ok(expected_frame)) =
        (actual_decoder.read_frame(), expected_decoder.read_frame())
    {
        let compare_result = compare_frames(expected_w, expected_h, &actual_frame, &expected_frame);
        if !compare_result.is_empty() {
            return Err(format!("Frame #{frame_idx} mismatch: {compare_result}"));
        }
        frame_idx += 1;
    }

    if let Err(y4m::Error::EOF) = actual_decoder.read_frame() {
    } else {
        return Err(format!("Unexpected number of frames. {frame_idx}"));
    }

    Ok(())
}
