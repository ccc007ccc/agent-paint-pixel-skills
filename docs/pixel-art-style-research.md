# Pixel Art Style Research Notes

This document records web research used to improve the `agent-paint-pixel-skills` skill.

## Sources Consulted

### Pixel Art Technique And Craft

- Pixel Joint, "The Pixel Art Tutorial": https://pixeljoint.com/forum/forum_posts.asp?TID=11299
- Pixel Parmesan, "Anti-Aliasing Fundamentals for Pixel Artists": https://pixelparmesan.com/blog/anti-aliasing-fundamentals-for-pixel-artists
- Pixel Parmesan, "Dithering for Pixel Artists": https://pixelparmesan.com/blog/dithering-for-pixel-artists
- Drububu, "pixel art & dithering; mimic gradients with limited colors": https://www.drububu.com/tutorial/pixel-art-and-dithering.html
- Lospec, "Pixel Art Dithering Tutorials": https://lospec.com/pixel-art-tutorials/tags/dithering
- Pixnote, "Pixel Art Outlines & Anti-Aliasing": https://pixnote.net/en/learn/outlines/
- Pixnote, "Pixel Art Dithering Guide": https://pixnote.net/en/learn/dithering/
- Pixnote, "Isometric Pixel Art Guide": https://pixnote.net/en/learn/isometric/
- Pixel Parmesan, "Fundamentals of Isometric Pixel Art": https://pixelparmesan.com/blog/fundamentals-of-isometric-pixel-art
- SLYNYRD, "Pixelblog - 41 - Isometric Pixel art": https://www.slynyrd.com/blog/2022/11/28/pixelblog-41-isometric-pixel-art

### Hardware, Palette, And Era Constraints

- Noel Berry, "NES Limitations": https://noelberry.ca/posts/nes/index.html
- Mega Cat Studios, "Creating NES Graphics": https://megacatstudios.com/blogs/retro-development/creating-nes-graphics
- C64-Wiki, "Graphics Modes": https://www.c64-wiki.com/wiki/Graphics_Modes
- Lospec, "Game Boy palette tag": https://lospec.com/palette-list/tag/gameboy/
- Lospec, "NES palette tag": https://lospec.com/palette-list/tag/NES
- Lospec, "Commodore 64 Palette": https://lospec.com/palette-list/commodore64
- PicToPixel, "Pixel Art Styles Explained: GameBoy, NES, and More": https://pictopixel.art/blog/pixel-art-styles-explained-gameboy-nes-and-more

### Papers And Academic References

- Kopf, J. and Lischinski, D. "Depixelizing pixel art." ACM SIGGRAPH 2011. DOI: 10.1145/1964921.1964994
- "Depixelizing pixel art in real-time." ACM I3D 2015. DOI: 10.1145/2699276.2721395
- "Geometric Total Variation for Image Vectorization, Zooming and Pixel Art Depixelizing." 2020. DOI: 10.1007/978-3-030-41404-7_28
- "Application of Neural Network in Pixel Art Creation: Bi-directional Conversion between Photo and Pixel Art with GAN Base Model." 2022. DOI: 10.1109/iccece54139.2022.9712735
- "On the Challenges of Generating Pixel Art Character Sprites Using GANs." AIIDE 2022. Semantic Scholar: https://www.semanticscholar.org/paper/5470e0ee8f1f0fc95664d91e1bc58d901bb54919
- "Pixel art character generation as an image-to-image translation problem using GANs." Graphical Models 2024. DOI: 10.1016/j.gmod.2024.101213
- Maerten, A. and Soydaner, D. "From paintbrush to pixel: A review of deep neural networks in AI-generated art." arXiv:2302.10913

## Research Synthesis

### What Matters Most For LLM Authored Pixel Art

1. Work from constraints, not freeform pixels.
   Hardware-era styles and modern pixel-art tutorials repeatedly stress limits: small resolution, small palettes, tile grids, and platform-specific restrictions. APX should encode those constraints before the model draws rows.

2. Silhouette first.
   Pixel Joint and related tutorials emphasize readability. At tiny sizes, a strong silhouette and clear value separation matter more than surface detail.

3. Color count is a style control.
   Game Boy, NES, C64, 1-bit, and modern minimalist styles are mainly distinguished by palette size, value ramps, and how much texture is allowed.

4. Pixel clusters are more important than individual pixels.
   Good pixel art reads as deliberate clusters and bands of value. Isolated single pixels should be rare and meaningful, such as sparkle, eye glint, or noise in a texture style.

5. Dithering is not generic shading.
   Dithering should communicate texture, constrained color depth, or a deliberate retro display look. Over-dithering on modern screens reads noisy.

6. Manual anti-aliasing is selective.
   Pixel Parmesan and Pixnote both frame AA as a controlled illusion of smoother curves. Use it on focal curves, not on every edge.

7. Era styles need coherent limitations.
   NES-inspired art should use small sub-palettes and tile-like thinking; Game Boy should use 4-ish values; C64 can use 16-color and chunky/dithered character-mode flavor; SNES/16-bit can allow richer ramps and larger sprites.

8. Isometric art is a projection system, not just a diagonal look.
   Isometric references consistently describe equal foreshortening and no vanishing point. For APX, enforce 2:1 diagonals, consistent verticals, and separate layers for floor, walls, objects, and highlights.

9. Academic work supports preserving topology.
   Depixelization papers are about enlarging pixel art without losing important contours and topology. The practical lesson for generation is to preserve crisp shape relationships, corners, and contour continuity.

10. Neural-generation papers reinforce the need for validation and iteration.
    Pixel-art GAN/image-translation research treats pixel art as a constrained image translation problem. For AgentPaint, the better interface is: style brief -> APX source -> validation -> render -> patch iteration.

## Style Axes For AgentPaint

Use these axes before selecting a named style:

- `resolution`: icon 16x16, sprite 32x32, asset 48x48/64x64, scene 64x64+
- `palette_size`: 2, 4, 8, 12, 16, 24+
- `outline`: none, dark outline, selective outline, colored outline
- `shading`: flat, cel, ramped, dithered, high-detail
- `texture`: clean clusters, checker dithering, noise pixels, material strokes
- `projection`: front, side, top-down, 3/4, isometric
- `era`: 1-bit, Game Boy, NES, C64, 16-bit console, arcade, modern indie
- `mood`: cozy, heroic, horror, cyberpunk, pastoral, industrial, whimsical

## Practical Impact On Skill Design

The updated skill should:

- ask or infer a style brief before drawing;
- choose a palette recipe from the style;
- choose layer names from the style;
- specify when to use rows versus chunks;
- specify anti-patterns per style;
- validate APX and render a preview;
- use patch files for style corrections such as "more Game Boy", "less noisy", "stronger silhouette", or "more isometric".
