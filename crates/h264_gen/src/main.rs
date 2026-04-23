use h264_gen::bitstream_gen::{generate_bitstream, BitstreamConfig};
use std::env;
use std::fs;
use std::process;

const HELP_TEXT: &str = r#"
H.264 Bitstream Generator

This tool reads a JSON file describing a sequence of H.264 NAL units and generates a valid
Annex B bitstream (.264 file).

USAGE:
    h264_gen <input.json> <output.264>
    h264_gen --schema
    h264_gen --help | -h

INPUT JSON SCHEMA:
Run `h264_gen --schema` to get the full JSON Schema for the input format.
The input is a JSON array of NAL Unit Descriptors. Each descriptor defines the NAL header
and its payload (either an SPS, PPS, Slice, or RawHex).

NOTES:
- The JSON closely maps to the internal `SequenceParameterSet`, `PicParameterSet`, and `SliceHeader` structs.
- When injecting a Slice, you can supply optional `writer_sps` and `writer_pps` objects. These are ONLY needed
  if you are intentionally generating malformed Slice headers referencing unknown IDs, to provide the bit-writer
  with the layout context it needs (e.g., bit widths).
- Slice data supports high-level directives: `I_PCM` (generates flat colors) and `P_Skip` (generates skip runs).
- You can also inject arbitrary hex payloads via the `RawHex` structure.
- Emulation prevention bytes (`0x03`) are inserted automatically by the tool. Do not add them in your RawHex
  unless you are injecting an already-escaped RBSP payload.
"#;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("{}", HELP_TEXT);
        process::exit(0);
    }

    if args.len() == 2 && args[1] == "--schema" {
        let schema = schemars::schema_for!(BitstreamConfig);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        process::exit(0);
    }

    if args.len() != 3 {
        eprintln!(
            "Usage: {} <input.json> <output.264>\nRun with --help for detailed documentation.",
            args[0]
        );
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let json_data = match fs::read_to_string(input_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input file '{}': {}", input_path, e);
            process::exit(1);
        }
    };

    let config: BitstreamConfig = match serde_json::from_str(&json_data) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            process::exit(1);
        }
    };

    let bitstream = match generate_bitstream(&config) {
        Ok(bs) => bs,
        Err(e) => {
            eprintln!("Error generating bitstream: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(output_path, bitstream) {
        eprintln!("Error writing output file '{}': {}", output_path, e);
        process::exit(1);
    }

    println!("Successfully generated {}", output_path);
}
