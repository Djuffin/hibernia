use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use hibernia::api::{
    create_decoder, Codec, DecodedPicture, DecoderConfig, DefaultAllocator, EncodedPacket,
    FlushMode, StreamFormat, VideoDecoder, VideoDecoderCallbacks, VideoPlane,
};
use hibernia::h264::nal_parser::NalParser;
use iced::widget::{column, container, image, row, text};
use iced::{Element, Length, Subscription, Task, Theme};

struct PlayerCallbacks;

impl VideoDecoderCallbacks for PlayerCallbacks {
    fn on_picture_available(&self) {}
    fn on_format_changed(&self, _format: StreamFormat) {}
}

const HELP: &str = "\
Usage: hplay <input.h264> [fps]

Opens a GUI window and plays the decoded video.
  fps   Target playback framerate (default: 30).
";

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args.iter().any(|a| a == "--help" || a == "-h") {
        eprint!("{HELP}");
        if args.len() < 2 {
            std::process::exit(1);
        } else {
            return Ok(());
        }
    }

    let path: PathBuf = args[1].clone().into();
    let fps: f64 = args
        .get(2)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(30.0)
        .max(1.0);

    iced::application(
        move || Player::new(path.clone(), fps),
        Player::update,
        Player::view,
    )
    .title(Player::title)
    .subscription(Player::subscription)
    .theme(theme)
    .run()
}

fn theme(_state: &Player) -> Theme {
    Theme::Dark
}

struct Player {
    frames_rx: mpsc::Receiver<DecoderEvent>,
    fps: f64,
    frame_count: u64,
    current_frame: Option<FrameImage>,
    ended: bool,
    error: Option<String>,
}

struct FrameImage {
    handle: image::Handle,
    width: u32,
    height: u32,
}

enum DecoderEvent {
    Frame(FrameImage),
    End,
    Error(String),
}

struct Source {
    nal_iter: Box<dyn Iterator<Item = std::io::Result<Vec<u8>>> + Send>,
    decoder: Box<dyn VideoDecoder>,
    drained: bool,
    done: bool,
}

impl Source {
    fn open(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let nal_parser = NalParser::new(reader);
        let decoder = create_decoder(
            DecoderConfig::new(Codec::H264),
            std::sync::Arc::new(DefaultAllocator),
            std::sync::Arc::new(PlayerCallbacks),
        )
        .expect("create_decoder");
        Ok(Self { nal_iter: Box::new(nal_parser), decoder, drained: false, done: false })
    }

    fn next_picture(&mut self) -> Option<DecodedPicture> {
        if self.done {
            return None;
        }
        loop {
            if let Ok(Some(pic)) = self.decoder.get_picture() {
                return Some(pic);
            }
            match self.nal_iter.next() {
                Some(Ok(nal)) => {
                    let mut buf = Vec::with_capacity(nal.len() + 4);
                    buf.extend_from_slice(&[0, 0, 0, 1]);
                    buf.extend_from_slice(&nal);
                    let _ = self.decoder.decode(EncodedPacket::from_vec(buf));
                }
                Some(Err(_)) => continue,
                None => {
                    if !self.drained {
                        self.drained = true;
                        let _ = self.decoder.flush(FlushMode::Drain);
                        continue;
                    }
                    self.done = true;
                    return None;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Player {
    fn new(path: PathBuf, fps: f64) -> (Self, Task<Message>) {
        // Small bounded channel: acts as natural backpressure if the UI falls
        // behind the decoder's real-time pace.
        let (tx, rx) = mpsc::sync_channel::<DecoderEvent>(2);
        let decode_path = path.clone();
        thread::spawn(move || decoder_loop(&decode_path, fps, tx));

        let player = Self {
            frames_rx: rx,
            fps,
            frame_count: 0,
            current_frame: None,
            ended: false,
            error: None,
        };
        (player, Task::none())
    }

    fn title(&self) -> String {
        match &self.current_frame {
            Some(f) => {
                format!("hplay — {}x{} — frame {}", f.width, f.height, self.frame_count)
            }
            None => String::from("hplay"),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                // Drain the channel non-blockingly. Keep the newest frame.
                loop {
                    match self.frames_rx.try_recv() {
                        Ok(DecoderEvent::Frame(f)) => {
                            self.current_frame = Some(f);
                            self.frame_count += 1;
                        }
                        Ok(DecoderEvent::End) => {
                            self.ended = true;
                            break;
                        }
                        Ok(DecoderEvent::Error(e)) => {
                            self.error = Some(e);
                            self.ended = true;
                            break;
                        }
                        Err(mpsc::TryRecvError::Empty) => break,
                        Err(mpsc::TryRecvError::Disconnected) => {
                            self.ended = true;
                            break;
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        if let Some(err) = &self.error {
            return container(text(err.as_str()).size(18))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into();
        }

        let canvas: Element<'_, Message> = match &self.current_frame {
            Some(f) => image(f.handle.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(iced::ContentFit::Contain)
                .into(),
            None => container(text("decoding…").size(16))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        };

        let status_text = match &self.current_frame {
            Some(f) => format!(
                "frame {}  |  {}×{}  |  target {:.0} fps{}",
                self.frame_count,
                f.width,
                f.height,
                self.fps,
                if self.ended { "  |  end of stream" } else { "" },
            ),
            None => String::from("decoding…"),
        };

        column![canvas, row![text(status_text).size(12)].padding(6)].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.ended && self.error.is_some() {
            return Subscription::none();
        }
        // Poll the channel at ~60 Hz. This is independent of target video fps;
        // the decoder thread paces itself to real time.
        iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }
}

fn decoder_loop(path: &Path, fps: f64, tx: mpsc::SyncSender<DecoderEvent>) {
    let mut source = match Source::open(path) {
        Ok(s) => s,
        Err(e) => {
            let _ = tx.send(DecoderEvent::Error(format!(
                "can't open {}: {}",
                path.display(),
                e
            )));
            return;
        }
    };

    let start = Instant::now();
    let mut idx: u64 = 0;
    while let Some(pic) = source.next_picture() {
        // Pace to real-time fps so we don't race ahead of the UI.
        let target = start + Duration::from_secs_f64(idx as f64 / fps);
        let now = Instant::now();
        if now < target {
            thread::sleep(target - now);
        }
        let frame = render_frame(&pic);
        if tx.send(DecoderEvent::Frame(frame)).is_err() {
            return; // Receiver dropped — UI is gone.
        }
        idx += 1;
    }
    let _ = tx.send(DecoderEvent::End);
}

fn render_frame(pic: &DecodedPicture) -> FrameImage {
    let disp_w = pic.format.display_width;
    let disp_h = pic.format.display_height;

    let y_plane = pic.frame.plane(VideoPlane::Y).expect("luma");
    let u_plane = pic.frame.plane(VideoPlane::U).expect("u plane");
    let v_plane = pic.frame.plane(VideoPlane::V).expect("v plane");

    let mut rgba = vec![0u8; disp_w * disp_h * 4];

    let crop_x = pic.format.crop_left;
    let crop_y = pic.format.crop_top;

    // 4:2:0 chroma subsampling is the only format the decoder produces.
    let x_dec = 1usize;
    let y_dec = 1usize;

    for sy in 0..disp_h {
        let y_row = (crop_y + sy) * y_plane.stride + crop_x;
        let cy_abs = (sy + crop_y) >> y_dec;
        let u_row = cy_abs * u_plane.stride;
        let v_row = cy_abs * v_plane.stride;
        let out_row = sy * disp_w * 4;
        for sx in 0..disp_w {
            let y_val = y_plane.data[y_row + sx];
            let cx_abs = (sx + crop_x) >> x_dec;
            let u_val = u_plane.data[u_row + cx_abs];
            let v_val = v_plane.data[v_row + cx_abs];
            let (r, g, b) = yuv_to_rgb(y_val, u_val, v_val);
            let i = out_row + sx * 4;
            rgba[i] = r;
            rgba[i + 1] = g;
            rgba[i + 2] = b;
            rgba[i + 3] = 255;
        }
    }

    FrameImage {
        handle: image::Handle::from_rgba(disp_w as u32, disp_h as u32, rgba),
        width: disp_w as u32,
        height: disp_h as u32,
    }
}

/// BT.601 limited-range YUV to RGB. The SPS VUI carries the real colorimetry,
/// but we don't honor it for a simple preview player.
fn yuv_to_rgb(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let y = (y as i32 - 16).max(0) as f32 * 1.164_383;
    let cb = u as f32 - 128.0;
    let cr = v as f32 - 128.0;
    let r = y + 1.596_027 * cr;
    let g = y - 0.391_762 * cb - 0.812_968 * cr;
    let b = y + 2.017_232 * cb;
    (
        r.clamp(0.0, 255.0) as u8,
        g.clamp(0.0, 255.0) as u8,
        b.clamp(0.0, 255.0) as u8,
    )
}
