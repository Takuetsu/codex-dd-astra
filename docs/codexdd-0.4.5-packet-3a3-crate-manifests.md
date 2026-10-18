# CodexDD 0.4.5 — Packet 3A.3: exact upstream crate manifest staging

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Nine Cargo subcrate manifests staged from pinned official 0.163.0 source. The workspace is intentionally not yet buildable as a complete 0.163.0 integration. **Cheap Python validation pending.**  
**Integration branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Verified three-way source contract

The exact `rust-v0.162.0` Git blob for **each of these nine paths** matches the 0.4.4 production blob byte-for-byte. Therefore they do not contain CodexDD fork-side edits, and their upstream `rust-v0.163.0` blob can be copied without overwriting a local customization:

| Official upstream 0.163.0 path | Exact Git blob | Dependency change |
| --- | --- | --- |
| `codex-rs/arg0/Cargo.toml` | `fad5169ffa9e150e70fbe725b853f68594ee38bc` | Remove former shell escalation crate |
| `codex-rs/code-mode/Cargo.toml` | `daa2de54718268e12418ca981af1481841312867` | Add `rand` |
| `codex-rs/codex-mcp/Cargo.toml` | `f54c6f1cbdf112af2f93524508bcce9935700740` | Add process/absolute-path/cargo-bin utilities |
| `codex-rs/core/Cargo.toml` | `39c860803af2517d97344d491cd9b995e2e89288` | Add `hyper-util`, `tonic`, `tower`; remove shell escalation dependency |
| `codex-rs/exec-server/Cargo.toml` | `0d1f1f5209ca66c6efdd9f6bb54058e89bc7a542` | Add Linux-only sandbox and `which` |
| `codex-rs/exec/Cargo.toml` | `d5190c2caa2d37217a0335e4c9dd84f796486458` | Add `insta` dev dependency |
| `codex-rs/external-agent-migration/Cargo.toml` | `3f49a1ae00cc48720b766d9e76bc050e954cf0fa` | Add `linkify` |
| `codex-rs/tools/Cargo.toml` | `fc75ad22ce146ee5dd852ef2cad03b4bbcdffffb` | Remove obsolete workspace utility/tracing dependencies |
| `codex-rs/tui/Cargo.toml` | `ea0894b3e1d3777f4c598727b03199c1f69cbd05` | Add `opentelemetry_sdk` dev dependency |

All nine new fork blobs have been created from official upstream content and have the **same Git blob SHAs**. They are staged together with the previously verified 3A.1 root `Cargo.toml` blob `8ae5758c9baa6091b8ab5dda4e5b848365c0009d`; 3A.1 retains fork-specific security controls for `rustls` and `noyalib-serde-yaml`.

## Deliberate incomplete boundaries

- **Cargo.lock is still the exact baseline 0.4.4 blob** `0c913b46674d3593ee84f1fee5508d16e33efae5`. It cannot be correctly regenerated until the **remaining upstream source graph**, dependencies and security changes have been reconciled. Do not overwrite it with upstream's lockfile or claim `cargo --locked` is expected to pass on this partial source.
- Upstream removed `codex-rs/shell-escalation/` from its workspace; the historical fork crate source has **not** been blindly deleted. Evaluate remaining references, Windows elevated/non-elevated fallback and F2 before removing source or declaring its APIs obsolete.
- All **109 overlapping paths** still require semantic review, including the 15 original conflicts; no generated compressed schema has been regenerated, and upstream-only Rust source changes have not been fully transplanted.
- The previous 3A.2 Bazel workflow reconciliation remains **documented but not applied**. Production, official release marker `rust-v0.162.0`, product marker `0.4.4`, old historical artifacts, feature-branch auto-publishing gates and CI permissions remain unchanged.

## Tests and next packet

`.github/scripts/test_codexdd_045_3a_manifest_stage.py` adds **12 inexpensive static tests** reading committed Git blobs, parsing pinned Cargo manifests, ensuring alias/security/added dependencies, preserving lockfile and version markers and confirming source is still incomplete. This is a source check only; it is **not** a compile, Windows smoke test or native release receipt.

Next complete 3A.4 dependency/source-reference reconnaissance, then coordinate the 1,205 non-overlap upstream source paths with the 109-path overlap ledger and use reviewed, isolated 3B/3C/3D packets. Run only targeted cheap tests at the next meaningful checkpoint and defer all expensive release validation to an actual source-complete candidate.
