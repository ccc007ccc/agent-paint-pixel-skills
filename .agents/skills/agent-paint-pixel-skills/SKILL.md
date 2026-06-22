---
name: agent-paint-pixel-skills
description: Generate, edit, validate, and render AgentPaint APX pixel-art source files with style-aware pixel-art guidance. Use when Codex needs to create layered pixel art at the exact requested resolution, choose a pixel-art style or era, convert a pixel-art idea into APX JSON, repair APX validation errors, render APX to PNG, export RGBA JSON, or make local pixel edits with rows/chunks instead of raw RGBA arrays.
---

# Agent Paint Pixel Skills

## Workflow

1. Work in the user's current workspace or the output directory they request.
2. Do not look for or require the AgentPaint source repository during normal skill use.
3. Use the installed `agentpaint` CLI from `PATH`. If `agentpaint --help` fails, ask the user to install the CLI instead of falling back to `cargo run`.
4. Create or edit an `.apx` JSON file rather than writing raw RGBA arrays.
5. Before drawing, infer or choose a style brief: style family, canvas, projection, palette size, outline policy, shading policy, texture policy, light direction, and layers.
6. Read bundled `references/pixel-art-style-guide.md` when the user asks for a specific style, era, mood, genre, or when making a new non-trivial sprite.
7. Use that guide as an index: infer the best style from the user's words, then load the matching bundled `references/styles/*.md` file. Load at most two style files for explicit hybrids.
8. Start the APX with `canvas`, `palette`, and a short layer plan.
9. Use full-canvas `rows` for 16x16 and 32x32 sprites.
10. Use `chunks` for local edits, 64x64+ images, and high-detail layers.
11. Run validation before rendering:

```bash
agentpaint validate <file.apx>
```

12. If validation fails, repair only the invalid layer, row, chunk, or palette entry.
13. For local edits, prefer patch files and the `patch` command over rewriting the full APX:

```bash
agentpaint patch <input.apx> --patch <patch.json> --out <output.apx>
```

14. Adjust layer order with `move_layer` instead of rewriting the `layers` array:

```json
{ "op": "move_layer", "layer": "highlight", "before": "base" }
```

15. Render after validation when the user wants an image preview or export:

```bash
agentpaint render <file.apx> --out <file.png>
```

16. Export RGBA JSON only when specifically requested:

```bash
agentpaint export-rgba <file.apx> --out <file.rgba.json>
```

## Hard Output Rules

- Match requested dimensions exactly. If the user asks for `200x200`, the APX must use `"canvas": { "width": 200, "height": 200 }`.
- Do not draw at a smaller or larger resolution and resize it later.
- Do not use scaling, upscaling, downscaling, resampling, or image conversion to satisfy the requested dimensions.
- Do not write or run helper programs such as Python, JavaScript, shell scripts, or image-generation scripts to draw the APX art unless the user explicitly asks for programmatic generation or format conversion.
- Author the artwork through APX JSON: palette symbols, rows, chunks, layers, and patch files.
- For large artwork, use meaningful layers and hand-authored chunks for regions such as background, hair, face, clothing, eyes, highlights, shadows, props, and effects.
- If a render looks weak, continue refining with APX edits or patch files before presenting it as complete.
- After rendering, inspect the APX or output dimensions when exact size matters:

```bash
agentpaint inspect <file.apx>
```

## APX Rules

- Use JSON with top-level `canvas`, `palette`, and `layers`.
- Use single-character palette keys.
- Always define `"."` as `"transparent"`.
- Prefer HEX colors such as `"#d93636"`; use `rgba(r,g,b,a)` only for intentional alpha.
- Keep every `rows` string exactly the canvas width for full-canvas rows.
- Keep full-canvas `rows` count exactly the canvas height.
- Keep every chunk rectangular and inside canvas bounds.
- Empty layers are valid; use them for placeholders or after `clear_layer`.
- Keep the `layers` array in Photoshop-style order: top/front layers first, bottom/back layers last.
- Remember that `layers[0]` is the visual top layer. Rendering composites from the bottom of the array upward.
- Use layer names such as `shadow`, `outline`, `base`, `mid_shadow`, `highlight`, `eyes`, `details`, and `effects`.
- Do not explain the APX format inside generated `.apx` files.

## Generation Pattern

For a new sprite:

1. Use the exact dimensions from the request. If unspecified, default to `16x16` for icons and `32x32` for character sprites.
2. Choose a style family from `references/pixel-art-style-guide.md`, then read the matching detailed style file under `references/styles/`.
3. Build a small palette with `.` plus symbols that match the detailed style constraints.
4. Create layers in the order recommended by the detailed style file.
5. Draw silhouette and major value clusters before details.
6. Add style-specific texture, dithering, anti-aliasing, or highlights only after the main form reads.
7. Validate after writing the file.
8. Render to PNG if visual inspection is useful.

For an edit:

1. Inspect the existing APX first.
2. Write a patch JSON with `set_palette`, `add_chunk`, `set_rows`, `replace_layer`, `add_layer`, `insert_layer`, `rename_layer`, `remove_layer`, `set_layer_visibility`, `set_layer_opacity`, `move_layer`, or `clear_layer`.
3. Apply it with `agentpaint patch`.
4. Validate and render after the edit.

For layer-order edits:

1. Inspect the current layer order with `agentpaint inspect <file.apx>`.
2. Treat the listed order as top-to-bottom; `to_index: 0` means visual top. Inspect chunks include `x`, `y`, `width`, and `height`.
3. Use `move_layer` with exactly one of `to_index`, `before`, or `after`.
4. Prefer semantic targets, such as `{ "op": "move_layer", "layer": "effects", "before": "lighting" }` when effects should sit above lighting.
5. Use `set_layer_visibility` and `set_layer_opacity` for quick layer trials instead of deleting work.
6. Validate and render after the move.

For a validation error:

1. Read the exact row, layer, or chunk named by the CLI.
2. Fix the smallest possible JSON fragment.
3. Re-run validation.

## Style Guidance

Use style constraints as hard authoring constraints, not decorative labels.

- For 1-bit or Game Boy styles, keep palette extremely small and rely on value, silhouette, and dithering.
- For NES-inspired styles, use strong readable outlines, flat/cel shading, and small sub-palettes per object.
- For C64-inspired styles, allow visible chunkiness and deliberate dithering.
- For 16-bit or arcade styles, allow richer ramps and selective highlights, but keep pixel clusters clean.
- For isometric styles, enforce 2:1 diagonals, equal foreshortening, and separate planar faces into layers.
- For modern clean icons, reduce detail and avoid noisy dithering.

Patch style drift instead of rewriting the whole file. Examples: reduce colors for Game Boy, strengthen outline for NES, remove random noise, replace dither chunks, or fix isometric diagonals.

## Reference

Read bundled `references/pixel-art-style-guide.md` first for shared principles and the style index. Then read the selected file under bundled `references/styles/` for detailed style constraints. Read bundled `references/apx-json-v0.md` only when detailed schema examples are needed. Use bundled `references/schemas/apx-v0.schema.json` and `references/schemas/apx-patch-v0.schema.json` as machine-readable references when schema guidance is needed.
