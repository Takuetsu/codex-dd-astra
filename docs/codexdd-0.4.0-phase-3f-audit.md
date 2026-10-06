# CodexDD 0.4.0 Phase 3F.1 Local Validation Orchestrator audit

## Status

**SOURCE AUDIT COMPLETE; PENDING DANIEL-CL 3F.1 WORK-PACKET VALIDATION.**

This document is the durable implementation audit for the CodexDD 0.4.0 Local Validation Orchestrator (LVO).

## Anchors

- Production / frozen base: `06465383abb1acf7c74c0a84b00726853b74fdc4` (CodexDD 0.3.14).
- Feature branch: `dd/codexdd-v0.4.0-local-validation-orchestrator`.
- Target platform: Windows.
- Upstream workspace version remains `0.159.2`.
- Product version is now `0.4.0`.

The feature branch is a direct descendant of the frozen production base. The audited commit history contains no upstream-sync, rust-version refresh, or merge commit. `codex-rs/Cargo.toml` and `Cargo.lock` are not part of the 0.4.0 feature diff.

## Exact feature inventory

The production-to-feature comparison contains 45 paths, all attributable to the LVO feature or its version/documentation transition:

- 9 generated app-server schema / TypeScript / Python SDK artifacts;
- 8 core runner, registration, dispatch-gate, and regression paths;
- 11 TUI adaptive evidence/lifecycle/status/persistence paths;
- 5 app-server/history/rollout persistence paths;
- 5 repository-owned validation PowerShell scripts;
- 4 LVO/current-policy documentation paths;
- 3 product-version/provenance regression paths.

There are no unexplained feature-only paths and no temporary GitHub oracle workflows in the intended final tree.

## Runner and shell-boundary audit

The bounded runner remains narrow:

- model-visible input is the fixed `targeted | work_packet | release` enum;
- arbitrary shell text is not accepted by `run_codexdd_validation`;
- each profile maps to one conventional repository-owned script;
- profile timeouts are fixed;
- repository discovery requires the expected repository markers plus the selected profile script;
- branch and HEAD identity are collected natively;
- execution delegates through the existing unified exec/sandbox/approval infrastructure;
- the internal delegated exec uses a distinct call ID from the native LVO receipt;
- full logs are stored outside the source worktree under bounded CodexDD runtime storage;
- retention remains capped at the 20 newest runs per repository;
- malformed/missing contract output and execution failures fail closed as infrastructure errors.

Protected mechanical validation remains source-edit-free. The normal shell allowlist retains bounded read/test/build commands, but direct execution of `codexdd-test-targeted.ps1`, `codexdd-test-workpacket.ps1`, or `codexdd-test-release.ps1` is explicitly rejected so the model cannot bypass the native receipt path.

The LVO tool itself is registered only when the trusted turn trigger is mechanical validation.

## Receipt and stale-authority audit

Implementation handoff authority remains native and same-turn:

- `ready_for_validation` for an Implementation Worker requires a successful native `work_packet` LVO receipt from the same thread and source turn;
- after an admitted repair, the same mechanical-validation turn must contain both a successful `targeted` receipt and a successful `work_packet` receipt;
- targeted-only, missing, failed, cross-thread, and cross-turn receipts cannot authorize handoff;
- `release` cannot substitute for the required `work_packet` receipt;
- source edits are blocked during the protected validation turn, so a successful same-turn receipt cannot be followed by an authorized source edit before handoff;
- leaving validation for a repair creates a later validation turn, so old PASS evidence cannot authorize the repaired candidate.

Native tool evidence, not final-answer prose, remains authoritative.

## Repair-loop audit

The repair policy remains bounded and global to the active work packet:

- the initial repair can only originate from a failed `work_packet` receipt;
- after repair state is active, a failed `targeted` or `work_packet` receipt may request the next repair;
- infrastructure/contract errors carry no repairable fingerprint and cannot authorize source edits;
- deterministic evidence selection prefers a `work_packet` failure over a targeted failure regardless of evidence-ref ordering;
- every admitted source-changing repair increments the same global repair-cycle counter, even when the failed stage/fingerprint changes;
- the budget is two source-changing repair cycles;
- a third otherwise-valid repair request latches `Blocked` rather than admitting more source edits;
- successful handoff clears the repair fingerprint, cycle count, and targeted-retest requirement.

## Persistence / resume / fork audit

Durable repair state and compact operator status remain in the canonical workflow-state path.

Restore fails closed when:

- repair cycles exceed the two-cycle limit;
- repair state is incomplete or internally inconsistent;
- repair state is attached to a non-Implementation Worker or lacks accepted complexity / implementation lifecycle state;
- compact validation status is incomplete or invalid.

Ephemeral authority such as the evidence registry, pending signals, pending attempts, and successor permits is not restored.

The existing detached-fork regression carries the LVO repair state and compact validation status into the child while clearing ephemeral successor authority.

## Status/operator-evidence audit defect

3F.1 found one real operator-evidence defect.

The native LVO receipt already contained `failure_fingerprint` on a repairable validation failure, but the compact persisted validation status did not retain that field. As a result, `/status` could show:

- the failed profile;
- failed stage;
- run ID;
- branch/SHA;

while still reporting `LVO failure fingerprint: None` until `implementation_work` had already been accepted and the repair-state fingerprint was populated.

That was observability drift, not an authority bypass, but it violated the 3D.1 operator-evidence contract.

### Repair

The compact validation status now carries an optional native receipt `failure_fingerprint` through:

- native receipt ingestion;
- in-memory adaptive state;
- app-server workflow protocol;
- rollout/history persistence;
- resume restore;
- detached fork;
- generated schema/SDK surfaces.

`/status` now prefers the latest native failed-receipt fingerprint and falls back to the active repair-state fingerprint. This makes the failure identity visible immediately without consuming repair budget or changing repair admission.

The persisted status rejects a blank fingerprint or a fingerprint attached to a non-`fail` result.

A dedicated status regression proves that a failed receipt is visible while repair usage is still `0/2`.

## Consequential gate audit

The LVO feature does not authorize or automate:

- GitHub promotion/CI;
- merge or squash-merge;
- production installation;
- release publication;
- deployment;
- destructive Git operations;
- external-machine changes.

The `release` validation profile is only a local validation workload. It does not grant any release action.

## GitHub-side repair validation

The audit repair is green:

- Rust formatting check: PASS;
- validation-status constructor scan: PASS;
- `codex-history` library-test compile: PASS;
- `codex-tui` library-test compile on Windows: PASS;
- app-server protocol/schema and Python SDK regeneration: PASS;
- generated artifacts include optional `failureFingerprint` on the compact validation status;
- all temporary 3F.1 formatter/compile/schema oracle workflows were removed after the successful runs.

The first Daniel-CL `work_packet` attempt then stopped at the `rust-format-check` stage before running later validation stages. The exact repository formatter was run through a temporary GitHub oracle; it changed exactly one Rust path, `codex-rs/core/src/tools/handlers/mod.rs`, by one formatting-only line replacement. No behavioral code changed. The temporary formatter oracle was removed immediately afterward.

The second Daniel-CL `work_packet` passed formatting and then stopped at the exact `core-tui-clippy` stage. Two redundant closures in the LVO runner were fixed first. An exact Windows stage oracle then exposed six additional TUI Clippy errors: four panic-path `expect()` violations in LVO-touched adaptive code and two redundant closures in unchanged TUI app/test code. The adaptive cases were changed to fail closed instead of panicking, and the two unchanged TUI closures were mechanically simplified.

The exact Windows work-packet rustfmt stage and exact `cargo clippy --tests -p codex-core -p codex-tui` stage both passed after those repairs. The temporary stage oracle was removed immediately afterward. No Rust source changed after that green strict stage run.

The next Daniel-CL `work_packet` is therefore the authoritative final 3F.1 gate.

## Pending 3F.1 closure

Rerun the repository-owned `work_packet` profile on Daniel-CL against the strict-format/Clippy-clean branch. Phase 4 remains separate and owns the broader pre-install release matrix.
