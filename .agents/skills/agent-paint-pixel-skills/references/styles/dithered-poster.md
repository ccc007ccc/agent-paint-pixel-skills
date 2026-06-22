# Dithered Poster

Use this style for atmospheric scenes, sky gradients, water, fog, print-like compositions, and limited-palette posters.

## Visual DNA

- Large graphic shapes.
- Deliberate ordered dithering.
- Limited palette with perceived intermediate tones.
- Texture is part of the composition.

## Palette

- Use 4-12 colors.
- Build strong value steps.
- Use one or two dither pairs per major gradient.

## Layer Order

Top to bottom:

```text
effects
foreground_details
texture_lines
dither_gradient
main_shapes
background_values
```

## APX Guidance

- Use full rows for large background value bands.
- Use chunks for dither patches, fog, clouds, water texture, and foreground detail.
- Patch noisy areas by replacing only the dither chunk.

## Do

- Keep dither patterns deliberate and consistent.
- Use large readable shapes under the texture.
- Let texture follow light or atmosphere.

## Avoid

- Mixing checker, line, and noise dithers randomly.
- Dithering every material.
- Losing the main silhouette in texture.
