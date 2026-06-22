# AgentPaint

AgentPaint is a small Rust CLI and Codex Skill for LLM-friendly pixel art.

Instead of asking an LLM to write raw RGBA arrays, AgentPaint uses APX JSON:

- HEX/RGB palette symbols.
- Text rows where one character equals one pixel.
- Photoshop-style top-to-bottom layers and local chunks for complex sprites.
- CLI validation, rendering, patching, and RGBA export.

## Quick Start

After installing, use the CLI from any workspace:

```bash
agentpaint --help
agentpaint validate <file.apx>
agentpaint inspect <file.apx>
agentpaint render <file.apx> --out <file.png>
agentpaint export-rgba <file.apx> --out <file.rgba.json>
```

Apply a local edit:

```bash
agentpaint patch <file.apx> --patch <patch.json> --out <patched.apx>
agentpaint render <patched.apx> --out <patched.png>
```

Adjust layer order without rewriting the APX:

```bash
agentpaint patch <file.apx> --patch <layer-order.patch.json> --out <reordered.apx>
```

Layer list order matches Photoshop: `layers[0]` is the visual top/front layer, and the last layer is the visual bottom/back layer.

## Examples

See [docs/examples.md](docs/examples.md) for generated APX examples across multiple styles, aspect ratios, and resolutions, including a `128x64` dithered-poster scene and patch examples for local edits and layer controls.

## Install

See [docs/install.md](docs/install.md).

Windows PowerShell:

```powershell
.\scripts\install.ps1 -UpdatePath
```

macOS/Linux:

```bash
sh ./scripts/install.sh --update-path
```

After install:

```bash
agentpaint --help
```

## APX Format

```json
{
  "canvas": { "width": 16, "height": 16 },
  "palette": {
    ".": "transparent",
    "K": "#171717",
    "R": "#d93636"
  },
  "layers": [
    {
      "name": "base",
      "rows": [
        "................",
        ".....RRRR......."
      ]
    }
  ]
}
```

Full schema files:

- [schemas/apx-v0.schema.json](schemas/apx-v0.schema.json)
- [schemas/apx-patch-v0.schema.json](schemas/apx-patch-v0.schema.json)

## Pixel Art Style Guidance

The Codex Skill includes style-aware guidance for 1-bit, Game Boy, NES-inspired, C64-inspired, 16-bit, arcade, top-down RPG, isometric, cozy indie, cyberpunk, horror, modern icon, portrait, and dithered-poster styles.

- Research notes: [docs/pixel-art-style-research.md](docs/pixel-art-style-research.md)
- Style index and shared rules: [.agents/skills/agent-paint-pixel-skills/references/pixel-art-style-guide.md](.agents/skills/agent-paint-pixel-skills/references/pixel-art-style-guide.md)
- Detailed style files: [.agents/skills/agent-paint-pixel-skills/references/styles](.agents/skills/agent-paint-pixel-skills/references/styles)

## Codex Skill

The source copy of the skill lives at:

```text
.agents/skills/agent-paint-pixel-skills
```

Install it globally with the install script or copy it to:

```text
$HOME/.agents/skills/agent-paint-pixel-skills
```

Then invoke it from Codex with:

```text
Use $agent-paint-pixel-skills to generate a 32x32 layered APX sprite and render it.
```

The installed skill does not require the AgentPaint source repository at runtime. It writes APX files in the user's current workspace and calls the installed `agentpaint` CLI from `PATH`.

The skill is expected to honor requested image dimensions exactly. A `200x200` request must produce a `200x200` APX canvas and a `200x200` render. It should not draw a smaller image and resize it, and it should not write helper scripts to generate the artwork unless the user explicitly asks for programmatic generation.

## Development

When developing the CLI from this source repository, `cargo run -- <command>` is equivalent to running the installed `agentpaint` binary:

```bash
cargo run -- validate examples/one-bit-lighthouse-16.apx
```
