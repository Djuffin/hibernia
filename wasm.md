# Hibernia WebAssembly Demo

This guide explains how to set up, build, and run the Hibernia WebAssembly (WASM) demo.

## Prerequisites

1. **Install Rust and Cargo:**
   Do it.

2. **Add the WASM Target:**
   You need to add the `wasm32-unknown-unknown` target so the Rust compiler can build for WebAssembly:
   ```
   rustup target add wasm32-unknown-unknown
   ```

3. **Install `wasm-pack`:**
   `wasm-pack` is a tool for building Rust-generated WebAssembly and its JavaScript bindings. Install it using its initialization script:
   ```
   cargo install wasm-pack
   ```

## Building the Demo

The demo HTML page expects the compiled WebAssembly and JS bindings to be located in the `demo/pkg` directory.

1. Build the WASM module for the web, targeting the `demo/pkg` output directory:
   ```
   wasm-pack build --target web --out-dir demo/pkg
   ```

2. Ensure the test video file is in the `demo/` directory. For example, you can copy one of the test assets:
   ```
   cp data/SVA_BA2_D.264 demo/
   ```

## Running the Demo Locally

Modern web browsers require WebAssembly modules and ES modules to be served over HTTP/HTTPS rather than
via the `file://` protocol due to strict MIME type checking and CORS policies.

1. Start a local HTTP server in the root of the project directory. Python provides an easy way to do this:
   ```
   python3 -m http.server 8080
   ```

2. Open your browser and navigate to:
   ```text
   http://localhost:8080/demo/
   ```

You should see the Hibernia H.264 Decoder Demo page. Press play to start decoding the video live in your browser using WebAssembly and WebCodecs!
