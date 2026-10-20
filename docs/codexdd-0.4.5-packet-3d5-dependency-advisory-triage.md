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
