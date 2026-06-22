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

Patch file:

```json
{
  "operations": [
    { "op": "set_palette", "symbol": "H", "color": "#ffffff" },
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
