# CodexDD 0.4.6 — 3B.2i-a read-only descendant candidate verifier (WIP)

**Verified predecessor:** `42bc7152fbb2faf548d55cd6b9bbbaa0b86bba16`, exact-head GitHub Actions run #42635292402 green (source preflight, Rustfmt, Windows Core/TUI build and focused regressions).
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.

## Scope
This checkpoint adds a prospective `[codexdd_candidate]` proposal grammar requiring explicit previous and candidate 40-hex Git SHAs and a nonempty owner instruction body. Parsing alone is **not authentication or authorization**. The candidate verifier operates only on an existing durably latched first-user Implementation Worker with a source-change task and an exact source root/SHA. It read-only checks the original authorized SHA, canonical worktree root, observed HEAD, clean staged/unstaged/untracked status and candidate ancestry via `git merge-base --is-ancestor`. It returns a prospective snapshot with only the source SHA modified; no Core runtime state, persisted record, role, scope, task kind, or source root changes.

Three new `codexdd_046_3b2i_*` focused regressions cover strict proposal parsing, a clean descendant without rebinding, and fail-closed cases for unbound or launcher-only Worker, wrong previous/current SHA and dirty candidate (edits preserved). The existing branch-scoped Windows workflow enumerates and executes exactly three new tests.

## Unresolved authorization boundary
This is **3B.2i-a only**: no candidate owner-authenticated turn-input admission, no live authority transition, and no durable transition checkpoint. Next, implement a separately authorized idle-turn Core management path, ensure append-and-flush before runtime rebinding, reject steered/model/tool origin, reconcile the TUI's original SHA with the new candidate, and test interruption, resume, valid/invalid owner authority, and native Node/Rust protected profiles. A proposed SHA or Git HEAD change alone must never authorize MechanicalValidation.

## Validation
This source packet has been assembled using GitHub repository APIs; no local Rust toolchain or Daniel-CL SSH validation was available in this session, so Rustfmt/compiler correctness **is not yet established**. Required next steps are cheap local source formatting/compile and regression tests on the exact committed SHA before broader GitHub CI, then real Windows Node/Rust and independent Validator acceptance. No 0.4.5 thaw, production source change, release, version bump, game worktree modification or installation.
