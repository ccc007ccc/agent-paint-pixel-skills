use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub canvas: Canvas,
    pub palette: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
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
pub struct Animation {
    pub canvas: Canvas,
    pub palette: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(default)]
    pub layers: Vec<Layer>,
    pub frames: Vec<AnimationFrame>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnimationFrame {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default = "default_frame_duration_ms")]
    pub duration_ms: u32,
    #[serde(default)]
    pub operations: Vec<PatchOperation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOperation {
    SetPalette {
        symbol: String,
        color: String,
    },
    SetBackground {
        color: String,
    },
    ClearBackground,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersampledImage {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub pixels: Vec<[u8; 4]>,
}

#[derive(Debug)]
pub struct RenderedAnimationFrame {
    pub duration_ms: u32,
    pub pixels: Vec<[u8; 4]>,
}

#[derive(Debug, Serialize)]
pub struct InspectSummary {
    pub width: u32,
    pub height: u32,
    pub background: Option<String>,
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

fn default_frame_duration_ms() -> u32 {
    120
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
        if symbol_char == '_' {
            errors.push("palette key `_` is reserved for the background placeholder".to_string());
            continue;
        }

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

    if let Some(background) = &project.background {
        if let Err(error) = parse_color(background) {
            errors.push(format!(
                "background has invalid color `{background}`: {error}"
            ));
        }
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

        let layer_pixels = render_layer_pixels(project, layer, &palette, pixel_count);
        for (index, source) in layer_pixels.into_iter().enumerate() {
            if source[3] == 0 {
                continue;
            }
            output[index] = alpha_over(output[index], source, layer.opacity);
        }
    }

    Ok(output)
}

pub fn psd_bytes(project: &Project) -> Result<Vec<u8>> {
    let errors = validation_errors(project);
    if !errors.is_empty() {
        return Err(anyhow!("APX validation failed:\n{}", errors.join("\n")));
    }
    validate_psd_limits(project)?;

    let composite_pixels = render_project(project)?;
    let layer_pixels = render_psd_layers(project)?;

    let mut bytes = Vec::new();
    write_psd_header(&mut bytes, project.canvas.width, project.canvas.height);
    write_u32(&mut bytes, 0);
    write_u32(&mut bytes, 0);

    let layer_and_mask = psd_layer_and_mask_section(project, &layer_pixels)?;
    write_u32_checked(
        &mut bytes,
        layer_and_mask.len(),
        "PSD layer and mask section",
    )?;
    bytes.extend_from_slice(&layer_and_mask);

    write_image_data(&mut bytes, &composite_pixels);
    Ok(bytes)
}

pub fn fit_integer_scale(
    source_width: u32,
    source_height: u32,
    target_width: u32,
    target_height: u32,
) -> Result<u32> {
    if source_width == 0 || source_height == 0 {
        return Err(anyhow!("source dimensions must be greater than 0"));
    }
    if target_width == 0 || target_height == 0 {
        return Err(anyhow!("target dimensions must be greater than 0"));
    }

    let width_scale = target_width / source_width;
    let height_scale = target_height / source_height;
    Ok(width_scale.min(height_scale).max(1))
}

pub fn supersample_pixels(
    width: u32,
    height: u32,
    pixels: &[[u8; 4]],
    scale: u32,
) -> Result<SupersampledImage> {
    if width == 0 || height == 0 {
        return Err(anyhow!("source dimensions must be greater than 0"));
    }
    if scale == 0 {
        return Err(anyhow!("supersample scale must be greater than 0"));
    }

    let expected_len = (width as usize)
        .checked_mul(height as usize)
        .ok_or_else(|| anyhow!("source pixel count overflows usize"))?;
    if pixels.len() != expected_len {
        return Err(anyhow!(
            "pixel count mismatch: expected {expected_len}, got {}",
            pixels.len()
        ));
    }

    let output_width = width
        .checked_mul(scale)
        .ok_or_else(|| anyhow!("supersampled width overflows u32"))?;
    let output_height = height
        .checked_mul(scale)
        .ok_or_else(|| anyhow!("supersampled height overflows u32"))?;
    let output_len = (output_width as usize)
        .checked_mul(output_height as usize)
        .ok_or_else(|| anyhow!("supersampled pixel count overflows usize"))?;
    let mut output = vec![[0, 0, 0, 0]; output_len];

    for y in 0..height {
        for x in 0..width {
            let source = pixels[(y * width + x) as usize];
            let start_x = x * scale;
            let start_y = y * scale;
            for dy in 0..scale {
                let output_y = start_y + dy;
                let row_start = (output_y * output_width + start_x) as usize;
                for dx in 0..scale {
                    output[row_start + dx as usize] = source;
                }
            }
        }
    }

    Ok(SupersampledImage {
        width: output_width,
        height: output_height,
        scale,
        pixels: output,
    })
}

pub fn apply_patch_document(project: Project, patch: &PatchDocument) -> Result<Project> {
    if patch.operations.is_empty() {
        return Err(anyhow!("patch must contain at least one operation"));
    }

    apply_patch_operations(project, &patch.operations)
}

pub fn apply_patch_operations(
    mut project: Project,
    operations: &[PatchOperation],
) -> Result<Project> {
    for operation in operations {
        match operation {
            PatchOperation::SetPalette { symbol, color } => {
                ensure_palette_symbol(symbol)?;
                parse_color(color)?;
                project.palette.insert(symbol.clone(), color.clone());
            }
            PatchOperation::SetBackground { color } => {
                parse_color(color)?;
                project.background = Some(color.clone());
            }
            PatchOperation::ClearBackground => {
                project.background = None;
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

pub fn animation_base_project(animation: &Animation) -> Project {
    Project {
        canvas: animation.canvas.clone(),
        palette: animation.palette.clone(),
        background: animation.background.clone(),
        layers: animation.layers.clone(),
    }
}

pub fn animation_validation_errors(animation: &Animation) -> Vec<String> {
    let base = animation_base_project(animation);
    let base_errors = validation_errors(&base);
    let mut errors: Vec<String> = base_errors
        .iter()
        .map(|error| format!("base {error}"))
        .collect();

    if animation.frames.is_empty() {
        errors.push("frames must contain at least one frame".to_string());
    }

    if !base_errors.is_empty() {
        return errors;
    }

    for (frame_index, frame) in animation.frames.iter().enumerate() {
        if frame.duration_ms == 0 {
            errors.push(format!(
                "frame {frame_index} duration_ms must be greater than 0"
            ));
        }

        if let Err(error) = apply_patch_operations(base.clone(), &frame.operations) {
            errors.push(format!("frame {frame_index} patch failed: {error:#}"));
        }
    }

    errors
}

pub fn render_animation_frame_project(
    animation: &Animation,
    frame_index: usize,
) -> Result<Project> {
    let frame = animation
        .frames
        .get(frame_index)
        .ok_or_else(|| anyhow!("frame index {frame_index} is out of range"))?;
    let base = animation_base_project(animation);
    apply_patch_operations(base, &frame.operations)
        .map_err(|error| anyhow!("frame {frame_index} patch failed: {error:#}"))
}

pub fn render_animation_frames(animation: &Animation) -> Result<Vec<RenderedAnimationFrame>> {
    let errors = animation_validation_errors(animation);
    if !errors.is_empty() {
        return Err(anyhow!("APXA validation failed:\n{}", errors.join("\n")));
    }

    animation
        .frames
        .iter()
        .enumerate()
        .map(|(frame_index, frame)| {
            let project = render_animation_frame_project(animation, frame_index)?;
            let pixels = render_project(&project)?;
            Ok(RenderedAnimationFrame {
                duration_ms: frame.duration_ms,
                pixels,
            })
        })
        .collect()
}

pub fn chunk_dimensions(chunk: &Chunk) -> Option<(usize, usize)> {
    let width = infer_chunk_width(&chunk.rows)?;
    Some((width, chunk.rows.len()))
}

fn ensure_single_symbol(symbol: &str) -> Result<()> {
    if symbol.chars().count() != 1 {
        return Err(anyhow!(
            "palette symbol `{symbol}` must be a single character"
        ));
    }
    Ok(())
}

fn ensure_palette_symbol(symbol: &str) -> Result<()> {
    ensure_single_symbol(symbol)?;
    if symbol == "_" {
        return Err(anyhow!(
            "palette symbol `_` is reserved for the background placeholder"
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
        let width = row_width(row, project.canvas.width as usize);
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

    let Some(expected_width) = infer_chunk_width(&chunk.rows) else {
        errors.push(format!(
            "{label} uses `_` row shorthand but has no explicit-width row"
        ));
        return;
    };
    if expected_width == 0 {
        errors.push(format!("{label} width must be greater than 0"));
    }

    for (row_index, row) in chunk.rows.iter().enumerate() {
        let width = row_width(row, expected_width);
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
        if symbol == '_' {
            continue;
        }
        if !symbols.contains(&symbol) {
            errors.push(format!(
                "{label} row {row_index} column {column_index} uses undefined palette symbol `{symbol}`"
            ));
        }
    }
}

fn parsed_palette(project: &Project) -> Result<BTreeMap<char, [u8; 4]>> {
    let mut palette: BTreeMap<char, [u8; 4]> = project
        .palette
        .iter()
        .map(|(symbol, value)| {
            let key = symbol
                .chars()
                .next()
                .ok_or_else(|| anyhow!("empty palette key"))?;
            Ok((key, parse_color(value)?))
        })
        .collect::<Result<_>>()?;

    let background = match &project.background {
        Some(color) => parse_color(color)?,
        None => [0, 0, 0, 0],
    };
    palette.insert('_', background);
    Ok(palette)
}

fn render_psd_layers(project: &Project) -> Result<Vec<Vec<[u8; 4]>>> {
    let palette = parsed_palette(project)?;
    let pixel_count = checked_pixel_count(project)?;
    let mut rendered = Vec::with_capacity(project.layers.len());

    for layer in project.layers.iter().rev() {
        rendered.push(render_layer_pixels(project, layer, &palette, pixel_count));
    }

    Ok(rendered)
}

fn render_layer_pixels(
    project: &Project,
    layer: &Layer,
    palette: &BTreeMap<char, [u8; 4]>,
    pixel_count: usize,
) -> Vec<[u8; 4]> {
    let mut pixels = vec![[0, 0, 0, 0]; pixel_count];
    if let Some(rows) = &layer.rows {
        paint_rows(
            &mut pixels,
            project.canvas.width,
            project.canvas.width as usize,
            0,
            0,
            rows,
            1.0,
            palette,
        );
    }

    for chunk in &layer.chunks {
        let Some((chunk_width, _)) = chunk_dimensions(chunk) else {
            continue;
        };
        paint_rows(
            &mut pixels,
            project.canvas.width,
            chunk_width,
            chunk.x,
            chunk.y,
            &chunk.rows,
            1.0,
            palette,
        );
    }

    pixels
}

fn validate_psd_limits(project: &Project) -> Result<()> {
    if project.canvas.width > 30_000 || project.canvas.height > 30_000 {
        return Err(anyhow!(
            "PSD export supports canvas dimensions up to 30000x30000"
        ));
    }
    if project.layers.len() > i16::MAX as usize {
        return Err(anyhow!("PSD export supports at most {} layers", i16::MAX));
    }
    checked_pixel_count(project)?;
    Ok(())
}

fn checked_pixel_count(project: &Project) -> Result<usize> {
    (project.canvas.width as usize)
        .checked_mul(project.canvas.height as usize)
        .ok_or_else(|| anyhow!("canvas pixel count overflows usize"))
}

fn write_psd_header(bytes: &mut Vec<u8>, width: u32, height: u32) {
    bytes.extend_from_slice(b"8BPS");
    write_u16(bytes, 1);
    bytes.extend_from_slice(&[0; 6]);
    write_u16(bytes, 4);
    write_u32(bytes, height);
    write_u32(bytes, width);
    write_u16(bytes, 8);
    write_u16(bytes, 3);
}

fn psd_layer_and_mask_section(project: &Project, layer_pixels: &[Vec<[u8; 4]>]) -> Result<Vec<u8>> {
    let mut layer_info = Vec::new();
    write_i16(&mut layer_info, project.layers.len() as i16);

    for layer in project.layers.iter().rev() {
        write_layer_record(&mut layer_info, project, layer)?;
    }

    for pixels in layer_pixels {
        write_layer_channel_data(&mut layer_info, pixels);
    }

    if layer_info.len() % 2 != 0 {
        layer_info.push(0);
    }

    let mut section = Vec::new();
    write_u32_checked(&mut section, layer_info.len(), "PSD layer info section")?;
    section.extend_from_slice(&layer_info);
    write_u32(&mut section, 0);
    Ok(section)
}

fn write_layer_record(bytes: &mut Vec<u8>, project: &Project, layer: &Layer) -> Result<()> {
    let channel_byte_count = checked_channel_byte_count(project)?;
    write_i32(bytes, 0);
    write_i32(bytes, 0);
    write_i32(bytes, project.canvas.height as i32);
    write_i32(bytes, project.canvas.width as i32);
    write_u16(bytes, 4);

    for channel_id in [0, 1, 2, -1] {
        write_i16(bytes, channel_id);
        write_u32_checked(bytes, channel_byte_count + 2, "PSD layer channel data")?;
    }

    bytes.extend_from_slice(b"8BIM");
    bytes.extend_from_slice(b"norm");
    bytes.push(to_u8(layer.opacity * 255.0));
    bytes.push(0);
    bytes.push(psd_layer_flags(layer));
    bytes.push(0);

    let extra = layer_extra_data(&layer.name);
    write_u32_checked(bytes, extra.len(), "PSD layer extra data")?;
    bytes.extend_from_slice(&extra);
    Ok(())
}

fn psd_layer_flags(layer: &Layer) -> u8 {
    // In Photoshop-compatible PSDs, bit 1 marks a layer as hidden.
    if layer.visible { 0 } else { 1 << 1 }
}

fn checked_channel_byte_count(project: &Project) -> Result<usize> {
    checked_pixel_count(project)
}

fn layer_extra_data(name: &str) -> Vec<u8> {
    let mut extra = Vec::new();
    write_u32(&mut extra, 0);
    write_u32(&mut extra, 0);
    write_pascal_layer_name(&mut extra, name);
    write_unicode_layer_name(&mut extra, name);
    extra
}

fn write_pascal_layer_name(bytes: &mut Vec<u8>, name: &str) {
    let mut name_bytes: Vec<u8> = name
        .bytes()
        .map(|byte| {
            if byte.is_ascii_graphic() || byte == b' ' {
                byte
            } else {
                b'_'
            }
        })
        .collect();
    if name_bytes.is_empty() {
        name_bytes.extend_from_slice(b"Layer");
    }
    name_bytes.truncate(u8::MAX as usize);

    bytes.push(name_bytes.len() as u8);
    bytes.extend_from_slice(&name_bytes);
    while bytes.len() % 4 != 0 {
        bytes.push(0);
    }
}

fn write_unicode_layer_name(bytes: &mut Vec<u8>, name: &str) {
    let utf16: Vec<u16> = name.encode_utf16().collect();
    let mut data = Vec::with_capacity(4 + utf16.len() * 2);
    write_u32(&mut data, utf16.len() as u32);
    for code_unit in utf16 {
        write_u16(&mut data, code_unit);
    }

    bytes.extend_from_slice(b"8BIM");
    bytes.extend_from_slice(b"luni");
    write_u32(bytes, data.len() as u32);
    bytes.extend_from_slice(&data);
}

fn write_layer_channel_data(bytes: &mut Vec<u8>, pixels: &[[u8; 4]]) {
    for channel in [0, 1, 2, 3] {
        write_u16(bytes, 0);
        for pixel in pixels {
            bytes.push(pixel[channel]);
        }
    }
}

fn write_image_data(bytes: &mut Vec<u8>, pixels: &[[u8; 4]]) {
    write_u16(bytes, 0);
    for channel in [0, 1, 2, 3] {
        for pixel in pixels {
            bytes.push(pixel[channel]);
        }
    }
}

fn write_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn write_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn write_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn write_u32_checked(bytes: &mut Vec<u8>, value: usize, label: &str) -> Result<()> {
    let value = u32::try_from(value).map_err(|_| anyhow!("{label} exceeds PSD size limits"))?;
    write_u32(bytes, value);
    Ok(())
}

fn write_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn paint_rows(
    output: &mut [[u8; 4]],
    canvas_width: u32,
    paint_width: usize,
    offset_x: u32,
    offset_y: u32,
    rows: &[String],
    opacity: f32,
    palette: &BTreeMap<char, [u8; 4]>,
) {
    for (row_index, row) in rows.iter().enumerate() {
        let symbols: Box<dyn Iterator<Item = char> + '_> = if row == "_" {
            Box::new(std::iter::repeat('_').take(paint_width))
        } else {
            Box::new(row.chars())
        };

        for (column_index, symbol) in symbols.enumerate() {
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

fn row_width(row: &str, shorthand_width: usize) -> usize {
    if row == "_" {
        shorthand_width
    } else {
        row.chars().count()
    }
}

fn infer_chunk_width(rows: &[String]) -> Option<usize> {
    rows.iter()
        .find(|row| row.as_str() != "_")
        .map(|row| row.chars().count())
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
    fn underscore_defaults_to_transparent_and_supports_full_row_shorthand() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 3, "height": 2 },
              "palette": {
                ".": "transparent",
                "R": "#ff0000"
              },
              "layers": [
                {
                  "name": "paint",
                  "rows": [
                    "_",
                    "R_R"
                  ]
                }
              ]
            }
            "##,
        )
        .unwrap();

        let errors = validation_errors(&project);
        assert!(errors.is_empty(), "{errors:?}");

        let pixels = render_project(&project).unwrap();
        assert_eq!(
            pixels,
            vec![
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [255, 0, 0, 255],
                [0, 0, 0, 0],
                [255, 0, 0, 255],
            ]
        );
    }

    #[test]
    fn underscore_uses_background_color_when_background_is_set() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 3, "height": 2 },
              "background": "#123456",
              "palette": {
                ".": "transparent",
                "R": "#ff0000"
              },
              "layers": [
                {
                  "name": "paint",
                  "rows": [
                    "_",
                    "R_R"
                  ]
                }
              ]
            }
            "##,
        )
        .unwrap();

        let errors = validation_errors(&project);
        assert!(errors.is_empty(), "{errors:?}");

        let pixels = render_project(&project).unwrap();
        assert_eq!(
            pixels,
            vec![
                [18, 52, 86, 255],
                [18, 52, 86, 255],
                [18, 52, 86, 255],
                [255, 0, 0, 255],
                [18, 52, 86, 255],
                [255, 0, 0, 255],
            ]
        );
    }

    #[test]
    fn rejects_palette_underscore_because_it_is_reserved() {
        let mut project = sample_project();
        project
            .palette
            .insert("_".to_string(), "#123456".to_string());

        let errors = validation_errors(&project);
        assert!(errors.iter().any(|error| error.contains("reserved")));
    }

    #[test]
    fn chunk_underscore_shorthand_uses_explicit_row_width() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 3, "height": 2 },
              "background": "#010203",
              "palette": {
                ".": "transparent",
                "R": "#ff0000"
              },
              "layers": [
                {
                  "name": "paint",
                  "chunks": [
                    { "x": 0, "y": 0, "rows": ["R_R", "_"] }
                  ]
                }
              ]
            }
            "##,
        )
        .unwrap();

        let errors = validation_errors(&project);
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(
            chunk_dimensions(&project.layers[0].chunks[0]).unwrap(),
            (3, 2)
        );

        let pixels = render_project(&project).unwrap();
        assert_eq!(
            pixels,
            vec![
                [255, 0, 0, 255],
                [1, 2, 3, 255],
                [255, 0, 0, 255],
                [1, 2, 3, 255],
                [1, 2, 3, 255],
                [1, 2, 3, 255],
            ]
        );
    }

    #[test]
    fn rejects_chunk_underscore_shorthand_without_explicit_width() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 3, "height": 2 },
              "background": "#010203",
              "palette": {
                ".": "transparent"
              },
              "layers": [
                {
                  "name": "paint",
                  "chunks": [
                    { "x": 0, "y": 0, "rows": ["_", "_"] }
                  ]
                }
              ]
            }
            "##,
        )
        .unwrap();

        let errors = validation_errors(&project);
        assert!(errors.iter().any(|error| {
            error.contains("uses `_` row shorthand but has no explicit-width row")
        }));
    }

    #[test]
    fn patch_can_set_and_clear_background() {
        let project = sample_project();
        let patch: PatchDocument = serde_json::from_str(
            r##"
            {
              "operations": [
                { "op": "set_background", "color": "#010203" },
                {
                  "op": "set_rows",
                  "layer": "base",
                  "rows": ["_", "_", "_", "_"]
                }
              ]
            }
            "##,
        )
        .unwrap();

        let patched = apply_patch_document(project, &patch).unwrap();
        assert_eq!(patched.background.as_deref(), Some("#010203"));
        assert_eq!(render_project(&patched).unwrap()[0], [1, 2, 3, 255]);

        let cleared = apply_patch_operations(patched, &[PatchOperation::ClearBackground]).unwrap();
        assert_eq!(cleared.background, None);
        assert_eq!(render_project(&cleared).unwrap()[0], [0, 0, 0, 0]);
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
    fn applies_layer_opacity_once_after_layer_pixels() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 1, "height": 1 },
              "palette": {
                ".": "transparent",
                "R": "#ff0000",
                "B": "#0000ff"
              },
              "layers": [
                {
                  "name": "paint",
                  "opacity": 0.5,
                  "rows": ["R"],
                  "chunks": [
                    { "x": 0, "y": 0, "rows": ["B"] }
                  ]
                }
              ]
            }
            "##,
        )
        .unwrap();

        let pixels = render_project(&project).unwrap();
        assert_eq!(pixels[0], [0, 0, 255, 128]);
    }

    #[test]
    fn calculates_integer_fit_scale_for_model_preview() {
        assert_eq!(fit_integer_scale(32, 32, 1280, 720).unwrap(), 22);
        assert_eq!(fit_integer_scale(128, 64, 1280, 720).unwrap(), 10);
        assert_eq!(fit_integer_scale(200, 200, 1280, 720).unwrap(), 3);
        assert_eq!(fit_integer_scale(1600, 900, 1280, 720).unwrap(), 1);
    }

    #[test]
    fn supersamples_pixels_by_exact_integer_blocks() {
        let red = [255, 0, 0, 255];
        let green = [0, 255, 0, 255];
        let blue = [0, 0, 255, 255];
        let clear = [0, 0, 0, 0];
        let image = supersample_pixels(2, 2, &[red, green, blue, clear], 3).unwrap();

        assert_eq!(image.width, 6);
        assert_eq!(image.height, 6);
        assert_eq!(image.scale, 3);
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(image.pixels[(y * 6 + x) as usize], red);
                assert_eq!(image.pixels[(y * 6 + x + 3) as usize], green);
                assert_eq!(image.pixels[((y + 3) * 6 + x) as usize], blue);
                assert_eq!(image.pixels[((y + 3) * 6 + x + 3) as usize], clear);
            }
        }
    }

    #[test]
    fn exports_parseable_layered_psd() {
        let project = sample_project();
        let bytes = psd_bytes(&project).unwrap();

        assert_eq!(&bytes[0..4], b"8BPS");
        assert_eq!(u16::from_be_bytes([bytes[12], bytes[13]]), 4);
        assert_eq!(
            u32::from_be_bytes([bytes[18], bytes[19], bytes[20], bytes[21]]),
            4
        );
        assert_eq!(
            u32::from_be_bytes([bytes[14], bytes[15], bytes[16], bytes[17]]),
            4
        );

        let psd = psd::Psd::from_bytes(&bytes).unwrap();
        assert_eq!(psd.width(), 4);
        assert_eq!(psd.height(), 4);
        assert_eq!(psd.layers().len(), 2);

        let layer_names: Vec<_> = psd.layers().iter().map(|layer| layer.name()).collect();
        assert_eq!(layer_names, vec!["shadow", "base"]);
        assert_eq!(psd_layer_record_flags(&bytes), vec![0, 0]);

        let shadow = psd.layer_by_name("shadow").unwrap();
        let shadow_pixels = shadow.rgba();
        let shadow_index = ((2 * 4 + 2) * 4) as usize;
        assert_eq!(
            &shadow_pixels[shadow_index..shadow_index + 4],
            &[0, 0, 0, 128]
        );

        let expected = render_project(&project).unwrap();
        let expected_bytes: Vec<u8> = expected.into_iter().flatten().collect();
        assert_eq!(psd.rgba(), expected_bytes);
    }

    #[test]
    fn exports_psd_with_unicode_layer_name() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 1, "height": 1 },
              "palette": {
                ".": "transparent",
                "R": "#ff0000"
              },
              "layers": [
                { "name": "顶层", "rows": ["R"] }
              ]
            }
            "##,
        )
        .unwrap();

        let bytes = psd_bytes(&project).unwrap();
        let psd = psd::Psd::from_bytes(&bytes).unwrap();
        assert!(psd.layer_by_name("顶层").is_some());
    }

    #[test]
    fn exports_photoshop_compatible_visibility_flags() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 1, "height": 1 },
              "palette": {
                ".": "transparent",
                "R": "#ff0000",
                "B": "#0000ff"
              },
              "layers": [
                { "name": "visible_top", "rows": ["R"] },
                { "name": "hidden_bottom", "visible": false, "rows": ["B"] }
              ]
            }
            "##,
        )
        .unwrap();

        let bytes = psd_bytes(&project).unwrap();
        let flags = psd_layer_record_flags(&bytes);
        assert_eq!(flags, vec![1 << 1, 0]);
    }

    #[test]
    fn exports_transparency_channels_and_layer_opacity() {
        let project: Project = serde_json::from_str(
            r##"
            {
              "canvas": { "width": 2, "height": 1 },
              "palette": {
                ".": "transparent",
                "S": "rgba(10,20,30,0.25)",
                "O": "#112233"
              },
              "layers": [
                { "name": "semi_top", "opacity": 0.5, "rows": ["S."] },
                { "name": "hidden_bottom", "visible": false, "rows": ["OO"] }
              ]
            }
            "##,
        )
        .unwrap();

        let pixels = render_project(&project).unwrap();
        assert_eq!(pixels, vec![[10, 20, 30, 32], [0, 0, 0, 0]]);

        let bytes = psd_bytes(&project).unwrap();
        let layers = psd_layer_records(&bytes);
        assert_eq!(layers.len(), 2);

        let hidden_bottom = &layers[0];
        assert_eq!(hidden_bottom.flags, 1 << 1);
        assert_eq!(hidden_bottom.opacity, 255);
        assert_eq!(hidden_bottom.channel(0), &[0x11, 0x11]);
        assert_eq!(hidden_bottom.channel(1), &[0x22, 0x22]);
        assert_eq!(hidden_bottom.channel(2), &[0x33, 0x33]);
        assert_eq!(hidden_bottom.channel(-1), &[255, 255]);

        let semi_top = &layers[1];
        assert_eq!(semi_top.flags, 0);
        assert_eq!(semi_top.opacity, 128);
        assert_eq!(semi_top.channel(0), &[10, 0]);
        assert_eq!(semi_top.channel(1), &[20, 0]);
        assert_eq!(semi_top.channel(2), &[30, 0]);
        assert_eq!(semi_top.channel(-1), &[64, 0]);
    }

    fn psd_layer_record_flags(bytes: &[u8]) -> Vec<u8> {
        psd_layer_records(bytes)
            .into_iter()
            .map(|layer| layer.flags)
            .collect()
    }

    #[derive(Debug)]
    struct TestPsdLayerRecord {
        opacity: u8,
        flags: u8,
        channel_data: Vec<(i16, Vec<u8>)>,
    }

    impl TestPsdLayerRecord {
        fn channel(&self, channel_id: i16) -> &[u8] {
            self.channel_data
                .iter()
                .find_map(|(candidate, data)| {
                    if *candidate == channel_id {
                        Some(data.as_slice())
                    } else {
                        None
                    }
                })
                .unwrap()
        }
    }

    fn psd_layer_records(bytes: &[u8]) -> Vec<TestPsdLayerRecord> {
        let mut offset = 26;
        offset = skip_length_prefixed_section(bytes, offset);
        offset = skip_length_prefixed_section(bytes, offset);

        let layer_and_mask_len = read_u32_at(bytes, offset) as usize;
        offset += 4;
        let layer_and_mask_end = offset + layer_and_mask_len;
        assert!(layer_and_mask_end <= bytes.len());

        let layer_info_len = read_u32_at(bytes, offset) as usize;
        offset += 4;
        let layer_info_end = offset + layer_info_len;
        assert!(layer_info_end <= layer_and_mask_end);

        let layer_count = read_i16_at(bytes, offset).unsigned_abs() as usize;
        offset += 2;

        let mut layers = Vec::with_capacity(layer_count);
        for _ in 0..layer_count {
            offset += 16;
            let channel_count = read_u16_at(bytes, offset) as usize;
            offset += 2;
            let mut channel_lengths = Vec::with_capacity(channel_count);
            for _ in 0..channel_count {
                let channel_id = read_i16_at(bytes, offset);
                let channel_len = read_u32_at(bytes, offset + 2) as usize;
                channel_lengths.push((channel_id, channel_len));
                offset += 6;
            }

            offset += 8;
            let opacity = bytes[offset];
            offset += 2;
            let flags = bytes[offset];
            offset += 2;
            let extra_len = read_u32_at(bytes, offset) as usize;
            offset += 4 + extra_len;

            layers.push((opacity, flags, channel_lengths));
        }

        layers
            .into_iter()
            .map(|(opacity, flags, channel_lengths)| {
                let mut channel_data = Vec::with_capacity(channel_lengths.len());
                for (channel_id, channel_len) in channel_lengths {
                    assert_eq!(read_u16_at(bytes, offset), 0);
                    offset += 2;
                    let data_len = channel_len - 2;
                    let data = bytes[offset..offset + data_len].to_vec();
                    offset += data_len;
                    channel_data.push((channel_id, data));
                }

                TestPsdLayerRecord {
                    opacity,
                    flags,
                    channel_data,
                }
            })
            .collect()
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

    fn sample_animation() -> Animation {
        serde_json::from_str(
            r##"
            {
              "canvas": { "width": 2, "height": 2 },
              "palette": {
                ".": "transparent",
                "R": "#ff0000"
              },
              "layers": [
                {
                  "name": "slime",
                  "rows": [
                    ".R",
                    "RR"
                  ]
                }
              ],
              "frames": [
                {
                  "name": "squash",
                  "duration_ms": 100,
                  "operations": []
                },
                {
                  "name": "jump",
                  "duration_ms": 80,
                  "operations": [
                    {
                      "op": "set_rows",
                      "layer": "slime",
                      "rows": [
                        "RR",
                        ".R"
                      ]
                    }
                  ]
                }
              ]
            }
            "##,
        )
        .unwrap()
    }

    #[test]
    fn validates_animation_with_empty_first_frame_patch() {
        let errors = animation_validation_errors(&sample_animation());
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn renders_animation_frames_from_base_and_frame_patches() {
        let frames = render_animation_frames(&sample_animation()).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].duration_ms, 100);
        assert_eq!(frames[1].duration_ms, 80);
        assert_ne!(frames[0].pixels, frames[1].pixels);
    }

    #[test]
    fn animation_frames_inherit_background_placeholder_color() {
        let mut animation = sample_animation();
        animation.background = Some("#010203".to_string());
        animation.layers[0].rows = Some(vec!["_R".to_string(), "R_".to_string()]);

        let frames = render_animation_frames(&animation).unwrap();
        assert_eq!(frames[0].pixels[0], [1, 2, 3, 255]);
        assert_eq!(frames[0].pixels[1], [255, 0, 0, 255]);
    }

    #[test]
    fn rejects_animation_frame_patch_that_breaks_canvas_rules() {
        let mut animation = sample_animation();
        animation.frames[1].operations = vec![PatchOperation::SetRows {
            layer: "slime".to_string(),
            rows: vec!["R".to_string()],
        }];

        let errors = animation_validation_errors(&animation);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("frame 1 patch failed"))
        );
    }
}
