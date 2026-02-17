# LegalDocsReview

AI-assisted legal document review desktop app built with Tauri + React.

## What It Does

- Upload PDFs (stored locally in the app data directory)
- Extract text from PDFs
- Extract key clauses and fields
- Generate risk scoring / risk distribution
- Compare two documents
- Manage analysis templates
- Generate review reports

AI providers supported:

- OpenAI (Chat Completions API)
- Anthropic Claude (Messages API)

## Tech Stack

- Tauri v2 (Rust backend)
- React + Vite + TypeScript (frontend)
- Tailwind CSS
- SQLite (via `rusqlite`, stored in app data dir as `legal_docs_review.db`)
- Vitest (tests)

## Development

Prerequisites:

- Node.js + `pnpm`
- Rust toolchain for Tauri (see [Tauri prerequisites](https://tauri.app/start/prerequisites/))

Install deps:

```sh
pnpm install
```

Run the frontend in the browser:

```sh
pnpm dev
```

Run the desktop app (Tauri):

```sh
pnpm tauri dev
```

Run the desktop app in lean mode (temporary Rust build cache + auto cleanup on exit):

```sh
pnpm dev:lean
```

Build:

```sh
pnpm build
```

Tests:

```sh
pnpm test
```

Workspace cleanup:

```sh
pnpm clean
```

Targeted heavy-artifact cleanup (keeps dependencies for faster next startup):

```sh
pnpm clean:heavy
```

Full local cleanup for reproducible caches (includes dependencies + local cache folders):

```sh
pnpm clean:full
```

Deep cleanup (also removes `node_modules`):

```sh
pnpm clean:deep
```

Preview cleanup actions without deleting files:

```sh
pnpm clean:dry-run
```

## Dev Modes and Disk Tradeoffs

Normal dev (`pnpm tauri dev`):

- Fastest repeat startup because Rust build artifacts are reused from `src-tauri/target`.
- Uses more disk over time (Rust target artifacts are typically the largest local growth).

Lean dev (`pnpm dev:lean`):

- Runs the same desktop command, but sets `CARGO_TARGET_DIR` to a temporary folder outside the repo.
- Automatically deletes temporary build cache and runs targeted heavy cleanup when the app exits.
- Uses less persistent disk in the repo, but startup is slower because Rust artifacts are rebuilt more often.

Cleanup guidance:

- Use `pnpm clean:heavy` for day-to-day disk control without deleting dependencies.
- Use `pnpm clean:full` when you want to reclaim maximum local space and are okay with reinstall/rebuild cost.

## API Keys / Local Data

- API keys are entered in the app’s Settings screen and stored locally in the app SQLite DB.
- Uploaded PDFs are copied into the app data directory under `documents/`.
- `.env` is ignored by git; do not commit secrets.
