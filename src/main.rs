use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::{AnimationDecoder, Delay, Frame as ImageFrame, ImageBuffer, ImageReader, Rgba};
use serde::Serialize;

use agentpaint::{
    Animation, InspectChunk, InspectLayer, InspectSummary, PatchDocument, Project,
    RenderedAnimationFrame, RgbaAnimationFrame, RgbaExport, animation_base_project,
    animation_from_rgba_frames, animation_validation_errors, apply_patch_document,
    chunk_dimensions, fit_integer_scale, project_from_rgba_pixels, psd_bytes,
    render_animation_frame_project, render_animation_frames, render_project, supersample_pixels,
    validation_errors,
};

#[derive(Debug, Parser)]
#[command(name = "agentpaint")]
#[command(version)]
#[command(
    about = "Validate, import, patch, render, animate, and export layered APX pixel-art projects."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Validate {
        input: PathBuf,
    },
    Inspect {
        input: PathBuf,
    },
    Render {
        input: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    ImportImage {
        input: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
        #[arg(long, default_value = "paint")]
        layer_name: String,
    },
    Supersample {
        input: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
        #[arg(long, default_value_t = 1280)]
        target_width: u32,
        #[arg(long, default_value_t = 720)]
        target_height: u32,
        #[arg(long)]
        scale: Option<u32>,
    },
    Patch {
        input: PathBuf,
        #[arg(long)]
        patch: PathBuf,
        #[arg(short, long)]
        out: Option<PathBuf>,
        #[arg(long)]
        in_place: bool,
    },
    ExportRgba {
        input: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    ExportPsd {
        input: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    ValidateAnimation {
        input: PathBuf,
    },
    ImportGif {
        input: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
        #[arg(long, default_value = "paint")]
        layer_name: String,
    },
    InspectAnimation {
        input: PathBuf,
    },
    RenderFrame {
        input: PathBuf,
        #[arg(long)]
        frame: usize,
        #[arg(short, long)]
        out: PathBuf,
    },
    SupersampleFrame {
        input: PathBuf,
        #[arg(long)]
        frame: usize,
        #[arg(short, long)]
        out: PathBuf,
        #[arg(long, default_value_t = 1280)]
        target_width: u32,
        #[arg(long, default_value_t = 720)]
        target_height: u32,
        #[arg(long)]
        scale: Option<u32>,
    },
    RenderGif {
        input: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { input } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            println!("OK: {}", input.display());
            Ok(())
        }
        Commands::Inspect { input } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            let summary = inspect_project(&project);
            println!("{}", serde_json::to_string_pretty(&summary)?);
            Ok(())
        }
        Commands::Render { input, out } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            let pixels = render_project(&project)?;
            save_png(&project, &pixels, &out)?;
            println!("Wrote {}", out.display());
            Ok(())
        }
        Commands::ImportImage {
            input,
            out,
            layer_name,
        } => {
            let (width, height, pixels) = load_rgba_image(&input)?;
            let project = project_from_rgba_pixels(width, height, &pixels, &layer_name)?;
            write_project(&out, &project)?;
            println!("Wrote {}", out.display());
            Ok(())
        }
        Commands::Supersample {
            input,
            out,
            target_width,
            target_height,
            scale,
        } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            let pixels = render_project(&project)?;
            let scale = match scale {
                Some(scale) => scale,
                None => fit_integer_scale(
                    project.canvas.width,
                    project.canvas.height,
                    target_width,
                    target_height,
                )?,
            };
            let supersampled =
                supersample_pixels(project.canvas.width, project.canvas.height, &pixels, scale)?;
            save_png_pixels(
                supersampled.width,
                supersampled.height,
                &supersampled.pixels,
                &out,
            )?;
            println!(
                "Wrote {} ({}x{}, scale {})",
                out.display(),
                supersampled.width,
                supersampled.height,
                supersampled.scale
            );
            Ok(())
        }
        Commands::Patch {
            input,
            patch,
            out,
            in_place,
        } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            let patch_document = load_patch(&patch)?;
            let patched = apply_patch_document(project, &patch_document)?;
            let output = patch_output_path(&input, out, in_place)?;
            write_project(&output, &patched)?;
            println!("Wrote {}", output.display());
            Ok(())
        }
        Commands::ExportRgba { input, out } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            let pixels = render_project(&project)?;
            let export = RgbaExport {
                width: project.canvas.width,
                height: project.canvas.height,
                pixels,
            };
            fs::write(&out, serde_json::to_string_pretty(&export)?)
                .with_context(|| format!("failed to write {}", out.display()))?;
            println!("Wrote {}", out.display());
            Ok(())
        }
        Commands::ExportPsd { input, out } => {
            let project = load_project(&input)?;
            ensure_valid(&project)?;
            let bytes = psd_bytes(&project)?;
            fs::write(&out, bytes).with_context(|| format!("failed to write {}", out.display()))?;
            println!("Wrote {}", out.display());
            Ok(())
        }
        Commands::ValidateAnimation { input } => {
            let animation = load_animation(&input)?;
            ensure_animation_valid(&animation)?;
            println!("OK: {}", input.display());
            Ok(())
        }
        Commands::ImportGif {
            input,
            out,
            layer_name,
        } => {
            let (width, height, frames) = load_gif_animation(&input)?;
            let animation = animation_from_rgba_frames(width, height, &frames, &layer_name)?;
            write_animation(&out, &animation)?;
            println!("Wrote {}", out.display());
            Ok(())
        }
        Commands::InspectAnimation { input } => {
            let animation = load_animation(&input)?;
            ensure_animation_valid(&animation)?;
            let summary = inspect_animation(&animation);
            println!("{}", serde_json::to_string_pretty(&summary)?);
            Ok(())
        }
        Commands::RenderFrame { input, frame, out } => {
            let animation = load_animation(&input)?;
            ensure_animation_valid(&animation)?;
            let project = render_animation_frame_project(&animation, frame)?;
            let pixels = render_project(&project)?;
            save_png(&project, &pixels, &out)?;
            println!("Wrote {}", out.display());
            Ok(())
        }
        Commands::SupersampleFrame {
            input,
            frame,
            out,
            target_width,
            target_height,
            scale,
        } => {
            let animation = load_animation(&input)?;
            ensure_animation_valid(&animation)?;
            let project = render_animation_frame_project(&animation, frame)?;
            let pixels = render_project(&project)?;
            let scale = match scale {
                Some(scale) => scale,
                None => fit_integer_scale(
                    project.canvas.width,
                    project.canvas.height,
                    target_width,
                    target_height,
                )?,
            };
            let supersampled =
                supersample_pixels(project.canvas.width, project.canvas.height, &pixels, scale)?;
            save_png_pixels(
                supersampled.width,
                supersampled.height,
                &supersampled.pixels,
                &out,
            )?;
            println!(
                "Wrote {} ({}x{}, scale {})",
                out.display(),
                supersampled.width,
                supersampled.height,
                supersampled.scale
            );
            Ok(())
        }
        Commands::RenderGif { input, out } => {
            let animation = load_animation(&input)?;
            ensure_animation_valid(&animation)?;
            let frames = render_animation_frames(&animation)?;
            save_gif(&animation, &frames, &out)?;
            println!("Wrote {}", out.display());
            Ok(())
        }
    }
}

fn load_project(path: &Path) -> Result<Project> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
}

fn load_patch(path: &Path) -> Result<PatchDocument> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
}

fn load_animation(path: &Path) -> Result<Animation> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
}

fn load_rgba_image(path: &Path) -> Result<(u32, u32, Vec<[u8; 4]>)> {
    let image = ImageReader::open(path)
        .with_context(|| format!("failed to read {}", path.display()))?
        .with_guessed_format()
        .with_context(|| format!("failed to detect image format for {}", path.display()))?
        .decode()
        .with_context(|| format!("failed to decode {}", path.display()))?
        .to_rgba8();
    let (width, height) = image.dimensions();
    Ok((width, height, rgba_pixels(&image)))
}

fn load_gif_animation(path: &Path) -> Result<(u32, u32, Vec<RgbaAnimationFrame>)> {
    let file = File::open(path).with_context(|| format!("failed to read {}", path.display()))?;
    let decoder = GifDecoder::new(BufReader::new(file))
        .with_context(|| format!("failed to decode GIF {}", path.display()))?;
    let decoded_frames = decoder
        .into_frames()
        .collect_frames()
        .with_context(|| format!("failed to read GIF frames from {}", path.display()))?;
    if decoded_frames.is_empty() {
        return Err(anyhow!("GIF contains no frames: {}", path.display()));
    }

    let (width, height) = decoded_frames[0].buffer().dimensions();
    let mut frames = Vec::with_capacity(decoded_frames.len());
    for (index, frame) in decoded_frames.into_iter().enumerate() {
        let duration_ms = delay_to_duration_ms(frame.delay());
        let buffer = frame.into_buffer();
        if buffer.dimensions() != (width, height) {
            return Err(anyhow!(
                "GIF frame {index} dimensions {}x{} do not match first frame {}x{}",
                buffer.width(),
                buffer.height(),
                width,
                height
            ));
        }
        frames.push(RgbaAnimationFrame {
            duration_ms,
            pixels: rgba_pixels(&buffer),
        });
    }

    Ok((width, height, frames))
}

fn ensure_valid(project: &Project) -> Result<()> {
    let errors = validation_errors(project);
    if errors.is_empty() {
        return Ok(());
    }

    Err(anyhow!("APX validation failed:\n{}", errors.join("\n")))
}

fn ensure_animation_valid(animation: &Animation) -> Result<()> {
    let errors = animation_validation_errors(animation);
    if errors.is_empty() {
        return Ok(());
    }

    Err(anyhow!("APXA validation failed:\n{}", errors.join("\n")))
}

fn patch_output_path(input: &Path, out: Option<PathBuf>, in_place: bool) -> Result<PathBuf> {
    match (out, in_place) {
        (Some(_), true) => Err(anyhow!("use either --out or --in-place, not both")),
        (Some(path), false) => Ok(path),
        (None, true) => Ok(input.to_path_buf()),
        (None, false) => Err(anyhow!("patch requires --out <file.apx> or --in-place")),
    }
}

fn write_project(path: &Path, project: &Project) -> Result<()> {
    let text = format!("{}\n", serde_json::to_string_pretty(project)?);
    fs::write(path, text).with_context(|| format!("failed to write {}", path.display()))
}

fn write_animation(path: &Path, animation: &Animation) -> Result<()> {
    let text = format!("{}\n", serde_json::to_string_pretty(animation)?);
    fs::write(path, text).with_context(|| format!("failed to write {}", path.display()))
}

fn save_png(project: &Project, pixels: &[[u8; 4]], out: &Path) -> Result<()> {
    save_png_pixels(project.canvas.width, project.canvas.height, pixels, out)
}

fn save_png_pixels(width: u32, height: u32, pixels: &[[u8; 4]], out: &Path) -> Result<()> {
    let image = rgba_image(width, height, pixels);
    image
        .save(out)
        .with_context(|| format!("failed to write {}", out.display()))
}

fn save_gif(animation: &Animation, frames: &[RenderedAnimationFrame], out: &Path) -> Result<()> {
    if frames_have_partial_alpha(frames) {
        eprintln!(
            "Warning: GIF export cannot preserve partial alpha; semi-transparent pixels will be quantized."
        );
    }

    let file = File::create(out).with_context(|| format!("failed to write {}", out.display()))?;
    let mut encoder = GifEncoder::new(file);
    encoder
        .set_repeat(Repeat::Infinite)
        .with_context(|| format!("failed to initialize GIF {}", out.display()))?;

    for frame in frames {
        let image = rgba_image(
            animation.canvas.width,
            animation.canvas.height,
            &frame.pixels,
        );
        let delay = Delay::from_numer_denom_ms(frame.duration_ms, 1);
        encoder
            .encode_frame(ImageFrame::from_parts(image, 0, 0, delay))
            .with_context(|| format!("failed to encode GIF {}", out.display()))?;
    }

    Ok(())
}

fn frames_have_partial_alpha(frames: &[RenderedAnimationFrame]) -> bool {
    frames
        .iter()
        .flat_map(|frame| frame.pixels.iter())
        .any(|pixel| pixel[3] != 0 && pixel[3] != 255)
}

fn rgba_image(width: u32, height: u32, pixels: &[[u8; 4]]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    for (index, rgba) in pixels.iter().enumerate() {
        let x = (index as u32) % width;
        let y = (index as u32) / width;
        image.put_pixel(x, y, Rgba(*rgba));
    }
    image
}

fn rgba_pixels(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<[u8; 4]> {
    image.pixels().map(|pixel| pixel.0).collect()
}

fn delay_to_duration_ms(delay: Delay) -> u32 {
    let (numerator, denominator) = delay.numer_denom_ms();
    if denominator == 0 {
        return 1;
    }

    let rounded_up = u64::from(numerator).div_ceil(u64::from(denominator));
    u32::try_from(rounded_up).unwrap_or(u32::MAX).max(1)
}

fn inspect_project(project: &Project) -> InspectSummary {
    InspectSummary {
        width: project.canvas.width,
        height: project.canvas.height,
        background: project.background.clone(),
        layer_order: "top_to_bottom".to_string(),
        palette_symbols: project.palette.keys().cloned().collect(),
        layers: project
            .layers
            .iter()
            .enumerate()
            .map(|(index, layer)| InspectLayer {
                index,
                name: layer.name.clone(),
                visible: layer.visible,
                opacity: layer.opacity,
                full_rows: layer.rows.as_ref().map_or(0, Vec::len),
                chunks: layer
                    .chunks
                    .iter()
                    .map(|chunk| {
                        let (width, height) = chunk_dimensions(chunk).unwrap_or((0, 0));
                        InspectChunk {
                            x: chunk.x,
                            y: chunk.y,
                            width,
                            height,
                        }
                    })
                    .collect(),
            })
            .collect(),
    }
}

#[derive(Debug, Serialize)]
struct AnimationInspectSummary {
    width: u32,
    height: u32,
    background: Option<String>,
    layer_order: String,
    palette_symbols: Vec<String>,
    base_layers: Vec<InspectLayer>,
    frames: Vec<InspectAnimationFrame>,
}

#[derive(Debug, Serialize)]
struct InspectAnimationFrame {
    index: usize,
    name: Option<String>,
    duration_ms: u32,
    operation_count: usize,
}

fn inspect_animation(animation: &Animation) -> AnimationInspectSummary {
    let base = animation_base_project(animation);
    let base_summary = inspect_project(&base);
    AnimationInspectSummary {
        width: base_summary.width,
        height: base_summary.height,
        background: base_summary.background,
        layer_order: base_summary.layer_order,
        palette_symbols: base_summary.palette_symbols,
        base_layers: base_summary.layers,
        frames: animation
            .frames
            .iter()
            .enumerate()
            .map(|(index, frame)| InspectAnimationFrame {
                index,
                name: frame.name.clone(),
                duration_ms: frame.duration_ms,
                operation_count: frame.operations.len(),
            })
            .collect(),
    }
}
