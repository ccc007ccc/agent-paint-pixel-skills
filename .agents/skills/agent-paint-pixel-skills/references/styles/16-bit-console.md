# 16-bit Console

Use this style for polished RPG props, side-scroller assets, characters, items, and richer game objects.

## Visual DNA

- More colors than 8-bit, but still controlled.
- Material-specific ramps.
- Colored shadows and selective highlights.
- Clean clusters and readable detail.

## Palette

- Use 12-24 colors for a full asset.
- Build 2-4 material ramps.
- Use hue-shifted shadows only when they improve form.

## Layer Order

Top to bottom:

```text
details
highlight
selective_aa
material_ramps
material_shadow
base
outline
cast_shadow
```

## APX Guidance

- Use full rows for main 32x32 character silhouettes.
- Use chunks for folds, gems, face details, shine, and material accents.
- Patch style drift by reducing noisy pixels and grouping clusters.

## Do

- Use clear light direction.
- Use selective AA on important curves.
- Keep material ramps separated.

## Avoid

- Pillow shading.
- Equal detail everywhere.
- Over-smoothing edges.
