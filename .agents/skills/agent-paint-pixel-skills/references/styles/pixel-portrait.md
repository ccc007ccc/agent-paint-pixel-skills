# Pixel Portrait

Use this style for avatars, busts, profile pictures, face icons, and character portraits.

## Visual DNA

- Clear head silhouette.
- Hair and face planes as clusters.
- Controlled eyes, nose, mouth, and cheek pixels.
- Selective highlights.

## Palette

- Use 12-24 colors for full portraits.
- Use separate ramps for skin, hair, clothing, and accent.
- Avoid noisy skin ramps.

## Layer Order

Top to bottom:

```text
accessories
highlights
nose_mouth
eyes
face_shadow
hair
skin_base
neck_shoulders
head_silhouette
```

## APX Guidance

- Use 32x32 minimum for recognizable faces.
- Use chunks for eyes, nose, mouth, hair shine, and accessories.
- Use `move_layer` to keep facial details above skin and hair where needed.

## Do

- Use face planes instead of gradients.
- Keep eyes readable and aligned.
- Use hair clusters as large shapes first.

## Avoid

- Heavy dithering on skin.
- Over-detailed mouths at tiny sizes.
- Random single pixels on the face.
