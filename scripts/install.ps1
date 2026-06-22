param(
  [switch]$SkipCli,
  [switch]$SkipSkill,
  [switch]$UpdatePath,
  [string[]]$SkillTargets = @("universal"),
  [switch]$AllSkillTargets,
  [switch]$ProjectSkills,
  [string]$ProjectPath = (Get-Location).Path,
  [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$CargoBin = Join-Path $HOME ".cargo\bin"

if (-not $SkipCli) {
  if ($DryRun) {
    Write-Host "[dry-run] cargo install --path $RepoRoot --locked --force"
  } else {
    cargo install --path $RepoRoot --locked --force
  }
}

if (-not $SkipSkill) {
  $InstallSkills = Join-Path $PSScriptRoot "install-skills.ps1"
  & $InstallSkills `
    -SkillTargets $SkillTargets `
    -ProjectPath $ProjectPath `
    -All:$AllSkillTargets `
    -Project:$ProjectSkills `
    -DryRun:$DryRun
}

if ($UpdatePath) {
  $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
  $Parts = @()
  if ($UserPath) {
    $Parts = $UserPath -split ';'
  }
  if ($Parts -notcontains $CargoBin) {
    $NewPath = if ($UserPath) { "$CargoBin;$UserPath" } else { $CargoBin }
    if ($DryRun) {
      Write-Host "[dry-run] Add $CargoBin to the user PATH"
    } else {
      [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
      Write-Host "Added $CargoBin to the user PATH. Open a new terminal to use it."
    }
  }
}

Write-Host "AgentPaint install complete."
Write-Host "CLI: $CargoBin\agentpaint.exe"
Write-Host "Skill source: $(Join-Path $RepoRoot '.agents\skills\agent-paint-pixel-skills')"
