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
