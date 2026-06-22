# AgentPaint CLI + Skill Spec

## Checklist

- [x] Initialize the `AgentPaint` git repository and add a Rust/generated-artifact `.gitignore`.
- [x] Create a Rust CLI crate with the binary command `agentpaint`.
- [x] Define APX JSON v0 with `canvas`, `palette`, `layers`, `rows`, and `chunks`.
- [x] Implement `validate` for dimensions, row width, palette symbols, colors, bounds, and opacity.
- [x] Implement `render` to composite layers and export PNG.
- [x] Implement `inspect` and `export-rgba` for agent/script-friendly inspection.
- [x] Use Photoshop-style top-to-bottom layer ordering, with `layers[0]` as the visual top layer.
- [x] Add focused unit tests for validation, rendering, patching, and layer behavior.
- [x] Regenerate the example suite after consistency checks.
- [x] Add patch operations for layer insertion, renaming, visibility, opacity, movement, clearing, and removal.
- [x] Create an installable Codex Skill that tells LLMs how to generate layered APX pixel art with the installed `agentpaint` CLI.
- [x] Verify with `cargo test`, `validate`, `render`, and `export-rgba`.

## APX JSON v0

APX is the source format the LLM writes. The CLI validates and renders it.

```json
{
  "canvas": { "width": 4, "height": 4 },
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
agentpaint render <input.apx> --out <output.png>
agentpaint patch <input.apx> --patch <patch.json> --out <output.apx>
agentpaint patch <input.apx> --patch <patch.json> --in-place
agentpaint export-rgba <input.apx> --out <output.json>
```

## Skill Goal

The installed skill should make Codex generate APX source files instead of raw RGBA arrays. It should be usable from any user workspace after installation and should not require the AgentPaint source repository at runtime. The skill should instruct Codex to:

- start with canvas, palette, and layer plan;
- match the requested output dimensions exactly in the APX canvas;
- avoid resizing or scaling as a substitute for drawing at the requested size;
- avoid helper scripts for drawing unless the user explicitly asks for programmatic generation;
- use short palette symbols and `.` for transparency;
- use full rows for small sprites;
- use chunks for larger images or local edits;
- run `agentpaint validate` before rendering;
- use `agentpaint patch` for local edits;
- repair only the invalid layer/chunk when validation fails.
