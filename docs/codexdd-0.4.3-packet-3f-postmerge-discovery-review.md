# CodexDD 0.4.3 — post-tooling-merge discovery and reconciliation review index

**Authority: READ-ONLY SOURCE-REVIEW INDEX. Not a v2 manifest, semantic sign-off, native validation receipt, or release/candidate authorization.**

## Fresh original discovery: immutable post-PR #50 production identity

| Identity | Verified value |
| --- | --- |
| GitHub discovery | [#38155514359](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155514359), workflow dispatch **success** |
| Discovery source | `dd/astra-policy-v2` @ `16fdc9826e800ebabab920eadd8e1d976d2f645f` |
| Production tree | `76ce3087f8808dc57ed3db809ea0b919076e5f65` |
| Product baseline | `0.4.2` (the PR #50 merge contained tooling, not runtime) |
| Tracked upstream | `rust-v0.159.2` @ `ff6aec96948b70d94983af2641a6b67c94faeff5`, tree `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` |
| Target upstream | `rust-v0.162.0` @ `c1382380de69521303b416720a52f42d51af6248`, tree `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |
| Immutable original state | `blocked_transplant_conflict`, `transplant_dry_run: conflict` |
| Candidate key | `rust-v0.162.0@c1382380de69521303b416720a52f42d51af6248` |
| Original artifact | `upstream-candidate-rust-v0.162.0-38155514359` |

Downloaded, inspected and SHA-256 hashed the two original GitHub Actions artifact members:

- `manifest.json`: `3286752a1f6c31dd2b4f0f9fcb0ccadb34bcf004e3bf686a8811c2dd2ff8ebb3`
- `transplant-conflicts.txt`: `c3456c3e93130126f8f6f6bc1366bdafcc0c11835244ba70c31b91a564a450a4`
- The newline-joined, **no trailing newline**, ordered 136-path manifest overlap list SHA-256 is `27e3145698e3b5c486fd9660d61dbffb87ebf755c058bd29fb20958b01872bcd`; it equals the historical `docs/codexdd-0.4.3-packet-3a3-overlap-ownership.json` inventory checksum.
- The original textual conflict report has **22 paths**, with its exact byte digest equal to the old ownership ledger conflict-report checksum. The fresh artifact therefore preserves the path identity/order underlying earlier source ownership and reconciliation documentation while updating the production SHA and original-manifest digest.

The historical ownership ledger is still explicitly pinned to pre-tooling production `4828e3b4232781963594cfaaebdc49c5531de8b1` and old discovery `37960772252`. **Do not overwrite those historical fields or pass its old manifest digest to v2.** This review index is the post-merge crosswalk, not a claim that old semantic approvals automatically transfer.

## Isolated source state and fresh CI

- Approved PR #50 tooling was integrated into the **isolated** 0.4.3 source at merge commit `7e71bcda193b5bb2d9cffcce7a1068e7c7a992b6`, merging only 23 paths of v2 tools with no changes under `codex-rs/`.
- The subsequent pinned workflow/source checkpoint is `544a29516353cb1e416b6097c46d8c83b23c7eb9`; runtime markers on that branch are product `0.4.3`, tracked `rust-v0.162.0`.
- [Re-anchored updater + native Windows LVO contract #38155292794](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292794): verified **success**, including v1 compatibility and new v2 contract/receipt tests.
- [Re-anchored targeted Rust runtime tests #38155292769](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769) and [CLI/core version/identity #38155292765](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292765): **in progress at drafting**; inspect results before claiming an accepted candidate source.
- This note is documentation-only. It does not establish a source snapshot frozen at the new documentation commit, final generated drift, or native Windows execution.

## Reconciled overlap source-owner crosswalk (historical source status, not semantic approval)

All 136 paths have named owners and source-reconciliation statuses in the existing ledger; all 22 textual conflicts are represented. The statuses below are *source-merge checkpoints*, not per-path release acceptance.

| Owner packet | Overlaps | Original textual conflicts | Existing source evidence |
| --- | ---: | ---: | --- |
| 3B.1 | 8 | 3 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b1b-config-reconciliation.md) |
| 3B.2 | 21 | 0 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b2c-lifecycle-and-tool-compatibility.md) |
| 3B.3 | 14 | 0 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b3c-app-server-handlers.md) |
| 3B.4 | 13 | 5 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b4-generated-schemas.md) |
| 3B.5 | 8 | 0 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b5-test-source-reconciliation.md) |
| 3C.1 | 4 | 4 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c1-windows-daemon-startup.md) |
| 3C.2 | 19 | 7 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 3C.3 | 15 | 3 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c3a-slash-dispatch.md) |
| 3C.4 | 26 | 0 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c4-tui-regression-source.md) |
| 3D.1 | 5 | 0 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3d1-model-status.md) |
| 3D.2 | 1 | 0 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3d2-cli.md) |
| 3E.2 | 2 | 0 | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3e2-build-evidence.md) |

Prior ledger resolution-status distribution (still uses pending validation language):

- 53: `source_merged_pending_native_validation`
- 34: `source_merged_pending_tui_compilation_and_native_validation`
- 26: `source_merged_pending_regression_compile_and_windows_native_validation`
- 13: `generator_regenerated_pending_release_validation`
- 5: `source_merged_pending_model_and_snapshot_runtime_validation`
- 2: `source_merged_pending_rollout_lookup_runtime_validation`
- 1: `source_merged_pending_bazel_windows_sharding_validation`
- 1: `cargo_generated_from_integrated_source_pending_release_validation`
- 1: `source_merged_pending_cli_runtime_validation`

## Ordered original textual conflicts: evidence to review, **not yet owner-accepted**

The following paths exactly correspond to the original blocked discovery's 22-path list by its unchanged report digest. Source docs are references to earlier implementation/review, **not** newly endorsed per-conflict `semantic_evidence_url` values. Each path requires a review SHA, test URL, explicit resolution strategy, and semantic evidence before a v2 sidecar can be accepted.

| # | Original path | Source packet | Historical status | Prior source evidence |
| ---: | --- | --- | --- | --- |
| 1 | `codex-rs/app-server-daemon/src/backend/windows.rs` | 3C.1 | source merged pending native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c1-windows-daemon-startup.md) |
| 2 | `codex-rs/app-server-daemon/src/lib.rs` | 3C.1 | source merged pending native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c1-windows-daemon-startup.md) |
| 3 | `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst` | 3B.4 | generator regenerated pending release validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b4-generated-schemas.md) |
| 4 | `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst` | 3B.4 | generator regenerated pending release validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b4-generated-schemas.md) |
| 5 | `codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts` | 3B.4 | generator regenerated pending release validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b4-generated-schemas.md) |
| 6 | `codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts` | 3B.4 | generator regenerated pending release validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b4-generated-schemas.md) |
| 7 | `codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts` | 3B.4 | generator regenerated pending release validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b4-generated-schemas.md) |
| 8 | `codex-rs/config/src/config_toml.rs` | 3B.1b | source merged pending native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b1b-config-reconciliation.md) |
| 9 | `codex-rs/core/src/config/mod.rs` | 3B.1b | source merged pending native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b1b-config-reconciliation.md) |
| 10 | `codex-rs/rollout/src/policy.rs` | 3B.1a | source merged pending native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3b1b-config-reconciliation.md) |
| 11 | `codex-rs/tui/src/app/tests/permission_selection_tests.rs` | 3C.3b | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c3a-slash-dispatch.md) |
| 12 | `codex-rs/tui/src/app/thread_routing.rs` | 3C.2 | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 13 | `codex-rs/tui/src/app/thread_session_state.rs` | 3C.2 | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 14 | `codex-rs/tui/src/app/working_directory.rs` | 3C.2 | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 15 | `codex-rs/tui/src/app_server_session.rs` | 3C.2 | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 16 | `codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs` | 3C.2 | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 17 | `codex-rs/tui/src/app_server_session/rollout_history.rs` | 3C.2 | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 18 | `codex-rs/tui/src/chatwidget/slash_dispatch.rs` | 3C.3a | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c3a-slash-dispatch.md) |
| 19 | `codex-rs/tui/src/daemon_startup.rs` | 3C.1 | source merged pending native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c1-windows-daemon-startup.md) |
| 20 | `codex-rs/tui/src/session_start.rs` | 3C.2 | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c2-session-state-checkpoint.md) |
| 21 | `codex-rs/tui/src/slash_command.rs` | 3C.3a | source merged pending tui compilation and native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c3a-slash-dispatch.md) |
| 22 | `codex-rs/tui/src/startup_orchestration.rs` | 3C.1 | source merged pending native validation | [source packet](https://github.com/Takuetsu/codex-dd-astra/blob/544a29516353cb1e416b6097c46d8c83b23c7eb9/docs/codexdd-0.4.3-packet-3c1-windows-daemon-startup.md) |

## Required v2 sidecar/manifest acceptance work — fail closed

The merged `.github/scripts/upstream_reconciliation_v2.py` enforces exact canonical UTF-8 JSON, SHA/tree, official tag object and immutable blocked-discovery bindings. The future source-bound sidecar and v2 manifest require **actual verified evidence**, including:

1. **22 / 22** ordered conflict-resolution records with valid owner packet, strategy (`manual_merge`, `generator_rebuilt` or `upstream_supersedes`), reviewed commit SHA, relevant executable test URL and semantic acceptance link.
2. **136 / 136** ordered overlap-review records with disposition, all sensitive classification categories preserved, actual test URL and semantic evidence link. The 124 flagged sensitive paths and the 12 previously unclassified paths are not waived.
3. Complete provenance for every generated protocol/SDK overlap, with reproducible command, final source commit SHA and generator drift CI evidence; the old generator run alone is not proof of drift-free final source.
4. Six source-SHA-bound security reviews: `windows_elevated_f2`, `adaptive_model_routing`, `worker_scope_and_lifecycle`, `lvo_receipt`, `updater_fail_closed`, and `resume_fork`, plus an explicit owner source-specific semantic review link.
5. Exact current production SHA/tree `16fdc982...`, immutable original blocked manifest hash `3286752a1f6c31dd2b4f0f9fcb0ccadb34bcf004e3bf686a8811c2dd2ff8ebb3`, unchanged original conflict report hash `c3456c3e93130126f8f6f6bc1366bdafcc0c11835244ba70c31b91a564a450a4`, official upstream tag-object identities and **one frozen** source commit/tree. Any new source change requires regenerating relevant receipts/evidence.
6. After evidence review, stage *only four* regular metadata blobs (new v2 manifest, sidecar, copied original blocked manifest and copied original conflict report) in a one-parent commit on the then-live production. The publish-once workflow must verify staging, immutable artifacts and absence of the canonical branch before any write.

**Release blockers unchanged:** exact native Daniel-CL 21-stage release profile, elevated F2/SSH and detached daemon smoke, semantic owner source review, candidate-bound native PASS receipt and separately approved publication/PR/merge/install gates. Do not manufacture success links, semantic sign-off, generated drift, host assertions, or a candidate ref to make the schema pass.

*This index supports review. It grants no authority to merge, publish, install or promote.*

## Official annotated upstream tag-object cross-check — 2026-10-11

Using the official `openai/codex` GitHub repository's **actual ref objects** (not a fork-local tag alias):

| Official annotated tag | Exact official tag-object SHA | Peeled commit SHA | Target tree from original discovery |
| --- | --- | --- | --- |
| `rust-v0.159.2` | `8b9fa496bbf2c47aebd62e85a080b9a522a455b5` | `ff6aec96948b70d94983af2641a6b67c94faeff5` | `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` |
| `rust-v0.162.0` | `1f3f93473394b620b35580859b7e6864f7a9f948` | `c1382380de69521303b416720a52f42d51af6248` | `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |

Both remote refs resolve to annotated Git `tag` objects, and both tag objects point directly to the listed commits. The target tree identities remain bound to the fresh original manifest; the read-only discovery gate checks these tag/ref/tree identities again, failing closed if a ref changes. GitHub reported `verification.verified: false` for both tag objects; this check establishes official Git object identity via the public repository API, **not** a verified cryptographic tag signature or owner semantic approval.
