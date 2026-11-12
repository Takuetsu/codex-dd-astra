# CodexDD 0.4.6 — Packet 3C.1b.2g: app-server native two-Worker evidence boundary

**Date:** 2026-11-12. **Parent:** `bd81688edd5f214e97d97df1a52b1ca38ed462c9` (3C.1b.2f native Core integration accepted).
**State:** SOURCE IMPLEMENTATION, focused Windows native app-server test **PENDING**. TUI READY admission and 3C.1 completion **NOT YET ACCEPTED**.

## Scope

The preceding packet proved actual Core-owned native PowerShell execution in two *separate in-memory Core sessions*. This packet tests a wider seam: real Windows app-server JSON-RPC `thread/start`, two distinct source-pinned Worker identities, model-origin built-in validation tool invocations, the app-server `item/completed` event stream, and a fresh process cold-resume of both Worker identity snapshots.

## Fixture and assertions

The new `codexdd_046_3c1b2g_real_app_server_two_worker_receipts_and_cold_resume` test is Windows-only and uses a *disposable* Git candidate with exactly two committed one-stage native PowerShell scripts. The scripts contain contract-v1 output and no network, source mutation, model-supplied shell commands or personal project content. A local mock Responses server makes deterministic built-in tool calls with fixed profile selectors; **the Core built-in handlers really execute the committed scripts**, then the real app-server transports their native completed receipts over JSON-RPC.

In sequence:
1. The Implementation app-server thread starts with an explicit immutable Core `adaptive_worker` config map (role, scope, source root, exact SHA, source_change). Its model turn calls `run_codexdd_validation` with `work_packet`. The test waits for the real `item/completed` event and requires matching thread/turn/call IDs, native PASS, completed status and timestamp, exact canonical source path, matching SHA, and a nonempty native run ID.
2. The Validation thread starts as a **different real app-server thread** with the same source/scope but a different role. Its model turn calls `run_codexdd_independent_validation` with `independent_review`. The same checks apply, but the tool name and profile must be the distinct independent native producer. Thread IDs and native run IDs must differ, and the source candidate must remain clean.
3. The original app-server shuts down gracefully. A second app-server process cold-resumes both thread IDs and must restore their separate Core-owned role, authorized scope, exact SHA and root. No role promotion or source repinning is authorized.

The test uses a no-prompt danger-full-access **sandbox setting only inside these isolated test-only Windows sessions** to permit the known committed fixture scripts in unattended CI. It does not loosen any production execution policy.

The dedicated `[3c1b2g-native]` Windows job runs exact source branch/SHA preflight, Rustfmt, pinned Windows Rust/DotSlash/V8, and verifies exactly one new native app-server integration test under `cargo test --locked -p codex-app-server --test all`. It runs no full release compile, deployment or BREAKWATER work.

## Native acceptance attempt #124 and root cause

[GitHub Actions #43282916077](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43282916077), exact source `5e75a1dce702a4d10be05aa1d284a9838667cbdd`: source preflight and Rustfmt green; native app-server test **FAILED** after approximately 120 seconds waiting for an absent tool receipt. Actual Core log: `unsupported call: run_codexdd_validation`. A mock model tried to call the Implementation validation tool from an **ordinary** user turn. The Core registry correctly exposes that tool **only** on `codexdd_adaptive_mechanical_validation` turns. Earlier direct Handler tests bypassed the tool registry; changing production registration to make the tool available on arbitrary turns would violate the separation guarantee.

Correction: the fixture supplies `turn_trigger=codexdd_adaptive_mechanical_validation` **only** for its explicitly authorized, source-pinned Implementation session; the independent Validation Worker remains a normal (non-terminalization) turn and must have a separate Core-bound role/SHA. The test now also treats an early `turn/completed` without a matching `item/completed` receipt as immediate failure, instead of silently consuming its entire 120-second timeout. This is strictly a test-harness correction. No production authorization or tool exposure was broadened. CI rerun needed before any PASS claim.

## What a green result does and does not mean

A green result demonstrates the two separate Core native receipts are forwarded into **correct app-server item completion notifications** and survive identity restoration after a fresh process. It is a stronger interface acceptance than an in-memory handoff unit test.

**Still unproven:** actual TUI automatic first-Worker-to-Validator navigation, TUI processing of the app-server wire receipts against the authorized owner gate ledger, a fully trusted READY_FOR_OWNER_QA terminal from a live separate-Worker sequence, full six-stage independent profile behavior, 3D owned runtime listener/HTTP health, and the entire CodexDD 0.4.6 Windows distribution and release suite.

**Next:** integrate the live app-server evidence stream with a real TUI handoff/gate admission test, including negative stale-origin, wrong-role, mismatched-SHA, duplicate, expired, missing-owner-gate and restart-replay refusals. No installed binaries, BREAKWATER or Cycle Vengeance worktrees change until full 0.4.6 release acceptance; then use the approved approximately one-week single-milestone BREAKWATER soak.
