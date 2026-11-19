# CodexDD 0.4.6 — 3B.3c.1 explicit prerequisite-ledger foundation

**Date:** 2026-11-11. **Parent:** `4cd38f4da3e1bc430e0b046b8b31f8357f01ab7c` (documented 3B.3b native acceptance).
**State:** 3B.3c.1 FOCUSED WINDOWS SOURCE/RUSTFMT/NATIVE ACCEPTED on `9b159f54e7b5582f72b95db8e3434322dc73593b`; full 3B.3c and milestone remain IN PROGRESS.

## Packet

- **Outcome:** Represent exact authorized source-profile, owned live-health and independent Validator prerequisites without silently upgrading a narrow success to full readiness.
- **Authority:** 0.4.6 P0/A01, A02, A07, and the 3B.3a handoff. This is a pure in-memory proof model for an already authorized source/scope, not authority to create a new Worker, run services or grant terminal READY.
- **Owned paths:** `tui/src/adaptive_gate_prerequisites.rs`, its module declaration in `tui/src/lib.rs`, and focused synthetic cases in `tui/src/adaptive_evidence_tests.rs`. Documentation here.
- **Excluded:** TUI `ReadyForOwnerQa` admission, Core Worker role changes, persistent snapshot/rollout schema, project or host operations, runtime service start/kill, benchmark route, frozen 0.4.5 and production release.
- **Stop/hand-off:** Commit exact source, obtain cheap source/Rustfmt and focused Windows tests; repair formatting/compiler failures narrowly. 3B.3d will handle persisted handoff and 3C.1 terminal admission; owned health proof belongs to 3D.1.

## Invariants

The task's required gates are explicit: `SourceProfile(targeted|work_packet|release)`, `OwnedRuntimeHealth(approved_ports)` and exactly one `IndependentValidation` gate. A source-only task need not require operations, but every owner-QA path must require independent validation. Empty scopes, invalid SHA/root, duplicate/unsupported source profiles, duplicate health ports and missing/duplicate independent validation are rejected.

Source gate observation uses 3B.3b `match_native_source_gate` and checks exact bound source-root/SHA, Implementation role/scope, source turn, run ID, native profile and timestamp. Source evidence remains a reference; on every readiness read it is checked again against the live registry. A later conflicting ID or stale/mismatched proof turns the gate pending again. No generic shell success, model prose or prior `exec-*` is elevated to source proof.

An operations prerequisite cannot be satisfied without **actual owned listener and successful health-response evidence**. That producer is not yet integrated (3D.1), so it remains explicitly pending or blocked. Independent Validation likewise remains pending until a fresh, role-bound Validator receipt is available (3C.1). This model intentionally cannot manufacture `ReadyForOwnerQa` from a source-only PASS.

## Focused synthetic source tests (written, unexecuted)

- `codexdd_046_3b3c_a01_focused_pass_retains_required_full_gate`: targeted PASS retains required `release` + independent gates and rejects a targeted ref reused as release
- `codexdd_046_3b3c_a02_native_source_cannot_establish_live_service_health`: source PASS and generic success do not prove listeners 4331/4332; typed operations setup block remains visible
- `codexdd_046_3b3c_a07_implementation_pass_is_not_independent_validation`: source PASS preserves independent gate; later conflicting provenance invalidates previously observed native source receipt
- `codexdd_046_3b3c_requirements_reject_missing_or_duplicate_authority`: reject missing/duplicate independent, unsupported profile, duplicate/empty ports and blank scope

The tests use disposable temporary roots and synthetic typed completion fixture records. They assert *source matching logic only* and make no claim about actual Node/BREAKWATER preview health, a real independent agent, official full regression, owner QA or release.

## Validation and remaining dependencies

3B.3b **accepted** at source `a249ab5bf06839f07219a05510260739f4252abf` through Windows GitHub Actions [#43054633002](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43054633002): source preflight and Rustfmt passed, native Core 1/1 and TUI 6/6 passed. This does *not* validate 3B.3c's new files.

3B.3c.1 is a new source checkpoint requiring its own exact-head cheap format and focused native proof. No permissions or operator-owned gates are waived. Separate unfinished obligations: private BREAKWATER Node/independent live validation; 3B.3d durable handoff; 3C.1 trusted terminal gate/transport; 3D.1 actual runtime-health owner; 3B.4 progress digest, and later full acceptance/release.

## Native acceptance (2026-11-11)

- **Authoritative checkpoint:** `9b159f54e7b5582f72b95db8e3434322dc73593b`. [GitHub Actions #43070130453](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43070130453) completed SUCCESS after source preflight and Rustfmt both PASS.
- **Native Windows log:** Core selected source-root summary **1/1 PASS**, existing 3B.3b provenance negatives **6/6 PASS**, new 3B.3c.1 A01/A02/A07/invalid-requirements **4/4 PASS**. Total 11/11 focused tests, zero failures. The earlier formatting-failed run #43067720315 is superseded.
- **Accepted exactly:** typed in-memory required-gate model and synthetic negative tests. No actual preview port or health request was run; no independent live Validator, durable resume, terminal READY check, real BREAKWATER Node profile, official release, installation or production merge is accepted.
- **Next:** 3B.3d narrow source-/Worker-bound gate checkpoint and safe restore, then explicit app-server/rollout persistence wiring. 3C.1 separately owns trusted terminal admission; 3D.1 owns approved live-health evidence.
