# Pixel Art Style Guide For AgentPaint

Use this file as the style router. It gives shared pixel-art principles and points to the detailed style files under `references/styles/`.

When the user does not name a style, infer one from the subject, mood, resolution, and requested use. Load only the detailed style file that best matches the request. If the result is a hybrid, load at most two style files and state the combined constraints in your private style brief.

## Shared Pixel-Art Principles

- Start from constraints: canvas size, palette size, projection, outline policy, shading policy, texture policy, and light direction.
- Make the silhouette readable before adding internal detail.
- Use clusters and value bands. Avoid random single pixels unless they serve eyes, sparkle, grit, stars, or a deliberate texture.
- Use dithering only for limited-color gradients, material texture, or historical display flavor.
- Use manual anti-aliasing only on focal curves and shallow diagonals. Do not blur every edge.
- Keep palette ramps coherent: dark, mid, light, and highlight for each material.
- For tiny sprites, prioritize iconic shape and contrast over literal detail.
- For 64x64+ work, prefer chunk-heavy layers and local edits.
- Validate and render before judging style quality.
- Patch style drift locally: palette, outline, dither chunks, layer order, or details.

## Style Selection Workflow

1. Identify subject type: icon, item, character, tileset, portrait, scene, UI glyph, or environment prop.
2. Infer mood and era from the user's wording.
3. Select one style file from the index. Use two only for explicit hybrids, such as "cozy Game Boy" or "cyberpunk isometric".
4. Build a style brief before writing APX:

```text
style_family:
style_file:
canvas:
subject:
projection:
palette_size:
outline_policy:
shading_policy:
texture_policy:
light_direction:
layer_order:
anti_patterns_to_avoid:
```

Do not put this brief inside `.apx` unless the user asks for documentation.

## Style Index

| User wording | Choose | Detailed file |
| --- | --- | --- |
| 1-bit, monochrome, black and white, print, old Mac, stark horror | 1-bit monochrome | `styles/1-bit-monochrome.md` |
| Game Boy, handheld, green screen, 4 colors, nostalgic small sprite | Game Boy 4-tone | `styles/game-boy-4-tone.md` |
| NES, 8-bit console, simple platformer, retro tile, old Nintendo | NES-inspired 8-bit | `styles/nes-inspired-8-bit.md` |
| C64, Commodore, demoscene, chunky dither, 16 hardware colors | C64-inspired | `styles/c64-inspired.md` |
| SNES, Genesis, 16-bit, polished RPG prop, side-scroller asset | 16-bit console | `styles/16-bit-console.md` |
| arcade, fighting game, action pose, large dramatic character | Arcade/fighting sprite | `styles/arcade-fighting-sprite.md` |
| top-down RPG, tile, dungeon, farming map, terrain asset | Top-down RPG tiles | `styles/top-down-rpg-tiles.md` |
| isometric, room, building, 2:1 diagonal, axonometric prop | Isometric | `styles/isometric.md` |
| cozy, cute, farming, soft animal, food, gentle indie | Cozy indie | `styles/cozy-indie.md` |
| cyberpunk, neon, robot, sign, night street, sci-fi monitor | Cyberpunk neon | `styles/cyberpunk-neon.md` |
| horror, dark fantasy, cursed item, cave, monster, unsettling | Horror/dark fantasy | `styles/horror-dark-fantasy.md` |
| clean icon, app badge, UI glyph, tool symbol, minimal readable | Modern clean pixel icon | `styles/modern-clean-pixel-icon.md` |
| avatar, portrait, face, bust, profile picture | Pixel portrait | `styles/pixel-portrait.md` |
| poster, atmospheric scene, gradient, fog, water, print-like dither | Dithered poster | `styles/dithered-poster.md` |

## Layer Order

APX stores layers like a Photoshop layer panel: first layer is top/front, last layer is bottom/back. Rendering composites from the bottom of the array upward.

Default order, top to bottom:

```text
details
effects
lighting / glow_pixels
highlight / rim_highlight
face / eyes
secondary_color / material_ramps
material_shadow / shadow_detail
base
outline / edge_lines
terrain / floor
ground_shadow / cast_shadow
far_shapes
background
```

Style-specific layer orders live in each style file.

Use `move_layer` for order changes:

```json
{ "op": "move_layer", "layer": "details", "to_index": 0 }
{ "op": "move_layer", "layer": "highlight", "before": "base" }
{ "op": "move_layer", "layer": "shadow", "after": "base" }
```

## Shared Anti-Patterns

- Too many colors for a tiny sprite.
- Smooth gradients that ignore pixel clusters.
- Random pixel noise instead of purposeful texture.
- Pillow shading with no clear light source.
- Banding where parallel stair-steps hug outlines.
- Inconsistent projection, especially in isometric and top-down art.
- Mixing era constraints without intent.
- Details that only read when zoomed far beyond 1x.

## Patch Prompts

- "stronger silhouette": replace or add outline/base chunks.
- "more Game Boy": reduce palette to 4 values and remove hue variety.
- "more NES": reduce colors per object and strengthen dark outline.
- "less noisy": clear or replace dither/detail chunks.
- "more 16-bit": add controlled material ramps and selective highlights.
- "more isometric": fix diagonal edges to 2:1 and separate planar faces.
- "more cozy": replace black outlines with colored outlines and soften contrast.
- "more cyberpunk": darken base and add limited neon accents.
