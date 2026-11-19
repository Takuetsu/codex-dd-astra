# CodexDD 0.4.6 — Packet 3C.1b.2b: Core-native independent Validator producer

**Date:** 2026-11-12. **Parent:** `a38d8585b9e3e77c1618457c436aa3b708586a4e` (3C.1b.2a exact Rustfmt repair; Windows acceptance pending at drafting).
**State:** SOURCE CHECKPOINT ONLY — native Windows compilation and focused tests PENDING; fresh Worker handoff and live independent READY unproven. Full 3C.1 and 0.4.6 remain OPEN.

## Bound outcome

Introduce a distinct Core built-in `run_codexdd_independent_validation` producer for an explicitly pinned **Validation** Worker, without giving it the Implementation-only `run_codexdd_validation` tool. The producer accepts only `{"profile":"independent_review"}` and resolves only the active, current repository's `scripts/codexdd-test-independent.ps1`. It uses the same guarded native execution path and independently captured process exit code as existing LVO, with a different native tool name/profile, unique run ID and Core-generated completion notification. Never accept a generic `exec-*` result as this receipt.

## Authority and failure boundaries

- Core registration requires role Validation, source-change task, nonblank bounded authorized scope, pinned source root/SHA, and a normal worker turn; terminalization remains report-only. A direct invocation repeats these checks even if a tool registration is accidentally exposed.
- Both before and after native execution, Core resolves the selected Git worktree's canonical root, verifies exact clean HEAD and owner-bound root/SHA, and never falls back to a parent Git repository. Missing role, scope, script, source, native output, process exit or approval becomes a non-PASS setup blocker.
- Independent script output must obey existing `CODEXDD_VALIDATION_JSON` contract v1 with fixed profile `independent-review`, successful nonempty stage list, independently observed exit 0 and status PASS. Failure never creates a repair fingerprint or a successful evidence record.
- The receipt is emitted only by the distinct built-in Core handler, with `tool=run_codexdd_independent_validation`, `profile=independent_review`, `head_sha`, `source_root`, `run_id` and `result`, and never fabricates READY. The TUI contract from 3C.1b.2a independently checks this identity against the current Validator Worker and owner-specified gate.
- No source changes to BREAKWATER/CV, no installed binary updates, no 0.4.5 unfreeze, and no production source promotion.

## Focused native proof

New exact-prefix `codexdd_046_3c1b2b_` Core tests check (1) fixed profile and rejection of Implementation tool impersonation, (2) refused missing/unbound/wrong roles or scopes, and (3) exact clean Git root and SHA versus dirty/wrong-source rejection. An isolated opt-in Windows GitHub job requires exactly **three enumerated + three executed** Core tests, after source preflight and Rustfmt. The test's source identity is the exact development SHA; no 21-stage release suite.

**Limits:** No real independent native PASS, actual separate Validator handoff, owner-authorized test profile for BREAKWATER, proven TUI admission, runtime service health, or game project readiness from unit tests. Packet 3C.1b.2c must connect fresh Worker authorization to receipt observation and task-specific evidence. Do not close 3C.1 or mark owner QA READY until a real independent session, exact-source profile execution and necessary operation gates pass.
