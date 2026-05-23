# Hibernia

A clean-room implementation of an H.264 (AVC) video decoder written in pure Rust.

Hibernia targets 8-bit 4:2:0 progressive content — the most common subset of H.264 in
practice.

## Features

*   **Pure Rust**, no C dependencies, prioritizes safe code.
*   **Spec-compliant**: follows [ITU-T H.264](https://www.itu.int/rec/T-REC-H.264).

## Supported

Baseline, Main, and High profiles, within the constraints below.

## Not supported

The decoder returns `FeatureNotSupported` when a stream uses any of:

*   Chroma formats other than 4:2:0 (4:0:0, 4:2:2, 4:4:4).
*   Bit depths other than 8 (High10 / High422 / High444).
*   Interlaced video (`frame_mbs_only_flag = 0`, MBAFF, PAFF).
*   `gaps_in_frame_num_value_allowed_flag = 1`.
*   Slice groups / FMO (`num_slice_groups_minus1 > 0`).
*   Constrained intra prediction (`constrained_intra_pred_flag = 1`).
*   SP / SI slices (parsed but reconstruction not implemented).
*   Extension profiles: Scalable (SVC), Multiview (MVC), Stereo, 3D / depth.
*   SEI message contents (parsed at NAL level but ignored).

## Demo

You can try out the live WebAssembly demo here: **[Hibernia Decoder Demo](https://Djuffin.github.io/hibernia/demo/)**

## Usage

Add `hibernia` to your `Cargo.toml`:

```toml
[dependencies]
hibernia = "0.2.0"
```

### Basic Example

### Basic example

```rust
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use hibernia::api::{
    create_decoder, Codec, DecoderConfig, DefaultAllocator, EncodedPacket, FlushMode,
    StreamFormat, VideoDecoderCallbacks, VideoPlane,
};
use hibernia::h264::nal_parser::NalParser;

struct PrintCallbacks;
impl VideoDecoderCallbacks for PrintCallbacks {
    fn on_picture_available(&self) {}
    fn on_format_changed(&self, format: StreamFormat) {
        println!("format: {}x{} (pixel {:?})", format.display_width, format.display_height, format.pixel_format);
    }
}

fn main() {
    let file = File::open("test.264").expect("file not found");
    let nal_parser = NalParser::new(BufReader::new(file));

    let mut decoder = create_decoder(
        DecoderConfig::new(Codec::H264),
        Arc::new(DefaultAllocator),
        Arc::new(PrintCallbacks),
    )
    .expect("create_decoder");

    // Feed per-NAL packets. Each one carries an Annex-B start code so
    // the decoder's bitstream splitter can find the NAL inside.
    for nal_result in nal_parser {
        let nal = nal_result.expect("nal parse");
        let mut buf = Vec::with_capacity(nal.len() + 4);
        buf.extend_from_slice(&[0, 0, 0, 1]);
        buf.extend_from_slice(&nal);
        decoder.decode(EncodedPacket::from_vec(buf)).expect("decode");

        while let Some(pic) = decoder.get_picture().expect("get_picture") {
            let y = pic.frame.plane(VideoPlane::Y).expect("luma");
            println!(
                "frame {}x{}, luma stride {}",
                pic.format.display_width, pic.format.display_height, y.stride,
            );
        }
    }

    // End of stream: drain remaining pictures held in the DPB.
    decoder.flush(FlushMode::Drain).expect("flush");
    while let Some(_pic) = decoder.get_picture().expect("get_picture") {
        // ...
    }
}
```

## License

This project is licensed under the MIT License -- see the [LICENSE](LICENSE) file for details.
