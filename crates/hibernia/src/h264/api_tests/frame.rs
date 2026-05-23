//! Tests for `VideoFrame` / `PlaneView` semantics on emitted pictures.

use std::sync::Arc;

use crate::api::config::{Codec, DecoderConfig};
use crate::api::frame::VideoFrame;
use crate::api::VideoPlane;

use super::support::{
    baseline_bframe_packets, default_decoder, drive_through, plane_data_is_addressable,
    CountingCallbacks,
};

fn first_picture() -> crate::api::DecodedPicture {
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let mut pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    pics.remove(0)
}

#[test]
fn plane_y_u_v_all_present_for_i420_stream() {
    let pic = first_picture();
    assert!(pic.frame.plane(VideoPlane::Y).is_some(), "Y plane must be present");
    assert!(pic.frame.plane(VideoPlane::U).is_some(), "U plane must be present");
    assert!(pic.frame.plane(VideoPlane::V).is_some(), "V plane must be present");
}

#[test]
fn plane_uv_returns_none_for_planar_i420() {
    // I420 is fully planar; the interleaved UV channel must not exist.
    let pic = first_picture();
    assert!(pic.frame.plane(VideoPlane::UV).is_none());
}

#[test]
fn plane_alpha_returns_none() {
    let pic = first_picture();
    assert!(pic.frame.plane(VideoPlane::Alpha).is_none());
}

#[test]
fn planes_returns_y_u_v_then_none() {
    let pic = first_picture();
    let planes = pic.frame.planes();
    assert!(planes[0].is_some(), "first slot is Y");
    assert!(planes[1].is_some(), "second slot is U");
    assert!(planes[2].is_some(), "third slot is V");
    assert!(planes[3].is_none(), "fourth slot empty for I420");
    assert_eq!(planes[0].as_ref().unwrap().plane, VideoPlane::Y);
    assert_eq!(planes[1].as_ref().unwrap().plane, VideoPlane::U);
    assert_eq!(planes[2].as_ref().unwrap().plane, VideoPlane::V);
}

#[test]
fn plane_view_dimensions_match_format() {
    let pic = first_picture();
    let y = pic.frame.plane(VideoPlane::Y).unwrap();
    let u = pic.frame.plane(VideoPlane::U).unwrap();
    let v = pic.frame.plane(VideoPlane::V).unwrap();
    // Luma matches the coded geometry; chroma is half (4:2:0).
    assert_eq!(y.width, pic.format.coded_width);
    assert_eq!(y.height, pic.format.coded_height);
    assert_eq!(u.width, pic.format.coded_width / 2);
    assert_eq!(u.height, pic.format.coded_height / 2);
    assert_eq!(v.width, u.width);
    assert_eq!(v.height, u.height);
}

#[test]
fn plane_view_stride_is_at_least_width() {
    let pic = first_picture();
    for ch in [VideoPlane::Y, VideoPlane::U, VideoPlane::V] {
        let view = pic.frame.plane(ch).unwrap();
        assert!(view.stride >= view.width, "stride {} < width {}", view.stride, view.width);
    }
}

#[test]
fn plane_view_data_is_addressable() {
    let pic = first_picture();
    for ch in [VideoPlane::Y, VideoPlane::U, VideoPlane::V] {
        let view = pic.frame.plane(ch).unwrap();
        assert!(
            plane_data_is_addressable(&view),
            "{:?}: data.len() = {} insufficient for {}x{} stride {}",
            ch,
            view.data.len(),
            view.width,
            view.height,
            view.stride,
        );
    }
}

#[test]
fn plane_view_plane_field_matches_channel() {
    let pic = first_picture();
    assert_eq!(pic.frame.plane(VideoPlane::Y).unwrap().plane, VideoPlane::Y);
    assert_eq!(pic.frame.plane(VideoPlane::U).unwrap().plane, VideoPlane::U);
    assert_eq!(pic.frame.plane(VideoPlane::V).unwrap().plane, VideoPlane::V);
}

#[test]
fn arc_video_frame_is_send_sync() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn VideoFrame>();
    assert_send_sync::<Arc<dyn VideoFrame>>();
}

#[test]
fn frame_share_across_threads_keeps_data_alive() {
    // Take a frame, clone the Arc, send the clone to another thread,
    // verify the worker can still read the visible area. This
    // doubles as a Send + Sync runtime check for Arc<dyn VideoFrame>.
    let pic = first_picture();
    let frame_clone = Arc::clone(&pic.frame);
    let handle = std::thread::spawn(move || {
        let y = frame_clone.plane(VideoPlane::Y).unwrap();
        // Read first and last visible bytes.
        let first = y.data[0];
        let last = y.data[(y.height - 1) * y.stride + y.width - 1];
        (first, last)
    });
    let (_first, _last) = handle.join().expect("worker thread");
    // The original Arc is still alive too.
    let _y = pic.frame.plane(VideoPlane::Y).unwrap();
}

#[test]
fn pictures_carry_independent_frames() {
    // Two consecutive pictures must report distinct Arc<dyn VideoFrame>
    // (different underlying allocations). Prevents accidental aliasing
    // between display-order neighbors.
    let callbacks = CountingCallbacks::shared();
    let mut decoder =
        default_decoder(DecoderConfig::new(Codec::H264), callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(pics.len() >= 2);
    let first_ptr = Arc::as_ptr(&pics[0].frame) as *const ();
    let second_ptr = Arc::as_ptr(&pics[1].frame) as *const ();
    assert_ne!(first_ptr, second_ptr);
}
