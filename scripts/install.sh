#!/usr/bin/env sh
set -eu

UPDATE_PATH=0
SKIP_CLI=0
SKIP_SKILL=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --update-path) UPDATE_PATH=1 ;;
    --skip-cli) SKIP_CLI=1 ;;
    --skip-skill) SKIP_SKILL=1 ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
  shift
done

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
CARGO_BIN="$HOME/.cargo/bin"
SKILL_SOURCE="$REPO_ROOT/.agents/skills/agent-paint-pixel-skills"
SKILL_ROOT="$HOME/.agents/skills"
SKILL_TARGET="$SKILL_ROOT/agent-paint-pixel-skills"

if [ "$SKIP_CLI" -eq 0 ]; then
  cargo install --path "$REPO_ROOT" --locked --force
fi

if [ "$SKIP_SKILL" -eq 0 ]; then
  if [ ! -d "$SKILL_SOURCE" ]; then
    echo "Skill source not found: $SKILL_SOURCE" >&2
    exit 1
  fi

  mkdir -p "$SKILL_ROOT"
  case "$SKILL_TARGET" in
    "$SKILL_ROOT"/*) rm -rf "$SKILL_TARGET" ;;
    *)
      echo "Refusing to write outside skill root: $SKILL_TARGET" >&2
      exit 1
      ;;
  esac
  cp -R "$SKILL_SOURCE" "$SKILL_TARGET"
fi

if [ "$UPDATE_PATH" -eq 1 ]; then
  PROFILE_FILE="$HOME/.profile"
  PATH_LINE='export PATH="$HOME/.cargo/bin:$PATH"'
  if [ ! -f "$PROFILE_FILE" ] || ! grep -Fq "$PATH_LINE" "$PROFILE_FILE"; then
    printf '\n%s\n' "$PATH_LINE" >> "$PROFILE_FILE"
    echo "Added Cargo bin to $PROFILE_FILE. Open a new shell to use it."
  fi
fi

echo "AgentPaint install complete."
echo "CLI: $CARGO_BIN/agentpaint"
echo "Skill: $SKILL_TARGET"
