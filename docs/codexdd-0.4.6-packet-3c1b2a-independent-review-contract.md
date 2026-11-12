# CodexDD 0.4.6 — packet 3C.1b.2a: independent Validator provenance contract

**Date:** 2026-11-11. **Parent:** `742f2cd5f09d13249d9bfb5880dc3a66400b8108` (3C.1b.1 native acceptance recorded).
**State:** SOURCE IMPLEMENTATION + SYNTHETIC MATCHING TESTS ONLY; Rustfmt, native Windows and real Core producer acceptance still PENDING. Full 3C.1 and 0.4.6 remain open.

## One outcome

Define and enforce a strict matcher for evidence produced by a **separately authorized, Core-native Validation Worker**. This is contract scaffolding only. The dedicated `run_codexdd_independent_validation` Core tool does **not** exist yet, is not exposed to models, and must not be simulated with an arbitrary `exec-*` success. No end-to-end positive READY is authorized by this packet.

## Owned files / scope

- `codex-rs/tui/src/adaptive_evidence.rs`: reserve the distinct native tool/profile identity; accept native source provenance only for a completed successful profile-native tool result with a canonical-root-ready SHA, run ID, and completion timestamp. Bind first-assignment role/scope/source identity only if (Implementation + native LVO) or (Validation + the dedicated independent review producer). Match independent review by **different Implementation and Validation thread IDs**, current Validator turn ID, first-assignment scope, source canonical root, exact SHA, native PASS status/profile, run ID, and minimum completion time.
- `codex-rs/tui/src/adaptive_gate_prerequisites.rs`: allow an exact independently matched review to be recorded as an **in-memory reference**, re-check it on each outstanding-gates query, and revoke it on conflict or stale source. No persisted successful record: checkpoint saves only required gates and bounded blockers, and restart restores all gates as pending.
- `codex-rs/tui/src/adaptive_evidence_tests.rs`: three deterministic synthetic record tests for matching distinct Validator identity and restart downgrade; generic/same-worker/Implementation-only successes rejected; source/run/time mismatch and conflicting ref revoking a prior match.
- `.github/workflows/codexdd-046-3b1-github-regression.yml`: exact source, narrow 3C.1b.2a three-test selector alongside existing 24 controls. GitHub-only diagnostic follows the operator-established 3B.3/3C.1 focused approach; no broad 21-stage run, prod install, game-worktree access or expensive full build gate.

## Important limitations and next packets

**This is not a live Core independent review.** A valid native event must eventually come from a separately registered and Core-verified built-in producer, with Role=Validation, exact bound source identity, and observable task-specific test/quality proof. Only that Core producer can mint trusted native review completion metadata. It must not weaken the Implementation-only `run_codexdd_validation` policy or bypass sandbox approvals.

**Not yet wired:** owner-authorized required-gate assignment across the fresh Worker handoff, secure source receipt transfer/revalidation, the native Validation-only producer, the TUI observer that records its receipt, and task-specific owned live-health proof (3D.1). Until each required gate is truly satisfied, `owner_qa_prerequisite_admission` still refuses READY. Positive matcher unit tests use simulated trusted notifications, **never a genuine independent session/production PASS**.

**Next:** after focused native compilation acceptance, implement a separately authorized Core independent review tool (3C.1b.2b), then its source-bound fresh-Worker handoff and actual evidence admission (3C.1b.2c), preserving no legacy READY replay. Run real disposable-host integration before calling 3C.1 complete. Stop on unsupported role/source/permissions; do not forge acceptance, use stale CI green as task proof, or touch BREAKWATER/CV.
