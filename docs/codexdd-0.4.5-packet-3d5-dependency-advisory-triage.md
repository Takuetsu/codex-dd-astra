# CodexDD 0.4.5 — Packet 3D.5: dependency advisory triage (nonrelease)

**Status: advisory gate RED, unresolved; source/security floors retained.**
**Target: Windows-only CodexDD 0.4.5, unreleased rust-v0.163.0 integration.**

## Reproducible evidence

- [Cargo-deny GitHub run 39446488018](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39446488018) on exact SHA `28471b05eb1676fc2920e17ec0dc961b657397a3` used the repository's unmodified reusable `.github/workflows/cargo-deny.yml` and pinned `codex-rs/deny.toml`.
- **Advisories FAILED**; bans, licenses, and sources passed. Ignore lists were not modified. The check also reported unrelated duplicate-version warnings.
- Approved Windows-resolved `Cargo.lock` is still Git blob `096939fcec32cb259bccc0cb36c5d4120701da57`. This is a nonrelease dependency/security assessment, not an authorization to change the lockfile or publish.
- Independent read-only comparison of `codex-rs/Cargo.lock` at production 0.4.4 SHA `b25550dcb2c6e84a84074b603086c9db6f32a63b` and the 0.4.5 branch shows **all six affected crate versions below existed in the 0.4.4 resolved graph**. Their presence is not a newly introduced 0.163 dependency change. Newer advisory database findings still require resolution before a clean release decision.

## Findings: no blanket exemptions

| Crate and locked version | Advisory | Finding and dependency owner |
| --- | --- | --- |
| `zip 2.4.2` | RUSTSEC-2026-0367 | ZIP extraction Unix SUID/SGID bit preservation and potential privilege escalation; direct dependency of `codex-core-plugins` |
| `zip 2.4.2` | RUSTSEC-2026-0368 | `ZipArchive::extract` may extract symlinks targeting outside the destination; `codex-core-plugins` |
| `zip 2.4.2` | RUSTSEC-2026-0369 | ZIP extraction Unix permissions through symlinks without confining symlink targets; `codex-core-plugins` |
| `basic-toml 0.1.10` | RUSTSEC-2026-0374 | Unmaintained; reached through `i18n-config` |
| `zbus_names 3.0.0` | RUSTSEC-2026-0371 | Unmaintained; reached through `zbus 4.4.0` |
| `zvariant 4.2.0` | RUSTSEC-2026-0370 | Unmaintained; reached through `zbus 4.4.0` |
| `zvariant_derive 4.2.0` | RUSTSEC-2026-0372 | Unmaintained; reached through `zvariant 4.2.0` |
| `spin 0.9.8` | Yanked version warning (no RustSec ID in receipt) | Reached through `flume 0.12.0` and `heapless 0.7.17`; warning, not one of the seven advisory errors |

`zip` is used by `codex-core-plugins`, whose `Cargo.toml` includes a direct `zip` workspace dependency. No claim of harmlessness follows from the product being Windows-only: two reports concern Unix permissions specifically, but symlink/path traversal behavior must be separately reviewed for the actual extraction call sites and Windows semantics.

## Bounded next actions

1. **Read-only source analysis:** locate every `zip` extraction call reachable from `codex-core-plugins`; determine whether APIs named in the advisories are used for untrusted downloads or marketplace/plugin bundles, including any explicit path/permission guards.
2. **Upstream remediation analysis:** determine compatible, secured `zip` versions and the required dependency/feature changes. Do not blindly copy an official upstream Cargo.lock, upgrade across an API-major boundary, or downgrade existing CodexDD `rustls`, `quinn-proto`, or `noyalib-serde-yaml` security versions.
3. **Transitive maintenance:** review `i18n-config` and `zbus` dependency upgrade/removal availability, plus the yanked `spin` path. Propose separately testable, pinned resolver changes only when compatibility and downstream use are understood.
4. **Security acceptance:** rerun a full unmodified cargo-deny check after any remediation, or request a distinct, evidence-backed release decision on inherited unresolved risks. No permanent/silent `deny.toml` ignores and no downgrade of audit severity.
5. Only after successful focused Windows native tests, source overlap/provenance closure, schema reproducibility, and this advisory decision should the detached SHA-bound 21-stage release suite be authorized.

The unrelated source provenance checkpoint is [417/417 green](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39446968108) at commit `d596cf5028e2fa5ee8c1c01e239aeb4d595edeec`. This does not waive the Cargo advisory gate.

Production remains CodexDD 0.4.4 tracking `rust-v0.162.0`. Automatic upstream publication, installation, and version promotion remain disabled.

## Post-ZIP 9 follow-up audit (2026-10-20)

**Still RED: different advisory/license blockers; ZIP advisory remediation is distinct from cargo-deny acceptance.**

- [Cargo-deny run #39463513838](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39463513838) on source SHA `8ebadcc04cc5386aec48067ad7e90f85861d8efb` evaluated the reviewed Cargo-generated ZIP 9.0.5 lockfile (blob `48ae4fb45a83de4d72dee6200b83a770dbf8f760`). The previously failing `zip 2.4.2` RUSTSEC-2026-0367/0368/0369 findings were not reported, confirming they are no longer present in the selected graph.
- Four inherited unmaintained advisories **still fail**: `basic-toml 0.1.10` (RUSTSEC-2026-0374, pulled via `i18n-config 0.4.8`/the `age` dependency chain), plus `zbus_names 3.0.0` (RUSTSEC-2026-0371), `zvariant 4.2.0` (RUSTSEC-2026-0370), and `zvariant_derive 4.2.0` (RUSTSEC-2026-0372). These last three use `zbus 4.4.0` through `codex-tui`, `keyring 3.6.3`, and `secret-service 4.0.0`; advisory notes that `zbus_names` moves to `zbus` 6, not a direct nonbreaking patch.
- `spin 0.9.8` remains a **yanked-version warning**, reached through `flume 0.12.0` and `heapless 0.7.17`; it is not one of the four remaining advisory errors.
- A **new license-policy rejection**, not a known exploit or vulnerability, arises from `zip 9.0.5` default feature `bzip2` → `bzip2 0.6.1` → `libbz2-rs-sys 0.2.5` with SPDX license identifier `bzip2-1.0.6`, currently not in `codex-rs/deny.toml`'s approved licenses. Thus `advisories FAILED` and `licenses FAILED`; `bans` and `sources` passed. The audit is not waived.
- The ZIP 9 upstream feature manifest specifies `bzip2` as enabled by default, and the independent `deflate` feature enables the common ZIP compression format. The curated-plugin archive fallback unpacks GitHub zipball and export ZIPs. **Proposed separately reviewed next packet**: determine if both sources guarantee only stored/deflate entries. If verified, trial `default-features = false, features = ["deflate"]` in an isolated Windows Cargo resolver preview and rerun archive fallback/security tests and unmodified cargo-deny. If other algorithms are required, retain them and separately review SPDX `bzip2-1.0.6` license compliance before any narrow allowlist exception. Do not broaden the global allowlist to silence failure by assumption.
- Keep historical staging hashes in the prior sections as evidence of their period. New ZIP migration source/lock provenance and native test status are documented in Packet 3D.6. **Do not initiate the expensive release suite or promote the production markers while any security/behavior/CI acceptance is pending.**
