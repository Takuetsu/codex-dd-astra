# CodexDD 0.4.5 — Packet 3D.5: dependency advisory triage (nonrelease)

**Status: advisory gate RED, unresolved; source/security floors retained.**
**Target: Windows-only CodexDD 0.4.5, unreleased rust-v0.163.0 integration.**

## Reproducible evidence

- [Cargo-deny GitHub run 39446488018](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39446488018) on exact SHA `28471b05eb1676fc2920e17ec0dc961b657397a3` used the repository's unmodified reusable `.github/workflows/cargo-deny.yml` and pinned `codex-rs/deny.toml`.
- **Advisories FAILED**; bans, licenses, and sources passed. Ignore lists were not modified. The check also reported unrelated duplicate-version warnings.
- Approved Windows-resolved `Cargo.lock` is still Git blob `096939fcec32cb259bccc0cb36c5d4120701da57`. This is a nonrelease dependency/security assessment, not an authorization to change the lockfile or publish.
- Independent read-only comparison of `codex-rs/Cargo.lock` at production 0.4.4 SHA `b25550dcb2c6e84a84074b603086c9db6f32a63b` and the 0.4.5 branch shows **all six affected crate versions below existed in the 0.4.4 resolved graph**. Their presence is not a newly introduced 0.163 dependency change. Newer advisory database findings still require resolution before a clean release decision.

## Findings: no blanket exemptions

| Crate and locked version | Advisory                                          | Finding and dependency owner                                                                                                 |
| ------------------------ | ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `zip 2.4.2`              | RUSTSEC-2026-0367                                 | ZIP extraction Unix SUID/SGID bit preservation and potential privilege escalation; direct dependency of `codex-core-plugins` |
| `zip 2.4.2`              | RUSTSEC-2026-0368                                 | `ZipArchive::extract` may extract symlinks targeting outside the destination; `codex-core-plugins`                           |
| `zip 2.4.2`              | RUSTSEC-2026-0369                                 | ZIP extraction Unix permissions through symlinks without confining symlink targets; `codex-core-plugins`                     |
| `basic-toml 0.1.10`      | RUSTSEC-2026-0374                                 | Unmaintained; reached through `i18n-config`                                                                                  |
| `zbus_names 3.0.0`       | RUSTSEC-2026-0371                                 | Unmaintained; reached through `zbus 4.4.0`                                                                                   |
| `zvariant 4.2.0`         | RUSTSEC-2026-0370                                 | Unmaintained; reached through `zbus 4.4.0`                                                                                   |
| `zvariant_derive 4.2.0`  | RUSTSEC-2026-0372                                 | Unmaintained; reached through `zvariant 4.2.0`                                                                               |
| `spin 0.9.8`             | Yanked version warning (no RustSec ID in receipt) | Reached through `flume 0.12.0` and `heapless 0.7.17`; warning, not one of the seven advisory errors                          |

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

## Post-BZIP2 remediation: current audited resolver at 0.4.5 (October 20, 2026)

**Release advisory gate is still RED, but the ZIP9 BZIP2 license blocker is resolved.**

- Exact integration security fix: [commit c6c65a7](https://github.com/Takuetsu/codex-dd-astra/commit/c6c65a7f603d651b35288310a2a220e30d79370f), with canonical LF workspace manifest blob `c78e1e92e3cfd21a4d16ceea2d55b43d1976e8e4` and byte-identical Cargo-generated lock blob `a8eb8a076276cb3db5674b33c67d743b1c3d21fc`.
- This matches the exact [green no-BZIP2 Windows preview #39464423065](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39464423065), sealed artifact `12285200057` (archive SHA-256 `03624d85fb5e1d8e4e2d12fddea3cb4e1adfd4f2df5deabab12df20340eec975`), and [green one-shot orphan blob import #39467022057](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39467022057).
- Workspace ZIP now explicitly selects its reviewed v9 algorithms _excluding_ BZIP2; new resolver removed both `bzip2 0.6.1` and `libbz2-rs-sys 0.2.5`. No license allowlist or deny-advisory exceptions were added.
- [Unmodified cargo-deny #39467522626](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39467522626) reports **advisories FAILED, bans ok, licenses ok, sources ok**. The only errors are inherited informational **unmaintained** advisories `RUSTSEC-2026-0374` (basic-toml), `RUSTSEC-2026-0371` (zbus_names), `RUSTSEC-2026-0370` (zvariant), and `RUSTSEC-2026-0372` (zvariant_derive). `spin 0.9.8` is still a yanked warning; no newly identified ZIP advisory or BZIP2 license error remains.
- [Windows/Linux locked target-graph probe #39468311160](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39468311160), artifact `12286560303` (SHA-256 `f1f2c609889525eb3360e0f572c0fc0bd84ff259e922f9a9f7a8fc617ed5e36c`), proves `zbus 4.4.0`, `zbus_names 3.0.0`, `zvariant 4.2.0` and `zvariant_derive 4.2.0` **not reachable via the Windows target dependency edges** from any workspace member, but reachable on Linux through `codex-tui`. This does **not** change the all-target cargo-deny policy or exempt Linux risk.
- `basic-toml 0.1.10` **is Windows reachable** via `codex-secrets -> age 0.12.1 -> i18n-embed-fl 0.10.1 -> i18n-config 0.4.8 -> basic-toml`. That upstream dependency chain requires separate maintenance/remediation review; replacing secrets infrastructure merely to pass an informational audit is not authorized.
- The same probe generated a compatible, **uncommitted** Cargo-only `spin 0.9.8 -> 0.9.9` trial: its lock blob `c121f97894bee98b48eab6302751723052a99caa` changes only the spin package version/checksum and retains security pins. This is a future separately approved warning cleanup, not an accepted source or release lock. Do not restart expensive native tests just to clear this warning.

**Next release decision:** Resolve the inherited maintenance advisories via compatible tested upstream changes or a clearly reviewed, justified, time-bounded risk disposition. Do not silently ignore them, remove all-target checks, or infer a green cargo-deny gate from Windows reachability alone. Finish native focused tests and generated schema/regression checks on the exact source before any 21-stage release validation. No Daniel-CL action is needed for this dependency analysis.

## Current ZIP runtime closure and maintenance disposition (October 20, 2026)

- The exact Windows ZIP security regression run [#39479991587](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39479991587) **passed** at branch commit `16af9fcf2b6583838112e386e0fafc2790ab8410`. Its six native cases include an unsupported BZIP2-method archive fixture that must fail without replacing an existing curated-plugin snapshot or its SHA record. The original fixture incorrectly assumed the ZIP 9 writer defaulted to Stored (method 0), although it produced Deflate (method 8); this fixture error was corrected, and the run is now green.
- The earlier source guards, Windows library compilation and TUI test compilation also passed at source-equivalent predecessor `f10b862d13e9bb11effa92058c87937c2dcd5d3e`. The final `16af9fcf` commit changed only the focused ZIP CI test-list separator. These passes do not clear the inherited advisory gate, substitute for the 21-stage native release suite, or authorize installation.
- The remaining RustSec findings are **four informational unmaintained-package notices** rather than the resolved ZIP advisories; no new exploitable vulnerability is claimed by those four notices. `basic-toml` is reachable through the Windows secrets graph. The three `zbus`/`zvariant` findings are not Windows-reachable in the currently accepted locked graph but remain in the all-target audit. `spin 0.9.8` is still a yanked warning, not one of the four advisory failures.
- Detailed dependency-owner analysis, mechanically compatible alternatives, and a non-waiving release decision matrix are recorded in [Packet 3D.7](codexdd-0.4.5-packet-3d7-inherited-advisory-release-decision.md). **Cargo-deny remains RED**: do not silently add exceptions, downgrade accepted dependencies, prune Linux packages out of the all-target audit, or label a narrower Windows-only check as an all-target PASS.
