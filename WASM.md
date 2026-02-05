# Compiling Hibernia to WebAssembly

Hibernia can be compiled to WebAssembly to be used as a video decoder in a web browser.

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Building the WASM Package

Run the following command in the root of the project:

```bash
wasm-pack build --target web
```

This will create a `pkg/` directory containing the WebAssembly binary and JavaScript glue code.

## Running the Web Demo

A demo is provided in the `www/` directory.

1.  Build the WASM package as described above.
2.  Copy the contents of the `pkg/` directory to `www/pkg/`.
3.  Copy a sample H.264 bitstream (e.g., from the `data/` directory) to `www/input.h264`.
4.  Serve the `www/` directory using a local web server:

    ```bash
    # Example using python
    python3 -m http.server 8080 --directory www
    ```

5.  Open your browser and navigate to `http://localhost:8080`.
6.  Click the "Decode" button to see the decoder in action.
