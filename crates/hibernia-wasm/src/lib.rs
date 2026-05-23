use hibernia::api::{
    create_decoder, Codec, DecodedPicture, DecoderConfig, DefaultAllocator, EncodedPacket,
    FlushMode, StreamFormat, VideoDecoder, VideoDecoderCallbacks, VideoPlane,
};
use hibernia::h264::nal_parser::NalParser;
use std::io::Cursor;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmFrame {
    pic: DecodedPicture,
}

impl WasmFrame {
    fn view(&self, plane: VideoPlane) -> hibernia::api::PlaneView<'_> {
        self.pic.frame.plane(plane).expect("plane present")
    }
}

#[wasm_bindgen]
impl WasmFrame {
    pub fn y_ptr(&self) -> *const u8 {
        self.view(VideoPlane::Y).data.as_ptr()
    }

    pub fn y_len(&self) -> usize {
        self.view(VideoPlane::Y).data.len()
    }

    pub fn y_stride(&self) -> usize {
        self.view(VideoPlane::Y).stride
    }

    pub fn u_ptr(&self) -> *const u8 {
        self.view(VideoPlane::U).data.as_ptr()
    }

    pub fn u_len(&self) -> usize {
        self.view(VideoPlane::U).data.len()
    }

    pub fn u_stride(&self) -> usize {
        self.view(VideoPlane::U).stride
    }

    pub fn v_ptr(&self) -> *const u8 {
        self.view(VideoPlane::V).data.as_ptr()
    }

    pub fn v_len(&self) -> usize {
        self.view(VideoPlane::V).data.len()
    }

    pub fn v_stride(&self) -> usize {
        self.view(VideoPlane::V).stride
    }

    pub fn width(&self) -> usize {
        self.pic.format.coded_width
    }

    pub fn height(&self) -> usize {
        self.pic.format.coded_height
    }

    pub fn display_width(&self) -> usize {
        self.pic.format.display_width
    }

    pub fn display_height(&self) -> usize {
        self.pic.format.display_height
    }

    pub fn crop_left(&self) -> usize {
        self.pic.format.crop_left
    }

    pub fn crop_top(&self) -> usize {
        self.pic.format.crop_top
    }
}

struct WasmCallbacks;

impl VideoDecoderCallbacks for WasmCallbacks {
    fn on_picture_available(&self) {}
    fn on_format_changed(&self, _format: StreamFormat) {}
}

#[wasm_bindgen]
pub struct WasmDecoder {
    decoder: Box<dyn VideoDecoder>,
    parser: NalParser<Cursor<Vec<u8>>>,
    drained: bool,
}

#[wasm_bindgen]
impl WasmDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Self {
        console_error_panic_hook::set_once();
        let cursor = Cursor::new(data.to_vec());
        let decoder = create_decoder(
            DecoderConfig::new(Codec::H264),
            Arc::new(DefaultAllocator),
            Arc::new(WasmCallbacks),
        )
        .expect("create_decoder");
        Self { decoder, parser: NalParser::new(cursor), drained: false }
    }

    pub fn decode_next_frame(&mut self) -> Result<Option<WasmFrame>, JsValue> {
        if let Some(pic) = self.decoder.get_picture().map_err(map_err)? {
            return Ok(Some(WasmFrame { pic }));
        }

        for nal_result in &mut self.parser {
            let nal = nal_result.map_err(|e| JsValue::from_str(&e.to_string()))?;
            let mut buf = Vec::with_capacity(nal.len() + 4);
            buf.extend_from_slice(&[0, 0, 0, 1]);
            buf.extend_from_slice(&nal);
            self.decoder.decode(EncodedPacket::from_vec(buf)).map_err(map_err)?;
            if let Some(pic) = self.decoder.get_picture().map_err(map_err)? {
                return Ok(Some(WasmFrame { pic }));
            }
        }

        if !self.drained {
            self.drained = true;
            self.decoder.flush(FlushMode::Drain).map_err(map_err)?;
        }
        if let Some(pic) = self.decoder.get_picture().map_err(map_err)? {
            return Ok(Some(WasmFrame { pic }));
        }

        Ok(None)
    }
}

fn map_err(e: hibernia::api::DecoderError) -> JsValue {
    JsValue::from_str(&format!("{:?}", e))
}
