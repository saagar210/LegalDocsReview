# Decisions

## 2026-02-10
1. **Scope selection:** prioritize frontend hook hardening + tests over backend changes.
   - Rationale: backend Rust tests are environment-blocked; frontend path allows verifiable, safe improvement.
   - Alternative rejected: speculative Rust changes without full verifiability.

2. **Hook behavior update:** clear stale `documents`/`stats` when refresh fails.
   - Rationale: keeps state consistent with failure status and prevents stale render assumptions.
   - Alternative rejected: preserve stale data alongside error.

3. **Delete error propagation strategy:** set hook `error` then rethrow.
   - Rationale: maintains compatibility for callers relying on thrown errors while improving hook observability.
   - Alternative rejected: swallow error and return boolean.
