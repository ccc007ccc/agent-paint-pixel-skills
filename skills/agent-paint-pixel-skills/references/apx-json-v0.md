# APX JSON v0

Minimal project:

```json
{
  "canvas": { "width": 16, "height": 16 },
  "palette": {
    ".": "transparent",
    "K": "#171717",
    "R": "#d93636",
    "W": "#ffffff"
  },
  "layers": [
    {
      "name": "outline",
      "rows": [
        "................",
        ".....KKKKKK.....",
        "....K......K....",
        ".....KKKKKK.....",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................"
      ]
    }
  ]
}
```

Chunk-only layer:

```json
{
  "name": "eyes",
  "chunks": [
    {
      "x": 5,
      "y": 6,
      "rows": [
        "W..W",
        "K..K"
      ]
    }
  ]
}
```

Empty transparent layer:

```json
{
  "name": "paint_over"
}
```

Supported color values:

- `"transparent"`
- `"#rgb"`
- `"#rrggbb"`
- `"#rrggbbaa"`
- `"rgb(255,0,0)"`
- `"rgba(0,0,0,0.35)"`

Background placeholder:

- `.` is the normal transparent palette symbol and must remain `"transparent"`.
- `_` is reserved. Do not define `_` in `palette`.
- Without top-level `"background"`, `_` renders transparent.
- With top-level `"background": "#rrggbb"` or another supported color, `_` renders as that background color.
- In full-canvas `rows`, a row that is exactly `"_"` expands to the full canvas width.
- In `chunks`, a row that is exactly `"_"` expands to the chunk width inferred from another normal row in the same chunk. A chunk made only of `"_"` rows is invalid because its width is unknown.

Example with an opaque background:

```json
{
  "canvas": { "width": 4, "height": 4 },
  "background": "#dfe8c8",
  "palette": {
    ".": "transparent",
    "R": "#d93636",
    "K": "#171717"
  },
  "layers": [
    {
      "name": "paint",
      "rows": [
        "_",
        "_RR_",
        "_KK_",
        "_"
      ]
    }
  ]
}
```

Patch file:

```json
{
  "operations": [
    { "op": "set_palette", "symbol": "H", "color": "#ffffff" },
    { "op": "set_background", "color": "#dfe8c8" },
    {
      "op": "add_chunk",
      "layer": "eyes",
      "x": 5,
      "y": 6,
      "rows": ["H..H"]
    },
    {
      "op": "move_layer",
      "layer": "highlight",
      "before": "base"
    }
  ]
}
```

Patch operations:

- `set_palette`
- `set_background`
- `clear_background`
- `add_layer`
- `insert_layer`
- `replace_layer`
- `rename_layer`
- `remove_layer`
- `set_layer_visibility`
- `set_layer_opacity`
- `set_rows`
- `add_chunk`
- `move_layer`
- `clear_layer`

Animation file:

```json
{
  "canvas": { "width": 16, "height": 16 },
  "background": "#e7f0d0",
  "palette": {
    ".": "transparent",
    "K": "#171717",
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
    {
      "name": "idle",
      "duration_ms": 120,
      "operations": []
    },
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

- `.apxa` uses APX `canvas`, `palette`, and base `layers`, plus `frames`.
- `.apxa` can also use APX `"background"`; `_` in base layers or frame patch rows resolves through that background unless a frame patch changes it.
- Every frame starts from the same base layers and applies only that frame's `operations`.
- Empty frame operations are valid for a hold pose.
- Render animations with `agentpaint render-gif <file.apxa> --out <file.gif>`.
- Render one frame for inspection with `agentpaint render-frame <file.apxa> --frame 0 --out <file.png>`.
- For AI visual inspection, use `agentpaint supersample-frame <file.apxa> --frame 0 --out <file-preview.png>` and inspect the preview, not the raw low-resolution frame.
- Do not scale or resize frames; canvas size is the GIF frame size.

AI visual inspection:

- Do not inspect raw low-resolution pixel-art renders directly with a multimodal model.
- Use `agentpaint supersample <file.apx> --out <file-preview.png>` for still APX files.
- Use `agentpaint supersample-frame <file.apxa> --frame <index> --out <file-preview.png>` for APXA animation frames.
- Supersampled previews use integer nearest-neighbor scaling only and default to the largest scale fitting within `1280x720`.

PSD export:

- Export layered PSD with `agentpaint export-psd <file.apx> --out <file.psd>`.
- PSD layer names inherit APX `layer.name` values, including Unicode names.
- PSD layer order inherits the APX `layers` array: top/front first, bottom/back last.
- Palette alpha maps to the PSD layer transparency channel, and APX `layer.opacity` maps to PSD layer opacity.
- APX `visible: false` exports as a hidden PSD layer; default/true visibility exports as a visible PSD layer.

Layer order:

- APX stores layers in Photoshop-style top-to-bottom order.
- `layers[0]` is the visual top/front layer.
- The last layer is the visual bottom/back layer.
- Rendering composites from the bottom of the array upward.
- Put `details`, `effects`, `eyes`, and tiny highlights near the top.
- Put `outline`, material color, and `base` in the middle.
- Put `shadow` and `background` near the bottom.
- Empty layers are valid and render as transparent.

Move layer examples:

```json
{ "op": "move_layer", "layer": "details", "to_index": 0 }
{ "op": "move_layer", "layer": "highlight", "before": "base" }
{ "op": "move_layer", "layer": "shadow", "after": "base" }
```

Layer edit examples:

```json
{ "op": "add_layer", "layer": { "name": "sparkle", "chunks": [{ "x": 8, "y": 3, "rows": ["H"] }] } }
{ "op": "insert_layer", "layer": { "name": "background", "chunks": [{ "x": 0, "y": 15, "rows": ["KKKKKKKKKKKKKKKK"] }] }, "after": "shadow" }
{ "op": "rename_layer", "layer": "shadow", "name": "cast_shadow" }
{ "op": "set_layer_visibility", "layer": "cast_shadow", "visible": false }
{ "op": "set_layer_opacity", "layer": "glow", "opacity": 0.45 }
{ "op": "clear_layer", "layer": "paint_over" }
```
