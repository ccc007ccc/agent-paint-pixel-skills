# C64-Inspired

Use this style for chunky retro scenes, demoscene texture, computer-era objects, and gritty limited-palette illustrations.

## Visual DNA

- Muted hardware-like colors.
- Chunky forms.
- Visible ordered dithering and repeated texture.
- Character-mode or block-mode feeling.

## Palette

- Use up to 16 colors.
- Prefer muted brights and earthy midtones.
- Reserve the brightest colors for small accents.

## Layer Order

Top to bottom:

```text
effects
details
outline
dither_texture
large_color_blocks
background
```

## APX Guidance

- Use chunks heavily for dither patches, repeated bricks, water, clouds, and texture.
- Full rows are fine for simple background bands.
- Patch texture by replacing a local chunk rather than rewriting the whole scene.

## Do

- Use deliberate dither patterns.
- Keep blocky silhouettes.
- Let repeated texture communicate material.

## Avoid

- Smooth 16-bit ramps.
- Too many tiny single-pixel speckles without a pattern.
- Neon-only palettes unless hybridized intentionally.
