#!/usr/bin/env sh

set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"

if [ ! -f "$REPO_ROOT/package.json" ] || [ ! -f "$REPO_ROOT/src-tauri/Cargo.toml" ]; then
  echo "Refusing to run outside LegalDocsReview repo root: $REPO_ROOT" >&2
  exit 1
fi

LEAN_TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/legaldocs-lean.XXXXXX")"
LEAN_CARGO_TARGET_DIR="$LEAN_TMP_ROOT/cargo-target"
mkdir -p "$LEAN_CARGO_TARGET_DIR"

cleanup() {
  # Cleanup only heavy reproducible build artifacts in the repo, then remove temp cache dir.
  sh "$REPO_ROOT/scripts/clean-heavy.sh" >/dev/null 2>&1 || true
  rm -rf "$LEAN_TMP_ROOT" >/dev/null 2>&1 || true
  # Cargo may still be tearing down child processes after signal handling.
  if [ -d "$LEAN_TMP_ROOT" ]; then
    sleep 1
    rm -rf "$LEAN_TMP_ROOT" >/dev/null 2>&1 || true
  fi
}

trap cleanup EXIT HUP INT TERM

echo "lean-dev temp cache: $LEAN_TMP_ROOT"
echo "running: pnpm tauri dev"

(
  cd "$REPO_ROOT"
  CARGO_TARGET_DIR="$LEAN_CARGO_TARGET_DIR" pnpm tauri dev
)
