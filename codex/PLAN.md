# Delta Plan

## A) Executive Summary
- The repository is a Tauri v2 desktop app with React/TypeScript frontend and Rust backend (`src/` + `src-tauri/`).
- Frontend app-level data loading for documents is centralized in `src/hooks/useDocuments.ts`.
- Dashboard data display is directly dependent on `useDocuments` (`src/pages/Dashboard.tsx`).
- Existing automated frontend tests cover layout and analysis components, but not the documents data hook.
- Backend uses SQLite via `rusqlite` and command handlers exposed by Tauri invoke.
- Frontend baseline verification is green; Rust test execution is blocked in this environment due to missing `glib-2.0`.

### Key Risks
- Data hook regressions can silently break dashboard loading/deletion behavior without targeted tests.
- Error handling in async hook operations can drift and produce stale UI state.
- Inability to run Rust tests locally in this container may hide backend regressions.

### Improvement Themes (Prioritized)
1. Add targeted test coverage for `useDocuments` lifecycle + mutation behavior.
2. Tighten `useDocuments` error-state consistency under refresh/delete failures.
3. Preserve baseline app behavior while increasing confidence and resume traceability.

## B) Constraints & Invariants (Repo-derived)
### Explicit invariants
- Tauri command interfaces remain unchanged (`src/lib/commands.ts`).
- `useDocuments` public return shape stays stable for existing consumers.
- No backend contract/schema changes.

### Implicit invariants (inferred)
- Dashboard expects loading/error/documents/stats semantics from `useDocuments`.
- Frontend tests should run headlessly with Vitest + jsdom.

### Non-goals
- No Rust-side refactors or DB migration changes.
- No UI redesign.
- No new API provider logic.

## C) Proposed Changes by Theme (Prioritized)
### Theme 1: Hook test coverage
- **Current approach:** No direct tests for `useDocuments` behavior.
- **Proposed change:** Add `src/hooks/useDocuments.test.ts` for initial load success/failure and delete-refresh flow.
- **Why:** Covers critical app state transitions and catches regressions early.
- **Tradeoffs:** Slightly longer test runtime; improved confidence outweighs cost.
- **Scope boundary:** Frontend tests only.
- **Migration approach:** Add isolated mock-based tests without touching runtime contracts.

### Theme 2: Hook error-state consistency
- **Current approach:** refresh failure sets only `error`; delete failure does not update hook error state.
- **Proposed change:** On refresh failure clear stale documents/stats; on delete failure set error and rethrow.
- **Why:** Prevent stale data presentation and preserve predictable hook state.
- **Tradeoffs:** Consumers may now see hook-level error after delete failures (desirable consistency).
- **Scope boundary:** `useDocuments` internals only.
- **Migration approach:** Non-breaking internal behavior improvement validated by new tests.

## D) File/Module Delta (Exact)
- **ADD**
  - `src/hooks/useDocuments.test.ts` — unit tests for hook behavior.
- **MODIFY**
  - `src/hooks/useDocuments.ts` — error handling consistency in refresh/delete paths.
  - `codex/*.md` — session artifacts for plan/log/verification/checkpoints/changelog.
- **REMOVE/DEPRECATE**
  - None.

## E) Data Models & API Contracts (Delta)
- Current models/contracts in `src/types/index.ts` and Tauri invoke commands unchanged.
- No schema/interface changes.
- Compatibility is fully preserved.
- No migrations required.

## F) Implementation Sequence (Dependency-Explicit)
1. **Objective:** Establish baseline + discovery evidence.
   - Files: `codex/VERIFICATION.md`, `codex/SESSION_LOG.md`, `codex/CHECKPOINTS.md`
   - Preconditions: repo cloned and dependencies installed.
   - Verification: `pnpm test`, `pnpm build`, `cargo test --manifest-path src-tauri/Cargo.toml`
   - Rollback: remove artifact entries if baseline logging is inaccurate.
2. **Objective:** Add hook tests.
   - Files: `src/hooks/useDocuments.test.ts`
   - Dependencies: baseline green for frontend.
   - Verification: `pnpm test src/hooks/useDocuments.test.ts`
   - Rollback: remove test file.
3. **Objective:** Harden hook error handling.
   - Files: `src/hooks/useDocuments.ts`
   - Dependencies: test scaffold exists.
   - Verification: `pnpm test src/hooks/useDocuments.test.ts`, `pnpm test`
   - Rollback: revert hook edits.
4. **Objective:** Final hardening verification + artifact updates.
   - Files: `codex/*.md`
   - Verification: `pnpm test`, `pnpm build`, `cargo test --manifest-path src-tauri/Cargo.toml`
   - Rollback: revert documentation-only deltas.

## G) Error Handling & Edge Cases
- Current pattern: convert async errors to string in hooks/components.
- Proposed improvement: ensure failed refresh clears stale state; failed delete records hook error while preserving throw semantics.
- Edge cases covered:
  - initial load failure
  - delete path with follow-up refresh
  - successful initial load semantics

## H) Integration & Testing Strategy
- Integration points: `useDocuments` with command layer (`@/lib/commands`).
- Add unit tests for hook behavior with mocked command functions.
- Regression confidence via full frontend test run + build.
- Definition of Done:
  - new hook tests pass
  - all existing frontend tests pass
  - build passes
  - known Rust env limitation documented

## I) Assumptions & Judgment Calls
- Assumption: stale documents/stats should not persist after refresh failure.
- Assumption: exposing delete failure through hook error is useful and non-breaking.
- Judgment call: avoid backend changes due explicit environment blocker and scope minimization.
