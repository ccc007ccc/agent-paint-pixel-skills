#!/usr/bin/env sh
set -eu

ALL=0
PROJECT=0
DRY_RUN=0
NO_UNIVERSAL=0
PROJECT_PATH=$(pwd)
TARGETS="universal"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --target|--skill-target)
      shift
      TARGETS="${TARGETS:+$TARGETS }$1"
      ;;
    --targets|--skill-targets)
      shift
      TARGETS=$(printf '%s' "$1" | tr ',' ' ')
      ;;
    --all|--all-skill-targets) ALL=1 ;;
    --project) PROJECT=1 ;;
    --project-path)
      shift
      PROJECT_PATH=$1
      ;;
    --dry-run) DRY_RUN=1 ;;
    --no-universal-companion) NO_UNIVERSAL=1 ;;
    -h|--help)
      cat <<'EOF'
Usage: scripts/install-skills.sh [options]

Options:
  --target <name>              Install one target. Can be repeated.
  --targets <a,b,c>            Install comma-separated targets.
  --all, --all-skill-targets   Install all supported targets.
  --project                    Install into project paths instead of user paths.
  --project-path <path>        Project root for --project. Defaults to cwd.
  --dry-run                    Print actions without writing files.

Targets:
  universal, codex, claude-code, copilot, gemini, kiro, cline,
  roo-code, kilo-code, factory, goose, opencode, antigravity,
  cursor, windsurf, trae, junie, all
EOF
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
  shift
done

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
SKILL_NAME="agent-paint-pixel-skills"
SKILL_SOURCE="$REPO_ROOT/.agents/skills/$SKILL_NAME"

if [ ! -f "$SKILL_SOURCE/SKILL.md" ]; then
  echo "Skill source not found or invalid: $SKILL_SOURCE" >&2
  exit 1
fi

normalize_target() {
  case "$1" in
    all) printf '%s\n' all ;;
    codex|agents) printf '%s\n' universal ;;
    claude|claude-code) printf '%s\n' claude-code ;;
    github-copilot|copilot) printf '%s\n' copilot ;;
    gemini-cli|gemini) printf '%s\n' gemini ;;
    roo|roo-code) printf '%s\n' roo-code ;;
    kilo|kilo-code) printf '%s\n' kilo-code ;;
    *) printf '%s\n' "$1" ;;
  esac
}

target_kind() {
  case "$1" in
    cursor) printf '%s\n' cursor ;;
    windsurf) printf '%s\n' markdown-rule ;;
    trae) printf '%s\n' trae-rule ;;
    junie) printf '%s\n' junie ;;
    *) printf '%s\n' skill ;;
  esac
}

target_root() {
  target=$1
  if [ "$PROJECT" -eq 1 ]; then
    case "$target" in
      universal) printf '%s\n' "$PROJECT_PATH/.agents/skills" ;;
      claude-code) printf '%s\n' "$PROJECT_PATH/.claude/skills" ;;
      copilot) printf '%s\n' "$PROJECT_PATH/.github/skills" ;;
      gemini) printf '%s\n' "$PROJECT_PATH/.gemini/skills" ;;
      kiro) printf '%s\n' "$PROJECT_PATH/.kiro/skills" ;;
      cline) printf '%s\n' "$PROJECT_PATH/.clinerules/skills" ;;
      roo-code) printf '%s\n' "$PROJECT_PATH/.roo/skills" ;;
      kilo-code) printf '%s\n' "$PROJECT_PATH/.kilocode/skills" ;;
      factory) printf '%s\n' "$PROJECT_PATH/.factory/skills" ;;
      goose) printf '%s\n' "$PROJECT_PATH/.goose/skills" ;;
      opencode) printf '%s\n' "$PROJECT_PATH/.opencode/skills" ;;
      antigravity) printf '%s\n' "$PROJECT_PATH/.agent/skills" ;;
      cursor) printf '%s\n' "$PROJECT_PATH/.cursor/rules" ;;
      windsurf) printf '%s\n' "$PROJECT_PATH/.windsurf/rules" ;;
      trae) printf '%s\n' "$PROJECT_PATH/.trae/rules" ;;
      junie) printf '%s\n' "$PROJECT_PATH/.junie/skills" ;;
      *) echo "Unknown target: $target" >&2; exit 2 ;;
    esac
  else
    case "$target" in
      universal) printf '%s\n' "$HOME/.agents/skills" ;;
      claude-code) printf '%s\n' "$HOME/.claude/skills" ;;
      copilot) printf '%s\n' "$HOME/.copilot/skills" ;;
      gemini) printf '%s\n' "$HOME/.gemini/skills" ;;
      kiro) printf '%s\n' "$HOME/.kiro/skills" ;;
      cline) printf '%s\n' "$HOME/.cline/skills" ;;
      roo-code) printf '%s\n' "$HOME/.roo/skills" ;;
      kilo-code) printf '%s\n' "$HOME/.kilocode/skills" ;;
      factory) printf '%s\n' "$HOME/.factory/skills" ;;
      goose) printf '%s\n' "$HOME/.config/goose/skills" ;;
      opencode) printf '%s\n' "$HOME/.config/opencode/skills" ;;
      antigravity) printf '%s\n' "$HOME/.gemini/antigravity/skills" ;;
      cursor) printf '%s\n' "$HOME/.cursor/rules" ;;
      windsurf) printf '%s\n' "$HOME/.codeium/windsurf/skills" ;;
      trae) printf '%s\n' "$HOME/.trae/rules" ;;
      junie) printf '%s\n' "$HOME/.junie/skills" ;;
      *) echo "Unknown target: $target" >&2; exit 2 ;;
    esac
  fi
}

skill_description() {
  sed -n 's/^description:[[:space:]]*//p' "$SKILL_SOURCE/SKILL.md" | head -n 1 | sed 's/^"//;s/"$//'
}

adapter_body() {
  desc=$(skill_description)
  cat <<EOF
# Agent Paint Pixel Skills

$desc

Use when the user asks to generate, edit, validate, inspect, animate, export, or repair AgentPaint APX/APXA pixel art.

Runtime rules:

- Use the installed \`agentpaint\` CLI from PATH.
- Do not search for the AgentPaint source repository during normal art generation.
- Match requested canvas dimensions exactly; do not draw small and resize.
- Author \`.apx\` or \`.apxa\` JSON, then run \`agentpaint validate\` or \`agentpaint validate-animation\`.
- For visual inspection, create a nearest-neighbor preview with \`agentpaint supersample\` or \`agentpaint supersample-frame\`.
- Export PSD with \`agentpaint export-psd\` when layered Photoshop output is requested.

If a standard skill folder is available at \`.agents/skills/$SKILL_NAME\` or \`$HOME/.agents/skills/$SKILL_NAME\`, read its \`SKILL.md\` for the full workflow and bundled references.
EOF
}

copy_skill_dir() {
  root=$1
  dest="$root/$SKILL_NAME"
  if [ "$dest" = "$SKILL_SOURCE" ]; then
    echo "Skill source already at target: $dest"
    return
  fi
  if [ "$DRY_RUN" -eq 1 ]; then
    echo "[dry-run] Copy $SKILL_SOURCE -> $dest"
    return
  fi
  mkdir -p "$root"
  case "$dest" in
    "$root"/*) rm -rf "$dest" ;;
    *) echo "Refusing to write outside target root: $dest" >&2; exit 1 ;;
  esac
  cp -R "$SKILL_SOURCE" "$dest"
}

write_adapter() {
  adapter_target=$1
  root=$2
  kind=$(target_kind "$adapter_target")
  if [ "$kind" = "cursor" ]; then
    file="$root/$SKILL_NAME.mdc"
    if [ "$DRY_RUN" -eq 1 ]; then
      echo "[dry-run] Write Cursor adapter -> $file"
      return
    fi
    mkdir -p "$root"
    {
      printf '%s\n' '---'
      printf 'description: %s\n' "$(skill_description)"
      printf '%s\n' 'alwaysApply: false'
      printf '%s\n' '---'
      adapter_body
    } > "$file"
  elif [ "$kind" = "trae-rule" ]; then
    file="$root/$SKILL_NAME.md"
    if [ "$DRY_RUN" -eq 1 ]; then
      echo "[dry-run] Write Trae adapter -> $file"
      return
    fi
    mkdir -p "$root"
    {
      printf '%s\n' '---'
      printf '%s\n' 'type: Auto'
      printf '%s\n' '---'
      adapter_body
    } > "$file"
  elif [ "$kind" = "junie" ]; then
    file="$root/$SKILL_NAME/guidelines.md"
    if [ "$DRY_RUN" -eq 1 ]; then
      echo "[dry-run] Write Junie adapter -> $file"
      return
    fi
    mkdir -p "$(dirname "$file")"
    adapter_body > "$file"
  else
    file="$root/$SKILL_NAME.md"
    if [ "$DRY_RUN" -eq 1 ]; then
      echo "[dry-run] Write markdown adapter -> $file"
      return
    fi
    mkdir -p "$root"
    adapter_body > "$file"
  fi
}

install_one() {
  install_target=$1
  root=$(target_root "$install_target")
  kind=$(target_kind "$install_target")
  if [ "$kind" = "skill" ]; then
    copy_skill_dir "$root"
    echo "Installed $install_target: $root/$SKILL_NAME"
  else
    write_adapter "$install_target" "$root"
    echo "Installed $install_target adapter: $root"
    if [ "$NO_UNIVERSAL" -eq 0 ]; then
      copy_skill_dir "$(target_root universal)"
    fi
  fi
}

if [ "$ALL" -eq 1 ]; then
  TARGETS="universal claude-code copilot gemini kiro cline roo-code kilo-code factory goose opencode antigravity cursor windsurf trae junie"
fi

expanded_targets=""
for raw in $TARGETS; do
  target=$(normalize_target "$raw")
  if [ "$target" = "all" ]; then
    expanded_targets="universal claude-code copilot gemini kiro cline roo-code kilo-code factory goose opencode antigravity cursor windsurf trae junie"
    break
  fi
  expanded_targets="${expanded_targets:+$expanded_targets }$target"
done
TARGETS=$expanded_targets

seen=" "
for target in $TARGETS; do
  case "$seen" in
    *" $target "*) continue ;;
  esac
  seen="$seen$target "
  install_one "$target"
done
