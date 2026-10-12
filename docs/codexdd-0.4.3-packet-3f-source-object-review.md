# CodexDD 0.4.3 — SHA-verified full source-object reconciliation audit

**Status: READ-ONLY FOUR-TREE INVENTORY PASS; 136 semantic overlap approvals, native Windows acceptance and source-bound v2 attestation still pending.**

This is a grounded source-object comparison, **not** a semantic approval or release receipt. The accompanying [machine-readable 136-row ledger](codexdd-0.4.3-packet-3f-source-object-review.json) freezes exact file blob IDs, original classifier categories, provisional conflict resolution strategy, source packet, and relevant *area-level* CI evidence for every original overlap.

## Source identities and exact coverage

| Role | Git commit | Git tree |
| --- | --- | --- |
| Tracked official `rust-v0.159.2` | `ff6aec96948b70d94983af2641a6b67c94faeff5` | `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` |
| Historical production before PR #50 | `4828e3b4232781963594cfaaebdc49c5531de8b1` | `f486f0b7ec5a909849246ee91f01af098c08421a` |
| Current post-tooling production | `16fdc9826e800ebabab920eadd8e1d976d2f645f` | `76ce3087f8808dc57ed3db809ea0b919076e5f65` |
| Official target `rust-v0.162.0` | `c1382380de69521303b416720a52f42d51af6248` | `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |
| Tested isolated source snapshot before audit documents | `2e70b04e1c80b5d9d2ef9ae67a3c9a7e902e63ff` | `9676469c99221ab0be361c06048ed6f2df1b8bd4` |

All five recursive Git trees were fetched untruncated, including the 23 newly merged approved v2 tooling paths. Paths were compared by exact Git mode and blob IDs across presence/absence and modification, not approximate text matches.

- Current production customization delta: **279** paths compared with official 0.159.2; upstream target delta: **2,507**.
- **136/136** source overlaps match the historical owner ledger in exact order; 22/22 original textual conflicts remain accounted for. Original discovery [#38155514359](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155514359) preserved their identities; its expected **blocked** result is not rewritten to `clean`.
- **134** source overlaps contain source union or regenerated content differing from both ancestors. **2** intentionally match official 0.162.0 byte-for-byte; those still require semantic review.
- **2,368/2,371** upstream-only paths are exact target matches; **137/143** fork-only paths are exact production matches. Remaining 3 upstream-only and 6 fork-only exceptions are enumerated below.
- **36 integration-only paths** (before this audit) are 0.4.3 documentation and temporary read-only validation workflow files. No unexplained standalone runtime source paths were found. All 124 previously flagged overlaps and 12 old-classifier blind spots retain their review classification.
- Current check evidence: [628 targeted Rust tests](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769), [CLI/core/identity](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292765), [v1/v2 and Windows PowerShell fixtures](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292794), [postmerge discovery](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155949309); these are **area-limited** and not per-path native Windows PASS.

### Documented non-overlap exceptions

**Upstream-only source adjustments (3)**

- `codex-rs/tui/src/app/tests/navigation_reconnect_tests.rs`: differs intentionally from official target; needs reviewed regression evidence, see pinned source test runs.
- `codex-rs/tui/src/chatwidget/snapshots/codex_tui__chatwidget__tests__slash_copy_whole_status.snap`: differs intentionally from official target; needs reviewed regression evidence, see pinned source test runs.
- `codex-rs/tui/src/status/snapshots/codex_tui__status__tests__status_snapshot_remote_server.snap`: differs intentionally from official target; needs reviewed regression evidence, see pinned source test runs.

**Fork-only source adjustments (6)**

- `codex-rs/cli/tests/version_reporting.rs`: differs intentionally from current production; retain version/API adaptation proof before acceptance.
- `codex-rs/codexdd-version.txt`: differs intentionally from current production; retain version/API adaptation proof before acceptance.
- `codex-rs/core/src/tools/handlers/codexdd_validation.rs`: differs intentionally from current production; retain version/API adaptation proof before acceptance.
- `codex-rs/tui/src/chatwidget/tests/adaptive_effort_tests.rs`: differs intentionally from current production; retain version/API adaptation proof before acceptance.
- `codex-rs/tui/src/chatwidget/tests/adaptive_pressure_tests.rs`: differs intentionally from current production; retain version/API adaptation proof before acceptance.
- `codex-rs/upstream-codex-release.txt`: differs intentionally from current production; retain version/API adaptation proof before acceptance.

## All 22 original textual conflicts — proposed disposition only

These are **suggestions based on upstream/blob equality or earlier source packet documentation**, not human review or complete v2 `semantic_evidence_url` acceptance. The linked 136-row ledger contains exact blob IDs, historical status, and packet evidence for each.

| # | Conflict path | Owner | Suggested strategy | Source identity |
| ---: | --- | --- | --- | --- |
| 1 | `codex-rs/app-server-daemon/src/backend/windows.rs` | 3C.1 | `manual_merge` | Manual/generated union |
| 2 | `codex-rs/app-server-daemon/src/lib.rs` | 3C.1 | `manual_merge` | Manual/generated union |
| 3 | `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst` | 3B.4 | `generator_rebuilt` | Manual/generated union |
| 4 | `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst` | 3B.4 | `generator_rebuilt` | Manual/generated union |
| 5 | `codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts` | 3B.4 | `generator_rebuilt` | Manual/generated union |
| 6 | `codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts` | 3B.4 | `generator_rebuilt` | Manual/generated union |
| 7 | `codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts` | 3B.4 | `generator_rebuilt` | Manual/generated union |
| 8 | `codex-rs/config/src/config_toml.rs` | 3B.1 | `manual_merge` | Manual/generated union |
| 9 | `codex-rs/core/src/config/mod.rs` | 3B.1 | `manual_merge` | Manual/generated union |
| 10 | `codex-rs/rollout/src/policy.rs` | 3B.1 | `manual_merge` | Manual/generated union |
| 11 | `codex-rs/tui/src/app/tests/permission_selection_tests.rs` | 3C.3 | `upstream_supersedes` | Target identical |
| 12 | `codex-rs/tui/src/app/thread_routing.rs` | 3C.2 | `manual_merge` | Manual/generated union |
| 13 | `codex-rs/tui/src/app/thread_session_state.rs` | 3C.2 | `manual_merge` | Manual/generated union |
| 14 | `codex-rs/tui/src/app/working_directory.rs` | 3C.2 | `manual_merge` | Manual/generated union |
| 15 | `codex-rs/tui/src/app_server_session.rs` | 3C.2 | `manual_merge` | Manual/generated union |
| 16 | `codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs` | 3C.2 | `manual_merge` | Manual/generated union |
| 17 | `codex-rs/tui/src/app_server_session/rollout_history.rs` | 3C.2 | `manual_merge` | Manual/generated union |
| 18 | `codex-rs/tui/src/chatwidget/slash_dispatch.rs` | 3C.3 | `manual_merge` | Manual/generated union |
| 19 | `codex-rs/tui/src/daemon_startup.rs` | 3C.1 | `manual_merge` | Manual/generated union |
| 20 | `codex-rs/tui/src/session_start.rs` | 3C.2 | `manual_merge` | Manual/generated union |
| 21 | `codex-rs/tui/src/slash_command.rs` | 3C.3 | `manual_merge` | Manual/generated union |
| 22 | `codex-rs/tui/src/startup_orchestration.rs` | 3C.1 | `manual_merge` | Manual/generated union |

## Exact remaining owner review gates

1. **136 independent semantic dispositions:** review official target changes and retained fork contracts per path; cite source-specific evidence and relevant tests. This audit provides proposed strategy and hash references but does not fill the v2 attestation's `semantic_evidence_url` with an invented approval.
2. **22 ordered conflicts:** accept the per-path strategy and test evidence, including all 4 Windows daemon/F2 files, 5 regenerated schemas, 7 TUI/session files and command-routing conflicts. Upstream-byte-equal permission selection still requires security validation.
3. **Generated artifacts:** release-source-bound `generator_source_sha` and exact drift testing must be rerun before the v2 manifest can treat old 3B.4 generator receipt as final.
4. **6 security areas:** Windows elevated F2, adaptive routing, Worker scope/lifecycle, LVO receipt, fail-closed updater and resume/fork must have source-SHA-pinned evidence and explicit owner review. Historical CI passing only a subset does not authorize omitted areas.
5. **Native Windows:** run mandatory 21-stage release profile and elevated/non-elevated daemon and SSH/F2 checks on a pinned canonical candidate. A GitHub Windows PowerShell parser/receipt fixture is not a release profile.
6. **Promotion:** after explicit owner acceptance, create exactly four metadata-only evidence blobs on a one-parent staging commit from current production, then guarded single candidate publication; no candidate or receipt has been created by this audit.

No runtime source, production, candidate branch, Windows installation or release identity was changed. This review is safe to inspect before choosing future exact semantic evidence URLs and owner acceptance.
