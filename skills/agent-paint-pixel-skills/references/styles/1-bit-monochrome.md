# 1-bit Monochrome

Use this style for stark icons, early Macintosh-like art, print graphics, tiny horror assets, and high-contrast UI glyphs.

## Visual DNA

- Two values only: foreground and background.
- Shape design carries the image.
- Negative space is as important as filled pixels.
- Texture comes from checker, stripe, hatch, or sparse noise patterns.

## Palette

- `.` transparent.
- `K = #000000`
- Optional `W = #ffffff` if the background is not transparent.

Do not add gray, colored highlights, or soft AA.

## Layer Order

Top to bottom:

```text
details
dither_texture
cutouts
silhouette
```

## APX Guidance

- Use full rows for 16x16 and 32x32 icons.
- Use chunks for local hatch/dither edits.
- Use `clear_layer` or chunk replacement to remove noise quickly.

## Do

- Use large readable masses.
- Use one dither pattern per material.
- Use isolated pixels only as intentional sparkle, eyes, or grit.

## Avoid

- Midtones.
- Anti-aliased edges.
- Random noise that weakens silhouette.
