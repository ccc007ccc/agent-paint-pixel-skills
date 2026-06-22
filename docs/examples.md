# AgentPaint Examples

The `examples/` directory contains APX source examples generated in separate Codex runs, plus patch examples for local edits and layer controls.

PNG previews and patched/reordered APX outputs are generated artifacts and are ignored by git.

## Example Matrix

| File | Size | Style | Structure | Purpose |
| --- | ---: | --- | --- | --- |
| `one-bit-lighthouse-16.apx` | `16x16` | 1-bit monochrome | Full-canvas `rows` | Tiny icon, strict palette, simple top-to-bottom layers |
| `nes-ember-sword-24x16.apx` | `24x16` | NES-inspired 8-bit | Full-canvas `rows` | Non-square item/banner sprite with console-style palette limits |
| `horror-candle-32.apx` | `32x32` | Horror/dark fantasy | Mostly `chunks` | Medium prop with focal details, glow, shadow, and texture layers |
| `cyberpunk-noodle-stall-48x32.apx` | `48x32` | Cyberpunk neon | `chunks` | Wide scene composition with separate neon, glow, silhouette, and background layers |
| `isometric-rooftop-garden-64.apx` | `64x64` | Isometric | Chunk-heavy | Multi-plane isometric tile with roof faces, planters, glass, details, and shadow |
| `topdown-ruins-96x64.apx` | `96x64` | Top-down RPG | Chunk-heavy | Larger terrain vignette with floor, walls, moss, debris, grass, and treasure layers |
| `dithered-train-window-128x64.apx` | `128x64` | Dithered poster | Chunk-heavy | Largest example; wide poster-like scene with sky bands, dither texture, frame, reflections, and foreground |

All APX examples use Photoshop-style layer order: `layers[0]` is the visual top/front layer, and the last layer is the visual bottom/back layer.

## Patch Examples

| Patch | Base APX | Demonstrates |
| --- | --- | --- |
| `horror-candle-local.patch.json` | `horror-candle-32.apx` | `set_palette` and `add_chunk` for local color/detail edits |
| `isometric-rooftop-layer-control.patch.json` | `isometric-rooftop-garden-64.apx` | `insert_layer`, `move_layer`, `rename_layer`, `set_layer_visibility`, and `set_layer_opacity` |

Patch outputs used during verification:

```bash
agentpaint patch examples/horror-candle-32.apx --patch examples/horror-candle-local.patch.json --out examples/horror-candle-32-patched.apx
agentpaint patch examples/isometric-rooftop-garden-64.apx --patch examples/isometric-rooftop-layer-control.patch.json --out examples/isometric-rooftop-garden-64-reordered.apx
```

## Validation Commands

Validate and inspect every APX source:

```bash
agentpaint validate examples/<file>.apx
agentpaint inspect examples/<file>.apx
```

Render previews when visual inspection is needed:

```bash
agentpaint render examples/<file>.apx --out examples/<file>.png
```
