# CodexDD 0.4.5 — Packet 3F.3: focused native Windows core conflict behavior

**Status: EXACT-SOURCE READ-ONLY WINDOWS CORE RUNTIME 15/15 PASSED; NOT A RELEASE CANDIDATE.**

## Rationale and minimal target

- The authentic upstream 0.163 dry-run [#39613477624](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39613477624) identified 15 direct conflicts, including `codex-rs/core/src/context_manager/history_tests.rs`, `normalize.rs` and the security-sensitive `codex-rs/core/src/tools/registry.rs`.
- [3F.2](codexdd-0.4.5-packet-3f2-upstream-conflict-replay.md) froze all source SHA identities and the `NOT_ACCEPTED` semantic/release status. [GitHub source guards #39614998605](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39614998605) passed **432/432** static checks on SHA `8ed61061964e224b50fe27a903528d562b2a510a`, including the direct conflict manifest.
- Existing `.github/workflows/codexdd-045-windows-core-authority.yml` already proved nine focused core authority/replay cases at older nonrelease [run #39490784260](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39490784260). Reuse this **one** proven Windows-2022 harness and its cache rather than create another environment or run the complete 21-stage suite. Do not assume prior green is exact-head runtime acceptance.

## Source and test boundaries

- Workflow change only: add three history/context boundary regressions plus three registry policy/authorization regressions to the existing nine test filters, for **15 individually reported cargo test filters**. Stop on first failure and require an actual `test result: ok. >0 passed` line per filter; otherwise mark CI RED and upload the real log.
- Pinned protected blobs: `history_tests.rs` `4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d`, `normalize.rs` `4ca88c6a668e9896df06ca81940a0e41e357898d`, `registry.rs` `fc75b3bbf91f1881bd32faaaca93356fbb598b5e`.
- New history cases: `delivered_assistant_context_invalidates_reviews_without_changing_authorization`, `checkpoint_replayed_messages_keep_legacy_review_when_the_source_survives`, `legacy_inter_agent_assistant_messages_are_not_turn_boundaries`.
- New registry cases: `adaptive_reconnaissance_rejects_hook_rewritten_write_command`, `adaptive_reconnaissance_rejection_finishes_blocked_without_start`, `adaptive_validation_terminalization_allows_only_trusted_signal_tool`.
- CI is branch-bound, read-only `contents: read`, pinned Rust 1.95.0, exact unchanged `Cargo.toml`, `Cargo.lock`, `deny.toml` and product release markers, Windows Rusty V8 input, no publishing/installer/merge permission. No Daniel-CL command needed.

## What completion would and would not prove

A green Windows native job would specifically improve the **core history/registry original conflict** behavioral evidence, including a malicious PreToolUse hook rewrite which must be rejected *after* the hook; it does not assert that the protected TUI, package build, or all-target security gate is release green. The four inherited INFO/unmaintained RustSec advisories still require remediation or explicit owner-reviewed expiry before 0.4.5 release acceptance. Do not weaken `deny.toml`, auto-publish, edit product markers or invoke the long native release suite in this packet.

## Focused native Windows closeout — October 21, 2026

[Windows core authority run #39615406741](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39615406741) **passed all fifteen individually matched tests** on exact SHA `ac7064492a580f52efd724f8b19ab2eaf6d15e49`, including three context history/replay cases and three post-hook registry policy/cancellation cases directly targeting the original 0.163 core conflicts. [TUI run #39618752181](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39618752181) independently passed its **25/25** selectors, covering six original TUI-conflict files. Thus all nine Rust-file direct-conflict areas have focused native behavior evidence. These remain bounded, nonrelease CI receipts: they cannot replace complete source-overlap signoff, the native Bazel package gate, unresolved inherited cargo-deny advisories, or a frozen candidate's detached 21-stage Windows release suite.
