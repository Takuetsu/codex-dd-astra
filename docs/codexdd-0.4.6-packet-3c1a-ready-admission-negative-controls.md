# CodexDD 0.4.6 — 3C.1a: fail-closed READY admission (negative controls)

**Date:** 2026-11-11. **Base:** `fbd1d1ca3b82a6769c10c343bc6517fa21ab1d13` (3B.3d.2 native acceptance documented).
**State:** SOURCE / TEST CHECKPOINT ONLY; no exact-head Rustfmt or native Windows compile/test proof yet. 3C.1 and 0.4.6 remain OPEN.

## Outcome and boundaries

Block the known false-positive terminal in the current native TUI `ReadyForOwnerQa` path. A bound Validation Worker with an arbitrary successful `exec-*` is not sufficient evidence that the owner-required source profile, owned runtime/health, or independent Validator acceptance has been satisfied. Keep the successful focused source facts in the evidence registry and name outstanding requirements; do **not** manufacture READY, mark an incomplete gate as a product failure, or automatically escalate the model solely because READY was rejected.

**Owned code:** `tui/src/chatwidget/adaptive_trusted_signal.rs`, `tui/src/adaptive_gate_prerequisites.rs`, plus the existing test module. Production 0.4.4 and frozen 0.4.5 untouched. No private BREAKWATER/CV checkout, privileged process, source mutation, release or installation.

## Admission contract

Before the existing terminal latch, a successful `ready_for_owner_qa` signal must also have: first-assignment-locked Validation Worker; nonempty authorized task scope; exact assigned source pin; explicitly bound required-gate ledger; matching scope/root/SHA; **zero outstanding required gates** in the current evidence registry. The ledger always retains independent Validation as mandatory; actual independent Validator and owned-health evidence producers have not yet been wired, therefore no synthetic test in this packet grants positive owner READY.

An invalid READY attempt is cancelled without terminal latch or `SetReadyForOwnerQa` persistence. Preserve the evidence and show the missing/blocked gate. Do not drive capability failure pressure or a new autonomous successor simply to hide a pending prerequisite. RepairRequired and ReadyForValidation transitions are not loosened.

## Tests (written; Windows pending)

- Replace the unsafe legacy `green_reviewer_terminal_overrides_armed_failure_pressure_and_admits_no_successor` oracle: its arbitrary green command must **not** latch READY even after unrelated failures; no automatic escalation.
- `codexdd_046_3c1a_green_review_still_requires_full_and_independent_gates`: a pinned, locked Validation Worker and generic successful command leave an explicitly assigned full source-release, real health, and independent Validation gate pending.
- `codexdd_046_3c1a_foreign_worktree_cannot_acquire_owner_qa_from_green_exec`: source pin and required worktree differ; reject without terminal even if a command reports success.

## Important incomplete work

This checkpoint implements the **negative terminal admission boundary only**. Positive admission requires an independently bound Validator evidence producer (3C.1b), owner-authorized gate assignment/transfer to the Validation Worker, and a verified owned runtime-health producer (3D.1) when required. Restored legacy READY snapshots and direct app-server persistence authority also need separate 3C.1 review. Do not claim the 0.4.6 READY workflow is usable or accept a release from a green synthetic test.

Next: exact-head source/Rustfmt plus focused native Windows `codexdd_046_3c1a_` and old READY/RepairRequired controls, then implement the positive admission and bounded recovery. Cheap local Windows checks before broad CI remain preferred; GitHub-only diagnostic jobs must remain narrow and source-scoped.
