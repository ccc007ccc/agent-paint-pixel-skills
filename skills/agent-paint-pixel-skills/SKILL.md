---
name: agent-paint-pixel-skills
description: Generate, edit, validate, render, visually inspect, animate, import, and export AgentPaint APX/APXA pixel-art source files with style-aware pixel-art guidance. Use when an agent needs to create layered pixel art at the exact requested resolution, import low-color raster pixel art or GIFs into APX/APXA, create point-upscaled previews for AI visual inspection, create GIF animation from APXA frames, choose a pixel-art style or era, convert a pixel-art idea into APX JSON, repair APX validation errors, render APX to PNG, export layered PSD, export RGBA JSON, or make local pixel edits with rows/chunks instead of raw RGBA arrays.
---

# Agent Paint Pixel Skills

## Scope

Use this skill for APX/APXA pixel-art source authoring, editing, validation, rendering, animation, import, and export. Do not use APX for ordinary photo editing, vector/SVG work, general bitmap retouching, or non-pixel-art assets unless the user explicitly asks for AgentPaint, APX/APXA, or pixel-art conversion.

## Core Rules

- R1 Exact size: the APX/APXA `canvas` must match the requested dimensions. Do not draw smaller and resize, upscale, downscale, resample, crop, or otherwise use conversion as a substitute for drawing at the requested size.
- R2 Source format: create and edit `.apx` or `.apxa` JSON, not raw RGBA arrays or ad hoc generation scripts, unless the user explicitly requests programmatic generation or format conversion.
- R3 Validate before output: run `agentpaint validate` or `agentpaint validate-animation` before render/export/presenting source.
- R4 Visual inspection: when judging pixel art visually, inspect an integer nearest-neighbor preview from `agentpaint supersample` or `agentpaint supersample-frame`, not the raw low-resolution render. If no image-viewing capability exists, validate/inspect structurally and state that visual inspection was unavailable.
- R5 Local edits: for changes to existing APX, prefer a patch file plus `agentpaint patch` over rewriting the whole project.
- R6 Layer order: APX/APXA layers are Photoshop-style top-to-bottom; `layers[0]` is visual top/front and rendering composites from the bottom upward.

## Commands

```bash
agentpaint validate <file.apx>
agentpaint inspect <file.apx>
agentpaint import-image <input.png> --out <output.apx>
agentpaint render <file.apx> --out <file.png>
agentpaint supersample <file.apx> --out <file-preview.png>
agentpaint patch <file.apx> --patch <patch.json> --out <patched.apx>
agentpaint export-psd <file.apx> --out <file.psd>
agentpaint export-rgba <file.apx> --out <file.rgba.json>

agentpaint validate-animation <file.apxa>
agentpaint import-gif <input.gif> --out <output.apxa>
agentpaint inspect-animation <file.apxa>
agentpaint render-frame <file.apxa> --frame 0 --out <frame.png>
agentpaint supersample-frame <file.apxa> --frame 0 --out <frame-preview.png>
agentpaint render-gif <file.apxa> --out <file.gif>
```

GIF export cannot preserve partial alpha. Fully transparent pixels stay transparent, but semi-transparent pixels are quantized; avoid soft transparency when GIF is the final target.

## Workflow

1. Confirm the task fits Scope and apply R1-R6.
2. Use the installed `agentpaint` CLI from `PATH`. If `agentpaint --help` fails, ask the user to install the CLI instead of searching for the AgentPaint repo.
3. For a new still sprite, create APX with `canvas`, `palette`, and layers. Use Small Grid Mode for single-subject sprites up to `32x32`. For `64x64+`, scenes, rooms, maps, buildings, terrain, large props, or portraits, read `references/large-canvas.md` before writing.
4. For a new animation, create APXA with base layers and per-frame patch operations. Validate, render/supersample at least one key frame when checking motion, then `render-gif` if GIF output is requested.
5. For edits, inspect the current APX, write a small patch JSON, run `agentpaint patch`, then validate and render/supersample as needed.
6. For layer-order edits, inspect first, then use `move_layer` with exactly one of `to_index`, `before`, or `after`; do not rewrite the layers array by hand.
7. For imports, use `import-image` or `import-gif` only when converting existing raster artwork. Import is best for low-color pixel art; high-color photos or antialiased images can produce huge valid APX/APXA files that are hard to edit by hand.
8. For validation errors, fix the exact row, chunk, layer, palette value, or frame operation named by the CLI and rerun validation.

## Small Grid Mode

Use for `16x16`, `24x24`, `32x32`, and other single-subject sprites up to `32x32`, unless the user explicitly asks for many Photoshop-style layers.

- Use one primary full-canvas `rows` layer named `paint`.
- For `24x24`, write exactly 24 row strings, each exactly 24 characters; use no chunks in the first complete draft.
- Use a compact palette: `"."` transparent, reserved `_` background/transparent placeholder, then digit symbols `"0"`-`"9"` for 2-10 visible colors. For opaque sprites, set top-level `"background"` and use `_` for background pixels or full background rows.
- Keep every visible pixel intentional. Do not leave holes in the silhouette, face, object body, or background.
- Build silhouette first, major value/color clusters second, and small highlights/details last. Avoid decorative micro-noise.
- Add extra layers only when they improve compositing, animation, PSD export, or later edits.

## APX/APXA Essentials

- APX requires top-level `canvas`, `palette`, and `layers`. APXA adds `frames`.
- Palette keys must be single characters. Always define `"."` as `"transparent"` and never define `"_"` in `palette`.
- Without top-level `"background"`, `_` renders transparent. With `"background": "<color>"`, `_` renders as that color. A full-canvas row exactly equal to `"_"` expands to the full canvas width.
- Full-canvas `rows` must match canvas height and width. Chunks must be rectangular and stay inside the canvas.
- Empty layers are valid and useful after `clear_layer`.
- Choose meaningful layer names such as `shadow`, `outline`, `base`, `mid_shadow`, `highlight`, `eyes`, `details`, and `effects`; PSD export preserves these names and R6 order.
- APXA frames start from the same base layers and apply only that frame's `operations`. Empty frame operations are valid for a hold pose or first frame.
- Use `set_rows`, `clear_layer`, `add_chunk`, `set_layer_visibility`, `set_layer_opacity`, `move_layer`, `set_background`, and `clear_background` for frame motion and local edits.
- See `references/apx-json-v0.md` for schema details, examples, color syntax, patch operations, PSD behavior, and layer move examples.

## Style Selection

Use style constraints as hard authoring constraints, not decorative labels.

1. Read `references/pixel-art-style-guide.md` when the user asks for a named style, era, mood, genre, larger non-trivial sprite, scene, or visual tone.
2. Use that guide as the router, then load the matching `references/styles/*.md` file. Load at most two style files for explicit hybrids.
3. Patch style drift instead of rewriting the whole file: reduce colors for Game Boy, strengthen outlines for NES, remove random noise, replace dither chunks, or fix isometric diagonals.

## References

- `references/large-canvas.md`: required for still images at `64x64+`, scenes, rooms, maps, buildings, terrain, large props, large portraits, and any artwork with broad surfaces that could become flat.
- `references/pixel-art-style-guide.md`: style router and shared principles.
- `references/styles/*.md`: detailed style constraints for one selected style, or at most two explicit hybrid styles.
- `references/apx-json-v0.md`: APX/APXA schema details, color syntax, patch operation examples, PSD export notes, and animation rules.
- `references/schemas/apx-v0.schema.json`, `references/schemas/apx-patch-v0.schema.json`, `references/schemas/apxa-v0.schema.json`: machine-readable schema references.
