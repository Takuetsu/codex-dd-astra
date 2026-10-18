# CodexDD 0.4.5 — Packet 3A.1: exact upstream provenance and initial dependency merge

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. A three-way `Cargo.toml` manifest integration has been staged on a separate feature branch. Source/lockfile regeneration, native validation, and owner reconciliation remain open.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`

## Independently checked source identities

| Identity | Exact value |
| --- | --- |
| Initial 0.4.4 production commit | `b25550dcb2c6e84a84074b603086c9db6f32a63b` |
| Production tree | `196252ceb8982ff52d2af3867ee3cd754bca5564` |
| Tracked official tag | `rust-v0.162.0` |
| Tracked annotated tag object | `1f3f93473394b620b35580859b7e6864f7a9f948` |
| Tracked peeled commit | `c1382380de69521303b416720a52f42d51af6248` |
| Target official annotated tag object | `129b8ec7ee6f4511557a4856279f529025c25af4` |
| Target peeled commit | `83086188ae0614bd882818ff1aa23f3194962012` |
| Target tree | `e91f335e7a03e1d14fa1e5fc7fc7620cdfc60842` |
| Official target `codex-rs/Cargo.toml` blob | `e84b9776a1a7b161a522773686888e10e248756c` |
| Staged security-reconciled `Cargo.toml` blob | `8ae5758c9baa6091b8ab5dda4e5b848365c0009d` |

The annotated tag was verified by the official upstream Git tag-object endpoint and resolved to the pinned target commit/tree. Production was rechecked before creating this branch. A moved ref requires new reconnaissance; no silent release retargeting.

## Three-way Cargo.toml reconciliation

The production `Cargo.toml` differs from upstream `rust-v0.162.0` in **one fork-specific security line**: the `serde_yaml` alias targeting `noyalib-serde-yaml =0.0.57`. The staged `Cargo.toml` starts from the complete official `rust-v0.163.0` source, preserving *all* its workspace changes, and intentionally differs from that source at only these two reviewed security settings:

1. Maintain `serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.57" }`. Upstream `serde_yaml = "0.9"` must **not** return.
2. Raise the `rustls` minimum from upstream `0.23.45` to `0.23.46`, consistent with the known 0.4.4 security-resolved version.

The upstream manifest changes preserved verbatim include workspace version `0.163.0` (not the CodexDD product version), `rmcp =3.5.1` at Git revision `4048ac40e958fb1a7d614617a2ae8d26666ee2fa`, `hyper-util 0.1.20`, `linkify 0.11`, and removal of `shell-escalation` and workspace `socket2`. This is not a claim that their downstream source references or transitive API changes have yet been reconciled.

## Outstanding 3A.1 and release gates

**Lockfile regeneration remains pending.** The repository's existing `codex-rs/Cargo.lock` is still the 0.4.4 resolved graph. It must **not** be replaced wholesale with the official upstream lockfile: regenerate from a reviewed combined source after affected crates are integrated; preserve `rustls >=0.23.46`, `quinn-proto >=0.11.18`, `noyalib-serde-yaml =0.0.57` and absence of legacy `serde_yaml`, while deliberately reviewing `rmcp =3.5.1`, `hyper 1.11.1`, security advisories, Windows build impact and any graph drift.

**No full build is expected to pass yet.** The 1,314 upstream-changed paths, 109 overlap paths and 15 original conflicts are *not* yet integrated or semantically accepted. The two generated compressed schemas must be regenerated from reviewed sources later. `.github/workflows/bazel.yml` remains unchanged until packet 3A.2; do not erase the fork's 90-minute Windows Bazel CI timeout policy by taking upstream's 30-minute default.

The CodexDD product marker remains `0.4.4` and tracked-upstream marker remains `rust-v0.162.0`. Do not alter them until source freeze and owner-approved release provenance. The original blocked 0.163.0 evidence is immutable; 3A.1 does **not** grant v2 reconciliation acceptance, native 21-stage Windows PASS, GitHub CI PASS, automated publication, merge, or installation.

**Next:** finish dependency/code reference reconnaissance and an explicit 3A.1 lockfile/security plan; stage separate 3A.2 Bazel review. Source integration checkpoints remain isolated from `dd/codexdd-v0.4.5-upstream-0.163.0-automation` and the production branch. Recheck all three identities before any eventual candidate publication.
