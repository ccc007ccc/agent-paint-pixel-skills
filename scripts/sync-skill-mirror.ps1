param(
  [switch]$Check
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$SkillName = "agent-paint-pixel-skills"
$Source = Join-Path $RepoRoot ".agents\skills\$SkillName"
$Mirror = Join-Path $RepoRoot "skills\$SkillName"

if (-not (Test-Path -LiteralPath (Join-Path $Source "SKILL.md"))) {
  throw "Skill source not found: $Source"
}

if ($Check) {
  git diff --no-index --exit-code -- $Source $Mirror
  if ($LASTEXITCODE -ne 0) {
    throw "Skill mirror is out of sync. Run .\scripts\sync-skill-mirror.ps1"
  }
  return
}

$repoSkillsRoot = [System.IO.Path]::GetFullPath((Join-Path $RepoRoot "skills")).TrimEnd('\', '/')
$mirrorFull = [System.IO.Path]::GetFullPath($Mirror)
$prefix = $repoSkillsRoot + [System.IO.Path]::DirectorySeparatorChar
if (-not $mirrorFull.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
  throw "Refusing to write outside repo skills directory: $mirrorFull"
}

if (Test-Path -LiteralPath $Mirror) {
  Remove-Item -LiteralPath $Mirror -Recurse -Force
}
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $Mirror) | Out-Null
Copy-Item -LiteralPath $Source -Destination $Mirror -Recurse -Force
Write-Host "Synced $Source -> $Mirror"
