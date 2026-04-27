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
hibernia = "0.1.0"
```

### Basic Example

```rust
use std::fs::File;
use std::io::BufReader;
use hibernia::h264::nal_parser::NalParser;
use hibernia::h264::decoder::Decoder;

fn main() {
    let file = File::open("test.264").expect("File not found");
    let reader = BufReader::new(file);

    // Parse NAL units from the byte stream
    let nal_parser = NalParser::new(reader);

    // Initialize the decoder
    let mut decoder = Decoder::new();

    for nal_result in nal_parser {
        let nal_data = nal_result.expect("Error parsing NAL");

        // Feed NAL unit to the decoder
        decoder.decode(&nal_data).expect("Decoding error");

        // Retrieve decoded pictures (if any are ready)
        while let Some(pic) = decoder.retrieve_picture() {
            println!("Decoded frame: {}x{}", pic.crop.display_width, pic.crop.display_height);
            // Process the picture (e.g., save to disk, display, etc.)
        }
    }

    // Flush the decoder to get the remaining pictures
    decoder.flush().expect("Flush error");
    while let Some(pic) = decoder.retrieve_picture() {
        println!("Decoded frame: {}x{}", pic.crop.display_width, pic.crop.display_height);
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
