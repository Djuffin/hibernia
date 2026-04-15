use std::fs;
use std::io::{self, Cursor};
use std::path::PathBuf;
use std::process::Command;

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(path: &str) -> io::Result<Self> {
        let path = PathBuf::from(path);
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
use hibernia::h264;
use hibernia::h264::nal_parser::NalParser;
use hibernia::y4m_cmp::compare_y4m_buffers;

fn run_ffmpeg(args: &[&str]) -> Result<bool, String> {
    let output = match Command::new("ffmpeg").args(args).output() {
        Ok(output) => output,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("ffmpeg not found, skipping test");
            return Ok(false);
        }
        Err(e) => return Err(format!("Failed to execute ffmpeg: {}", e)),
    };

    if !output.status.success() {
        println!("ffmpeg execution failed, skipping test");
        return Ok(false);
    }
    Ok(true)
}

fn decode_to_y4m(encoded_video_buffer: &[u8]) -> Result<Vec<u8>, String> {
    let cursor = Cursor::new(encoded_video_buffer);
    let nal_parser = NalParser::new(cursor);
    let mut decoder = h264::decoder::Decoder::new();

    let mut decoding_output = Vec::<u8>::new();
    {
        let mut writer_opt = Some(io::BufWriter::new(&mut decoding_output));
        let mut encoder_opt: Option<y4m::Encoder<io::BufWriter<&mut Vec<u8>>>> = None;

        let mut process_frame = |frame: h264::decoder::VideoFrame| {
            if encoder_opt.is_none() {
                let y_plane = &frame.planes[0];
                let w = y_plane.cfg.width as usize;
                let h = y_plane.cfg.height as usize;
                if let Some(writer) = writer_opt.take() {
                    encoder_opt = Some(
                        y4m::encode(w, h, y4m::Ratio { num: 30, den: 1 })
                            .with_colorspace(y4m::Colorspace::C420)
                            .write_header(writer)
                            .unwrap(),
                    );
                }
            }

            let mut planes = Vec::<Vec<u8>>::new();
            planes.resize_with(frame.planes.len(), Vec::new);

            for (i, plane) in frame.planes.iter().enumerate() {
                let data_size = plane.cfg.width * plane.cfg.height;
                let data: &mut Vec<u8> = &mut planes[i];
                data.resize(data_size as usize, 0);
                plane.copy_to_raw_u8(data, plane.cfg.width, 1);
            }

            let yuv_frame = y4m::Frame::new(
                [planes[0].as_slice(), planes[1].as_slice(), planes[2].as_slice()],
                None,
            );

            if let Some(enc) = &mut encoder_opt {
                enc.write_frame(&yuv_frame).unwrap();
            }
        };

        let mut nal_idx = 0usize;
        for nal_result in nal_parser {
            let nal_data = nal_result.map_err(|e| format!("NAL error: {e:?}"))?;
            decoder.decode(&nal_data).map_err(|e| {
                format!("Decoding error at NAL #{nal_idx}: {e:?}")
            })?;
            nal_idx += 1;

            while let Some(frame) = decoder.retrieve_frame() {
                process_frame(frame);
            }
        }

        decoder.flush().map_err(|e| format!("Flush error: {e:?}"))?;
        while let Some(frame) = decoder.retrieve_frame() {
            process_frame(frame);
        }
    }

    Ok(decoding_output)
}

#[test]
fn test_ffmpeg_baseline_testsrc() -> Result<(), String> {
    let test_dir = TestDir::new("tests/tmp_ffmpeg_baseline_testsrc").map_err(|e| e.to_string())?;

    let h264_path = test_dir.path().join("test_stream.264");
    let y4m_path = test_dir.path().join("output.y4m");

    let h264_path_str = h264_path.to_str().unwrap();
    let y4m_path_str = y4m_path.to_str().unwrap();

    // Generate H.264 baseline stream using ffmpeg
    // We use -pix_fmt yuv420p to ensure it's compatible with baseline profile
    if !run_ffmpeg(&[
        "-y",
        "-f",
        "lavfi",
        "-i",
        "testsrc=duration=1:size=176x144:rate=30",
        "-c:v",
        "libx264",
        "-profile:v",
        "baseline",
        "-pix_fmt",
        "yuv420p",
        h264_path_str,
    ])? {
        return Ok(());
    }

    // Generate reference Y4M from the H.264 stream
    if !run_ffmpeg(&["-y", "-i", h264_path_str, y4m_path_str])? {
        return Ok(());
    }

    let encoded_data = fs::read(&h264_path).map_err(|e| e.to_string())?;
    let expected_y4m = fs::read(&y4m_path).map_err(|e| e.to_string())?;

    let actual_y4m = decode_to_y4m(&encoded_data)?;

    compare_y4m_buffers(&actual_y4m, &expected_y4m)?;

    Ok(())
}
