# Skill Compatibility Notes

This document records the compatibility design for `agent-paint-pixel-skills`.

## Sources Checked

- OpenAI Codex Agent Skills docs: skills are folders with `SKILL.md`, optional `scripts/`, `references/`, `assets/`, and `agents/openai.yaml`; user-level skills live in `$HOME/.agents/skills`; repo-level skills live in `.agents/skills`; reusable distribution should use Codex plugins with `.codex-plugin/plugin.json` and root `skills/`.
- OpenAI Codex plugin docs: `.codex-plugin/plugin.json` is the plugin entry point; `skills` points to bundled skill folders relative to the plugin root.
- Anthropic `anthropics/skills`: each skill is a self-contained folder with `SKILL.md`; Claude Code marketplace distribution uses `.claude-plugin/marketplace.json` and plugin metadata.
- Agent Skills open standard at `agentskills.io`: required `SKILL.md` frontmatter fields are `name` and `description`; optional directories include `scripts/`, `references/`, and `assets`; progressive disclosure is the intended context model.
- `nextlevelbuilder/ui-ux-pro-max-skill`: uses `.claude-plugin`, `.claude/skills`, and `skill.json` style metadata for multi-tool discovery.
- `FrancyJGLisboa/agent-skill-creator`: uses one `SKILL.md` plus cross-platform installers and adapters for tools that use rule formats instead of native skill folders.
- `mohitagw15856/pm-claude-skills`: uses a large `skills/` catalog, plugin marketplace entries, MCP/export surfaces, and explicit cross-tool positioning.

## Design Chosen

AgentPaint keeps one standard skill as the source of truth:

```text
.agents/skills/agent-paint-pixel-skills/
  SKILL.md
  AGENTS.md
  agents/openai.yaml
  references/
```

For plugin distribution, the same skill is mirrored under:

```text
skills/agent-paint-pixel-skills/
```

This lets:

- Codex discover the checked-in repo skill from `.agents/skills`.
- Codex plugin installers discover bundled skills from root `skills/`.
- Claude plugin installers discover the same root `skills/` entry.
- Other native `SKILL.md` tools install the folder into their own global or project skill path.

## Runtime Boundary

The skill must not require the AgentPaint source repository at runtime. It should:

- Create `.apx` or `.apxa` files in the current user workspace.
- Call `agentpaint` from `PATH`.
- Use bundled skill references relative to the installed skill folder.
- Ask the user to install the CLI if `agentpaint --help` fails.

## Install Targets

Native skill-folder targets:

```text
universal/codex  $HOME/.agents/skills
claude-code      $HOME/.claude/skills
copilot          $HOME/.copilot/skills
gemini           $HOME/.gemini/skills
kiro             $HOME/.kiro/skills
cline            $HOME/.cline/skills
roo-code         $HOME/.roo/skills
kilo-code        $HOME/.kilocode/skills
factory          $HOME/.factory/skills
goose            $HOME/.config/goose/skills
opencode         $HOME/.config/opencode/skills
antigravity      $HOME/.gemini/antigravity/skills
```

Rule-adapter targets:

```text
cursor    .mdc rule
windsurf  .md rule
trae      .md rule
junie     guidelines.md
```

Rule adapters are intentionally short. They tell the agent when to use AgentPaint and point to the standard installed skill folder for the full workflow.

## Maintenance Rules

- Keep `SKILL.md` concise and move detailed format/style content into `references/`.
- Keep `AGENTS.md` as a companion pointer, not a second copy of the full skill.
- Keep `.codex-plugin/plugin.json`, `.claude-plugin/*`, and `skill.json` metadata aligned with the skill name and description.
- After changing `.agents/skills/agent-paint-pixel-skills`, mirror it to `skills/agent-paint-pixel-skills` before publishing a plugin release.
- Validate both the source skill and mirrored plugin skill.
