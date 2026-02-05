use wasm_bindgen::prelude::*;
use crate::h264::decoder::Decoder;

#[wasm_bindgen]
pub struct WasmDecoder {
    decoder: Decoder,
}

#[wasm_bindgen]
impl WasmDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            decoder: Decoder::new(),
        }
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<(), String> {
        self.decoder.decode(data).map_err(|e| format!("{:?}", e))
    }

    pub fn get_frame_count(&self) -> usize {
        self.decoder.get_frame_buffer().len()
    }

    pub fn get_width(&self, frame_idx: usize) -> u32 {
        self.decoder.get_frame_buffer().get(frame_idx)
            .map(|f| f.planes[0].cfg.width as u32)
            .unwrap_or(0)
    }

    pub fn get_height(&self, frame_idx: usize) -> u32 {
        self.decoder.get_frame_buffer().get(frame_idx)
            .map(|f| f.planes[0].cfg.height as u32)
            .unwrap_or(0)
    }

    pub fn get_y_plane(&self, frame_idx: usize) -> Option<Vec<u8>> {
        self.decoder.get_frame_buffer().get(frame_idx).map(|f| {
            let plane = &f.planes[0];
            let mut data = vec![0u8; plane.cfg.width * plane.cfg.height];
            plane.copy_to_raw_u8(&mut data, plane.cfg.width, 1);
            data
        })
    }

    pub fn get_u_plane(&self, frame_idx: usize) -> Option<Vec<u8>> {
        self.decoder.get_frame_buffer().get(frame_idx).map(|f| {
            let plane = &f.planes[1];
            let mut data = vec![0u8; plane.cfg.width * plane.cfg.height];
            plane.copy_to_raw_u8(&mut data, plane.cfg.width, 1);
            data
        })
    }

    pub fn get_v_plane(&self, frame_idx: usize) -> Option<Vec<u8>> {
        self.decoder.get_frame_buffer().get(frame_idx).map(|f| {
            let plane = &f.planes[2];
            let mut data = vec![0u8; plane.cfg.width * plane.cfg.height];
            plane.copy_to_raw_u8(&mut data, plane.cfg.width, 1);
            data
        })
    }

    pub fn clear_frames(&mut self) {
        self.decoder.clear_frame_buffer();
    }
}
