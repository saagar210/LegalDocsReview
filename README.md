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

Build:

```sh
pnpm build
```

Tests:

```sh
pnpm test
```

## API Keys / Local Data

- API keys are entered in the app’s Settings screen and stored locally in the app SQLite DB.
- Uploaded PDFs are copied into the app data directory under `documents/`.
- `.env` is ignored by git; do not commit secrets.
