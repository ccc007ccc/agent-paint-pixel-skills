# Game Boy 4-Tone

Use this style for handheld nostalgia, compact creatures, item icons, simple rooms, and low-color UI.

## Visual DNA

- Four values behave as the whole lighting system.
- Hue variety is minimal or absent.
- Materials differ by shape, value placement, and pattern.
- Readability comes from dark outlines and clean value clusters.

## Palette

Use 4 tone symbols:

- `D`: darkest outline/shadow.
- `M`: mid-dark.
- `L`: mid-light.
- `H`: highlight.

Typical green ramp:

```json
"D": "#0f380f",
"M": "#306230",
"L": "#8bac0f",
"H": "#9bbc0f"
```

## Layer Order

Top to bottom:

```text
pattern_detail
light
base_mid
outline
shadow
```

## APX Guidance

- Keep all objects within the same 4-value ramp.
- Use full rows for small sprites.
- Use chunks for eye glints, small object markings, and repeated texture.

## Do

- Separate materials with value pattern rather than hue.
- Use dithering sparingly for gradients or texture.

## Avoid

- Extra accent colors.
- 16-bit level shading.
- Smooth anti-aliased curves.
