# CodexDD 0.4.3 — owner-facing source semantic review packet

**Status: PREPARED FOR TECHNICAL REVIEW — NOT OWNER ACCEPTED.** Source-object reconciliation is complete but six security domains and native Windows release behavior remain distinct gates. This file cannot serve as a v2 signed attestation, success receipt or authorization to publish.

## Exact identity and evidence

- Fresh production `dd/astra-policy-v2`: `16fdc9826e800ebabab920eadd8e1d976d2f645f`, still **CodexDD 0.4.2** with tracked upstream `rust-v0.159.2`. Approved tooling-only PR #50 merged here.
- Isolated source-runtime checkpoint: `544a29516353cb1e416b6097c46d8c83b23c7eb9`, identifying CodexDD 0.4.3 atop official upstream 0.162.0; all later changes through this review have been limited to audit documentation and read-only CI, not `codex-rs/`.
- Current original discovery [#38155514359](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155514359) on the new production commit retains **`blocked_transplant_conflict`**, 22 ordered conflict paths and 136 overlaps. The original manifest digest is `3286752a1f6c31dd2b4f0f9fcb0ccadb34bcf004e3bf686a8811c2dd2ff8ebb3`; conflict report digest `c3456c3e93130126f8f6f6bc1366bdafcc0c11835244ba70c31b91a564a450a4`.
- [136-row source Git-object inventory](codexdd-0.4.3-packet-3f-source-object-review.json) records blob SHAs, classifications, source packets, area-level test references and **false** release/semantic acceptance flags for every overlap.
- [Full hash-audit CI #38157286824](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38157286824) passed the source-blob, 124 flagged + 12 previously-unclassified sensitivity, 22-conflict and ancestor/version checks.
- [Re-anchored 628-test runtime CI #38155292769](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769), [core/CLI #38155292765](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292765), and [Windows PowerShell v1/v2 contract #38155292794](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292794) all concluded success.
- [Pinned 13-output generator drift run #38157430174](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38157430174) was **running when this packet was drafted**; no passing result is preclaimed.

## 22 original textual conflicts — source decision basis, not approval

Original conflict order and owner assignments are immutable. Source reconciliation strategies below are proposed technical dispositions justified by existing packet source reviews; they are **not yet accepted owner semantic decisions**. Each source link points to the previously validated identical Rust-content checkpoint.

### Windows daemon / elevation / F2 — 4 conflicts; proposed `manual_merge`

Source-reconciled upstream 0.162 Win32 HANDLE/API updates with CodexDD typed elevated-daemon denial, implicit embedded fallback before socket discovery, no F2 warning only on confirmed elevation, visible diagnostic on failed probe, detached-launch handling and WSL exclusions. **Native elevated SSH, actual F2 observation, and non-elevated detached-daemon behavior are not accepted.**

Existing packet: [review source](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c1-windows-daemon-startup.md)

- [`codex-rs/app-server-daemon/src/backend/windows.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/app-server-daemon/src/backend/windows.rs)
- [`codex-rs/app-server-daemon/src/lib.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/app-server-daemon/src/lib.rs)
- [`codex-rs/tui/src/daemon_startup.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/daemon_startup.rs)
- [`codex-rs/tui/src/startup_orchestration.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/startup_orchestration.rs)

### Config / rollout persistence — 3 conflicts; proposed `manual_merge`

Merged upstream Daybreak, Guardian, config refresh and bounded persistence truncation with CodexDD startup-scoped Worker configuration, durable WorkflowState and transient AdaptiveRuntimeSignal. The 150 rollout tests passed; full security/truncation scenarios must be reviewed before approval.

Existing packet: [review source](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b1b-config-reconciliation.md)

- [`codex-rs/config/src/config_toml.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/config/src/config_toml.rs)
- [`codex-rs/core/src/config/mod.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/core/src/config/mod.rs)
- [`codex-rs/rollout/src/policy.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/rollout/src/policy.rs)

### Generated app-server protocol / SDK — 5 conflicts; proposed `generator_rebuilt`

The five originally conflicting precomputed/TypeScript outputs were source-generated with repository tooling, not manually patched. The original generator succeeded on its prior source SHA; a new pinned 13-output exact-drift CI run is required against the updated source.

Existing packet: [review source](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b4-generated-schemas.md)

- [`codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst)
- [`codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst)
- [`codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts)
- [`codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts)
- [`codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts)

### TUI session / resume / Worker binding — 7 conflicts; proposed `manual_merge`

Retained upstream remote permission authority, restart/resume and fork behavior, while restricting Worker binding to authorized startup flows and preserving trusted adaptive/workflow-state signals. Targeted lifecycle, status and history tests passed; Windows behavior still needs native acceptance.

Existing packet: [review source](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md)

- [`codex-rs/tui/src/app/thread_routing.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/app/thread_routing.rs)
- [`codex-rs/tui/src/app/thread_session_state.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/app/thread_session_state.rs)
- [`codex-rs/tui/src/app/working_directory.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/app/working_directory.rs)
- [`codex-rs/tui/src/app_server_session.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/app_server_session.rs)
- [`codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs)
- [`codex-rs/tui/src/app_server_session/rollout_history.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/app_server_session/rollout_history.rs)
- [`codex-rs/tui/src/session_start.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/session_start.rs)

### TUI adaptive/slash routing — 2 conflicts; proposed `manual_merge`

Preserved upstream Daybreak and MCP command behavior while restoring fork /adaptive control and scoped Worker creation; authorization still depends on nonempty authorized_scope. The slash/permission test groups passed, but real Windows interactive usage is pending.

Existing packet: [review source](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c3a-slash-dispatch.md)

- [`codex-rs/tui/src/chatwidget/slash_dispatch.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/chatwidget/slash_dispatch.rs)
- [`codex-rs/tui/src/slash_command.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/slash_command.rs)

### Upstream permission selection — 1 conflicts; proposed `upstream_supersedes`

The reconciled file intentionally matches the upstream 0.162.0 blob exactly; upstream's stronger permission-selection tests supersede the narrow old fork delta. The targeted permission suites passed; this equality is not by itself semantic sign-off.

Existing packet: [review source](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c4-tui-regression-source.md)

- [`codex-rs/tui/src/app/tests/permission_selection_tests.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/codex-rs/tui/src/app/tests/permission_selection_tests.rs)


## Six mandatory security acceptance domains

| Domain | Present source/test evidence | Remaining gate |
| --- | --- | --- |
| Windows elevated F2 and daemon | 3C.1 static 15/15 predicates, existing daemon/startup tests | Native elevated SSH F2/no-warning, failed elevation probe, explicit daemon denial, non-elevated detached lifecycle |
| Adaptive model routing | GPT-6 model catalog and `/status` snapshots, 628 targeted tests | Verify real model routing/effort transition with runtime status; separate from model list alone |
| Worker scope and lifecycle | 44 session lifecycle, 51 adaptive-effort, 4 misalignment tests | Authorized Worker scoping across fork/resume and actual delegated work |
| LVO receipts | Windows PS5.1 exit-status fixtures and core handler regression | Actual 21-stage release-profile candidate-bound PASS receipt |
| Fail-closed updater | 108+ tooling/v1 contract tests and fresh original blocked discovery | Exact 4-file staging evidence, immutable target/source/current prod, absent-ref single publish, explicit owner sign-off |
| Resume/fork | 8 history and 172 status/layout, plus targeted permission tests | Actual Windows resume/fork and F2/reconnect smoke including elevated/non-elevated branches |

## Review and gate sequence

1. Inspect each conflict strategy and source contract above, and **all 136** corresponding per-path dispositions in the Git-object ledger. Existing area-wide green test links do **not** prove a specific line is correct; stronger source-specific evidence can be added without altering runtime.
2. Confirm the generator-drift run is green and binds actual source commit/tree. Capture exact reproducible command and CI link for each generated overlap; do not cite the old generator run as if it were current-source validation.
3. Record separate source-SHA-bound security evidence for the six domains; owner may accept the **source semantic** review while explicitly reserving Windows-native execution for the later candidate.
4. Only after an explicit owner-specific acceptance of the frozen source, construct canonical v2 sidecar/manifest using **real** per-path semantic and test evidence URLs. Do not fill fields with these general docs merely to satisfy a parser.
5. Stage exactly four metadata blobs atop the then-current production; only then consider the separately authorized, guarded candidate publish. After publication, run real Windows release profile, capture native receipt and finish owner-gated review PR and final merge/install decisions.

**Not performed:** authorizing a semantic review, publishing a canonical candidate, accepting native release, merging 0.4.3 runtime, or updating Daniel-CL/Doug.
