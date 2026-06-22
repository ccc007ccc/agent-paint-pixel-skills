# Arcade/Fighting Sprite

Use this style for action poses, large expressive characters, bosses, and combat-ready silhouettes.

## Visual DNA

- Dramatic pose and silhouette.
- Strong contrast.
- Form-following anatomy, costume folds, or armor planes.
- Rim highlights and focal face/weapon details.

## Palette

- Use 16-32 colors.
- Use strong value separation between lit and shadow sides.
- Use accent colors only for focal gear, energy, or eyes.

## Layer Order

Top to bottom:

```text
face_details
effects
rim_highlight
muscle_or_fold_ramps
deep_shadow
base_pose
outline
cast_shadow
```

## APX Guidance

- Use 48x48 or 64x64 when possible.
- Use chunks for limbs, gear, face, energy, and highlights.
- Use `move_layer` to keep effects above body but below crisp facial details when needed.

## Do

- Push silhouette and gesture.
- Make the light direction obvious.
- Keep high-contrast clusters clean.

## Avoid

- Static symmetrical poses.
- Low-contrast anatomy.
- Tiny details that do not support the pose.
