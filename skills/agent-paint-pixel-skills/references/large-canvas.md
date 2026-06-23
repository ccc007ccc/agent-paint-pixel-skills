# Large Canvas Construction

Use this workflow for still images at `64x64` and larger, and for any image that contains multiple objects, terrain, buildings, rooms, scenes, large props, maps, or large portraits.

Do not rely on visual self-judgment as the main quality test. Build and audit the APX source structure, then validate and inspect a supersampled preview when possible.

## Flat Area Discipline

Large same-color regions are allowed as base construction, but they are a quality risk when they become the finished image. This mainly affects backgrounds, grass, hills, mountains, water, walls, clothing, hair, and large portraits.

- Treat any broad single-color plane as a `base_planes`, `skin_base`, `sky_base`, `ground`, or `large_objects` layer, not as finished art.
- Add at least one visible detail pass over every broad material surface unless it is intentionally blank UI space, transparent space, or a tiny icon background.
- For grass and ground, add clustered blades, tufts, stones, flower dots, path edge breaks, darker under-object shadows, and 2-4 variant patches. Do not paste identical grass clusters unchanged.
- For hills and mountains, add ridge lines, slope shadow bands, contour breaks, rock cracks, snow/grass caps, tree dots, atmospheric bands, or dithered value transitions.
- For sky and distant background, flatness can be acceptable, but add horizon bands, clouds, stars, distant silhouettes, or value bands when the background occupies a large part of the canvas.
- For large portraits, avoid circle/ellipse-only construction. Skin may occupy a large same-color area, but the face must be broken by facial planes: brow shadow, nose bridge, cheek value, jaw/chin shadow, ears, hairline, neck shadow, and eye/mouth clusters. Use 2-4 skin values unless the requested style is intentionally flat.
- Hair and clothing should not be single filled shapes. Add locks, edge cuts, folds, seams, rim light, cast shadows, or material highlights.
- A big chunk of repeated symbols is acceptable only if a higher layer visibly changes that area with edge, texture, shadow, trim, highlight, or small props.
- If a supersampled preview reads as smooth vector art, blobs, or large pasted rectangles, patch the APX before presenting it.

## Before Writing APX

- Divide the canvas into 4-8 named zones such as focal area, secondary area, route/floor/terrain, boundary, foreground accents, and empty/transparent space.
- List 8-20 local features that should exist as separate readable pixel clusters: openings, seams, trim, stones, small props, surface marks, cast shadows, highlights, edge breaks, signs, handles, cracks, plants, sparkle points, or material bands.
- List broad surfaces that could become flat, such as sky, grass, hill, mountain, wall, hair, clothing, or skin, and name the layer that will break each surface with texture, value, edge, trim, or facial structure.
- Decide which broad surfaces are intentionally plain or transparent. Transparency is valid when it is part of the asset; do not fill transparent space merely to hide it.
- For repeated elements, plan named variants before drawing them. A repeated element should not be pasted unchanged unless it is a deliberate tile pattern.

## Writing APX

- Build in passes: `base_planes` or `ground`, then `large_objects`, then `edge_lines`, then `material_shadow`, then `surface_texture`, then `trim`, then `small_props`, then `details`, then `highlights/effects`.
- Large chunks are allowed only as base planes, silhouette blocks, or intentionally empty/transparent areas. Do not use a few large chunks as the finished artwork.
- Every major object or broad surface should have at least two supporting passes above its base: edge/shape definition plus one of texture, shadow, trim, highlight, or small props.
- Keep layer names tied to visual function and compositing order, not just object categories. Prefer names such as `edge_lines`, `cast_shadow`, `material_shadow`, `base_planes`, `surface_texture`, `trim`, `small_props`, `details`, `highlights`, and `effects`.
- For top-down scenes, keep functional zones readable: paths, water, walls, floor, vegetation, props, and walkable areas should not accidentally overlap unless the subject calls for it.
- For isometric scenes, split top, left, and right planes into separate layers and keep 2:1 diagonals consistent before adding decoration.

## Before Finalizing

- Check the APX contains the planned local features as actual chunks or rows, not only as a description in text.
- Check repeated elements have variant chunks or different internal marks.
- Check large base chunks have higher-layer local detail unless they are intentionally plain or transparent.
- Check natural surfaces and portraits for excessive flat areas: grass, hills, mountains, walls, hair, clothing, and large skin regions should show material or facial structure above their base color.
- Do not reject a large same-color region solely because it is large. Judge whether it has a semantic reason and whether visible upper layers break it where pixel-art detail is expected.
- If the APX source reads like only a blockout, add another detail pass instead of presenting it.
