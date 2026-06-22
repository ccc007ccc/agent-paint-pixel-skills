# Isometric

Use this style for rooms, buildings, crates, terrain blocks, machines, and technical props.

## Visual DNA

- 2:1 diagonal edges.
- No vanishing point.
- Equal foreshortening across visible axes.
- Clear top, left, and right planes.

## Palette

- Use 10-24 colors.
- Assign separate values to top, left face, and right face.
- Keep edge lines consistent.

## Layer Order

Top to bottom:

```text
details
highlights
edge_lines
top_face
right_face
left_face
floor
ground_shadow
```

## APX Guidance

- Use chunks for each object or plane.
- Use `move_layer` if faces overlap incorrectly.
- Keep diagonal edges two horizontal pixels for one vertical pixel step.

## Do

- Split planes into layers.
- Keep verticals vertical.
- Put cast shadows below objects.

## Avoid

- Perspective convergence.
- Mixed 1:1 and 2:1 diagonals.
- Highlights that ignore the plane direction.
