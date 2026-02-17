#!/usr/bin/env sh

set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"

if [ ! -f "$REPO_ROOT/package.json" ] || [ ! -f "$REPO_ROOT/src-tauri/Cargo.toml" ]; then
  echo "Refusing to run outside LegalDocsReview repo root: $REPO_ROOT" >&2
  exit 1
fi

exec sh "$SCRIPT_DIR/clean-workspace.sh"
