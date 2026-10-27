# CodexDD 0.4.5 — Packet 3F.8: inherited maintenance closure and decision review

**2026-10-27. NONRELEASE; no risk exception is authorized.**

This packet follows the staged Zstd/ctutils exact-lock changes on `dd/codexdd-v0.4.5-dependency-security-remediation`, commit `928d3ac296f75c0651e7bf25c1669cd18210e630`. Reviewed lock blob: `8be43898f94d59f403f8e03d9d4b5361e492f086`; workspace manifest: `0eb50f54cfbc192e149bd90960878c27bf211a42`; unchanged `deny.toml`: `ea7fdc149fc880c6b36d75c29851641034289a4e`. Production remains 0.4.4 / rust-v0.162.0; 0.4.5 remains unreleased and draft PR #54 must remain draft.

## Individual findings — informational maintenance only

| RustSec | Crate and locked version | Target and role evidence | Preferred remedy | Acceptance |
| --- | --- | --- | --- | --- |
| RUSTSEC-2026-0374 | `basic-toml 0.1.10` | Windows and Linux build-time proc-macro via `age -> i18n-embed-fl -> i18n-config 0.4.8`. Not a linked Windows runtime library; build-time supply-chain exposure remains. | Published i18n-config maintenance release or verified, pinned upstream parser replacement. | OPEN; no waiver. |
| RUSTSEC-2026-0371 | `zbus_names 3.0.0` | Linux only via zbus 4.4; no Windows target path in prior exact locked graph. | Coordinated migration to zbus 6 `names` with all Linux owners. | OPEN; no waiver. |
| RUSTSEC-2026-0370 | `zvariant 4.2.0` | Linux only via zbus 4.4. | zbus 6 built-in types and wire codec. | OPEN; no waiver. |
| RUSTSEC-2026-0372 | `zvariant_derive 4.2.0` | Linux only via zvariant 4.2; proc macro. | zbus 6 derive types; migrate with Linux parent. | OPEN; no waiver. |

RustSec reports **INFO/unmaintained**, **no patched versions**, for each. Do not mistake an unmaintained notice for proof of exploitation, but do not ignore supply-chain implications.

## Concrete upstream feasibility

- As of October 27, the crates.io `i18n-config` release is **0.4.8** and depends on `basic-toml ^0.1`. The original `cargo-i18n` project has a **still-unpublished** Git commit `2ce7b1dfe9a8cc224288bf50cbc2a67a6de863c4` changing its version-0.4.8 crate to use maintained `toml ^1.1.6` and its deserialization error type. The original Rust public error variant changes representation; this is not an automatic compatibility guarantee.
- An isolated GitHub workflow `.github/workflows/codexdd-045-inherited-maintenance-preview.yml` tries this exact upstream source without any committed override. The existing Cargo.lock independently pins `toml 1.0.6` via `gstreamer-app-sys -> system-deps 7.0.8`; a same-major TOML update is also required. Do **not** treat any disposable scratch graph or source identity edit as a reviewed Cargo-generated release lock.
- On Linux, `codex-tui` declares `zbus 4.4.0` directly and `codex-keyring-store` declares `secret-service 4` and `keyring 3.6` with a Linux Secret Service backend. Published `secret-service 5.2.0` still requires **zbus 5**, and published `keyring 4.2.0` uses `zbus-secret-service-keyring-store 1.0.1`, itself requiring **zbus 5**. Updating CodexDD's one direct zbus reference to 6 **does not** remove all three notices, and may break Linux ABI/API compatibility. The migration requires several coordinated upstream owners, not a safe lock-only update.

Official sources:
- https://rustsec.org/advisories/RUSTSEC-2026-0374.html
- https://rustsec.org/advisories/RUSTSEC-2026-0371.html
- https://rustsec.org/advisories/RUSTSEC-2026-0370.html
- https://rustsec.org/advisories/RUSTSEC-2026-0372.html
- https://z-galaxy.github.io/zbus/upgrading-to-6.html
- https://github.com/kellpossible/cargo-i18n/commit/2ce7b1dfe9a8cc224288bf50cbc2a67a6de863c4

## Owner decision boundary

**Path A: complete dependency remediation.** Isolate/pin a verified, compatible upstream TOML parser replacement for `basic-toml`, then migrate all D-Bus dependency owners to zbus 6 with tested Linux semantics, not merely direct TUI. Require exact Cargo-generated lock, full all-target cargo-deny unmodified, platform-specific compile/behavior, schema/provenance, and all other release gates. No reduced scope or API regression accepted.

**Path B: four explicitly named INFO risk dispositions.** Requires **affirmative** owner authorization and security-review record **for each specific ID**. Current proposed expiry is **November 20, 2026, or the next CodexDD upstream integration, whichever occurs first**. Trigger earlier review if a RustSec severity/category changes, a real vulnerability is published affecting the crate, a Windows runtime dependency becomes reachable, a supported replacement ships, a build-tool dependency changes materially, or production target scope widens. The scope is only the four IDs above; new vulnerabilities (including crypto-bigint RUSTSEC-2026-0387) remain nonwaivable blockers.

For Path B, the release gate must explicitly distinguish `all-target cargo-deny without exceptions = RED` from `all-target cargo-deny with four owner-approved, recorded INFO exceptions = accepted-risk only`. Never call the latter *advisory-free*. Do not set a broad `unmaintained = allow`, target-prune Linux, blanket-ignore, or weaken unrelated checks.

**Status:** Neither path has yet closed. Do not modify `deny.toml` or add exceptions without owner authorization, promote draft PR #54, run the detached 21-stage Daniel-CL release suite, or install 0.4.5. The independent Windows-reachable `crypto-bigint 0.5.5` security blocker also remains open.

## Minimum validation before promotion

After any approved resolution: re-run locked Windows and Linux target graphs, exact Cargo lock diff/provenance, native secrets and credential-provider regressions, keyring/DBus when changed, all-target unchanged or explicitly approved narrowly excepted cargo-deny, and source/schema/semantic gates. The detached full 21-stage native Windows profile remains final, not exploratory, validation.

## Oct 27 verified pinned-source feasibility result

- [GitHub scratch run #40533479354](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40533479354) **PASSED**: exact pinned upstream i18n-config source and TOML 1.1.8 dependency resolution on Windows and Linux; both locked target graphs omit basic-toml.
- The unchanged security pins and release markers passed. The pinned upstream crate compiled in an independently scoped Rust fixture. Its version remains 0.4.8 and its public deserialization-error representation changed. **Not native Windows end-to-end compatibility, cargo-deny, source provenance, or production release acceptance.**
- The generated candidate manifest Git blob is `d46347640b6ca6e43e96a12ee14eec894178af4f`; candidate Cargo.lock blob `f19b0086eac09c6830c887be83db5ae4ae11587e`. Their complete bytes were recovered from the CI log and verified against Git SHA, not approximated or hand-authored.
- These exact files are staged for review on `dd/codexdd-v0.4.5-basic-toml-pinned-review` only. They remove basic-toml via unpublished, pinned upstream Git source and update the parser/TOML transitive group. **No deny.toml waiver.** The remaining three Linux-specific unmaintained notices and `crypto-bigint` are still open, so both PR #54 and the release gate remain blocked.

## Byte-exact source/lock reconciliation (Oct 27)

A full 1,477→1,476 Cargo package-entry comparison of the accepted Zstd/ctutils lock (`8be43898...`) with candidate `f19b0086...` found exactly six removed and five added package versions. The six removals are `basic-toml 0.1.10`, `serde_spanned 1.0.4`, `toml 1.0.6+spec-1.1.0`, `toml_datetime 1.1.1+spec-1.1.0`, `toml_parser 1.1.3+spec-1.1.0`, and `toml_writer 1.0.6+spec-1.1.0`. The five additions are `serde_spanned 1.1.2`, `toml 1.1.8+spec-1.1.0`, `toml_datetime 1.1.2+spec-1.1.0`, `toml_parser 1.1.5+spec-1.1.0`, and `toml_writer 1.1.3+spec-1.1.0`.

Exactly two still-present package blocks change: `i18n-config 0.4.8` (registry→exact upstream Git rev, remove basic-toml and `serde_derive` direct edges, `thiserror 1.0.69`→`2.0.18`, add `toml 1.1.8` edge) and `system-deps 7.0.8` (update its `toml` edge only). The workspace manifest adds precisely the one new `i18n-config` Git patch and its explicitly NONRELEASE comment. No other manifest lines change. The upstream crate has an unreleased breaking change in its TOML error representation; compile and behavior through actual `age`/CodexDD secrets integration is **not yet proven** by a standalone `i18n-config` crate check.

Do not merge or promote this candidate until tests check secret recovery, malformed config/error mapping, pinned repository source trust, all-target RustSec policy and Windows native regression behavior. This candidate does not authorize any advisory waiver.
