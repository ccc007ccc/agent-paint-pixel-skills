---
name: agent-paint-pixel-skills
description: Generate, edit, validate, render, visually inspect, animate, and export AgentPaint APX/APXA pixel-art source files with style-aware pixel-art guidance. Use when Codex needs to create layered pixel art at the exact requested resolution, create point-upscaled previews for AI visual inspection, create GIF animation from APXA frames, choose a pixel-art style or era, convert a pixel-art idea into APX JSON, repair APX validation errors, render APX to PNG, export layered PSD, export RGBA JSON, or make local pixel edits with rows/chunks instead of raw RGBA arrays.
---

# Agent Paint Pixel Skills

## Visual Inspection Gate

Any time you need to visually judge pixel art with `view_image`, screenshots, or another multimodal inspection path, first create a point-upscaled preview from the APX/APXA source and inspect that preview.

```bash
agentpaint supersample <file.apx> --out <file-preview.png>
agentpaint supersample-frame <file.apxa> --frame <index> --out <frame-preview.png>
```

Do not visually inspect the raw low-resolution render. The supersampled preview is only for AI inspection and must use integer nearest-neighbor scaling; it does not satisfy or change the requested artwork dimensions.

## Small Grid Mode

Use this mode for `16x16`, `24x24`, `32x32`, and other single-subject sprites up to `32x32`, unless the user explicitly asks for many Photoshop-style layers.

The goal is to reduce freedom and make the model reason about the whole sprite at once.

- Use one primary full-canvas `rows` layer named `paint`.
- For `24x24`, use no chunks in the first complete draft. Write exactly 24 row strings, each exactly 24 characters.
- Use a compact palette: `"."` is transparent, `"_"` is the reserved transparent/background placeholder, then use digit symbols `"0"`-`"9"` for 2-10 visible colors. For opaque sprites, set top-level `"background"` and use `_` for background pixels or full background rows. Do not use transparent gaps unless the user asked for transparency.
- Keep every visible pixel intentional. Do not leave holes in the silhouette, face, object body, or background.
- Build the sprite as one readable icon: large silhouette first, internal color/value blocks second, small highlights and accents last.
- Avoid decorative micro-noise. At this size, quality comes from clear contour, strong value grouping, and a few exact pixels for identity.
- Do not split a simple 24x24 or 32x32 sprite into many layers just because APX supports layers. Optional layers such as `shadow`, `outline`, or `highlight` are only useful after the single `paint` layer already reads well, or when PSD/layer control is requested.
- If the supersampled preview is not immediately readable as the subject, revise the full rows or local rows before adding more layers.

## Workflow

1. Work in the user's current workspace or the output directory they request.
2. Do not look for or require the AgentPaint source repository during normal skill use.
3. Use the installed `agentpaint` CLI from `PATH`. If `agentpaint --help` fails, ask the user to install the CLI instead of falling back to `cargo run`.
4. Create or edit an `.apx` JSON file for still art or an `.apxa` JSON file for animation rather than writing raw RGBA arrays.
5. For `32x32` and smaller single-subject sprites, apply Small Grid Mode before any style-specific layering.
6. Before drawing, infer or choose a style brief: style family, canvas, projection, palette size, outline policy, shading policy, texture policy, light direction, layers, and animation motion plan when relevant.
7. Read bundled `references/pixel-art-style-guide.md` when the user asks for a specific style, era, mood, genre, or when making a non-trivial sprite larger than `32x32`.
8. Use that guide as an index: infer the best style from the user's words, then load the matching bundled `references/styles/*.md` file. Load at most two style files for explicit hybrids.
9. Start the APX/APXA with `canvas`, `palette`, and a short layer plan.
10. Use full-canvas `rows` for 16x16, 24x24, and 32x32 sprites.
11. Use `chunks` for local edits, 64x64+ images, and high-detail layers.
12. Run validation before rendering:

```bash
agentpaint validate <file.apx>
```

13. If validation fails, repair only the invalid layer, row, chunk, or palette entry.
14. For local edits, prefer patch files and the `patch` command over rewriting the full APX:

```bash
agentpaint patch <input.apx> --patch <patch.json> --out <output.apx>
```

15. Adjust layer order with `move_layer` instead of rewriting the `layers` array:

```json
{ "op": "move_layer", "layer": "highlight", "before": "base" }
```

16. Render after validation when the user wants an image preview or export:

```bash
agentpaint render <file.apx> --out <file.png>
```

17. Before any AI visual inspection of pixel art, create and inspect a point-upscaled preview:

```bash
agentpaint supersample <file.apx> --out <file-preview.png>
agentpaint supersample-frame <file.apxa> --frame 0 --out <frame-preview.png>
```

Use the supersampled preview for `view_image` or any multimodal model inspection. Do not inspect the raw low-resolution PNG directly, because vision preprocessors can blur tiny pixel art through bilinear or bicubic resizing. `supersample` and `supersample-frame` use integer nearest-neighbor scaling only and default to the largest scale that fits within `1280x720`.

18. Export PSD when the user wants Photoshop-compatible layered output:

```bash
agentpaint export-psd <file.apx> --out <file.psd>
```

PSD export preserves APX `layer.name` values, including Unicode names, and keeps Photoshop-style top-to-bottom layer order. Palette alpha maps to the PSD layer transparency channel, APX `layer.opacity` maps to PSD layer opacity, and `visible: false` maps to a hidden PSD layer.

19. For animation, use APXA base layers plus per-frame patch operations. Validate, inspect, supersample one frame if visual inspection is useful, then render GIF:

```bash
agentpaint validate-animation <file.apxa>
agentpaint inspect-animation <file.apxa>
agentpaint render-frame <file.apxa> --frame 0 --out <frame.png>
agentpaint supersample-frame <file.apxa> --frame 0 --out <frame-preview.png>
agentpaint render-gif <file.apxa> --out <file.gif>
```

20. Export RGBA JSON only when specifically requested:

```bash
agentpaint export-rgba <file.apx> --out <file.rgba.json>
```

## Hard Output Rules

- Match requested dimensions exactly. If the user asks for `200x200`, the APX must use `"canvas": { "width": 200, "height": 200 }`.
- Do not draw at a smaller or larger resolution and resize it later.
- Do not use scaling, upscaling, downscaling, resampling, or image conversion to satisfy the requested dimensions.
- Do not write or run helper programs such as Python, JavaScript, shell scripts, or image-generation scripts to draw the APX art unless the user explicitly asks for programmatic generation or format conversion.
- Author the artwork through APX JSON: palette symbols, rows, chunks, layers, and patch files.
- For GIF animation, author the animation through APXA JSON: one exact-size base project plus exact-size frame operations. Do not generate a smaller animation and upscale it.
- Do not create animation by writing helper programs that output frames unless the user explicitly asks for programmatic animation generation.
- Do not visually inspect raw low-resolution pixel-art renders directly. If you need to judge the image, run `agentpaint supersample` or `agentpaint supersample-frame` and inspect the point-upscaled preview.
- For large artwork, use meaningful layers and hand-authored chunks for regions such as background, hair, face, clothing, eyes, highlights, shadows, props, and effects.
- Do not present large natural surfaces or large portraits as smooth flat blobs. Grass, hills, mountains, walls, hair, clothing, and large skin regions need visible material or facial-structure passes above their base color.
- When the user asks for PSD output, validate the APX and run `agentpaint export-psd`; do not substitute a flattened PNG or RGBA export.
- If a render looks weak, continue refining with APX edits or patch files before presenting it as complete.
- After rendering, inspect the APX/APXA or output dimensions when exact size matters:

```bash
agentpaint inspect <file.apx>
```

## APX Rules

- Use JSON with top-level `canvas`, `palette`, and `layers`.
- Use single-character palette keys.
- Always define `"."` as `"transparent"`.
- Do not define `"_"` in `palette`; `_` is reserved for transparent/background placeholder pixels.
- Without top-level `"background"`, `_` renders transparent. With top-level `"background": "<color>"`, `_` renders as that color.
- Use `"background"` when a non-transparent image has a broad base color. A full-canvas row that is exactly `"_"` expands to the full canvas width.
- In a chunk, a row that is exactly `"_"` expands to the chunk width inferred from another normal row in the same chunk. Do not make a chunk only from `"_"` rows.
- Prefer HEX colors such as `"#d93636"`; use `rgba(r,g,b,a)` only for intentional alpha.
- Keep every `rows` string exactly the canvas width for full-canvas rows.
- Keep full-canvas `rows` count exactly the canvas height.
- Keep every chunk rectangular and inside canvas bounds.
- Empty layers are valid; use them for placeholders or after `clear_layer`.
- Keep the `layers` array in Photoshop-style order: top/front layers first, bottom/back layers last.
- Remember that `layers[0]` is the visual top layer. Rendering composites from the bottom of the array upward.
- PSD export inherits these APX layer names and order, so choose meaningful `layer.name` values before exporting.
- Use layer names such as `shadow`, `outline`, `base`, `mid_shadow`, `highlight`, `eyes`, `details`, and `effects`.
- Do not explain the APX format inside generated `.apx` files.

## Large Canvas Construction

Use this workflow for still images at `64x64` and larger, and for any image that contains multiple objects, terrain, buildings, rooms, scenes, or large props.

Do not rely on visual self-judgment as the main quality test. Build and audit the APX source structure instead.

### Flat Area Discipline

Large same-color regions are allowed as base construction, but they are a quality risk when they become the finished image. This mainly affects backgrounds, grass, hills, mountains, water, walls, clothing, hair, and large portraits.

- Treat any broad single-color plane as a `base_planes`, `skin_base`, `sky_base`, `ground`, or `large_objects` layer, not as finished art.
- Add at least one visible detail pass over every broad material surface unless it is intentionally blank UI space, transparent space, or a tiny icon background.
- For grass and ground, add clustered blades, tufts, stones, flower dots, path edge breaks, darker under-object shadows, and 2-4 variant patches. Do not paste identical grass clusters unchanged.
- For hills and mountains, add ridge lines, slope shadow bands, contour breaks, rock cracks, snow/grass caps, tree dots, atmospheric bands, or dithered value transitions.
- For sky and distant background, flatness can be acceptable, but add horizon bands, clouds, stars, distant silhouettes, or value bands when the background occupies a large part of the canvas.
- For large portraits, avoid circle/ellipse-only construction. Skin may occupy a large same-color area, but the face must be broken by facial planes: brow shadow, nose bridge, cheek value, jaw/chin shadow, ears, hairline, neck shadow, and eye/mouth clusters. Use 2-4 skin values for a large portrait unless the requested style is intentionally flat.
- Hair and clothing should not be single filled shapes. Add locks, edge cuts, folds, seams, rim light, cast shadows, or material highlights.
- A big chunk of repeated symbols is acceptable only if a higher layer visibly changes that area with edge, texture, shadow, trim, highlight, or small props.
- If a supersampled preview reads as smooth vector art, blobs, or large pasted rectangles, patch the APX before presenting it.

Before writing APX:

- Divide the canvas into 4-8 named zones such as focal area, secondary area, route/floor/terrain, boundary, foreground accents, and empty/transparent space if the asset needs it.
- List 8-20 local features that should exist as separate readable pixel clusters. Examples of feature types are openings, seams, trim, stones, small props, surface marks, cast shadows, highlights, edge breaks, signs, handles, cracks, plants, sparkle points, or material bands.
- List broad surfaces that could become flat, such as sky, grass, hill, mountain, wall, hair, clothing, or skin, and name the layer that will break each surface with texture, value, edge, trim, or facial structure.
- Decide which broad surfaces are intentionally plain or transparent. Transparency is valid when it is part of the asset; do not fill transparent space merely to hide it.
- For repeated elements, plan named variants before drawing them. A repeated element should not be pasted unchanged unless it is a deliberate tile pattern.

When writing APX:

- Build in passes: `base_planes` or `ground`, then `large_objects`, then `edge_lines`, then `material_shadow`, then `surface_texture`, then `trim`, then `small_props`, then `details`, then `highlights/effects`.
- Large chunks are allowed only as base planes, silhouette blocks, or intentionally empty/transparent areas. Do not use a few large chunks as the finished artwork.
- Every major object or broad surface should have at least two supporting passes above its base: edge/shape definition plus one of texture, shadow, trim, highlight, or small props.
- Keep layer names tied to visual function and compositing order, not just object categories. Prefer names such as `edge_lines`, `cast_shadow`, `material_shadow`, `base_planes`, `surface_texture`, `trim`, `small_props`, `details`, `highlights`, and `effects`.
- For top-down scenes, keep functional zones readable: paths, water, walls, floor, vegetation, props, and walkable areas should not accidentally overlap unless the subject calls for it.
- For isometric scenes, split top, left, and right planes into separate layers and keep 2:1 diagonals consistent before adding decoration.

Before finalizing the APX source:

- Check the APX contains the planned local features as actual chunks or rows, not only as a description in text.
- Check repeated elements have variant chunks or different internal marks.
- Check large base chunks have higher-layer local detail unless they are intentionally plain or transparent.
- Check natural surfaces and portraits for excessive flat areas: grass, hills, mountains, walls, hair, clothing, and large skin regions should show material or facial structure above their base color.
- Do not reject a large same-color region solely because it is large. Judge whether it has a semantic reason and whether visible upper layers break it where pixel-art detail is expected.
- If the APX source reads like only a blockout, add another detail pass instead of presenting it.

## APXA Animation Rules

- Use JSON with top-level `canvas`, `palette`, `layers`, and `frames`.
- The APXA `layers` array is the base APX project. It uses the same Photoshop-style top-to-bottom order: `layers[0]` is visual top/front.
- Every frame starts from the same base layers and applies only that frame's `operations`.
- Frame `operations` use the same objects as APX patch files.
- Empty frame operations are valid for a hold pose or the first frame.
- Every frame renders at the exact APXA canvas size.
- Use `set_rows`, `clear_layer`, `add_chunk`, `set_layer_visibility`, `set_layer_opacity`, and `move_layer` for motion, squash/stretch, blinking, impact, and effects.
- Use `set_background` or `clear_background` when a frame needs to change how `_` resolves.
- Keep GIF palettes practical: aim for 256 colors or fewer and avoid relying on subtle alpha.

## Generation Pattern

For a new sprite:

1. Use the exact dimensions from the request. If unspecified, default to `16x16` for icons and `32x32` for character sprites.
2. If the sprite is `32x32` or smaller and has one main subject, use Small Grid Mode: one `paint` layer, full-canvas rows, compact digit palette, and no chunks for the first complete draft.
3. For `24x24`, treat the APX rows like a benchmark grid: 24 row strings, 24 symbols per row, 2-10 visible colors, background filled unless transparency is requested.
4. Choose a style family from `references/pixel-art-style-guide.md`, then read the matching detailed style file under `references/styles/` only when the request needs a named style, era, or genre constraint.
5. Build a small palette with `.` plus symbols that match the detailed style constraints or Small Grid Mode palette. Use top-level `background` plus `_` when the image should have a base background color.
6. Draw silhouette and major value clusters before details.
7. Add style-specific texture, dithering, anti-aliasing, or highlights only after the main form reads.
8. Add extra layers only when they improve compositing, animation, PSD export, or later edits. Do not add layers to small sprites by default.
9. For `64x64` and larger, apply Large Canvas Construction before validation.
10. Validate after writing the file.
11. Render to PNG if visual inspection is useful.
12. For AI visual inspection, run `agentpaint supersample <file.apx> --out <file-preview.png>` and inspect that preview, not the raw low-resolution PNG.

For a new animation:

1. Use `.apxa` and the exact requested dimensions. If unspecified, default to `16x16` for tiny sprite loops.
2. Choose the style as for a still sprite, then add a short motion plan: frame count, timing, contact poses, anticipation, peak, landing, and loop return.
3. Build base layers for stable parts and separate moving parts into layers such as `body`, `eyes`, `shadow`, `squash`, `stretch`, `effects`, and `highlights`.
4. Use `frames` with per-frame patch operations. Do not write separate full APX files for each frame unless the user asks for that export.
5. Validate with `agentpaint validate-animation <file.apxa>`.
6. Render at least one frame with `agentpaint render-frame` when checking a pose.
7. When visually checking a pose, inspect a supersampled frame preview from `agentpaint supersample-frame <file.apxa> --frame <index> --out <frame-preview.png>`, not the raw low-resolution frame PNG.
8. Render the GIF with `agentpaint render-gif <file.apxa> --out <file.gif>`.
9. If motion reads poorly, patch the APXA locally instead of replacing the whole animation.

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

Read bundled `references/pixel-art-style-guide.md` first for shared principles and the style index. Then read the selected file under bundled `references/styles/` for detailed style constraints. Read bundled `references/apx-json-v0.md` only when detailed schema examples are needed. Use bundled `references/schemas/apx-v0.schema.json`, `references/schemas/apx-patch-v0.schema.json`, and `references/schemas/apxa-v0.schema.json` as machine-readable references when schema guidance is needed.
