# CodexDD 0.4.6 — D02 Core-owned runtime-health producer integration contract

**Opened:** 2026-11-17  
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`  
**Status:** DESIGN / IMPLEMENTATION IN PROGRESS — **NO trusted Core runtime-health receipt; gate PENDING**.

## What current source actually proves

`tui/src/adaptive_worker.rs::preflight_owned_runtime_target` requires the original authorized Implementation Worker, operations-preview task kind, exact source root/SHA, one expressly approved port, a matching live `std::process::Child` handle and a clean candidate. `tui/src/adaptive_owned_runtime.rs` reads Windows IPv4 listener owner PID and a bounded fresh `GET /health` response; the QA review issues and consumes a private one-use ticket and deliberately leaves `OwnedRuntimeHealth` PENDING. These native tests launch a disposable child test binary; they do **not** exercise an installed Core-owned preview service or issue a Core-native receipt.

`tui/src/adaptive_gate_prerequisites.rs::outstanding` treats `OwnedRuntimeHealth` as PENDING until an independently revalidatable native producer exists. `tui/src/app_server_session.rs::AppServerObservedEvent` can bind actual events to a live in-process Core reader but does not by itself attest a service, held OS process handle or HTTP response. A generic `CommandExecution` result, copied `host.pid`, model narrative or persisted workflow snapshot cannot fill the gap.

## Required implementation boundary

1. **Owner authorization first.** Admit only original owner-configured `operations_preview` scope, canonical source root and exact current SHA, explicitly approved executable/argument template and port(s), and required IndependentValidation. Do not treat a generic source `work_packet` as an operations authority. Missing approval returns `SETUP_BLOCKED`.
2. **Core owns the child.** The actual guarded launch/execution authority must retain the live non-serializable process handle associated with the original request. Never adopt a PID from disk, attempt a foreign signal/restart, create a privileged background manager, or grant arbitrary shell access. If the owner process or Core session dies, any pending capability expires.
3. **Native observation and provenance.** In the Core-owned session, correlate an exact approved IPv4 loopback listener to the *owned child PID* via the Windows owner table, require a fresh bounded `GET /health` HTTP 200, and repeat both ownership and liveness checks. Same-port wildcard, ambiguous listener, missing listener, stale/foreign PID, wrong port, wrong source/role and non-200 fail closed. IPv6/dual-stack ownership is a separate required review before claiming all-interface confidence.
4. **Separate report from receipt.** Public app-server tool output can inform the user but must not grant a gate. A trusted native producer must associate a unique non-replayable run/observation ID and completion time with the original Core session, source, worker, command/port and process handle, and expose proof only through the trusted Core event path. An event-shaped struct supplied to a TUI handler is not provenance.
5. **Independent gate admission.** At QA time, revalidate the still-live Core-owned handle, source, scope, approved listener and fresh health. Only the *matching* requirement may be discharged; IndependentValidation stays separately required. The gate must return to PENDING on process exit, session drift, source drift or cold resume. No cached boolean PASS or serializable receipt survives restart.
6. **No authority expansion.** Reuse existing guarded execution, role/permission and source preflight. Do not change worker limits, backend routing, production 0.4.4, or live BREAKWATER/CV hosts.

## Evidence required before D02 acceptance

- On one exact Windows source SHA: approved owned-child live HTTP 200 accepted as a **real Core-native observation**; HTTP 503, no listener, wildcard/ambiguous IPv4 owner, wrong port, foreign PID, exited child, wrong source/role, stale observation, fake/replayed event and restored checkpoint all refuse.
- Prove no foreign process was killed/restarted/signaled. After a valid health receipt, original owner QA is still blocked without a valid independent Validator receipt.
- Demonstrate source-, Core-session-, child-, and turn-bound revalidation at the terminal gate, not merely an isolated disposable test that produces a protocol-shaped message.
- Run D01–D03 plus prior 3C security regressions at one final source SHA (3D.1d audit), followed by the 0.4.6 broader release acceptance program.

## Immediate checkpoint

`adaptive_owned_runtime.rs` has a focused fail-closed IPv4 wildcard/ambiguous listener classifier, tested with synthetic table rows and protected by the original live-child HTTP tests. This is **probe hardening only**; it is not the missing Core-native producer and does not turn `OwnedRuntimeHealth` into PASS.
