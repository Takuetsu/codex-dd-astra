# CodexDD 0.4.6 — 3D.1a bounded task-lane reconnaissance

**Opened:** 2026-11-14. **Status:** SOURCE RECONNAISSANCE CHECKPOINT; 3D IMPLEMENTATION AND NATIVE PROOF OPEN.
**Branch:** dd/codexdd-v0.4.6-agent-reliability
**Parent:** f64b7f7e5c95661133d4d772f24ce69b0e7eb5a5
**Previously accepted:** Phase 3C documentation checkpoint f64b7f7e5c95661133d4d772f24ce69b0e7eb5a5, native source c1e5be2dce8f35e234f5f91442b22239f59cffa2, Actions https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333 (11/11 tests passed). That gate does NOT prove a non-source operations or benchmark lane.

## Authorized outcome and non-goals

The design requires 3D.1: one real, bounded, authorized host-preview/recovery scenario and one 2-vs-4-thread non-source benchmark scenario. Do not route either through source-only MechanicalValidation/work_packet; verify original owner role and scope, exact clean source root and SHA, approved commands/resources, live process ownership and genuine observed health/measurement. Reuse normal guarded tools. If missing authority or a task would need a new privileged daemon manager, return actionable SETUP_BLOCKED rather than granting authority or inventing completion.

Do not run on live BREAKWATER, Cycle Vengeance, Doug's host or production CodexDD. Do not install, merge, bump 0.4.4 production version, unfreeze 0.4.5 or weaken Core/TUI role, sandbox, source or independent-evidence rules.

## D01–D03 classification and existing owners

| Case | Observation and expected result | Current owning source and gap |
| --- | --- | --- |
| D01: approved 2-vs-4 benchmark | The November 4 Google backlog records a Z-A0.57E non-source Vitest benchmark routed to wrong-checkout MechanicalValidation with no benchmark executed. Installed runtime causation is not proven. Expected: exactly two measured runs, thread counts 2 and 4, bounded command, explicit approved resource lock and original source identity; otherwise a typed setup block. | tui/src/adaptive_worker.rs parses Benchmark with root/SHA but bound_owner_gate_ledger rejects benchmark gates pending a dedicated native-evidence contract. tui/src/chatwidget/adaptive_effort.rs only says an authorized operations path is required. **Current-source P0 gap: no positive benchmark evidence lane.** |
| D02: owned live host preview | Owner observed no listener or healthy localhost response on ports 4331/4332 in Z-A0.58A but saw an unsupported green handoff. This is historical behavior, not proof a current process is defective. Expected: owned current process, explicitly approved port, actual successful HTTP health response, time and task/source identity. Source-only PASS cannot satisfy live health. | tui/src/adaptive_worker.rs accepts OperationsPreview only with owned_runtime_health and independent gates; tui/src/adaptive_gate_prerequisites.rs always leaves OwnedRuntimeHealth pending because no authorized producer exists. **Current-source P0 gap: no positive runtime health receipt.** |
| D03: stale foreign host.pid | 3A.3 defines a synthetic stale PID pointing to an unrelated protected process; no real current incident is established. Expected: refuse kill/restart/signal before owner identity is verified, preserving foreign process and returning owner handoff. | core/src/tools/registry.rs does not supply an owner-bound lifecycle controller; do not invent a privileged manager. **Contract-only; no product failure inferred.** |

Contract provenance: docs/regression-fixtures/codexdd-046-agent-reliability-cases.json (D01–D03); docs/codexdd-0.4.6-packet-3a3-regression-contract.md. Historical operator evidence: https://docs.google.com/document/d/1vJ617lLax8GFUVWjJaNE2veCfSyjD5OOydBvywWA8iA/edit.

## Existing safety boundary

- config/src/config_toml.rs and tui/src/adaptive_worker.rs require a true original owner assignment with root/SHA together for non-source kinds; preflight_owner_candidate_identity rejects dirty/wrong roots. No fallback to a child shell's Set-Location.
- tui/src/adaptive_worker.rs::preflight_mechanical_target refuses operations/benchmark entry to source MechanicalValidation; no source LVO PASS is authorized for a non-source task.
- tui/src/adaptive_gate_prerequisites.rs requires independently revalidatable Core source/Validator receipts and keeps live health pending; persisted gates are requirements, not green evidence.
- 3C's real App/Core replay, stale/foreign handoff, READY and role safety checks remain mandatory regressions.
- 3B.4 C01–C05 progress digest/refusal taxonomy is still open and must be separately accepted. Do not silently complete it here.

## Executable work packets

**3D.1b — Owned preview:** implement a narrow source-bound, original-owner-approved process/port/health admission path using existing read-only guarded execution, no new privileged service manager. In disposable same-owner Windows fixture: positive current HTTP 200 with proof of process ownership, SHA, approved port, timestamp, bounded output and deadline. Negatives: no listener, wrong port, stale health, wrong SHA/role and foreign PID. D03 must prove no signal/kill/restart attempt on a foreign process. If process ownership or permission cannot be proven, SETUP_BLOCKED with next authorized operator action; never synthetic runtime PASS.

**3D.1c — Benchmark:** bind a source-pinned original benchmark assignment, owner-approved exact command template, threads 2 and 4, resource-lock ownership, bounded time and output. Execute both with normal guarded tools on disposable fixture and capture distinct real results/run IDs. Negatives: unapproved command, absent lock, wrong root/SHA, one missing measurement, source-only LVO and duplicate stale evidence. No invented benchmark, unsafe process manager or owner QA.

**3D.1d — Audit:** run D01–D03 and unchanged 3C negative tests on same final source. Document source SHA, runner, actual pass counts and security nonclaims. Close only after one positive owned host preview and the two distinct benchmark runs have been observed; documentation alone is not PASS.

## Conditional P1 and sequencing

**3D.2 E01/E02** requires proof of an in-capacity worker-seat defect, not an expected refusal from eight requests exceeding supported V1/V2 limits. Need exact installed identity, current capacity, positive available-seat spawn and over-capacity refusal; no blind limit increase.

**3D.3 F01** requires a supported and configured backend reproducing tool_choice HTTP 400 with exact sanitized request/response; historical local-Qwen subtype unknown. No broad adapter fallback or authentication/permission weakening.

Owner's default cheap focused validation is Daniel-CL before CI; any isolated GitHub-only native exception must be justified and owner-authorized per docs/codexdd-major-phase-work-packet-standard.md. This documentation-only reconnaissance does not require Cargo or CI. The next task is 3D.1b. If source/host authorization is genuinely required, notify owner of the single necessary approval; do not make a speculative live-process change.
