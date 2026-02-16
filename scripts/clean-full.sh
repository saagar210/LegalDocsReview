#!/usr/bin/env sh

set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"

if [ ! -f "$REPO_ROOT/package.json" ] || [ ! -f "$REPO_ROOT/src-tauri/Cargo.toml" ]; then
  echo "Refusing to run outside LegalDocsReview repo root: $REPO_ROOT" >&2
  exit 1
fi

# First remove known repo build artifacts and dependencies.
sh "$SCRIPT_DIR/clean-workspace.sh" --with-deps

# Then remove other reproducible local caches if they exist.
remove_path() {
  target="$1"
  if [ -e "$target" ]; then
    rm -rf "$target"
    echo "removed: $target"
  fi
}

remove_path "$REPO_ROOT/.cache"
remove_path "$REPO_ROOT/.vite"
remove_path "$REPO_ROOT/.pnpm-store"
remove_path "$REPO_ROOT/.eslintcache"
remove_path "$REPO_ROOT/.turbo"
remove_path "$REPO_ROOT/.parcel-cache"

echo "Full local cleanup complete."
