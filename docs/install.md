# Install AgentPaint CLI And Skill

This guide installs:

- `agentpaint`, the Rust CLI, into Cargo's binary directory.
- `agent-paint-pixel-skills`, the Codex skill, into the user's global skills directory.

After installation, any CLI agent can call `agentpaint` from `PATH`, and Codex can use the skill from any repository.

Runtime use does not require the AgentPaint source repository. The repository is only needed to install or update the CLI and skill from source.

## Prerequisites

- Rust and Cargo installed.
- Codex CLI installed if you want Codex to use the skill.
- The AgentPaint repository available locally, only when installing from a local clone.

Verify Rust:

```bash
cargo --version
rustc --version
```

## Ask A CLI AI To Install It

From inside the AgentPaint repository, give Codex or another CLI coding agent this prompt:

```text
Install this AgentPaint project for local use:
1. Install the Rust CLI with `cargo install --path . --locked`.
2. Ensure Cargo's bin directory is on PATH.
3. Copy `.agents/skills/agent-paint-pixel-skills` into my global Codex skills directory at `$HOME/.agents/skills/agent-paint-pixel-skills`.
4. Verify `agentpaint --help` and that the skill folder contains `SKILL.md`.
5. Do not delete or overwrite unrelated files.
```

If the agent is running on Windows PowerShell, it should use the PowerShell commands below.

## Install The CLI

Run from the AgentPaint repository root:

```bash
cargo install --path . --locked
```

Or use the bundled installer:

```bash
sh ./scripts/install.sh --update-path
```

On Windows PowerShell:

```powershell
.\scripts\install.ps1 -UpdatePath
```

The binary is installed to Cargo's bin directory:

- Windows: `%USERPROFILE%\.cargo\bin`
- macOS/Linux: `$HOME/.cargo/bin`

Verify:

```bash
agentpaint --help
```

If `agentpaint` is not found, add Cargo's bin directory to `PATH`.

### Windows PowerShell PATH

For the current terminal session:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
agentpaint --help
```

Persist for future terminals:

```powershell
$cargoBin = "$env:USERPROFILE\.cargo\bin"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (($userPath -split ';') -notcontains $cargoBin) {
  [Environment]::SetEnvironmentVariable("Path", "$cargoBin;$userPath", "User")
}
```

Open a new terminal after changing the persistent PATH.

### macOS/Linux PATH

For the current terminal session:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
agentpaint --help
```

Persist for future shells by adding this line to your shell profile:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Install The Codex Skill

AgentPaint includes an installable skill source at:

```text
.agents/skills/agent-paint-pixel-skills
```

To make it available globally to Codex, copy it to:

```text
$HOME/.agents/skills/agent-paint-pixel-skills
```

### Windows PowerShell

Run from the AgentPaint repository root:

```powershell
$source = Join-Path (Get-Location) ".agents\skills\agent-paint-pixel-skills"
$targetRoot = Join-Path $HOME ".agents\skills"
$target = Join-Path $targetRoot "agent-paint-pixel-skills"

New-Item -ItemType Directory -Force -Path $targetRoot | Out-Null
if (Test-Path -LiteralPath $target) {
  Remove-Item -LiteralPath $target -Recurse -Force
}
Copy-Item -LiteralPath $source -Destination $target -Recurse -Force
```

### macOS/Linux

Run from the AgentPaint repository root:

```bash
mkdir -p "$HOME/.agents/skills"
rm -rf "$HOME/.agents/skills/agent-paint-pixel-skills"
cp -R ".agents/skills/agent-paint-pixel-skills" "$HOME/.agents/skills/agent-paint-pixel-skills"
```

Restart Codex after installing or updating a global skill if it does not appear immediately.

The bundled install scripts copy this skill automatically unless `--skip-skill` or `-SkipSkill` is used.

Verify the skill files:

```bash
ls "$HOME/.agents/skills/agent-paint-pixel-skills"
```

On Windows PowerShell:

```powershell
Get-ChildItem -LiteralPath "$HOME\.agents\skills\agent-paint-pixel-skills"
```

## Verify End To End

From any directory:

```bash
agentpaint --help
```

Expected result:

- `agentpaint --help` prints the available commands.
- The global skill folder contains `SKILL.md`.

After examples are regenerated, also verify `validate`, `inspect`, `render`, `export-rgba`, and `patch` against one APX file.

When Codex uses `$agent-paint-pixel-skills`, it should create APX files in the user's current workspace and run `agentpaint validate`, `agentpaint render`, and `agentpaint patch`. It should not search for the AgentPaint source repository unless the user is explicitly developing or reinstalling AgentPaint itself.

For generation requests, the APX canvas must match the requested size exactly. The skill should not satisfy a requested size by drawing a smaller image and resizing it, and it should not write helper scripts to draw the art unless the user explicitly asks for programmatic generation.

## Update

After pulling or editing AgentPaint:

```bash
cargo install --path . --locked --force
```

Then reinstall the skill by repeating the copy commands above, or by running the bundled install script again.

## Uninstall

Remove the CLI:

```bash
cargo uninstall agentpaint
```

Remove the global Codex skill:

```bash
rm -rf "$HOME/.agents/skills/agent-paint-pixel-skills"
```

On Windows PowerShell:

```powershell
Remove-Item -LiteralPath "$HOME\.agents\skills\agent-paint-pixel-skills" -Recurse -Force
```
