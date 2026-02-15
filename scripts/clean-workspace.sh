#!/usr/bin/env sh

set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"

WITH_DEPS=0
DRY_RUN=0

usage() {
  cat <<'EOF'
Usage: clean-workspace.sh [--with-deps] [--dry-run]

Options:
  --with-deps  Remove dependency directories (node_modules) in addition to build artifacts.
  --dry-run    Print what would be removed without deleting files.
  -h, --help   Show this help message.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --with-deps)
      WITH_DEPS=1
      ;;
    --dry-run)
      DRY_RUN=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
  shift
done

if [ ! -f "$REPO_ROOT/package.json" ] || [ ! -f "$REPO_ROOT/src-tauri/Cargo.toml" ]; then
  echo "Refusing to run outside LegalDocsReview repo root: $REPO_ROOT" >&2
  exit 1
fi

remove_path() {
  target="$1"
  if [ -e "$target" ]; then
    if [ "$DRY_RUN" -eq 1 ]; then
      echo "[dry-run] rm -rf $target"
    else
      rm -rf "$target"
      echo "removed: $target"
    fi
  fi
}

remove_found_files() {
  pattern="$1"
  if [ "$WITH_DEPS" -eq 1 ]; then
    find "$REPO_ROOT" -name "$pattern" -print | while IFS= read -r path; do
      if [ "$DRY_RUN" -eq 1 ]; then
        echo "[dry-run] rm -f $path"
      else
        rm -f "$path"
        echo "removed: $path"
      fi
    done
  else
    find "$REPO_ROOT" -name node_modules -prune -o -name "$pattern" -print | while IFS= read -r path; do
      if [ "$DRY_RUN" -eq 1 ]; then
        echo "[dry-run] rm -f $path"
      else
        rm -f "$path"
        echo "removed: $path"
      fi
    done
  fi
}

remove_path "$REPO_ROOT/dist"
remove_path "$REPO_ROOT/src-tauri/target"
remove_path "$REPO_ROOT/src-tauri/gen"

if [ "$WITH_DEPS" -eq 1 ]; then
  remove_path "$REPO_ROOT/node_modules"
fi

remove_found_files ".DS_Store"
remove_found_files "*.tsbuildinfo"

if [ "$DRY_RUN" -eq 1 ]; then
  echo "Dry run complete."
else
  echo "Cleanup complete."
fi
