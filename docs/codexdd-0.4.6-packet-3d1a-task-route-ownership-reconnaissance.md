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


## 3D.1b0 — early non-source next-action guard (source checkpoint, not host proof)

The existing /status message instructed both task kinds to wait for an unknown route, without indicating their **different** missing authority. A bounded read-only correction to tui/src/chatwidget/adaptive_effort.rs now distinguishes OperationsPreview (source, process ownership, allowed port, real HTTP response, no foreign restart) from Benchmark (owner-approved command, resource lock, separate 2-thread and 4-thread real measurements). Both explicitly require SETUP_BLOCKED if mandatory evidence is missing and forbid source work_packet and invented READY. BLOCKED/paused/first-user precedence remains unchanged. Regression codexdd_046_3d1b0_non_source_status_requires_owner_proof_and_safe_block exercises both route diagnostics and BLOCKED precedence.

**This is presentation/admission guidance only. It is not a trusted runtime health producer, benchmark executor, process manager, service check or acceptance PASS.** The D01–D03 positive fixture and required exact-source Windows tests remain in future 3D.1b/c/d subpackets. This code checkpoint does not relax 3C source/Validator roles or close 3B.4.


## 3D.1b1 — owned-process/port/source admission before probing (implementation checkpoint)

The existing pending `OwnedRuntimeHealth` ledger now exposes a strictly read-only `authorizes_runtime_health_port` check scoped to the *original* owner scope, canonical source root and SHA and a port literally named in the owner-issued gate. It does **not** report PASS.

`preflight_owned_runtime_target` (TUI Worker module) requires original Implementation role/scope, the `operations_preview` task kind, exact pinned root and SHA, an owned `std::process::Child` handle already held by the authorized caller, the same PID reported by host.pid, a live child both before and after clean Git preflight and an approved port. It never discovers, kills, restarts or signals a PID. It does **not** claim that a live HTTP listener is owned by that child and cannot satisfy `OwnedRuntimeHealth` by itself. Fresh live listener-to-child correlation and bounded HTTP response remain 3D.1b2.

Windows regression `codexdd_046_3d1b1_owned_host_pid_gate_and_source_preflight_refuse_foreign_process` creates an isolated Git candidate and a harmless test-owned sleeping child. It asserts foreign PID refusal leaves child alive, wrong port/role/SHA refusal, correct narrow preflight, exited-child refusal and that the gate stays PENDING. **The test explicitly kills only its own disposable child during cleanup.** Native Windows PASS is pending; no full 3D.1 or broader runtime claim.


## 3D.1b2a — native live listener-PID and HTTP observation (source checkpoint)

Following source-pinned 3D.1b1 [Actions #43613778015](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43613778015), **4 focused Windows tests passed**, zero failed/ignored. This verifies only owner/source/port/Child preflight and the fail-closed D03 foreign PID boundary.

`tui/src/adaptive_owned_runtime.rs` now probes **only an explicitly approved loopback IPv4 listener** via the Windows read-only `GetExtendedTcpTable` `TCP_TABLE_OWNER_PID_LISTENER` API, not shell `netstat` or a model-supplied PID. A bounded buffer, exact `127.0.0.1` bind, unambiguous PID, and equality with the originally owned `std::process::Child` are mandatory. It issues one bounded `GET /health` request (maximum 2 seconds and 512 status-line bytes), requires HTTP 200, and rechecks the owner table and child liveness after response; no foreign listener, exited child or unhealthy response may pass. The immutable observation includes source SHA, PID, approved port, status and time. **The observation does NOT itself satisfy a gate, persist a green receipt, invoke a process manager or mint owner QA.**

Tests use **the Cargo test binary itself as an explicitly owned child HTTP listener**, never production BREAKWATER/CV or a privileged host; positive HTTP 200, refusal of HTTP 503 and pending-ledger checks run on Windows with an exact-source opt-in. The helper does not run in ordinary tests and cleans up only its own child. Required follow-up 3D.1b2b: tie an independently verifiable runtime health receipt to the one original owner gate and recheck freshness/ownership at QA admission; no generic tool PASS. Required 3D.1c: actual 2-vs-4 bounded benchmark. Phase 3D stays OPEN pending true source-matched Windows native pass and subsequent owner-gate architecture review.


## 3D.1b2a — native Windows live-health result (verified 2026-11-14)

Exact-source Actions https://github.com/Takuetsu/codex-dd-astra/actions/runs/43619743848, SHA `5830022b54a5027eb7b9663b425832d5c600fc75`, passed source-contract and Rustfmt plus six native Windows TUI tests (two live PID/HTTP positive/503 controls, one 3D.1b1, one 3D.1b0 and two prior 3C safety tests). The disposable separate OS process returned a real 200; 503 refused. This is observation, NOT a Core-trusted receipt or OwnedRuntimeHealth PASS.

## 3D.1b2b1 — ephemeral owned-health ticket seam (implementation candidate)

A successful same-owner listener/HTTP observation now issues an opaque, non-Clonable, non-serializable, per-call ticket with original authorized scope, canonical root, SHA, PID, port, timestamp and random ID. One-time *consumption* requires the same owner, same Child handle, same source and port, age no more than five seconds, and a second fresh guarded native TCP/HTTP 200 observation. Stale, source/port drift, and exited-child refusals are executable regressions. Tickets are not persisted, cannot be replayed after cold resume, do not authorize shell commands and **never mutate the Owner QA ledger**. Native focused Windows evidence remains pending on this checkpoint.

**Important limit:** This is an admission/consumption primitive, not the 3D.1b2b Core-native trusted producer. The OwnedRuntimeHealth gate must remain Pending until a Core-bound actual runtime receipt can be independently verified (including live child continuity and source/time provenance). D01 benchmark and D01–D03 acceptance remain OPEN. No deployment, install, merge, or project workload was performed.


## 3D.1b2b2a — live owner-health QA review seam (source candidate)

Earlier exact-source [Actions #43625507085](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43625507085) at `0277f17d7ba99f940369dd80958839dd3e1b8260` passed source preflight, Rustfmt and **8/8 Windows native tests**, including two transient native ticket cases. The ticket remains single-use, non-Clone, non-Serialize, source/port/PID/scope-bound and limited to 5 seconds; stale, dead-child or foreign-source use refuses.

`review_live_owned_runtime_at_qa` now composes the original immutable owner ledger, live evidence registry, explicit *single-port pending* health gate, original authorized Worker/source, owner-held OS `Child`, and a fresh issue+consume of the native ticket **at the QA review boundary**. Each issuance and consumption checks the approved source, live PID-to-listener mapping and fresh loopback HTTP 200. A blocked gate, absent gate, multi-port ambiguity, wrong port or dead process fails closed. Its transient read-only result lists remaining independent gates, but **never writes a PASS**; the ordinary `ready_for_owner_qa` and persisted checkpoint remain pending. A Core-bound native health producer and independently revalidated gate integration are **still required**; this packet does not silently promote a TUI observation or generic exec result to that authority.

New Windows tests `codexdd_046_3d1b2b2_*` cover the true positive on a disposable same-owner HTTP child, mandatory independent gate still pending, unmodified original readiness, dead child, unapproved port, and owner-blocked gate. A source-matched native Windows PASS, plus the earlier eight safety tests, is required before accepting this non-authoritative review seam. **3D.1c benchmark, 3D.1d integrated acceptance, conditional 3D.2/3D.3 and separate 3B.4 remain OPEN.**


## 3D.1c1 — exact-owner two-vs-four native benchmark admission (source candidate)

A separate `tui/src/adaptive_benchmark.rs` admits only two **distinct, real Core app-server CommandExecution completion items**, never model text or a source MechanicalValidation PASS. The original Worker startup/first-message config now has a typed, deny-unknown-fields optional `benchmark` contract: separate literal `command_2`/`command_4`, an exact owner-approved scratch `resource_lock` path and bounded `max_duration_ms`. Missing benchmark authority, non-Implementation role, benchmark task-kind drift, wrong clean canonical Git root/SHA, missing/occupied scratch lock, unknown command, invalid source thread/turn, stale or missing outcome, absent real process ID, negative duration, nonzero exit, plugin substitution or reused process fails with `SETUP_BLOCKED`.

A unique scratch lock is acquired with create-new and a held OS file lock before accepting either measured result. The real executor remains **the existing guarded Core command tool**; this module does not launch an arbitrary privileged process manager, modify Worker capacity or grant shell permissions. The Windows CI fixture alone launches disposable PowerShell parallel work with exactly 2 and 4 units, measures two independent OS processes and converts those actual results to the app-server item shape for the acceptance seam. The runtime caller must separately authenticate the local embedded app-server notification origin; untrusted/remote notifications do not acquire authority merely by matching an item schema.

**Not yet accepted:** Source preflight, Rustfmt and dedicated native Windows `[3d1c-native]` must pass at one exact commit. This admission primitive does not yet authorize full production benchmarking, mint a trusted Core benchmark gate, or satisfy final D01-D03 acceptance. 3D.1b2b2 QA review also remains deliberately non-authoritative pending a Core-trusted native runtime-health producer. 3D.2/3D.3 conditional holds and separate 3B.4 OPEN status remain unchanged.


## 3D.1c2a — one-use benchmark admission and process-identity hardening

Phase 3D.1c1 source-pinned [Windows Actions #43641763370](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43641763370) passed both disposable OS parallel 2/4 measurements and the existing non-source safety control (**3/3 native**, source preflight/Rustfmt green), at SHA `8cb0dc455068109f22323b138f7e5f1fe171859a`. This only accepts typed benchmark completion records; it is not a live app-server gate or production benchmark result.

This follow-up bounds each original-owner admission instance to **one comparison**; only an event run begun after the instance was authorized may count. Already-used measurements refuse replay, nonnumeric/zero process IDs refuse native evidence, and zero or temporally impossible elapsed durations cannot pass. The Windows fixture exercises a replay of the two real measured process records and forged process identity as negative controls. No terminal PASS, process manager or source-only work_packet has been added.

Remaining 3D.1c2b: authenticate actual local Core app-server event ingress and tie lifetime of the held scratch lock to real command execution, rather than trusting app-server-shaped test fixture notifications. A future acceptance packet must prove the original launcher/first-turn owner authority and two genuinely observed command-completion events are used, including replay/foreign transport refusals. Keep D01/D02 native runtime gate acceptance and Phase 3D.1d OPEN.
