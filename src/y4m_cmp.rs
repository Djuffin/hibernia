use crate::h264::tables::MB_WIDTH;

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
    let actual_u = actual.get_u_plane();
    let expected_u = expected.get_u_plane();
    if let Some((x, y, a, e)) = compare_plane(width / 2, height / 2, actual_u, expected_u) {
        result.push_str(&format!("U-plane mismatch at {x},{y} : {a} != {e}\n"));
    }
    let actual_v = actual.get_v_plane();
    let expected_v = expected.get_v_plane();
    if let Some((x, y, a, e)) = compare_plane(width / 2, height / 2, actual_v, expected_v) {
        result.push_str(&format!("V-plane mismatch at {x},{y} : {a} != {e}\n"));
    }

    result
}
