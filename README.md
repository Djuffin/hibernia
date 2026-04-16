# Hibernia

A clean-room implementation of an H.264 (AVC) video decoder written in pure Rust.

> **🚧 Work in Progress 🚧**
>
> This crate is currently in early development. It currently supports **Baseline** and **Main** (progressive) profiles.
> Advanced features like Interlacing and High Profile tools are not yet implemented.

## Features

*   **Pure Rust**: No C dependencies.
*   **Spec-Compliant**: Implementation matches the [ITU-T H.264 Specification](https://www.itu.int/rec/T-REC-H.264).
*   **Safe**: Prioritizes safe Rust code.

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
