use hibernia::h264::decoder::{Decoder, Picture};
use hibernia::h264::nal_parser::NalParser;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmFrame {
    pic: Picture,
}

#[wasm_bindgen]
impl WasmFrame {
    pub fn y_ptr(&self) -> *const u8 {
        self.pic.frame.planes[0].data_origin().as_ptr()
    }

    pub fn y_len(&self) -> usize {
        self.pic.frame.planes[0].data_origin().len()
    }

    pub fn y_stride(&self) -> usize {
        self.pic.frame.planes[0].cfg.stride
    }

    pub fn u_ptr(&self) -> *const u8 {
        self.pic.frame.planes[1].data_origin().as_ptr()
    }

    pub fn u_len(&self) -> usize {
        self.pic.frame.planes[1].data_origin().len()
    }

    pub fn u_stride(&self) -> usize {
        self.pic.frame.planes[1].cfg.stride
    }

    pub fn v_ptr(&self) -> *const u8 {
        self.pic.frame.planes[2].data_origin().as_ptr()
    }

    pub fn v_len(&self) -> usize {
        self.pic.frame.planes[2].data_origin().len()
    }

    pub fn v_stride(&self) -> usize {
        self.pic.frame.planes[2].cfg.stride
    }

    pub fn width(&self) -> usize {
        self.pic.frame.planes[0].cfg.width
    }

    pub fn height(&self) -> usize {
        self.pic.frame.planes[0].cfg.height
    }

    pub fn display_width(&self) -> usize {
        self.pic.crop.display_width
    }

    pub fn display_height(&self) -> usize {
        self.pic.crop.display_height
    }

    pub fn crop_left(&self) -> usize {
        self.pic.crop.crop_left
    }

    pub fn crop_top(&self) -> usize {
        self.pic.crop.crop_top
    }
}

#[wasm_bindgen]
pub struct WasmDecoder {
    decoder: Decoder,
    parser: NalParser<Cursor<Vec<u8>>>,
}

#[wasm_bindgen]
impl WasmDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Self {
        console_error_panic_hook::set_once();
        let cursor = Cursor::new(data.to_vec());
        Self { decoder: Decoder::new(), parser: NalParser::new(cursor) }
    }

    pub fn decode_next_frame(&mut self) -> Result<Option<WasmFrame>, JsValue> {
        // First check if there's already a frame ready
        if let Some(pic) = self.decoder.retrieve_picture() {
            return Ok(Some(WasmFrame { pic }));
        }

        // Otherwise, parse NALs and decode until a frame is produced
        for nal_result in &mut self.parser {
            let nal_data = nal_result.map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.decoder.decode(&nal_data).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

            if let Some(pic) = self.decoder.retrieve_picture() {
                return Ok(Some(WasmFrame { pic }));
            }
        }

        // Flush if EOF reached
        self.decoder.flush().map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        if let Some(pic) = self.decoder.retrieve_picture() {
            return Ok(Some(WasmFrame { pic }));
        }

        Ok(None)
    }
}
