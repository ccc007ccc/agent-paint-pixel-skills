use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub canvas: Canvas,
    pub palette: BTreeMap<String, String>,
    #[serde(default)]
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
    pub name: String,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default)]
    pub rows: Option<Vec<String>>,
    #[serde(default)]
    pub chunks: Vec<Chunk>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chunk {
    pub x: u32,
    pub y: u32,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatchDocument {
    pub operations: Vec<PatchOperation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOperation {
    SetPalette {
        symbol: String,
        color: String,
    },
    AddLayer {
        layer: Layer,
    },
    InsertLayer {
        layer: Layer,
        #[serde(default)]
        to_index: Option<usize>,
        #[serde(default)]
        before: Option<String>,
        #[serde(default)]
        after: Option<String>,
    },
    ReplaceLayer {
        layer: Layer,
    },
    RenameLayer {
        layer: String,
        name: String,
    },
    RemoveLayer {
        layer: String,
    },
    SetLayerVisibility {
        layer: String,
        visible: bool,
    },
    SetLayerOpacity {
        layer: String,
        opacity: f32,
    },
    SetRows {
        layer: String,
        rows: Vec<String>,
    },
    AddChunk {
        layer: String,
        x: u32,
        y: u32,
        rows: Vec<String>,
    },
    MoveLayer {
        layer: String,
        #[serde(default)]
        to_index: Option<usize>,
        #[serde(default)]
        before: Option<String>,
        #[serde(default)]
        after: Option<String>,
    },
    ClearLayer {
        layer: String,
    },
}

#[derive(Debug, Serialize)]
pub struct RgbaExport {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[u8; 4]>,
}

#[derive(Debug, Serialize)]
pub struct InspectSummary {
    pub width: u32,
    pub height: u32,
    pub layer_order: String,
    pub palette_symbols: Vec<String>,
    pub layers: Vec<InspectLayer>,
}

#[derive(Debug, Serialize)]
pub struct InspectLayer {
    pub index: usize,
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub full_rows: usize,
    pub chunks: Vec<InspectChunk>,
}

#[derive(Debug, Serialize)]
pub struct InspectChunk {
    pub x: u32,
    pub y: u32,
    pub width: usize,
    pub height: usize,
}

fn default_visible() -> bool {
    true
}

fn default_opacity() -> f32 {
    1.0
}

pub fn validation_errors(project: &Project) -> Vec<String> {
    let mut errors = Vec::new();

    if project.canvas.width == 0 {
        errors.push("canvas.width must be greater than 0".to_string());
    }
    if project.canvas.height == 0 {
        errors.push("canvas.height must be greater than 0".to_string());
    }
    if project.layers.is_empty() {
        errors.push("layers must contain at least one layer".to_string());
    }

    let mut symbols = BTreeSet::new();
    let mut parsed_palette = BTreeMap::new();
    for (symbol, value) in &project.palette {
        if symbol.chars().count() != 1 {
            errors.push(format!("palette key `{symbol}` must be a single character"));
            continue;
        }

        let symbol_char = symbol.chars().next().unwrap();
        symbols.insert(symbol_char);
        match parse_color(value) {
            Ok(color) => {
                parsed_palette.insert(symbol_char, color);
            }
            Err(error) => errors.push(format!(
                "palette `{symbol}` has invalid color `{value}`: {error}"
            )),
        }
    }

    match parsed_palette.get(&'.') {
        Some(color) if color[3] == 0 => {}
        Some(_) => errors.push("palette `.` must be transparent".to_string()),
        None => errors.push("palette must define `.` as transparent".to_string()),
    }

    for (layer_index, layer) in project.layers.iter().enumerate() {
        let label = format!("layer `{}` at index {layer_index}", layer.name);

        if layer.name.trim().is_empty() {
            errors.push(format!("{label} must have a non-empty name"));
        }
        if !(0.0..=1.0).contains(&layer.opacity) {
            errors.push(format!("{label} opacity must be between 0 and 1"));
        }
        if let Some(rows) = &layer.rows {
            validate_full_rows(project, &symbols, &label, rows, &mut errors);
        }

        for (chunk_index, chunk) in layer.chunks.iter().enumerate() {
            validate_chunk(
                project,
                &symbols,
                &format!("{label} chunk {chunk_index}"),
                chunk,
                &mut errors,
            );
        }
    }

    errors
}

pub fn render_project(project: &Project) -> Result<Vec<[u8; 4]>> {
    let errors = validation_errors(project);
    if !errors.is_empty() {
        return Err(anyhow!("APX validation failed:\n{}", errors.join("\n")));
    }

    let palette = parsed_palette(project)?;
    let pixel_count = (project.canvas.width as usize) * (project.canvas.height as usize);
    let mut output = vec![[0, 0, 0, 0]; pixel_count];

    for layer in project.layers.iter().rev() {
        if !layer.visible {
            continue;
        }

        if let Some(rows) = &layer.rows {
            paint_rows(
                &mut output,
                project.canvas.width,
                0,
                0,
                rows,
                layer.opacity,
                &palette,
            );
        }

        for chunk in &layer.chunks {
            paint_rows(
                &mut output,
                project.canvas.width,
                chunk.x,
                chunk.y,
                &chunk.rows,
                layer.opacity,
                &palette,
            );
        }
    }

    Ok(output)
}

pub fn apply_patch_document(mut project: Project, patch: &PatchDocument) -> Result<Project> {
    if patch.operations.is_empty() {
        return Err(anyhow!("patch must contain at least one operation"));
    }

    for operation in &patch.operations {
        match operation {
            PatchOperation::SetPalette { symbol, color } => {
                ensure_single_symbol(symbol)?;
                parse_color(color)?;
                project.palette.insert(symbol.clone(), color.clone());
            }
            PatchOperation::AddLayer { layer } => {
                ensure_layer_name_available(&project, &layer.name)?;
                project.layers.insert(0, layer.clone());
            }
            PatchOperation::InsertLayer {
                layer,
                to_index,
                before,
                after,
            } => {
                ensure_layer_name_available(&project, &layer.name)?;
                let insert_index = resolve_layer_target_index(
                    &project,
                    &layer.name,
                    *to_index,
                    before.as_deref(),
                    after.as_deref(),
                    "insert_layer",
                )?;
                project.layers.insert(insert_index, layer.clone());
            }
            PatchOperation::ReplaceLayer { layer } => {
                let index = find_layer_index(&project, &layer.name)?;
                project.layers[index] = layer.clone();
            }
            PatchOperation::RenameLayer { layer, name } => {
                ensure_layer_name_is_valid(name)?;
                let index = find_layer_index(&project, layer)?;
                if project
                    .layers
                    .iter()
                    .enumerate()
                    .any(|(candidate_index, candidate)| {
                        candidate_index != index && candidate.name == *name
                    })
                {
                    return Err(anyhow!("layer `{name}` already exists"));
                }
                project.layers[index].name = name.clone();
            }
            PatchOperation::RemoveLayer { layer } => {
                let index = find_layer_index(&project, layer)?;
                project.layers.remove(index);
            }
            PatchOperation::SetLayerVisibility { layer, visible } => {
                let index = find_layer_index(&project, layer)?;
                project.layers[index].visible = *visible;
            }
            PatchOperation::SetLayerOpacity { layer, opacity } => {
                let index = find_layer_index(&project, layer)?;
                project.layers[index].opacity = *opacity;
            }
            PatchOperation::SetRows { layer, rows } => {
                let index = find_layer_index(&project, layer)?;
                project.layers[index].rows = Some(rows.clone());
            }
            PatchOperation::AddChunk { layer, x, y, rows } => {
                let index = find_layer_index(&project, layer)?;
                project.layers[index].chunks.push(Chunk {
                    x: *x,
                    y: *y,
                    rows: rows.clone(),
                });
            }
            PatchOperation::MoveLayer {
                layer,
                to_index,
                before,
                after,
            } => {
                move_layer(
                    &mut project,
                    layer,
                    *to_index,
                    before.as_deref(),
                    after.as_deref(),
                )?;
            }
            PatchOperation::ClearLayer { layer } => {
                let index = find_layer_index(&project, layer)?;
                project.layers[index].rows = None;
                project.layers[index].chunks.clear();
            }
        }
    }

    let errors = validation_errors(&project);
    if !errors.is_empty() {
        return Err(anyhow!(
            "patched APX validation failed:\n{}",
            errors.join("\n")
        ));
    }

    Ok(project)
}

pub fn chunk_dimensions(chunk: &Chunk) -> Option<(usize, usize)> {
    let first = chunk.rows.first()?;
    Some((first.chars().count(), chunk.rows.len()))
}

fn ensure_single_symbol(symbol: &str) -> Result<()> {
    if symbol.chars().count() != 1 {
        return Err(anyhow!(
            "palette symbol `{symbol}` must be a single character"
        ));
    }
    Ok(())
}

fn ensure_layer_name_is_valid(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(anyhow!("layer name must be non-empty"));
    }
    Ok(())
}

fn ensure_layer_name_available(project: &Project, name: &str) -> Result<()> {
    ensure_layer_name_is_valid(name)?;
    if project.layers.iter().any(|layer| layer.name == name) {
        return Err(anyhow!("layer `{name}` already exists"));
    }
    Ok(())
}

fn find_layer_index(project: &Project, name: &str) -> Result<usize> {
    project
        .layers
        .iter()
        .position(|layer| layer.name == name)
        .ok_or_else(|| anyhow!("layer `{name}` does not exist"))
}

fn move_layer(
    project: &mut Project,
    name: &str,
    to_index: Option<usize>,
    before: Option<&str>,
    after: Option<&str>,
) -> Result<()> {
    let old_index = find_layer_index(project, name)?;
    let layer = project.layers.remove(old_index);
    let insert_index =
        resolve_layer_target_index(project, name, to_index, before, after, "move_layer")?;
    project.layers.insert(insert_index, layer);
    Ok(())
}

fn resolve_layer_target_index(
    project: &Project,
    subject_name: &str,
    to_index: Option<usize>,
    before: Option<&str>,
    after: Option<&str>,
    op_name: &str,
) -> Result<usize> {
    let target_count = usize::from(to_index.is_some())
        + usize::from(before.is_some())
        + usize::from(after.is_some());
    if target_count != 1 {
        return Err(anyhow!(
            "{op_name} for `{subject_name}` requires exactly one of to_index, before, or after"
        ));
    }

    if before == Some(subject_name) || after == Some(subject_name) {
        return Err(anyhow!(
            "{op_name} target cannot be the same layer `{subject_name}`"
        ));
    }

    if let Some(index) = to_index {
        if index > project.layers.len() {
            return Err(anyhow!(
                "{op_name} to_index {index} exceeds layer count {}",
                project.layers.len()
            ));
        }
        return Ok(index);
    }

    if let Some(target) = before {
        return find_layer_index(project, target);
    }

    if let Some(target) = after {
        return Ok(find_layer_index(project, target)? + 1);
    }

    unreachable!("target_count already checked")
}

fn validate_full_rows(
    project: &Project,
    symbols: &BTreeSet<char>,
    label: &str,
    rows: &[String],
    errors: &mut Vec<String>,
) {
    if rows.len() != project.canvas.height as usize {
        errors.push(format!(
            "{label} full rows must have canvas height {}, got {}",
            project.canvas.height,
            rows.len()
        ));
    }

    for (row_index, row) in rows.iter().enumerate() {
        let width = row.chars().count();
        if width != project.canvas.width as usize {
            errors.push(format!(
                "{label} row {row_index} must have width {}, got {width}",
                project.canvas.width
            ));
        }
        validate_symbols(symbols, label, row_index, row, errors);
    }
}

fn validate_chunk(
    project: &Project,
    symbols: &BTreeSet<char>,
    label: &str,
    chunk: &Chunk,
    errors: &mut Vec<String>,
) {
    if chunk.rows.is_empty() {
        errors.push(format!("{label} must contain at least one row"));
        return;
    }

    let expected_width = chunk.rows[0].chars().count();
    if expected_width == 0 {
        errors.push(format!("{label} width must be greater than 0"));
    }

    for (row_index, row) in chunk.rows.iter().enumerate() {
        let width = row.chars().count();
        if width != expected_width {
            errors.push(format!(
                "{label} row {row_index} must have width {expected_width}, got {width}"
            ));
        }
        validate_symbols(symbols, label, row_index, row, errors);
    }

    let width = expected_width as u32;
    let height = chunk.rows.len() as u32;
    if chunk.x.saturating_add(width) > project.canvas.width {
        errors.push(format!(
            "{label} x range {}..{} exceeds canvas width {}",
            chunk.x,
            chunk.x.saturating_add(width),
            project.canvas.width
        ));
    }
    if chunk.y.saturating_add(height) > project.canvas.height {
        errors.push(format!(
            "{label} y range {}..{} exceeds canvas height {}",
            chunk.y,
            chunk.y.saturating_add(height),
            project.canvas.height
        ));
    }
}

fn validate_symbols(
    symbols: &BTreeSet<char>,
    label: &str,
    row_index: usize,
    row: &str,
    errors: &mut Vec<String>,
) {
    for (column_index, symbol) in row.chars().enumerate() {
        if !symbols.contains(&symbol) {
            errors.push(format!(
                "{label} row {row_index} column {column_index} uses undefined palette symbol `{symbol}`"
            ));
        }
    }
}

fn parsed_palette(project: &Project) -> Result<BTreeMap<char, [u8; 4]>> {
    project
        .palette
        .iter()
        .map(|(symbol, value)| {
            let key = symbol
                .chars()
                .next()
                .ok_or_else(|| anyhow!("empty palette key"))?;
            Ok((key, parse_color(value)?))
        })
        .collect()
}

fn paint_rows(
    output: &mut [[u8; 4]],
    canvas_width: u32,
    offset_x: u32,
    offset_y: u32,
    rows: &[String],
    opacity: f32,
    palette: &BTreeMap<char, [u8; 4]>,
) {
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, symbol) in row.chars().enumerate() {
            let source = palette[&symbol];
            if source[3] == 0 {
                continue;
            }

            let x = offset_x + column_index as u32;
            let y = offset_y + row_index as u32;
            let index = (y * canvas_width + x) as usize;
            output[index] = alpha_over(output[index], source, opacity);
        }
    }
}

fn alpha_over(destination: [u8; 4], source: [u8; 4], layer_opacity: f32) -> [u8; 4] {
    let source_alpha = (source[3] as f32 / 255.0) * layer_opacity;
    let destination_alpha = destination[3] as f32 / 255.0;
    let output_alpha = source_alpha + destination_alpha * (1.0 - source_alpha);

    if output_alpha <= f32::EPSILON {
        return [0, 0, 0, 0];
    }

    let mut output = [0, 0, 0, 0];
    for index in 0..3 {
        let source_channel = source[index] as f32 / 255.0;
        let destination_channel = destination[index] as f32 / 255.0;
        let channel = (source_channel * source_alpha
            + destination_channel * destination_alpha * (1.0 - source_alpha))
            / output_alpha;
        output[index] = to_u8(channel * 255.0);
    }
    output[3] = to_u8(output_alpha * 255.0);
    output
}

fn parse_color(value: &str) -> Result<[u8; 4]> {
    let trimmed = value.trim();
    if trimmed.eq_ignore_ascii_case("transparent") {
        return Ok([0, 0, 0, 0]);
    }

    if let Some(hex) = trimmed.strip_prefix('#') {
        return parse_hex(hex);
    }

    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("rgb(") && lower.ends_with(')') {
        let inside = &trimmed[4..trimmed.len() - 1];
        let channels = parse_channel_list(inside)?;
        if channels.len() != 3 {
            return Err(anyhow!("rgb() requires 3 channels"));
        }
        return Ok([channels[0], channels[1], channels[2], 255]);
    }

    if lower.starts_with("rgba(") && lower.ends_with(')') {
        let inside = &trimmed[5..trimmed.len() - 1];
        let parts: Vec<_> = inside.split(',').map(str::trim).collect();
        if parts.len() != 4 {
            return Err(anyhow!("rgba() requires 4 channels"));
        }
        let red = parse_u8_channel(parts[0])?;
        let green = parse_u8_channel(parts[1])?;
        let blue = parse_u8_channel(parts[2])?;
        let alpha = parse_alpha_channel(parts[3])?;
        return Ok([red, green, blue, alpha]);
    }

    Err(anyhow!(
        "expected transparent, #rgb, #rrggbb, #rrggbbaa, rgb(), or rgba()"
    ))
}

fn parse_hex(hex: &str) -> Result<[u8; 4]> {
    if !hex.is_ascii() {
        return Err(anyhow!("hex color must contain ASCII hex digits"));
    }

    match hex.len() {
        3 => {
            let red = expand_hex_digit(&hex[0..1])?;
            let green = expand_hex_digit(&hex[1..2])?;
            let blue = expand_hex_digit(&hex[2..3])?;
            Ok([red, green, blue, 255])
        }
        6 => Ok([
            parse_hex_pair(&hex[0..2])?,
            parse_hex_pair(&hex[2..4])?,
            parse_hex_pair(&hex[4..6])?,
            255,
        ]),
        8 => Ok([
            parse_hex_pair(&hex[0..2])?,
            parse_hex_pair(&hex[2..4])?,
            parse_hex_pair(&hex[4..6])?,
            parse_hex_pair(&hex[6..8])?,
        ]),
        _ => Err(anyhow!("hex color must have 3, 6, or 8 digits")),
    }
}

fn expand_hex_digit(digit: &str) -> Result<u8> {
    parse_hex_pair(&format!("{digit}{digit}"))
}

fn parse_hex_pair(pair: &str) -> Result<u8> {
    u8::from_str_radix(pair, 16).map_err(|_| anyhow!("invalid hex channel `{pair}`"))
}

fn parse_channel_list(inside: &str) -> Result<Vec<u8>> {
    inside
        .split(',')
        .map(|part| parse_u8_channel(part.trim()))
        .collect()
}

fn parse_u8_channel(value: &str) -> Result<u8> {
    let channel: u16 = value
        .parse()
        .map_err(|_| anyhow!("channel `{value}` must be an integer"))?;
    if channel > 255 {
        return Err(anyhow!("channel `{value}` must be between 0 and 255"));
    }
    Ok(channel as u8)
}

fn parse_alpha_channel(value: &str) -> Result<u8> {
    if let Ok(integer) = value.parse::<u16>() {
        if integer > 255 {
            return Err(anyhow!("alpha `{value}` must be between 0 and 255"));
        }
        return Ok(integer as u8);
    }

    let alpha: f32 = value
        .parse()
        .map_err(|_| anyhow!("alpha `{value}` must be a number"))?;
    if !(0.0..=1.0).contains(&alpha) {
        return Err(anyhow!(
            "alpha `{value}` must be between 0 and 1, or 0 and 255"
        ));
    }
    Ok(to_u8(alpha * 255.0))
}

fn to_u8(value: f32) -> u8 {
    value.round().clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project() -> Project {
        serde_json::from_str(
            r##"
            {
              "canvas": { "width": 4, "height": 4 },
              "palette": {
                ".": "transparent",
                "K": "#111111",
                "R": "#ff0000",
                "S": "rgba(0,0,0,0.5)"
              },
              "layers": [
                {
                  "name": "shadow",
                  "opacity": 1,
                  "chunks": [
                    { "x": 2, "y": 2, "rows": ["S"] }
                  ]
                },
                {
                  "name": "base",
                  "rows": [
                    "....",
                    ".RR.",
                    ".RR.",
                    "...."
                  ]
                }
              ]
            }
            "##,
        )
        .unwrap()
    }

    fn layer_index(project: &Project, name: &str) -> usize {
        find_layer_index(project, name).unwrap()
    }

    #[test]
    fn validates_sample_project() {
        assert!(validation_errors(&sample_project()).is_empty());
    }

    #[test]
    fn rejects_unknown_palette_symbols() {
        let mut project = sample_project();
        let base_index = layer_index(&project, "base");
        project.layers[base_index].rows.as_mut().unwrap()[1] = ".RX.".to_string();
        let errors = validation_errors(&project);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("undefined palette symbol `X`"))
        );
    }

    #[test]
    fn rejects_bad_row_width() {
        let mut project = sample_project();
        let base_index = layer_index(&project, "base");
        project.layers[base_index].rows.as_mut().unwrap()[1] = ".R.".to_string();
        let errors = validation_errors(&project);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("must have width 4"))
        );
    }

    #[test]
    fn renders_expected_pixel_count() {
        let pixels = render_project(&sample_project()).unwrap();
        assert_eq!(pixels.len(), 16);
        assert_eq!(pixels[5], [255, 0, 0, 255]);
        assert_eq!(pixels[10][3], 255);
    }

    #[test]
    fn parses_hex_and_rgba_colors() {
        assert_eq!(parse_color("#abc").unwrap(), [170, 187, 204, 255]);
        assert_eq!(parse_color("#01020304").unwrap(), [1, 2, 3, 4]);
        assert_eq!(parse_color("rgb(1,2,3)").unwrap(), [1, 2, 3, 255]);
        assert_eq!(parse_color("rgba(1,2,3,0.5)").unwrap(), [1, 2, 3, 128]);
    }

    #[test]
    fn applies_palette_and_chunk_patch() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "set_palette", "symbol": "H", "color": "#ffffff" },
                { "op": "add_chunk", "layer": "shadow", "x": 0, "y": 0, "rows": ["H"] }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        assert_eq!(patched.palette["H"], "#ffffff");
        assert_eq!(
            patched.layers[layer_index(&patched, "shadow")].chunks.len(),
            2
        );
    }

    #[test]
    fn applies_layer_replacement_patch() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                {
                  "op": "replace_layer",
                  "layer": {
                    "name": "shadow",
                    "chunks": [
                      { "x": 0, "y": 3, "rows": ["SSSS"] }
                    ]
                  }
                }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        let shadow_index = layer_index(&patched, "shadow");
        assert_eq!(patched.layers[shadow_index].chunks[0].x, 0);
        assert_eq!(patched.layers[shadow_index].chunks[0].y, 3);
    }

    #[test]
    fn rejects_patch_that_breaks_validation() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "set_rows", "layer": "base", "rows": ["too-short"] }
              ]
            }
            "##,
        )
        .unwrap();

        let error = apply_patch_document(project, &patch)
            .unwrap_err()
            .to_string();
        assert!(error.contains("patched APX validation failed"));
    }

    #[test]
    fn moves_layer_by_index() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "move_layer", "layer": "base", "to_index": 0 }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        assert_eq!(patched.layers[0].name, "base");
        assert_eq!(patched.layers[1].name, "shadow");
    }

    #[test]
    fn moves_layer_before_or_after_another_layer() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "move_layer", "layer": "base", "before": "shadow" }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        assert_eq!(patched.layers[0].name, "base");
        assert_eq!(patched.layers[1].name, "shadow");
    }

    #[test]
    fn rejects_ambiguous_layer_move() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "move_layer", "layer": "shadow", "to_index": 0, "before": "base" }
              ]
            }
            "##,
        )
        .unwrap();

        let error = apply_patch_document(project, &patch)
            .unwrap_err()
            .to_string();
        assert!(error.contains("requires exactly one"));
    }

    #[test]
    fn renders_layers_as_top_to_bottom_stack() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 1, "height": 1 },
              "palette": {
                ".": "transparent",
                "K": "#111111",
                "R": "#ff0000"
              },
              "layers": [
                { "name": "top", "rows": ["R"] },
                { "name": "bottom", "rows": ["K"] }
              ]
            }
            "##,
        )
        .unwrap();

        let pixels = render_project(&project).unwrap();
        assert_eq!(pixels[0], [255, 0, 0, 255]);
    }

    #[test]
    fn adds_layer_at_top_by_default() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                {
                  "op": "add_layer",
                  "layer": {
                    "name": "details",
                    "chunks": [
                      { "x": 0, "y": 0, "rows": ["K"] }
                    ]
                  }
                }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        assert_eq!(patched.layers[0].name, "details");
    }

    #[test]
    fn inserts_layer_relative_to_top_to_bottom_stack() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                {
                  "op": "insert_layer",
                  "layer": {
                    "name": "outline",
                    "rows": [
                      "....",
                      ".KK.",
                      ".KK.",
                      "...."
                    ]
                  },
                  "before": "base"
                }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        assert_eq!(patched.layers[0].name, "shadow");
        assert_eq!(patched.layers[1].name, "outline");
        assert_eq!(patched.layers[2].name, "base");
    }

    #[test]
    fn renames_layer_and_sets_layer_properties() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "rename_layer", "layer": "shadow", "name": "cast_shadow" },
                { "op": "set_layer_visibility", "layer": "cast_shadow", "visible": false },
                { "op": "set_layer_opacity", "layer": "cast_shadow", "opacity": 0.5 }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        let cast_shadow = &patched.layers[layer_index(&patched, "cast_shadow")];
        assert!(!cast_shadow.visible);
        assert_eq!(cast_shadow.opacity, 0.5);
    }

    #[test]
    fn rejects_duplicate_layer_rename() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "rename_layer", "layer": "shadow", "name": "base" }
              ]
            }
            "##,
        )
        .unwrap();

        let error = apply_patch_document(project, &patch)
            .unwrap_err()
            .to_string();
        assert!(error.contains("already exists"));
    }

    #[test]
    fn clears_layer_to_empty_transparent_layer() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "clear_layer", "layer": "shadow" }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        let shadow = &patched.layers[layer_index(&patched, "shadow")];
        assert!(shadow.rows.is_none());
        assert!(shadow.chunks.is_empty());
    }
}
