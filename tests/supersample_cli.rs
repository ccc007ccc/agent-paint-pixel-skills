use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use image::ImageReader;

#[test]
fn supersample_cli_writes_integer_scaled_png_blocks() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("agentpaint-supersample-test-{unique}"));
    fs::create_dir_all(&dir).unwrap();

    let input = dir.join("tiny.apx");
    let output = dir.join("tiny-preview.png");
    fs::write(
        &input,
        r##"
        {
          "canvas": { "width": 2, "height": 2 },
          "palette": {
            ".": "transparent",
            "R": "#ff0000",
            "G": "#00ff00",
            "B": "#0000ff"
          },
          "layers": [
            { "name": "base", "rows": ["RG", "B."] }
          ]
        }
        "##,
    )
    .unwrap();

    let output_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("supersample")
        .arg(&input)
        .arg("--scale")
        .arg("3")
        .arg("--out")
        .arg(&output)
        .status()
        .unwrap();
    assert!(output_status.success());

    let image = ImageReader::open(&output)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    assert_eq!(image.width(), 6);
    assert_eq!(image.height(), 6);
    assert_eq!(image.get_pixel(0, 0).0, [255, 0, 0, 255]);
    assert_eq!(image.get_pixel(2, 2).0, [255, 0, 0, 255]);
    assert_eq!(image.get_pixel(3, 0).0, [0, 255, 0, 255]);
    assert_eq!(image.get_pixel(0, 3).0, [0, 0, 255, 255]);
    assert_eq!(image.get_pixel(5, 5).0, [0, 0, 0, 0]);

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn supersample_frame_cli_writes_integer_scaled_animation_frame() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("agentpaint-supersample-frame-test-{unique}"));
    fs::create_dir_all(&dir).unwrap();

    let input = dir.join("tiny.apxa");
    let output = dir.join("tiny-frame-preview.png");
    fs::write(
        &input,
        r##"
        {
          "canvas": { "width": 2, "height": 2 },
          "palette": {
            ".": "transparent",
            "R": "#ff0000",
            "G": "#00ff00"
          },
          "layers": [
            { "name": "sprite", "rows": ["R.", ".."] }
          ],
          "frames": [
            { "name": "idle", "operations": [] },
            {
              "name": "shift",
              "operations": [
                { "op": "set_rows", "layer": "sprite", "rows": [".G", ".."] }
              ]
            }
          ]
        }
        "##,
    )
    .unwrap();

    let output_status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("supersample-frame")
        .arg(&input)
        .arg("--frame")
        .arg("1")
        .arg("--scale")
        .arg("2")
        .arg("--out")
        .arg(&output)
        .status()
        .unwrap();
    assert!(output_status.success());

    let image = ImageReader::open(&output)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    assert_eq!(image.width(), 4);
    assert_eq!(image.height(), 4);
    assert_eq!(image.get_pixel(0, 0).0, [0, 0, 0, 0]);
    assert_eq!(image.get_pixel(2, 0).0, [0, 255, 0, 255]);
    assert_eq!(image.get_pixel(3, 1).0, [0, 255, 0, 255]);

    fs::remove_dir_all(&dir).unwrap();
}
