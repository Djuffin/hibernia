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

    pub fn get_frame_rgba(&self, frame_idx: usize) -> Option<Vec<u8>> {
        self.decoder.get_frame_buffer().get(frame_idx).map(|f| {
            let width = f.planes[0].cfg.width;
            let height = f.planes[0].cfg.height;
            let mut rgba = vec![0u8; width * height * 4];

            let mut y_data = vec![0u8; width * height];
            f.planes[0].copy_to_raw_u8(&mut y_data, width, 1);

            let u_width = f.planes[1].cfg.width;
            let u_height = f.planes[1].cfg.height;
            let mut u_data = vec![0u8; u_width * u_height];
            f.planes[1].copy_to_raw_u8(&mut u_data, u_width, 1);

            let v_width = f.planes[2].cfg.width;
            let v_height = f.planes[2].cfg.height;
            let mut v_data = vec![0u8; v_width * v_height];
            f.planes[2].copy_to_raw_u8(&mut v_data, v_width, 1);

            for y in 0..height {
                for x in 0..width {
                    let y_val = y_data[y * width + x] as f32;
                    let u_val = u_data[(y >> 1) * u_width + (x >> 1)] as f32 - 128.0;
                    let v_val = v_data[(y >> 1) * v_width + (x >> 1)] as f32 - 128.0;

                    let r = (y_val + 1.402 * v_val).clamp(0.0, 255.0) as u8;
                    let g = (y_val - 0.344136 * u_val - 0.714136 * v_val).clamp(0.0, 255.0) as u8;
                    let b = (y_val + 1.772 * u_val).clamp(0.0, 255.0) as u8;

                    let idx = (y * width + x) * 4;
                    rgba[idx] = r;
                    rgba[idx+1] = g;
                    rgba[idx+2] = b;
                    rgba[idx+3] = 255;
                }
            }
            rgba
        })
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
