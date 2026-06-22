# Cyberpunk Neon

Use this style for robots, signs, night streets, terminals, monitors, sci-fi props, and neon city scenes.

## Visual DNA

- Dark base.
- Saturated neon accents.
- Hard silhouettes.
- Limited glow pixels and screen-like details.

## Palette

- Use 8-18 colors.
- Most colors should be dark or muted.
- Reserve cyan, magenta, yellow, green, or red for small high-impact accents.

## Layer Order

Top to bottom:

```text
crisp_foreground_details
effects
screen_details
glow_pixels
neon_accents
panel_shadow
base_dark
dark_silhouette
```

## APX Guidance

- Use chunks for signs, screens, wires, glow strips, and small LEDs.
- Use `move_layer` to keep glow/effects above base but below crisp detail when needed.
- Patch over-neon by reducing accent chunks.

## Do

- Keep neon limited and intentional.
- Use high contrast.
- Use tiny glow halos sparingly.

## Avoid

- Filling the whole sprite with neon.
- Low-contrast dark-on-dark details.
- Random RGB speckles.
