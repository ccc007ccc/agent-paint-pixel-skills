#!/usr/bin/env sh
set -eu

UPDATE_PATH=0
SKIP_CLI=0
SKIP_SKILL=0
ALL_SKILL_TARGETS=0
PROJECT_SKILLS=0
DRY_RUN=0
SKILL_TARGETS="universal"
SKILL_TARGETS_SET=0
PROJECT_PATH=$(pwd)

while [ "$#" -gt 0 ]; do
  case "$1" in
    --update-path) UPDATE_PATH=1 ;;
    --skip-cli) SKIP_CLI=1 ;;
    --skip-skill) SKIP_SKILL=1 ;;
    --skill-target|--target)
      shift
      if [ "$SKILL_TARGETS_SET" -eq 0 ]; then
        SKILL_TARGETS=""
        SKILL_TARGETS_SET=1
      fi
      SKILL_TARGETS="${SKILL_TARGETS:+$SKILL_TARGETS,}$1"
      ;;
    --skill-targets|--targets)
      shift
      SKILL_TARGETS=$1
      SKILL_TARGETS_SET=1
      ;;
    --all-skill-targets|--all) ALL_SKILL_TARGETS=1 ;;
    --project-skills|--project) PROJECT_SKILLS=1 ;;
    --project-path)
      shift
      PROJECT_PATH=$1
      ;;
    --dry-run) DRY_RUN=1 ;;
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

if [ "$SKIP_CLI" -eq 0 ]; then
  if [ "$DRY_RUN" -eq 1 ]; then
    echo "[dry-run] cargo install --path \"$REPO_ROOT\" --locked --force"
  else
    cargo install --path "$REPO_ROOT" --locked --force
  fi
fi

if [ "$SKIP_SKILL" -eq 0 ]; then
  INSTALL_SKILLS="$SCRIPT_DIR/install-skills.sh"
  set -- --targets "$SKILL_TARGETS" --project-path "$PROJECT_PATH"
  if [ "$ALL_SKILL_TARGETS" -eq 1 ]; then
    set -- "$@" --all
  fi
  if [ "$PROJECT_SKILLS" -eq 1 ]; then
    set -- "$@" --project
  fi
  if [ "$DRY_RUN" -eq 1 ]; then
    set -- "$@" --dry-run
  fi
  sh "$INSTALL_SKILLS" "$@"
fi

if [ "$UPDATE_PATH" -eq 1 ]; then
  PROFILE_FILE="$HOME/.profile"
  PATH_LINE='export PATH="$HOME/.cargo/bin:$PATH"'
  if [ ! -f "$PROFILE_FILE" ] || ! grep -Fq "$PATH_LINE" "$PROFILE_FILE"; then
    if [ "$DRY_RUN" -eq 1 ]; then
      echo "[dry-run] Add Cargo bin to $PROFILE_FILE"
    else
      printf '\n%s\n' "$PATH_LINE" >> "$PROFILE_FILE"
      echo "Added Cargo bin to $PROFILE_FILE. Open a new shell to use it."
    fi
  fi
fi

echo "AgentPaint install complete."
echo "CLI: $CARGO_BIN/agentpaint"
echo "Skill source: $SKILL_SOURCE"
