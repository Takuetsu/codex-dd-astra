# CodexDD 0.4.5 — Packet 3D.9: core integration test suite reconciliation

**Status: SOURCE STAGED / NATIVE WINDOWS COMPILATION PENDING. This is not a release candidate.**

## Error evidence and scope

- Previous full core integration-test compile-only job [#39532282609](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39532282609), at exact source `5d86b7a1d53b49dcfeb1c6de236d580a2909ccca`, failed with **49 compiler errors**. The 26-file scenario module path preflight had already passed; failures were in old 0.162 test fixtures and interfaces, including `ResponseItem::Message` and `ContentItem::OutputText` fields, changed client/config/plugin signatures, and missing test helpers.
- The separate Windows history compatibility job [#39532282583](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39532282583) **passed** at the same source SHA. History does not need rerunning merely to resolve core test compilation.
- This packet stages **97** core-test tree path changes, including **72** unchanged-fork upstream Rust source updates, **24** unchanged-fork upstream snapshot fixture updates, and **one** three-way-merged protected `unified_exec.rs` test. No `codex-rs/core/src` production runtime files, Cargo manifest/lock, deny policy, package marker, release marker, or deployment paths are edited.

## Exact source provenance

Four immutable source identities were compared using Git blob SHA for every file before staging:

| Role | Commit |
| --- | --- |
| Baseline official `rust-v0.162.0` | `c1382380de69521303b416720a52f42d51af6248` |
| Target official `rust-v0.163.0` | `83086188ae0614bd882818ff1aa23f3194962012` |
| Fork production 0.4.4 | `b25550dcb2c6e84a84074b603086c9db6f32a63b` |
| Reviewed branch parent | `5d86b7a1d53b49dcfeb1c6de236d580a2909ccca` |

For the **96 direct upstream copies**, the production blob equalled the official 0.162 baseline and the integration branch still held that baseline blob (or an absent new source). Each copied file matches its exact official 0.163 content and object identity. The 24 snapshots accompany upstream tests instead of inventing fixtures; they do not replace fork-modified snapshot files.

`codex-rs/core/tests/suite/unified_exec.rs` is deliberately **not** copied wholesale: the original production 0.4.4 file differed from official 0.162 by *only one inserted 83-line trusted adaptive-evidence regression*. That test was transplanted to the official 0.163 source while preserving upstream's command-origin metadata persistence test. Both named tests remain in the staged source. This closes the unchanged-fork identity overlap for `unified_exec.rs` only after actual native compile/behavior checks.

Other fork-modified `suite/agent_websocket.rs`, `suite/audio_truncation.rs`, and the previously reconciled `suite/scenarios.rs` retain their prior reviewed blob identities. The staged Git tree was compared to the exact branch parent and showed **73 Rust file changes + 24 snapshot file changes = 97 changes**.

## Acceptance and stopping rules

1. Confirm the targeted, exact-source native Windows `cargo test --locked -p codex-core --test all --no-run` job succeeds. On failure, use the uploaded **full** compile log artifact, not just its 150-line CI tail, and fix the precise remaining test/interface issue. Do not rerun unrelated passed libraries.
2. Distinguish full integration-test compilation from runtime execution; the compile-only gate must never be misreported as the 21-stage release PASS. Run only selected core runtime regressions when test compilation is green.
3. The original 109-overlap source audit, remaining Bazel owner reconciliation, security review of the four inherited unmaintained advisories, exact candidate SHA freeze, and detached 21-stage Windows release suite remain separate unreleased gates.
4. Product remains **CodexDD 0.4.4** with tracked official upstream **rust-v0.162.0** until all release decisions and evidence are accepted.

No operator action is required on Daniel-CL for this GitHub-only checkpoint.
