use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn export_psd_preserves_apx_layer_names_from_file() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("agentpaint-psd-test-{unique}"));
    fs::create_dir_all(&dir).unwrap();

    let input = dir.join("named-layers.apx");
    let output = dir.join("named-layers.psd");
    fs::write(
        &input,
        r##"
        {
          "canvas": { "width": 2, "height": 2 },
          "palette": {
            ".": "transparent",
            "R": "#ff0000",
            "B": "#0000ff"
          },
          "layers": [
            { "name": "top_highlight", "rows": [".R", ".."] },
            { "name": "底色", "rows": ["BB", "BB"] }
          ]
        }
        "##,
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_agentpaint"))
        .arg("export-psd")
        .arg(&input)
        .arg("--out")
        .arg(&output)
        .status()
        .unwrap();
    assert!(status.success());

    let bytes = fs::read(&output).unwrap();
    let psd = psd::Psd::from_bytes(&bytes).unwrap();
    let layer_names: Vec<_> = psd.layers().iter().map(|layer| layer.name()).collect();
    assert_eq!(layer_names, vec!["top_highlight", "底色"]);
    assert_eq!(psd_layer_record_flags(&bytes), vec![0, 0]);

    fs::remove_dir_all(&dir).unwrap();
}

fn psd_layer_record_flags(bytes: &[u8]) -> Vec<u8> {
    let mut offset = 26;
    offset = skip_length_prefixed_section(bytes, offset);
    offset = skip_length_prefixed_section(bytes, offset);

    let layer_and_mask_len = read_u32_at(bytes, offset) as usize;
    offset += 4;
    assert!(offset + layer_and_mask_len <= bytes.len());

    let layer_info_len = read_u32_at(bytes, offset) as usize;
    offset += 4;
    assert!(offset + layer_info_len <= bytes.len());

    let layer_count = read_i16_at(bytes, offset).unsigned_abs() as usize;
    offset += 2;

    let mut flags = Vec::with_capacity(layer_count);
    for _ in 0..layer_count {
        offset += 16;
        let channel_count = read_u16_at(bytes, offset) as usize;
        offset += 2 + channel_count * 6;
        offset += 8;
        offset += 2;
        flags.push(bytes[offset]);
        offset += 2;
        let extra_len = read_u32_at(bytes, offset) as usize;
        offset += 4 + extra_len;
    }
    flags
}

fn skip_length_prefixed_section(bytes: &[u8], offset: usize) -> usize {
    let len = read_u32_at(bytes, offset) as usize;
    let next = offset + 4 + len;
    assert!(next <= bytes.len());
    next
}

fn read_u16_at(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_i16_at(bytes: &[u8], offset: usize) -> i16 {
    i16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
