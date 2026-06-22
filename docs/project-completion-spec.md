# AgentPaint Project Completion Spec

## Current State

AgentPaint has a working v0.2 CLI foundation:

- Rust CLI crate named `agentpaint`.
- APX JSON source format with canvas, palette, layers, full rows, and chunks.
- Photoshop-style top-to-bottom layer ordering, where `layers[0]` is the visual top layer.
- `validate`, `inspect`, `render`, `patch`, and `export-rgba` commands.
- APX project and APX patch JSON Schema files.
- Unit tests and a regenerated style/resolution example matrix.
- Installable Codex Skill for APX pixel-art generation and patch iteration.
- Install documentation and install scripts.

## v0.2 Completion Checklist

- [x] Add machine-readable JSON Schema for APX project files.
- [x] Add machine-readable JSON Schema for APX patch files.
- [x] Implement `agentpaint patch` for safe local APX edits.
- [x] Add layer patch operations for insert, rename, visibility, opacity, move, clear, and removal.
- [x] Add patch tests covering palette, layer, and chunk edits.
- [x] Regenerate examples with varied styles, aspect ratios, resolutions, and layer-control patches.
- [x] Add install scripts for Windows PowerShell and POSIX shells.
- [x] Add a concise README with usage, format, and install entry points.
- [x] Update the Codex Skill to use the installed `agentpaint` command, the new `patch` command, and bundled schema files.
- [x] Run `cargo fmt`, `cargo test`, schema-adjacent CLI checks, and skill validation.

## v0.2 Patch Format

Patch files are JSON documents:

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
    }
  ]
}
```

Required behavior:

- `agentpaint patch <input.apx> --patch <patch.json> --out <output.apx>` writes a new file.
- `agentpaint patch <input.apx> --patch <patch.json> --in-place` updates the input file.
- The command must reject using both `--out` and `--in-place`.
- The command must reject missing output mode.
- The command must validate the patched project before writing.
- Patch operations must address layers by name.
- Empty layers must be valid so `clear_layer` can leave a transparent layer behind.

Initial operations:

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

## v0.3+ Backlog

- MCP server exposing validate/render/patch tools.
- Web UI for preview, layer visibility, zoom, and manual pixel edits.
- APX versioning field and migration command.
- Animated frame support.
- Layer groups and masks.
- Release packaging.
