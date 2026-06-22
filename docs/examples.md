# AgentPaint Examples

The `examples/` directory contains APX source examples generated in separate Codex runs, APXA animation sources, plus patch examples for local edits and layer controls.

PNG previews, GIF previews, and patched/reordered APX outputs are generated artifacts and are ignored by git by default. Two README showcase assets are intentionally tracked:

- `examples/red-slime-jump-32-highres.gif`: nearest-neighbor `512x512` display version of the exact-size `32x32` slime animation.
- `examples/examples-showcase-zh.png`: overview image for the generated examples.

## Example Matrix

| File | Size | Style | Structure | Purpose |
| --- | ---: | --- | --- | --- |
| `one-bit-lighthouse-16.apx` | `16x16` | 1-bit monochrome | Full-canvas `rows` | Tiny icon, strict palette, simple top-to-bottom layers |
| `nes-ember-sword-24x16.apx` | `24x16` | NES-inspired 8-bit | Full-canvas `rows` | Non-square item/banner sprite with console-style palette limits |
| `horror-candle-32.apx` | `32x32` | Horror/dark fantasy | Mostly `chunks` | Medium prop with focal details, glow, shadow, and texture layers |
| `cyberpunk-noodle-stall-48x32.apx` | `48x32` | Cyberpunk neon | `chunks` | Wide scene composition with separate neon, glow, silhouette, and background layers |
| `dithered-train-window-128x64.apx` | `128x64` | Dithered poster | Chunk-heavy | Largest example; wide poster-like scene with sky bands, dither texture, frame, reflections, and foreground |
| `red-slime-jump-16.apxa` | `16x16` | Modern clean pixel icon | APXA base layers plus frame patch operations | Six-frame red slime jump loop with squash, rise, apex, fall, landing, and exact-size GIF export |
| `red-slime-jump-32.apxa` | `32x32` | Modern clean pixel icon | APXA base layers plus frame patch operations | Separately generated six-frame red slime jump animation with frame previews and exact-size GIF export |

All APX/APXA examples use Photoshop-style layer order: `layers[0]` is the visual top/front layer, and the last layer is the visual bottom/back layer.

## Patch Examples

| Patch | Base APX | Demonstrates |
| --- | --- | --- |
| `horror-candle-local.patch.json` | `horror-candle-32.apx` | `set_palette` and `add_chunk` for local color/detail edits |

Patch outputs used during verification:

```bash
agentpaint patch examples/horror-candle-32.apx --patch examples/horror-candle-local.patch.json --out examples/horror-candle-32-patched.apx
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

Validate, inspect, preview, and render APXA animation:

```bash
agentpaint validate-animation examples/red-slime-jump-16.apxa
agentpaint inspect-animation examples/red-slime-jump-16.apxa
agentpaint render-frame examples/red-slime-jump-16.apxa --frame 3 --out examples/red-slime-jump-16-frame3.png
agentpaint render-gif examples/red-slime-jump-16.apxa --out examples/red-slime-jump-16.gif
agentpaint validate-animation examples/red-slime-jump-32.apxa
agentpaint render-frame examples/red-slime-jump-32.apxa --frame 3 --out examples/red-slime-jump-32-apex.png
agentpaint render-gif examples/red-slime-jump-32.apxa --out examples/red-slime-jump-32.gif
```

Create the README display GIF from the exact-size render:

```bash
ffmpeg -y -i examples/red-slime-jump-32.gif -vf "scale=512:512:flags=neighbor" -loop 0 examples/red-slime-jump-32-highres.gif
```
