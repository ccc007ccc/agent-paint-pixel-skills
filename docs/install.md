# Install AgentPaint CLI And Skills

Chinese guide: [install.zh-CN.md](install.zh-CN.md).

This project ships two installable parts:

- `agentpaint`, the Rust CLI installed into Cargo's binary directory.
- `agent-paint-pixel-skills`, a standard `SKILL.md` agent skill for APX/APXA pixel art.

Normal skill use does not require the AgentPaint source repository. Agents should call `agentpaint` from `PATH` and read the installed skill folder.

## Prerequisites

- Rust and Cargo.
- One or more CLI/IDE agents that support `SKILL.md`, rules, or plugin installs.
- This repository, only when installing from source.

Verify Rust:

```bash
cargo --version
rustc --version
```

## Quick Install

From the repository root:

```bash
sh ./scripts/install.sh --update-path
```

Windows PowerShell:

```powershell
.\scripts\install.ps1 -UpdatePath
```

Default behavior:

- Installs `agentpaint` with `cargo install --path . --locked --force`.
- Installs the skill to the universal/Codex path: `$HOME/.agents/skills/agent-paint-pixel-skills`.
- With `--update-path`, adds Cargo's bin directory to a shell startup file. The installer uses `.zprofile` for zsh, `.bash_profile` on macOS bash, `.bashrc` on other bash shells, `config.fish` for fish, and `.profile` as a fallback.

Verify:

```bash
agentpaint --help
```

## Install Skills For Other CLIs

The skill source lives at:

```text
.agents/skills/agent-paint-pixel-skills
```

Install only the skill, without reinstalling the CLI:

```bash
sh ./scripts/install.sh --skip-cli --skill-target claude-code
sh ./scripts/install.sh --skip-cli --skill-target gemini
sh ./scripts/install.sh --skip-cli --skill-target cursor --project
```

Windows PowerShell:

```powershell
.\scripts\install.ps1 -SkipCli -SkillTargets claude-code
.\scripts\install.ps1 -SkipCli -SkillTargets gemini
.\scripts\install.ps1 -SkipCli -SkillTargets cursor -ProjectSkills
```

Install to every supported target:

```bash
sh ./scripts/install.sh --skip-cli --all-skill-targets
```

Windows PowerShell:

```powershell
.\scripts\install.ps1 -SkipCli -AllSkillTargets
```

Supported targets:

```text
universal, codex, claude-code, copilot, gemini, kiro, cline,
roo-code, kilo-code, factory, goose, opencode, antigravity,
cursor, windsurf, trae, junie, all
```

`codex` is an alias for `universal`.

## Target Paths

User-level native `SKILL.md` installs:

| Target | Path |
| --- | --- |
| `universal` / `codex` | `$HOME/.agents/skills/agent-paint-pixel-skills` |
| `claude-code` | `$HOME/.claude/skills/agent-paint-pixel-skills` |
| `copilot` | `$HOME/.copilot/skills/agent-paint-pixel-skills` |
| `gemini` | `$HOME/.gemini/skills/agent-paint-pixel-skills` |
| `kiro` | `$HOME/.kiro/skills/agent-paint-pixel-skills` |
| `cline` | `$HOME/.cline/skills/agent-paint-pixel-skills` |
| `roo-code` | `$HOME/.roo/skills/agent-paint-pixel-skills` |
| `kilo-code` | `$HOME/.kilocode/skills/agent-paint-pixel-skills` |
| `factory` | `$HOME/.factory/skills/agent-paint-pixel-skills` |
| `goose` | `$HOME/.config/goose/skills/agent-paint-pixel-skills` |
| `opencode` | `$HOME/.config/opencode/skills/agent-paint-pixel-skills` |
| `antigravity` | `$HOME/.gemini/antigravity/skills/agent-paint-pixel-skills` |

Rule-adapter installs:

| Target | Output |
| --- | --- |
| `cursor` | `.cursor/rules/agent-paint-pixel-skills.mdc` or `$HOME/.cursor/rules/agent-paint-pixel-skills.mdc` |
| `windsurf` | `.windsurf/rules/agent-paint-pixel-skills.md` or `$HOME/.codeium/windsurf/skills/agent-paint-pixel-skills.md` |
| `trae` | `.trae/rules/agent-paint-pixel-skills.md` or `$HOME/.trae/rules/agent-paint-pixel-skills.md` |
| `junie` | `.junie/skills/agent-paint-pixel-skills/guidelines.md` or `$HOME/.junie/skills/agent-paint-pixel-skills/guidelines.md` |

For rule-adapter targets, the installer also installs a universal standard skill copy unless `--no-universal-companion` is used on `scripts/install-skills.sh` or `-NoUniversalCompanion` is used on `scripts/install-skills.ps1`.

## Project-Level Install

Use project-level install when a tool only reads repository rules, or when the skill should travel with a repo:

```bash
sh ./scripts/install.sh --skip-cli --project-skills --skill-target cursor
sh ./scripts/install.sh --skip-cli --project-skills --skill-target windsurf
sh ./scripts/install.sh --skip-cli --project-skills --skill-target universal
```

Windows PowerShell:

```powershell
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets cursor
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets windsurf
.\scripts\install.ps1 -SkipCli -ProjectSkills -SkillTargets universal
```

To install into another repository:

```bash
sh ./scripts/install.sh --skip-cli --project-skills --project-path /path/to/repo --skill-target universal
```

Windows PowerShell:

```powershell
.\scripts\install.ps1 -SkipCli -ProjectSkills -ProjectPath C:\path\to\repo -SkillTargets universal
```

## Plugin Distribution

The repository includes plugin manifests for installers that understand plugin marketplaces:

```text
.codex-plugin/plugin.json
.claude-plugin/plugin.json
.claude-plugin/marketplace.json
skills/agent-paint-pixel-skills/SKILL.md
```

Codex plugins use `.codex-plugin/plugin.json` and the root `skills/` directory. Claude Code plugin marketplaces use `.claude-plugin/marketplace.json` and `.claude-plugin/plugin.json`.

The runtime skill still requires `agentpaint` on `PATH`; plugin install handles the skill, not the Rust binary.

## Ask A CLI AI To Install It

From inside the AgentPaint repository, give any CLI coding agent this prompt:

```text
Install AgentPaint for local use:
1. Run the bundled installer for my platform.
2. Ensure Cargo's bin directory is on PATH.
3. Install agent-paint-pixel-skills for my CLI target.
4. Verify `agentpaint --help`.
5. Verify the installed skill folder or rule file exists.
6. Do not delete unrelated files.
```

For all supported skill targets:

```text
Install only the AgentPaint skill to every supported CLI target using the bundled installer. Do not reinstall the Rust CLI.
```

## Update

After pulling changes:

```bash
sh ./scripts/install.sh --update-path --all-skill-targets
```

Windows PowerShell:

```powershell
.\scripts\install.ps1 -UpdatePath -AllSkillTargets
```

## Uninstall

Remove the CLI:

```bash
cargo uninstall agentpaint
```

Then remove installed skill copies from whichever target paths you used.
