param(
  [switch]$SkipCli,
  [switch]$SkipSkill,
  [switch]$UpdatePath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$CargoBin = Join-Path $HOME ".cargo\bin"
$SkillSource = Join-Path $RepoRoot ".agents\skills\agent-paint-pixel-skills"
$SkillRoot = Join-Path $HOME ".agents\skills"
$SkillTarget = Join-Path $SkillRoot "agent-paint-pixel-skills"

if (-not $SkipCli) {
  cargo install --path $RepoRoot --locked --force
}

if (-not $SkipSkill) {
  if (-not (Test-Path -LiteralPath $SkillSource)) {
    throw "Skill source not found: $SkillSource"
  }

  New-Item -ItemType Directory -Force -Path $SkillRoot | Out-Null

  $SkillRootFull = [System.IO.Path]::GetFullPath($SkillRoot)
  $SkillTargetFull = [System.IO.Path]::GetFullPath($SkillTarget)
  if (-not $SkillTargetFull.StartsWith($SkillRootFull, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Refusing to write outside skill root: $SkillTargetFull"
  }

  if (Test-Path -LiteralPath $SkillTarget) {
    Remove-Item -LiteralPath $SkillTarget -Recurse -Force
  }
  Copy-Item -LiteralPath $SkillSource -Destination $SkillTarget -Recurse -Force
}

if ($UpdatePath) {
  $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
  $Parts = @()
  if ($UserPath) {
    $Parts = $UserPath -split ';'
  }
  if ($Parts -notcontains $CargoBin) {
    $NewPath = if ($UserPath) { "$CargoBin;$UserPath" } else { $CargoBin }
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "Added $CargoBin to the user PATH. Open a new terminal to use it."
  }
}

Write-Host "AgentPaint install complete."
Write-Host "CLI: $CargoBin\agentpaint.exe"
Write-Host "Skill: $SkillTarget"
