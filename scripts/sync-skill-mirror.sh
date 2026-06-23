#!/usr/bin/env sh
set -eu

CHECK=0
if [ "${1:-}" = "--check" ]; then
  CHECK=1
elif [ "${1:-}" != "" ]; then
  echo "Usage: sh ./scripts/sync-skill-mirror.sh [--check]" >&2
  exit 2
fi

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
SKILL_NAME="agent-paint-pixel-skills"
SOURCE="$REPO_ROOT/.agents/skills/$SKILL_NAME"
MIRROR="$REPO_ROOT/skills/$SKILL_NAME"

if [ ! -f "$SOURCE/SKILL.md" ]; then
  echo "Skill source not found: $SOURCE" >&2
  exit 1
fi

if [ "$CHECK" -eq 1 ]; then
  git diff --no-index --exit-code -- "$SOURCE" "$MIRROR"
  exit $?
fi

case "$MIRROR" in
  "$REPO_ROOT"/skills/*) ;;
  *)
    echo "Refusing to write outside repo skills directory: $MIRROR" >&2
    exit 1
    ;;
esac

rm -rf "$MIRROR"
mkdir -p "$(dirname "$MIRROR")"
cp -R "$SOURCE" "$MIRROR"
echo "Synced $SOURCE -> $MIRROR"
