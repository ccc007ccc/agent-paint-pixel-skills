# AgentPaint CLI + Skill Spec

## Checklist

- [x] Initialize the `AgentPaint` git repository and add a Rust/generated-artifact `.gitignore`.
- [x] Create a Rust CLI crate with the binary command `agentpaint`.
- [x] Define APX JSON v0 with `canvas`, `palette`, `layers`, `rows`, and `chunks`.
- [x] Add the reserved `_` transparent/background placeholder and top-level `background` color.
- [x] Implement `validate` for dimensions, row width, palette symbols, colors, bounds, and opacity.
- [x] Implement `render` to composite layers and export PNG.
- [x] Implement `import-image` and `import-gif` for raster-to-APX/APXA format conversion.
- [x] Implement `supersample` for point-upscaled, AI-readable visual previews.
- [x] Implement `inspect` and `export-rgba` for agent/script-friendly inspection.
- [x] Implement `export-psd` for layered APX to PSD export.
- [x] Use Photoshop-style top-to-bottom layer ordering, with `layers[0]` as the visual top layer.
- [x] Add focused unit tests for validation, rendering, patching, and layer behavior.
- [x] Regenerate the example suite after consistency checks.
- [x] Add patch operations for layer insertion, renaming, visibility, opacity, movement, clearing, and removal.
- [x] Define APXA animation v0 as APX base layers plus per-frame patch operations.
- [x] Implement `validate-animation`, `inspect-animation`, `render-frame`, and `render-gif`.
- [x] Create an installable cross-platform Agent Skill that tells LLMs how to generate layered APX pixel art with the installed `agentpaint` CLI.
- [x] Add Codex and Claude plugin metadata plus multi-target skill install scripts.
- [x] Verify with `cargo test`, `validate`, `render`, `supersample`, `export-rgba`, and `export-psd`.

## APX JSON v0

APX is the source format the LLM writes. The CLI validates and renders it.

```json
{
  "canvas": { "width": 4, "height": 4 },
  "background": "#dfe8c8",
  "palette": {
    ".": "transparent",
    "K": "#171717",
    "R": "#d93636",
    "W": "#ffffff",
    "S": "rgba(0,0,0,0.35)"
  },
  "layers": [
    {
      "name": "eyes",
      "visible": true,
      "opacity": 1,
      "chunks": [
        {
          "x": 1,
          "y": 1,
          "rows": [
            "W.W",
            "K.K"
          ]
        }
      ]
    },
    {
      "name": "base",
      "visible": true,
      "opacity": 1,
      "rows": [
        "....",
        ".RR.",
        ".RR.",
        "...."
      ]
    }
  ]
}
```

Rules:

- Palette keys must be single characters.
- `.` should be present and transparent.
- `_` is reserved and must not be used as a palette key.
- If `background` is omitted, `_` renders transparent. If `background` is set, `_` renders as that color.
- A full-canvas row exactly equal to `"_"` expands to the full canvas width. A chunk row exactly equal to `"_"` expands to the chunk width inferred from another normal row in the same chunk.
- Layer `visible` defaults to `true`.
- Layer `opacity` defaults to `1`.
- A layer can use full-canvas `rows`, `chunks`, both, or neither when it is an empty transparent layer.
- Full-canvas `rows` must match canvas height and width.
- Chunk rows can be rectangular but must remain inside the canvas.
- `layers[0]` is the visual top layer, matching Photoshop's layer panel.
- Rendering composites from the bottom of the layer array upward.

## CLI Commands

```txt
agentpaint validate <input.apx>
agentpaint inspect <input.apx>
agentpaint import-image <input.png> --out <output.apx>
agentpaint import-image <input.png> --out <output.apx> --layer-name paint
agentpaint render <input.apx> --out <output.png>
agentpaint supersample <input.apx> --out <preview.png>
agentpaint supersample <input.apx> --out <preview.png> --target-width 1280 --target-height 720
agentpaint supersample <input.apx> --out <preview.png> --scale <integer>
agentpaint patch <input.apx> --patch <patch.json> --out <output.apx>
agentpaint patch <input.apx> --patch <patch.json> --in-place
agentpaint export-rgba <input.apx> --out <output.json>
agentpaint export-psd <input.apx> --out <output.psd>
agentpaint validate-animation <input.apxa>
agentpaint import-gif <input.gif> --out <output.apxa>
agentpaint import-gif <input.gif> --out <output.apxa> --layer-name paint
agentpaint inspect-animation <input.apxa>
agentpaint render-frame <input.apxa> --frame <index> --out <output.png>
agentpaint supersample-frame <input.apxa> --frame <index> --out <preview.png>
agentpaint render-gif <input.apxa> --out <output.gif>
```

`import-image` decodes an existing raster image and writes a valid single-layer APX project at the source dimensions. Fully transparent pixels become `.`, visible unique RGBA colors receive automatically assigned single-character palette symbols, and the layer defaults to `paint` unless `--layer-name` is provided.

`import-gif` decodes an existing GIF into APXA at the source frame dimensions. The first GIF frame becomes the base `paint` layer; later frames become `set_rows` frame operations; frame durations are preserved in `duration_ms`. It does not scale, crop, or resample frames.

`supersample` renders the APX and writes a point-upscaled PNG for AI visual inspection. `supersample-frame` does the same for one APXA frame. Both commands use integer nearest-neighbor scaling only. With no `--scale`, they choose the largest integer scale that fits within `--target-width` x `--target-height`, defaulting to `1280x720`.

`export-psd` writes an 8-bit RGB PSD with transparency, a flattened preview, and one PSD layer for each APX layer. APX layer names are preserved in the PSD, including Unicode names, and layer order remains Photoshop-style top-to-bottom. Palette alpha maps to the PSD layer transparency channel, APX `layer.opacity` maps to PSD layer opacity, and `visible: false` maps to a hidden PSD layer.

`render-gif` preserves source frame dimensions and timing, but GIF encoding cannot preserve partial alpha. Fully transparent pixels stay transparent; semi-transparent pixels such as soft shadows are quantized by the encoder.

## APXA Animation v0

APXA is the animation source format. It uses the same `canvas`, `palette`, and Photoshop-style `layers` as APX, then adds a `frames` array. Each frame starts from the base layers and applies its own `operations` array using the same operation objects as APX patch files. Empty frame operations are valid for a hold or first pose.

```json
{
  "canvas": { "width": 16, "height": 16 },
  "palette": {
    ".": "transparent",
    "R": "#d93636"
  },
  "layers": [
    {
      "name": "body",
      "rows": [
        "................",
        "......RRRR......"
      ]
    }
  ],
  "frames": [
    { "name": "idle", "duration_ms": 120, "operations": [] },
    {
      "name": "jump",
      "duration_ms": 90,
      "operations": [
        { "op": "clear_layer", "layer": "body" },
        {
          "op": "add_chunk",
          "layer": "body",
          "x": 5,
          "y": 4,
          "rows": ["RRRRRR"]
        }
      ]
    }
  ]
}
```

Animation rules:

- Every rendered frame uses the exact APXA canvas dimensions.
- GIF export must not scale, resize, crop, or resample frames.
- GIF export cannot preserve partial alpha; avoid soft transparency when GIF is the final target.
- Layer order remains top-to-bottom: `layers[0]` is the visual top/front layer.
- Use frame operations for motion, squash/stretch, blinking, effects, and layer visibility changes.
- Keep the palette GIF-friendly; 256 colors or fewer is the practical target.

## Skill Goal

The installed skill should make compatible CLI agents generate APX source files instead of raw RGBA arrays. It should be usable from any user workspace after installation and should not require the AgentPaint source repository at runtime. The skill should instruct the agent to:

- start with canvas, palette, and layer plan;
- match the requested output dimensions exactly in the APX canvas;
- avoid resizing or scaling as a substitute for drawing at the requested size;
- avoid helper scripts for drawing unless the user explicitly asks for programmatic generation;
- use short palette symbols and `.` for transparency;
- use full rows for small sprites;
- use chunks for larger images or local edits;
- run `agentpaint validate` before rendering;
- use `agentpaint patch` for local edits;
- use `background` plus reserved `_` for opaque base backgrounds or transparent placeholders;
- use APXA and `agentpaint render-gif` for GIF animation;
- repair only the invalid layer/chunk when validation fails.
