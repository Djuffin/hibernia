//! Tests for `VideoFrameAllocator` integration: custom allocators
//! receive plane requests, errors propagate, and the
//! `DefaultAllocator` honors the alignment contract.

use std::sync::Arc;

use crate::api::callbacks::DecoderError;
use crate::api::config::{Codec, DecoderConfig};
use crate::api::decoder::VideoDecoder;
use crate::api::frame::{
    AllocError, BufferAllocation, FrameBuffer, PlaneAllocation, VideoFrameAllocator,
};
use crate::api::DefaultAllocator;
use crate::api::VideoPlane;
use crate::h264::decoder::Decoder;

use super::support::{
    baseline_bframe_packets, drive_through, CountingCallbacks, TrackingAllocator,
};

#[test]
fn default_allocator_returns_aligned_pointers() {
    let req = BufferAllocation {
        planes: [
            Some(PlaneAllocation { plane: VideoPlane::Y, size_bytes: 4096, alignment: 16 }),
            Some(PlaneAllocation { plane: VideoPlane::U, size_bytes: 1024, alignment: 16 }),
            Some(PlaneAllocation { plane: VideoPlane::V, size_bytes: 1024, alignment: 16 }),
            None,
        ],
    };
    let alloc = DefaultAllocator;
    let buf = alloc.alloc_frame(&req).expect("alloc");
    for ch in [VideoPlane::Y, VideoPlane::U, VideoPlane::V] {
        let ptr = buf.plane_ptr(ch).expect("plane present");
        assert_eq!(ptr.cast::<u8>().as_ptr() as usize % 16, 0, "plane {:?} not 16-aligned", ch);
    }
}

#[test]
fn default_allocator_rejects_non_power_of_two_alignment() {
    let req = BufferAllocation {
        planes: [
            Some(PlaneAllocation { plane: VideoPlane::Y, size_bytes: 64, alignment: 3 }),
            None,
            None,
            None,
        ],
    };
    let err = DefaultAllocator.alloc_frame(&req).map(|_| ()).unwrap_err();
    assert_eq!(err, AllocError::UnsupportedAlignment);
}

#[test]
fn tracking_allocator_invoked_once_per_decoded_picture() {
    let callbacks = CountingCallbacks::shared();
    let alloc = TrackingAllocator::new();
    let alloc_dyn: Arc<dyn VideoFrameAllocator> = alloc.clone();
    let mut decoder =
        Decoder::new(DecoderConfig::new(Codec::H264), alloc_dyn, callbacks).expect("construct");
    let pics = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    assert!(!pics.is_empty());
    // One alloc per primary coded picture. The fixture may have IDR +
    // a handful of B/P pictures.
    assert!(
        alloc.count() >= pics.len(),
        "expected >= one alloc per emitted picture (alloc count {} vs pics {})",
        alloc.count(),
        pics.len(),
    );
}

#[test]
fn tracking_allocator_sees_y_u_v_request_shape() {
    let callbacks = CountingCallbacks::shared();
    let alloc = TrackingAllocator::new();
    let alloc_dyn: Arc<dyn VideoFrameAllocator> = alloc.clone();
    let mut decoder = Decoder::new(DecoderConfig::new(Codec::H264), alloc_dyn, callbacks)
        .expect("construct");
    let _ = drive_through(&mut decoder, baseline_bframe_packets()).expect("drive");
    let last = alloc.last_request.lock().unwrap().clone().expect("at least one request");
    let y = last.planes[0].expect("Y slot populated");
    let u = last.planes[1].expect("U slot populated");
    let v = last.planes[2].expect("V slot populated");
    assert_eq!(y.plane, VideoPlane::Y);
    assert_eq!(u.plane, VideoPlane::U);
    assert_eq!(v.plane, VideoPlane::V);
    assert!(last.planes[3].is_none(), "4th slot empty for 4:2:0");
    // 4:2:0 -> chroma needs about a quarter of luma area, give or
    // take stride padding.
    assert!(u.size_bytes < y.size_bytes);
    assert!(v.size_bytes < y.size_bytes);
}

#[test]
fn alloc_out_of_memory_propagates_as_decoder_error() {
    let callbacks = CountingCallbacks::shared();
    let alloc = TrackingAllocator::new();
    alloc.set_failure(AllocError::OutOfMemory);
    let alloc_dyn: Arc<dyn VideoFrameAllocator> = alloc.clone();
    let mut decoder = Decoder::new(DecoderConfig::new(Codec::H264), alloc_dyn, callbacks)
        .expect("construct");

    // Feed the stream; the first allocation attempt should fail.
    let mut saw_alloc_error = false;
    for packet in baseline_bframe_packets() {
        match decoder.decode(packet) {
            Ok(()) => {}
            Err(DecoderError::Alloc(AllocError::OutOfMemory)) => {
                saw_alloc_error = true;
                break;
            }
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }
    assert!(saw_alloc_error, "OOM allocator must surface DecoderError::Alloc(OutOfMemory)");
}

#[test]
fn alloc_unsupported_alignment_propagates_as_decoder_error() {
    let callbacks = CountingCallbacks::shared();
    let alloc = TrackingAllocator::new();
    alloc.set_failure(AllocError::UnsupportedAlignment);
    let alloc_dyn: Arc<dyn VideoFrameAllocator> = alloc.clone();
    let mut decoder = Decoder::new(DecoderConfig::new(Codec::H264), alloc_dyn, callbacks)
        .expect("construct");
    let mut saw = false;
    for packet in baseline_bframe_packets() {
        if let Err(DecoderError::Alloc(AllocError::UnsupportedAlignment)) = decoder.decode(packet) {
            saw = true;
            break;
        }
    }
    assert!(saw, "UnsupportedAlignment must surface");
}

#[test]
fn from_alloc_error_for_decoder_error_works() {
    let dec: DecoderError = AllocError::OutOfMemory.into();
    assert_eq!(dec, DecoderError::Alloc(AllocError::OutOfMemory));
}
