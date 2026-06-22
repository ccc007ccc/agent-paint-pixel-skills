# Top-Down RPG Tiles

Use this style for terrain, floors, walls, buildings, doors, props, dungeon assets, farming tiles, and overworld objects.

## Visual DNA

- Consistent top-down or 3/4 top-down projection.
- Reusable tile edges.
- Walkable paths stay readable.
- Texture supports terrain rather than hiding function.

## Palette

- Use 8-20 colors.
- Build separate ramps for ground, wall, vegetation, water, and props.
- Keep collision or edge colors consistent across tiles.

## Layer Order

Top to bottom:

```text
details
characters_or_small_props
walkability_marks
material_texture
walls_or_large_props
terrain_edges
ground
```

## APX Guidance

- Use chunks for repeated tile variations.
- Keep object footprints obvious.
- Use `move_layer` to keep characters/props above terrain and details above base surfaces.

## Do

- Keep tile boundaries reusable.
- Use texture clusters in moderation.
- Preserve path readability.

## Avoid

- Decorative noise that hides collision boundaries.
- Mixed projections.
- Shadows that imply side-view perspective.
