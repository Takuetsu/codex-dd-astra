# CodexDD 0.4.6 — 3C.1b.2j2: App-owned fresh Validator handoff

**Opened:** 2026-11-12. **Parent:** 3C.1b.2j1 accepted at \`58dd0a3e06f4fbc619f7fc5d85209c36298f2af2\`, [GitHub Actions #43347103729](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43347103729). **Status:** source and focused native Windows verification PENDING. 3C.1, 0.4.6 and any installation remain OPEN.

## Scope and ownership

Close the App-owned **automatic** \`ReadyForValidation\` → new Validator session boundary: one fresh thread, exact independent \`Validation\` role and approved scope, copied source pin and original owner-gate requirements, chosen runtime CWD equal to the explicit authorized nested Git worktree (not the launcher milestone root), Core \`thread/start\` seeded with immutable \`adaptive_worker\` startup config, and one initial user-origin independent-review prompt. The new Validator never inherits the Implementation terminal/PASS; both owner gates remain pending until separate native proof.

Owner files: \`codex-rs/tui/src/app/session_lifecycle.rs\` and its focused integration tests in \`codex-rs/tui/src/app/tests/session_lifecycle_requests.rs\`. No change to Core profiles or release products.

## Verified gap in original implementation

\`start_fresh_session_with_worker_binding\` only preselected a pin when \`fresh.review_origin_thread_id\` existed. But \`fresh_for_new_worker\` can retain an assignment preflight while \`inherit_authorized_independent_review\` rejects an **unavailable**, **foreign** or **mismatched** owner ledger; in that case \`review_origin_thread_id\` is absent and the old path could start a fresh Validator at the launcher's default CWD. Even when an inherited root exists, the old start path canonicalized it but did **not recheck the exact current Git HEAD/clean state before thread/start**. Both violate source-fail-closed contract.

The production fix must block an automatic handoff unless a live source pin, original owner gate ledger and source origin ID are intact; additionally run the existing bounded read-only Git root/SHA/clean preflight on the selected candidate **before** starting the new Core thread. Do not transfer successful receipts or manufacture owner proof. Explicit manual new-session behavior with unrelated binding remains unchanged.

## New bounded regressions

- \`codexdd_046_3c1b2j2_app_starts_pinned_fresh_validator_with_owner_gates\`: on a disposable committed Git root deliberately distinct from launcher CWD, set a typed **already accepted** source terminal/owner-gate fixture, invoke the real TUI \`AppEvent::NewSession\` handler using the auto-generated binding, and inspect real embedded app-server \`thread/start\` JSON-RPC recording. Assert new thread ID, exact Validator role/scope/source SHA/root/task kind, initial independent prompt in its own \`UserTurn\`, copied but still pending owner gate ledger and no READY or synthetic native PASS.
- \`codexdd_046_3c1b2j2_auto_validator_refuses_unpinned_moved_or_drifted_source\`: three adversarial conditions: source owner ledger absent; owner root removed after terminal; HEAD advanced after approval. Assert **zero** \`thread/start\` requests, source thread still selected, launch CWD unchanged, and actionable \`SETUP_BLOCKED\` rather than wrong-root fallback.

Only the dedicated, commit-message-gated \`[3c1b2j2-native]\` Windows job can accept this source checkpoint. It must first pass source contract and Rustfmt and then enumerate exactly **two** \`codex-tui --lib\` tests on Windows. The typed READY fixture here does **not** replace genuine Core/native receipt proof from 3C.1b.2i or a full interactive TUI process. No installed binaries, BREAKWATER/CV, or game worktrees change.

## After green

Then proceed to 3C.1b.2j3: cold restart/replay/changed-SHA refusal through the complete App lifecycle, followed by 3C.2, remaining 3B.4/3D–3F and Phase 4 full native/release gates. The planned one-milestone ~one-week BREAKWATER soak starts **only after** completed 0.4.6 release acceptance and owner installation authorization.
