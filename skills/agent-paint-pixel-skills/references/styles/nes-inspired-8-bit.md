# NES-Inspired 8-bit

Use this style for platformer enemies, tiles, pickups, weapons, simple UI objects, and retro console sprites.

## Visual DNA

- Strong dark outlines.
- Flat or cel-like shading.
- Small sub-palettes per object.
- Tile-friendly shapes and simple readable silhouettes.

## Palette

- Use about 8-12 symbols total.
- Keep each object to roughly 3 colors plus a shared dark outline.
- Use distinct ramps for skin, cloth, metal, or terrain only when necessary.

## Layer Order

Top to bottom:

```text
details
highlight
secondary_color
base
outline
shadow
```

## APX Guidance

- Favor full rows for 16x16 and 32x32 assets.
- Use chunks for facial details, weapon shine, pickup symbols, or tile variations.
- If style drifts, patch by reducing colors and strengthening outline.

## Do

- Make the sprite readable at 1x.
- Use clear silhouette and few internal clusters.
- Keep material colors bold.

## Avoid

- Many near-identical shades.
- Complex gradients.
- Excessive texture on tiny sprites.
