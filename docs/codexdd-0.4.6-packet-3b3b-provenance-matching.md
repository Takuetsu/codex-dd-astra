# CodexDD 0.4.6 — Packet 3B.3b: strict native source-gate provenance

**Date:** 2026-11-11  
**Base:** `0465dc95d8f78183c796ab6034f7e46e6ab2825d` (3B.3a).  
**State:** FOCUSED WINDOWS NATIVE REGRESSIONS ACCEPTED (2026-11-11); 3B.3b closed; independent real-runtime acceptance remains OPEN.  
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`; production remains 0.4.4 and 0.4.5 frozen.

## One observable outcome

The native `run_codexdd_validation` Core handler now includes the canonical **selected source root** alongside the existing actual HEAD SHA, run ID, profile and native result. The selected root comes from Core's repository discovery and pre-/post-run source identity checks, **not** from model prose or user-invented receipt arguments. An unavailable canonical root is unknown, not fabricated.

The TUI evidence registry now exposes an **opt-in, strict matcher** for current, source-bound native validation receipts. Only a complete built-in LVO PASS can carry root/SHA/run/time provenance; normal command/MCP success, legacy partial tool completions and unknown source evidence remain diagnostic only. The live TUI ingestion snapshots the immutable first-assignment Implementation Worker role/scope/task-kind/pin separately from the Core-observed values. The matcher compares both against the independently specified gate requirement.

## Matching contract

Required facts: current Worker thread and role, assigned scope and source-changing task kind, canonical authorized worktree/root, exact 40-hex candidate SHA, native profile (`targeted`, `work_packet` or `release`), successful native result, run ID when requested, optional same-turn identity, and a completion timestamp not earlier than the task's accepted time floor.

Missing or mismatched facts return typed rejections: `MissingProvenance`, `WorkerBindingMismatch`, `SourceIdentityMismatch`, `GateScopeMismatch`, `RunIdMismatch` and `StaleEvidence`, while preserving earlier `CrossThread`, `SourceTurnMismatch`, `Conflicted` and `OutcomeMismatch`. A focused `targeted` PASS cannot satisfy a `work_packet` or whole-release gate, and a native source PASS is not a service health check or owner QA.

The matcher deliberately **does not yet replace** Validation Worker's broad `ready_for_owner_qa` terminal rule or introduce a pending prerequisites ledger. This belongs to **3B.3c**, with A01/A02/A07 tests. Durable snapshot/resume changes belong to 3B.3d. Existing Implementation `work_packet`/targeted same-turn handoff checks, Repair/Validation tool restrictions and Core policy blocks are unchanged.

## Focused regression cases added (not yet executed)

- `codexdd_046_3b3b_a03_requires_exact_source_sha_and_worktree`: exact match positive; old SHA/wrong worktree rejected
- `codexdd_046_3b3b_a04_rejects_other_worker_scope_and_narrower_gate`: wrong Worker/thread/role/scope or narrower profile rejected
- `codexdd_046_3b3b_legacy_unpinned_and_generic_success_are_not_gate_proof`: no native provenance or Worker pin cannot satisfy source gate
- `codexdd_046_3b3b_run_timestamp_and_turn_must_match_gate`: run, timestamp and source turn bound
- `codexdd_046_3b3b_provenance_conflict_is_permanently_inadmissible`: conflicting evidence ID stays rejected
- `codexdd_046_3b3b_native_pass_without_exact_fields_cannot_bind`: malformed / partial native facts fail closed

A small existing Core operator-summary assertion was updated to check the new `source_root` field. Tests use disposable temp roots and fake completion objects, never a private game checkout, external process, host listener or fabricated operational PASS.

## Validation status, nonclaims and handoff

Performed before commit: exact GitHub parent/ref and required source ownership inspected; all five modified files were staged as Git blobs from the same parent; replacement anchors, regression selectors and workflow trigger rules checked. No Rust toolchain or direct access to Daniel-CL is available from this editing environment; **Rustfmt, Cargo compile and native Windows regression cases have not yet been executed or accepted.** This is a recoverable, nonproduction **source-checkpoint awaiting focused Windows proof**, not a release/gate PASS.

The existing GitHub push workflow's ordinary source-contract and formatting jobs may automatically execute when these Core files move to the development branch. Its Windows native jobs require separate explicit commit markers and are not dispatched here. Neither auto-run source checks nor prior 3B.2o green receipts replace new exact-head native acceptance.

Next: run minimal Windows exact-head TUI tests (selector `codexdd_046_3b3b_`), existing adaptive evidence tests and relevant Core handler regressions on Daniel-CL after format/source checks; repair any compiler/format faults narrowly. Only after that should the 3B.3b native gate be called accepted. 3B.3c may then implement explicit source/operations/independent-Validator prerequisites, and 3B.3d durable pending-gate resume. The private BREAKWATER Node profile, independent live Validator, 3B.4 and all release gates remain open.

## 2026-11-11 authoritative 3B.3b closure

**Accepted exact source:** `a249ab5bf06839f07219a05510260739f4252abf`. [GitHub Actions run #43054633002](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43054633002) completed `success`; source-contract and Rustfmt jobs `success` and the exact-source Windows Core/TUI provenance job `success`. Native job logs explicitly show Core operator summary test `1 passed, 0 failed` and TUI provenance regressions `6 passed, 0 failed` (seven focused tests total). The corrected branch fixed the ten historical TUI `AdaptiveEvidenceRecord` fixture initializations after earlier run #43048358426 failed compilation; superseded run #43048453466 also failed and is not accepted as green.

**Accepted scope:** 3B.3b matching/negative controls A03/A04 and Core receipt source root. This is a native GitHub Windows **synthetic fixture** gate, not an owner-required full regression, actual preview health, independent live Validation Worker, private BREAKWATER Node profile, release profile, installation or production merge. The 0.4.6 milestone and overall 3B.3 phase remain OPEN. Keep these distinctions in summaries and later gate ledgers. This closure updates the initial pre-acceptance statements above; historical test-state descriptions remain as provenance of the original checkpoint.

**Next:** 3B.3c explicit required source/operations/independent-Validation gate states and missing-gate regression cases, followed by 3B.3d durable source-bound resume and Phase 3C.1 trusted terminal admission.
