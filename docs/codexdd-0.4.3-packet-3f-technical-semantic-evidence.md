# CodexDD 0.4.3 — packet 3F technical source-semantic review (decision draft)

**POST-REVIEW HARDENING VALIDATED LOCALLY (2026-10-11):** The fail-closed import-source guard and regression code at [`343be968e5c10b5703f95517eb6ed4be955f4f91`](https://github.com/Takuetsu/codex-dd-astra/commit/343be968e5c10b5703f95517eb6ed4be955f4f91) supersede the earlier `32ab09a141d626df5851965a557e0dce877ac2ec` *source fingerprint*. The operator screenshot shows **3/3** focused `append::tests` passing on Daniel-CL and implies the preceding formatting guard passed. Only two Rust source files changed; [updated 136-path Git-object audit](codexdd-0.4.3-packet-3f-posthardening-source-audit.md) plus [current 136-row machine-readable source inventory](codexdd-0.4.3-packet-3f-posthardening-136-blob-audit.json) account for the one changed original overlap blob and new upstream-only test. Historical audit/CI remains immutable, and **no source-owner semantic acceptance, SHA-attested candidate, final CI, native F2 or 21-stage release receipt** is asserted.

**Status: TECHNICAL EVIDENCE PREPARED; OWNER SOURCE SEMANTIC ACCEPTANCE NOT GIVEN.** This is a read-only inspection of exact already committed sources. It neither creates a v2 semantic acceptance link nor authorizes a candidate branch, release receipt, GitHub promotion or installation.

## Authority and provenance

- Official target: OpenAI Codex `rust-v0.162.0`, peeled `c1382380de69521303b416720a52f42d51af6248`.
- Production remains CodexDD `0.4.2` at `16fdc9826e800ebabab920eadd8e1d976d2f645f`.
- Historical formatted executable source snapshot: `32ab09a141d626df5851965a557e0dce877ac2ec`, tree `a0bce7166cccb52e9b4fc8d658e0c08616752e62`. Current hardened product-source SHA is `343be968e5c10b5703f95517eb6ed4be955f4f91`, tree `4f58649164ea5ffcfa782b8c9075d68eafe5db65`; no further product-source changes followed in the docs-only audit.
- The [current 136-row machine-readable source inventory](codexdd-0.4.3-packet-3f-posthardening-136-blob-audit.json) covers all 136 original paths with one post-review hardening blob; the [prior 136-path postformat inventory](codexdd-0.4.3-packet-3f-postformat-136-blob-audit.json) and [15 formatted deltas](codexdd-0.4.3-packet-3f-postformat-source-delta.md) remain immutable historical evidence.
- Four original Windows privilege/F2 conflict paths had **no test URLs** in the old per-path ledger. The native hosted Windows test evidence now exists (see below), though it does not prove elevated SSH use on Daniel-CL. All other old test references are principally area-wide and **not** line-specific.
- On Daniel-CL the operator independently observed the post-format Rustfmt check pass (exit 0, zero hunks), Windows validation contract fixture pass and v2 canonical receipt fixture pass. These are not final CI or native runtime release gates.

## Six source-conflict classes — ordered original 22 conflicts

### Windows elevated F2 / daemon (4)

**Source finding:** [`codex-rs/app-server-daemon/src/backend/windows.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/app-server-daemon/src/backend/windows.rs#L116-L159) shares one upstream Win32 TOKEN_ELEVATION probe, checks returned token size, returns typed ElevatedLaunchRestricted; [`codex-rs/app-server-daemon/src/lib.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/app-server-daemon/src/lib.rs#L232-L314) rejects privileged explicit lifecycle/update requests; [`codex-rs/tui/src/startup_orchestration.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/startup_orchestration.rs#L176-L203) decides embedded mode **before** socket discovery at line 326, and [`codex-rs/tui/src/daemon_startup.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/daemon_startup.rs#L54-L111) suppresses an F2 warning only on verified elevation.

**Existing execution:** [hosted Windows compile + 2 focused unit tests #38158359012](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38158359012).

**Not yet accepted:** Real elevated SSH startup/F2 with no warning; failed-probe diagnostic; explicit elevated-daemon denial; non-elevated detached daemon start/reuse/stop on Daniel-CL. Hosted tests do not exercise these end-to-end.

| Original conflict path | Exact frozen Rust/source blob | Technical disposition (not accepted) |
| --- | --- | --- |
| `codex-rs/app-server-daemon/src/backend/windows.rs` | `1cc1889712a5daa85642739bc9d17869967f8ac5` | `manual_merge` |
| `codex-rs/app-server-daemon/src/lib.rs` | `bc3cf057bc8b39dfec1e67dfd4fb0a736f41a61f` | `manual_merge` |
| `codex-rs/tui/src/daemon_startup.rs` | `2a4c1190086bee4b16f86252ab802bf06e4a0361` | `manual_merge` |
| `codex-rs/tui/src/startup_orchestration.rs` | `e03aa30b4723717cfc72935d118e074fbf4f2aae` | `manual_merge` |

### Config and truncated rollout persistence (3)

**Source finding:** [`codex-rs/config/src/config_toml.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/config/src/config_toml.rs#L1-L95) retains TOML Worker role/scope alongside new upstream settings; [`codex-rs/rollout/src/policy.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/rollout/src/policy.rs#L19-L73) returns Cow durable projection and preserves WorkflowState; [`codex-rs/rollout/src/persistence_metrics.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/rollout/src/persistence_metrics.rs#L100-L137) records post-projection bytes and persists the owned/truncated representation, avoiding the old boolean-only persistence bypass.

**Existing execution:** [628 targeted regression groups #38155292769](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769) and [13 artifact generator exact drift #38157430174](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38157430174).

**Not yet accepted:** Review permission/config refresh and truncation on real Windows session and replay, and get source-commit-bound final CI after the freeze; neither broad Linux test success nor Rust formatting substitutes for all semantics.

| Original conflict path | Exact frozen Rust/source blob | Technical disposition (not accepted) |
| --- | --- | --- |
| `codex-rs/config/src/config_toml.rs` | `72f0ad4756d6f4a523bd2ed886be3757343677eb` | `manual_merge` |
| `codex-rs/core/src/config/mod.rs` | `95327c51bc04000d8eb72fe41c6d3a4da632dff7` | `manual_merge` |
| `codex-rs/rollout/src/policy.rs` | `84818a2e6dbd70ac46f2a96b34488b5e1a1a1155` | `manual_merge` |

### Generated app-server protocol (5)

**Source finding:** Generated conflicts were rebuilt from the reconciled app-server/protocol source rather than patched as prose. The source-based generator generated stable, experimental, TypeScript, config and Python SDK outputs and compared **13 artifacts and Cargo.lock** against Git.

**Existing execution:** [schema/SDK exact drift #38157430174](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38157430174) — green on the pre-format source snapshot.

**Not yet accepted:** Bind each generated path to a reproducible generator command and **final source SHA** in the v2 sidecar, with a final drift run after source is frozen; no invented current-head receipt.

| Original conflict path | Exact frozen Rust/source blob | Technical disposition (not accepted) |
| --- | --- | --- |
| `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst` | `94bf97c4c9ec45371794d37d49864542967be50d` | `generator_rebuilt` |
| `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst` | `6b29934a6fa191e695c8c580a25fac34b92edcc5` | `generator_rebuilt` |
| `codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts` | `d396d4096bfa66dda6aa0cbdab1099ebd6763d23` | `generator_rebuilt` |
| `codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts` | `c239fc7bfa81f75d4725237dcd967d5484a6f8f2` | `generator_rebuilt` |
| `codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts` | `2c34825523a52c69be6093c7f90e3c1923f6e581` | `generator_rebuilt` |

### TUI session / resume / scoped Worker (7)

**Source finding:** [`codex-rs/tui/src/app_server_session/rollout_history.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/app_server_session/rollout_history.rs#L134-L220) routes ordinary interactive resume to ResumeWorkerBinding::Unbound, reserving Startup for initial resume; [`codex-rs/tui/src/app_server_session/rollout_history.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/app_server_session/rollout_history.rs#L306-L325) imports config-bound Worker only for Startup; [`codex-rs/tui/src/app/working_directory.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/app/working_directory.rs#L401-L437) creates a bound Worker only on a new managed worktree; [`codex-rs/tui/src/app/thread_session_state.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/app/thread_session_state.rs#L94-L170) keeps new/resume adaptive session state separate.

**Existing execution:** [TUI/Worker/history 628-test targeted groups #38155292769](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769) — green at pinned earlier source.

**Not yet accepted:** Real Windows startup, ordinary resume, fork, detached reconnect and permission overrides; confirm no accidental Worker-scope inheritance or duplicate trusted signal on reconnect.

| Original conflict path | Exact frozen Rust/source blob | Technical disposition (not accepted) |
| --- | --- | --- |
| `codex-rs/tui/src/app/thread_routing.rs` | `8aae877b6724dcad18f1c4243c594736460dc790` | `manual_merge` |
| `codex-rs/tui/src/app/thread_session_state.rs` | `15ccd2028149d135695927cecbe1eeea6790f0c3` | `manual_merge` |
| `codex-rs/tui/src/app/working_directory.rs` | `9754938a063bd903216c8089b0abaf7817d6636f` | `manual_merge` |
| `codex-rs/tui/src/app_server_session.rs` | `c95f1d080836e934d83296ebab20b01abf1af5be` | `manual_merge` |
| `codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs` | `22e9183f76ed8ea5545836f207b0496315f60509` | `manual_merge` |
| `codex-rs/tui/src/app_server_session/rollout_history.rs` | `037a97a658dc2187d390f049dca96f84c85a3348` | `manual_merge` |
| `codex-rs/tui/src/session_start.rs` | `7ecd2bd03e94180067d5a8869dd8dbee783f7ec4` | `manual_merge` |

### TUI adaptive/slash routing (2)

**Source finding:** [`codex-rs/tui/src/chatwidget/slash_dispatch.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/chatwidget/slash_dispatch.rs#L923-L951) refuses roleful /new Worker without nonempty authorized_scope; [`codex-rs/tui/src/chatwidget/adaptive_effort.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/chatwidget/adaptive_effort.rs#L411-L457) binds automatic Validation only after ready_for_validation, authorized scope and acceptable complexity; [`codex-rs/tui/src/adaptive_policy.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/adaptive_policy.rs#L116-L175) retains Luna → legacy Sol → Sol61 → Astra ladder.

**Existing execution:** [slash/permission/model-status regression #38155292769](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769).

**Not yet accepted:** Real TUI /adaptive, /status, /new, /copy; verify intended role authorization/effort escalation, permission display and no inherited untrusted role.

| Original conflict path | Exact frozen Rust/source blob | Technical disposition (not accepted) |
| --- | --- | --- |
| `codex-rs/tui/src/chatwidget/slash_dispatch.rs` | `63d8b4b41cc237167a6c32ace5eb60cb193e4c46` | `manual_merge` |
| `codex-rs/tui/src/slash_command.rs` | `edd98b26ffdfba98d095ca6631830f2571928fe7` | `manual_merge` |

### Upstream permission selection (1)

**Source finding:** Target-source blob is exact official 0.162.0 for this conflict (target/current SHA: d24946fa30baa9aff93c838b697cfec776e794b0). This is source equality, **not** approval to weaken historical fork permissions; later session and dynamic override flows still need review.

**Existing execution:** [permission group in 628 targeted tests #38155292769](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769).

**Not yet accepted:** Owner must explicitly accept upstream_supersedes and confirm effective permission policy survives resume, worktree, and remote server flows.

| Original conflict path | Exact frozen Rust/source blob | Technical disposition (not accepted) |
| --- | --- | --- |
| `codex-rs/tui/src/app/tests/permission_selection_tests.rs` | `d24946fa30baa9aff93c838b697cfec776e794b0` | `upstream_supersedes` |


## Mandatory six-domain security review — status matrix

| Domain | Static review outcome at frozen source | Missing behavioral gate | Acceptance |
| --- | --- | --- | --- |
| Windows elevated F2 / daemon | One token probe, typed elevated denial, early embedded exclusion and selective warning suppression present | Elevated SSH F2, failed token probe warning and non-elevated detached lifecycle | **PENDING** |
| Adaptive model routing | Deterministic Luna/Sol/Sol61/Astra route ladder present; /status route indicators preserved | Actual Windows live role/model/effort escalation, account permissions and /status | **PENDING** |
| Worker scope and lifecycle | Nonempty scope check for /new, startup-only resume binding, automatic Validation admission controls present | Fresh Worker, interactive resume/fork and trusted lifecycle state across restarts | **PENDING** |
| Local Validation Orchestrator receipts | `run_codexdd_validation` accepts only `targeted|work_packet|release` and delegates to existing exec; native v2 fixture pass | Full candidate-bound 21-stage Daniel-CL release profile and canonical signed-off receipt | **PENDING** |
| Fail-closed upstream updater | [v2 verification code](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/.github/scripts/upstream_reconciliation_v2.py#L422-L503) requires ordered 22 conflicts, 136 overlaps, complete generated provenance and six review URLs; v1 original discovery remains blocked | Manual source-owner semantic acceptance, four-file metadata staging, guarded absent-ref candidate publication and current production recheck | **PENDING** |
| Resume/fork persistence | Startup-only binding, read/write workflow state, `Cow` projected rollout persistence and transient-signal exclusion present | Native Windows resume/fork/reconnect with durable state, safe permissions and F2 result | **PENDING** |

### LVO and rollout-specific source predicates

- [LVO fixed-profile schema and delegated execution](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/core/src/tools/handlers/codexdd_validation.rs#L130-L370) limits tool arguments and records native completion evidence, but release-profile acceptance is separately gated by the candidate manifest and identity-bound receipt workflow.
- [Adaptive Worker write-phase predicate](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/chatwidget/adaptive_effort.rs#L202-L229) requires an active, scoped Implementation Worker, a complexity class and the MechanicalValidation phase. [Ready-for-Validation binding](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/tui/src/chatwidget/adaptive_effort.rs#L410-L458) checks terminal status/complexity/scope and does not upgrade an unbound thread.
- [Rollout persistence projection](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/rollout/src/policy.rs#L19-L83) uses the official durable borrowed/owned item transform. [Measured append](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/rollout/src/persistence_metrics.rs#L100-L143) records post-projection length and writes the projected item, not the original potentially oversized command/MCP payload. This is source inspection, not a proven Windows replay receipt.

## Supplemental original-classifier blind-spot review

The [source-specific review of all twelve originally unclassified sensitive overlaps](codexdd-0.4.3-packet-3f-uncategorized-risk-review.md) records exact frozen Git blobs, source predicates and outstanding targeted proofs. Particular follow-ups concern untrusted imported `WorkflowState`, rollout-trace argument/MCP privacy and a Windows elevated CLI test that skips its assertions when run without elevation. This technical review does **not** revise any historical classifier, authorize source semantics or substitute for native SSH/F2/release acceptance.

The five read-only 0.4.3 CI workflow frozen-source guards were aligned to the **post-rustfmt source checkpoint** `32ab09a141d626df5851965a557e0dce877ac2ec` in commit `6b2d5ecf3d21aff2a56c0c7356344dbe73e71c2f`. They previously compared to older source and would have failed after mechanical formatting. These workflow-only corrections leave the Rust/SDK source unchanged and do not constitute fresh passing CI; final runs remain gated after local-first validation and semantic owner acceptance.

## Gaps that cannot be silently relabeled green

1. The [historical 136-row ledger](codexdd-0.4.3-packet-3f-source-object-review.json) has twelve category-empty paths; the new immutable inventory proposes additional risk labels but **does not** rewrite original classifier output or assert owner acceptance. Include them in manual review, especially privileged daemon + CLI, core tool scenarios, rollout migration and generated Python SDK.
2. Original overlap review records list **suggested** strategies. Exactly 16 textual conflicts propose `manual_merge`, five `generator_rebuilt`, one `upstream_supersedes`; **none** is owner-accepted.
3. Original 628-test, generated schema and hosted native Windows CI are genuine prior green evidence, but some ran before the Rustfmt-only source commit. No source-HEAD-bound **final** CI pass is claimed for `32ab09a141d626df5851965a557e0dce877ac2ec`.
4. A local untracked `M` file was visible in the detached validation worktree during diagnostics. It must not enter a future candidate or an authoritative clean-worktree release receipt; the release validation must use a clean canonical candidate checkout.
5. The v2 validator verifies URL shape and source identity, **not** that a human actually approved semantic changes. Do not replace owner sign-off with URL-shaped placeholders or this document.
6. Existing non-elevated daemon behavior waiver from the 0.4.2 F2 hotfix does **not** carry forward to 0.4.3.

## Next strictly ordered gate

Prepare a frozen source-specific review decision on **all 136 paths, the 22 listed strategies and six sensitive domains** without weakening the original conflict classifier. After the owner **explicitly accepts the source semantics**, assemble the canonical v2 metadata evidence on a one-parent staging commit against live production, then request separate candidate publication authorization. Run the candidate-bound **native Daniel-CL 21-stage release** and live F2/daemon/resume smoke before final GitHub CI and promotion; final PR/merge/install remain separate owner decisions.

**Current disposition: BLOCKED ON OWNER SEMANTIC ACCEPTANCE; no changes to production, installation, release tag or canonical candidate.**
