# CodexDD 0.4.6 — packet 3C.1b.1: legacy READY replay quarantine

**Date:** 2026-11-11. **Base:** `24b596110cba9c3d3b849d155413fbca0b06f1d0` (3C.1a focused Windows accepted).
**State:** NATIVE ACCEPTED 2026-11-11 — 3C.1b.1 CLOSED. Full 3C.1, positive independent Validator and 0.4.6 remain open.

## Outcome and authority

Prevent a previously persisted `ready_for_owner_qa` string or an older standalone `SetReadyForOwnerQa` rollout item from restoring completion authority when the fresh in-memory native evidence registry is empty. This is 3C.1 bounded-recovery work, not positive trusted terminal admission.

## Smallest owned implementation

Change only `codex-rs/tui/src/session_state.rs` plus a narrow opt-in Windows test step. A snapshot with a previously saved READY terminal (with or without new `gate_checkpoint`) must return without a terminal, stay explicitly `paused_by_user`, have no processed READY turn, pending attempt, pending signal, or successor admission. Keep worker role, source pin, route and any unfinished/blocked requirements intact; do not fabricate new PASS or re-run a previously completed validation. Other durable terminals (`ready_for_validation`, `repair_required`, `blocked`) are preserved by their existing independent contract.

The standalone legacy rollout READY enum branch uses the same fail-closed pause. Existing backward-compatible JSON decoding is preserved, but decoding is not proof of current independent validation. No source mutation in BREAKWATER/CV, no privilege/role changes, no release/prod install, no frozen 0.4.5 import.

## Regression expectations (unexecuted at source checkpoint)

- `codexdd_046_3c1b1_standalone_legacy_ready_is_held_without_evidence`: standalone replay is never a terminal; remains paused with empty ephemeral authority.
- `codexdd_046_3c1b1_legacy_ready_snapshot_with_source_pin_is_not_readiness`: legacy snapshot with matching source and an old READY string decodes safely, but source pin alone does not restore an owner-QA terminal.
- Updated existing `restored_snapshot_preserves_non_ready_terminals_but_quarantines_unverified_ready`: the three independent non-READY terminal values remain compatible while old READY is deliberately quarantined.
- Re-run exact-source earlier 3B.3b–d and 3C.1a focused controls. A green prior checkpoint is not a substitute for running these changed source tests.

## Outstanding work and stop conditions

Positive 3C.1b must still establish a distinct, owner-authorized Validation Worker with current source-root/SHA, a trusted native independent review producer and appropriately approved whole-task proof. The required-gate ledger must not be made green by a generic shell command, persisted JSON, model prose, or the earlier Implementation Worker’s own validation. 3D.1 still owns actual listener ownership and HTTP response. No synthetic READY or production release is authorized from this packet.


## Exact-source native acceptance — 2026-11-11

[GitHub Actions #43130288430](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43130288430) completed **SUCCESS** at exact commit `6854d70ab94f59329b7771098177832e3696119d`. Source-contract preflight PASS, Rustfmt PASS, and scoped Windows native tests PASS: Core receipt 1/1; 3B.3b 6/6; 3B.3c 4/4; 3B.3d.1 3/3; 3B.3d.2 3/3; 3C.1a 4/4; new 3C.1b.1 legacy READY replay 2/2 and updated non-READY terminal compatibility 1/1: **24/24 focused tests**, 0 failures. Other workflows deliberately skipped. This is not an independent live Validator, actual ops/health service, Node project validation or release approval.
