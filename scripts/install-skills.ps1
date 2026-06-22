param(
  [string[]]$SkillTargets = @("universal"),
  [switch]$All,
  [switch]$Project,
  [string]$ProjectPath = (Get-Location).Path,
  [switch]$DryRun,
  [switch]$NoUniversalCompanion
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$SkillName = "agent-paint-pixel-skills"
$SkillSource = Join-Path $RepoRoot ".agents\skills\$SkillName"

if (-not (Test-Path -LiteralPath (Join-Path $SkillSource "SKILL.md"))) {
  throw "Skill source not found or invalid: $SkillSource"
}

function Expand-HomePath {
  param([string]$Path)
  if ($Path.StartsWith("~")) {
    return Join-Path $HOME $Path.Substring(2)
  }
  return $Path
}

function Normalize-Target {
  param([string]$Name)
  $key = $Name.ToLowerInvariant()
  switch ($key) {
    "all" { return "all" }
    "codex" { return "universal" }
    "agents" { return "universal" }
    "claude" { return "claude-code" }
    "claude-code" { return "claude-code" }
    "github-copilot" { return "copilot" }
    "copilot" { return "copilot" }
    "gemini-cli" { return "gemini" }
    "gemini" { return "gemini" }
    "roo" { return "roo-code" }
    "roo-code" { return "roo-code" }
    "kilo" { return "kilo-code" }
    "kilo-code" { return "kilo-code" }
    default { return $key }
  }
}

function Target-Info {
  param([string]$Target)
  switch ($Target) {
    "universal" { return @{ Kind = "skill"; Global = "~\.agents\skills"; Project = ".agents\skills"; Display = "Universal / Codex" } }
    "claude-code" { return @{ Kind = "skill"; Global = "~\.claude\skills"; Project = ".claude\skills"; Display = "Claude Code" } }
    "copilot" { return @{ Kind = "skill"; Global = "~\.copilot\skills"; Project = ".github\skills"; Display = "GitHub Copilot" } }
    "gemini" { return @{ Kind = "skill"; Global = "~\.gemini\skills"; Project = ".gemini\skills"; Display = "Gemini CLI" } }
    "kiro" { return @{ Kind = "skill"; Global = "~\.kiro\skills"; Project = ".kiro\skills"; Display = "Kiro" } }
    "cline" { return @{ Kind = "skill"; Global = "~\.cline\skills"; Project = ".clinerules\skills"; Display = "Cline" } }
    "roo-code" { return @{ Kind = "skill"; Global = "~\.roo\skills"; Project = ".roo\skills"; Display = "Roo Code" } }
    "kilo-code" { return @{ Kind = "skill"; Global = "~\.kilocode\skills"; Project = ".kilocode\skills"; Display = "Kilo Code" } }
    "factory" { return @{ Kind = "skill"; Global = "~\.factory\skills"; Project = ".factory\skills"; Display = "Factory Droid" } }
    "goose" { return @{ Kind = "skill"; Global = "~\.config\goose\skills"; Project = ".goose\skills"; Display = "Goose" } }
    "opencode" { return @{ Kind = "skill"; Global = "~\.config\opencode\skills"; Project = ".opencode\skills"; Display = "OpenCode" } }
    "antigravity" { return @{ Kind = "skill"; Global = "~\.gemini\antigravity\skills"; Project = ".agent\skills"; Display = "Antigravity" } }
    "cursor" { return @{ Kind = "cursor"; Global = "~\.cursor\rules"; Project = ".cursor\rules"; Display = "Cursor" } }
    "windsurf" { return @{ Kind = "markdown-rule"; Global = "~\.codeium\windsurf\skills"; Project = ".windsurf\rules"; Display = "Windsurf" } }
    "trae" { return @{ Kind = "trae-rule"; Global = "~\.trae\rules"; Project = ".trae\rules"; Display = "Trae" } }
    "junie" { return @{ Kind = "junie"; Global = "~\.junie\skills"; Project = ".junie\skills"; Display = "Junie" } }
    default { throw "Unknown skill target $Target. Use -All or one of: universal, claude-code, copilot, gemini, kiro, cline, roo-code, kilo-code, factory, goose, opencode, antigravity, cursor, windsurf, trae, junie." }
  }
}

function Install-Root {
  param($Info)
  if ($Project) {
    return Join-Path ([System.IO.Path]::GetFullPath($ProjectPath)) $Info.Project
  }
  return Expand-HomePath $Info.Global
}

function Assert-Target-Under-Root {
  param([string]$Root, [string]$Target)
  $rootFull = [System.IO.Path]::GetFullPath($Root).TrimEnd('\', '/')
  $targetFull = [System.IO.Path]::GetFullPath($Target)
  $prefix = $rootFull + [System.IO.Path]::DirectorySeparatorChar
  if (-not $targetFull.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Refusing to write outside target root: $targetFull"
  }
}

function Copy-SkillDirectory {
  param([string]$Root)
  $target = Join-Path $Root $SkillName
  Assert-Target-Under-Root -Root $Root -Target $target
  $sourceFull = [System.IO.Path]::GetFullPath($SkillSource)
  $targetFull = [System.IO.Path]::GetFullPath($target)
  if ($sourceFull.Equals($targetFull, [System.StringComparison]::OrdinalIgnoreCase)) {
    Write-Host "Skill source already at target: $target"
    return $target
  }
  if ($DryRun) {
    Write-Host "[dry-run] Copy $SkillSource -> $target"
    return $target
  }
  New-Item -ItemType Directory -Force -Path $Root | Out-Null
  if (Test-Path -LiteralPath $target) {
    Remove-Item -LiteralPath $target -Recurse -Force
  }
  Copy-Item -LiteralPath $SkillSource -Destination $target -Recurse -Force
  return $target
}

function Skill-Description {
  $lines = Get-Content -LiteralPath (Join-Path $SkillSource "SKILL.md") -TotalCount 20
  foreach ($line in $lines) {
    if ($line -match '^description:\s*(.+)$') {
      return $Matches[1].Trim('"')
    }
  }
  return "Generate and validate AgentPaint APX/APXA pixel art with the installed agentpaint CLI."
}

function Adapter-Body {
  $desc = Skill-Description
  return @"
# Agent Paint Pixel Skills

$desc

Use when the user asks to generate, edit, validate, inspect, animate, export, or repair AgentPaint APX/APXA pixel art.

Runtime rules:

- Use the installed `agentpaint` CLI from PATH.
- Do not search for the AgentPaint source repository during normal art generation.
- Match requested canvas dimensions exactly; do not draw small and resize.
- Author `.apx` or `.apxa` JSON, then run `agentpaint validate` or `agentpaint validate-animation`.
- For visual inspection, create a nearest-neighbor preview with `agentpaint supersample` or `agentpaint supersample-frame`.
- Export PSD with `agentpaint export-psd` when layered Photoshop output is requested.

If a standard skill folder is available at `.agents/skills/$SkillName` or `$HOME/.agents/skills/$SkillName`, read its `SKILL.md` for the full workflow and bundled references.
"@
}

function Write-Adapter {
  param([string]$Target, $Info, [string]$Root)
  $body = Adapter-Body
  if ($Info.Kind -eq "cursor") {
    $file = Join-Path $Root "$SkillName.mdc"
    $content = @"
---
description: $(Skill-Description)
alwaysApply: false
---
$body
"@
  } elseif ($Info.Kind -eq "trae-rule") {
    $file = Join-Path $Root "$SkillName.md"
    $content = @"
---
type: Auto
---
$body
"@
  } elseif ($Info.Kind -eq "junie") {
    $folder = Join-Path $Root $SkillName
    $file = Join-Path $folder "guidelines.md"
    $content = $body
  } else {
    $file = Join-Path $Root "$SkillName.md"
    $content = $body
  }

  if ($DryRun) {
    Write-Host "[dry-run] Write adapter for $Target -> $file"
    return $file
  }

  $parent = Split-Path -Parent $file
  New-Item -ItemType Directory -Force -Path $parent | Out-Null
  Set-Content -LiteralPath $file -Value $content -Encoding UTF8
  return $file
}

function Install-One {
  param([string]$Target)
  $info = Target-Info $Target
  $root = Install-Root $info
  if ($info.Kind -eq "skill") {
    $location = Copy-SkillDirectory -Root $root
  } else {
    $location = Write-Adapter -Target $Target -Info $info -Root $root
    if (-not $NoUniversalCompanion) {
      $universal = Target-Info "universal"
      $universalRoot = Install-Root $universal
      Copy-SkillDirectory -Root $universalRoot | Out-Null
    }
  }
  Write-Host "Installed $($info.Display): $location"
}

$targets = @()
if ($All) {
  $targets = @("universal", "claude-code", "copilot", "gemini", "kiro", "cline", "roo-code", "kilo-code", "factory", "goose", "opencode", "antigravity", "cursor", "windsurf", "trae", "junie")
} else {
  foreach ($target in $SkillTargets) {
    $normalized = Normalize-Target $target
    if ($normalized -eq "all") {
      $targets = @("universal", "claude-code", "copilot", "gemini", "kiro", "cline", "roo-code", "kilo-code", "factory", "goose", "opencode", "antigravity", "cursor", "windsurf", "trae", "junie")
      break
    }
    $targets += $normalized
  }
}

$targets = $targets | Select-Object -Unique
foreach ($target in $targets) {
  Install-One -Target $target
}
