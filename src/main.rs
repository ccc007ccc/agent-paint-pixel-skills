use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use image::{ImageBuffer, Rgba};

use agentpaint::{
    InspectChunk, InspectLayer, InspectSummary, PatchDocument, Project, RgbaExport,
    apply_patch_document, chunk_dimensions, render_project, validation_errors,
};

#[derive(Debug, Parser)]
#[command(name = "agentpaint")]
#[command(about = "Validate, patch, render, and export layered APX pixel-art projects.")]
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

fn ensure_valid(project: &Project) -> Result<()> {
    let errors = validation_errors(project);
    if errors.is_empty() {
        return Ok(());
    }

    Err(anyhow!("APX validation failed:\n{}", errors.join("\n")))
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

fn save_png(project: &Project, pixels: &[[u8; 4]], out: &Path) -> Result<()> {
    let mut image =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::new(project.canvas.width, project.canvas.height);
    for (index, rgba) in pixels.iter().enumerate() {
        let x = (index as u32) % project.canvas.width;
        let y = (index as u32) / project.canvas.width;
        image.put_pixel(x, y, Rgba(*rgba));
    }
    image
        .save(out)
        .with_context(|| format!("failed to write {}", out.display()))
}

fn inspect_project(project: &Project) -> InspectSummary {
    InspectSummary {
        width: project.canvas.width,
        height: project.canvas.height,
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
