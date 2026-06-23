use std::fs::{self, File};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use agentpaint::Animation;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, ImageBuffer, ImageReader, Rgba};

#[test]
fn import_image_cli_writes_renderable_apx() {
    let dir = test_dir("agentpaint-import-image-test");
    fs::create_dir_all(&dir).unwrap();

    let input = dir.join("tiny.png");
    let output = dir.join("tiny.apx");
    let rendered = dir.join("tiny-rendered.png");
    let source: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| match (x, y) {
        (0, 0) => Rgba([255, 0, 0, 255]),
        (1, 0) => Rgba([0, 255, 0, 255]),
        (0, 1) => Rgba([0, 0, 255, 128]),
        _ => Rgba([0, 0, 0, 0]),
    });
    source.save(&input).unwrap();

    let import_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("import-image")
        .arg(&input)
        .arg("--out")
        .arg(&output)
        .status()
        .unwrap();
    assert!(import_status.success());

    let validate_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("validate")
        .arg(&output)
        .status()
        .unwrap();
    assert!(validate_status.success());

    let render_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("render")
        .arg(&output)
        .arg("--out")
        .arg(&rendered)
        .status()
        .unwrap();
    assert!(render_status.success());

    let image = ImageReader::open(&rendered)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 2);
    assert_eq!(image.get_pixel(0, 0).0, [255, 0, 0, 255]);
    assert_eq!(image.get_pixel(1, 0).0, [0, 255, 0, 255]);
    assert_eq!(image.get_pixel(0, 1).0, [0, 0, 255, 128]);
    assert_eq!(image.get_pixel(1, 1).0, [0, 0, 0, 0]);

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn import_gif_cli_writes_renderable_apxa() {
    let dir = test_dir("agentpaint-import-gif-test");
    fs::create_dir_all(&dir).unwrap();

    let input = dir.join("tiny.gif");
    let output = dir.join("tiny.apxa");
    let rendered = dir.join("frame1.png");
    {
        let file = File::create(&input).unwrap();
        let mut encoder = GifEncoder::new(file);
        encoder.set_repeat(Repeat::Infinite).unwrap();

        let frame0: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| {
            if (x, y) == (0, 0) {
                Rgba([255, 0, 0, 255])
            } else {
                Rgba([0, 0, 0, 0])
            }
        });
        encoder
            .encode_frame(Frame::from_parts(
                frame0,
                0,
                0,
                Delay::from_numer_denom_ms(80, 1),
            ))
            .unwrap();

        let frame1: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| {
            if (x, y) == (1, 0) {
                Rgba([0, 255, 0, 255])
            } else {
                Rgba([0, 0, 0, 0])
            }
        });
        encoder
            .encode_frame(Frame::from_parts(
                frame1,
                0,
                0,
                Delay::from_numer_denom_ms(120, 1),
            ))
            .unwrap();
    }

    let import_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("import-gif")
        .arg(&input)
        .arg("--out")
        .arg(&output)
        .status()
        .unwrap();
    assert!(import_status.success());

    let validate_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("validate-animation")
        .arg(&output)
        .status()
        .unwrap();
    assert!(validate_status.success());

    let animation: Animation = serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
    assert_eq!(animation.frames.len(), 2);
    assert_eq!(animation.frames[0].duration_ms, 80);
    assert_eq!(animation.frames[1].duration_ms, 120);

    let render_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("render-frame")
        .arg(&output)
        .arg("--frame")
        .arg("1")
        .arg("--out")
        .arg(&rendered)
        .status()
        .unwrap();
    assert!(render_status.success());

    let image = ImageReader::open(&rendered)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 2);
    assert_eq!(image.get_pixel(0, 0).0, [0, 0, 0, 0]);
    assert_eq!(image.get_pixel(1, 0).0, [0, 255, 0, 255]);

    fs::remove_dir_all(&dir).unwrap();
}

fn test_dir(prefix: &str) -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{unique}"))
}
