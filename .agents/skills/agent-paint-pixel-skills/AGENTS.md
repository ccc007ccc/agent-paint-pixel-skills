# Agent Paint Pixel Skills

Use this companion file for agent tools that discover `AGENTS.md` but do not scan `SKILL.md` directly.

When the user asks to generate, edit, import, validate, inspect, animate, or export pixel art with AgentPaint, use the sibling `SKILL.md` as the authoritative workflow. The workflow must call the installed `agentpaint` CLI from `PATH`; it must not search for the AgentPaint source repository during normal artwork generation.

Core runtime commands:

- `agentpaint validate <file.apx>`
- `agentpaint inspect <file.apx>`
- `agentpaint import-image <input.png> --out <output.apx>`
- `agentpaint render <file.apx> --out <file.png>`
- `agentpaint supersample <file.apx> --out <file-preview.png>`
- `agentpaint patch <file.apx> --patch <patch.json> --out <patched.apx>`
- `agentpaint export-psd <file.apx> --out <file.psd>`
- `agentpaint export-rgba <file.apx> --out <file.rgba.json>`
- `agentpaint validate-animation <file.apxa>`
- `agentpaint import-gif <input.gif> --out <output.apxa>`
- `agentpaint inspect-animation <file.apxa>`
- `agentpaint render-frame <file.apxa> --frame 0 --out <frame.png>`
- `agentpaint supersample-frame <file.apxa> --frame 0 --out <frame-preview.png>`
- `agentpaint render-gif <file.apxa> --out <file.gif>`

Keep requested canvas dimensions exact. Do not draw smaller and resize. Do not write helper programs to generate artwork unless the user explicitly asks for programmatic generation. Use import commands only for converting existing low-color raster pixel art or GIFs into APX/APXA source.
