# CodexDD 0.4.5 — Packet 3F.9: owner-approved Linux INFO risk disposition

**Decision recorded: 2026-10-27. NONRELEASE / ACCEPTED RISK ONLY.**

## Authorization and exact scope

The CodexDD project owner explicitly replied "yes" on 2026-10-27 when asked to accept _only the three Linux-only INFO/unmaintained maintenance notices listed below_. The owner approved a narrow, time-limited review disposition, **not** a blanket RustSec waiver, an advisory-free audit, a merge, an installation, or an 0.4.5 release.

**Expiry: 2026-11-20T00:00:00Z, or the next upstream integration (whichever occurs first).** Review must be renewed before that deadline or on any change to the dependency graph, Windows reachability, warning category, or available maintained replacement. The expiry and 0.163.0 upstream ceiling are checked automatically in CI, not merely stated in documentation.

| Advisory          | Exact lock and applicability                                  | Owner-approved compensating controls and removal trigger                                                                                                                                          |
| ----------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RUSTSEC-2026-0371 | zbus_names 3.0.0, through zbus 4.4.0, Linux only              | Informational/unmaintained. No Windows workspace dependency path. Migrate all TUI and credential owners to maintained zbus 6 names APIs; reject changed Windows reachability or RustSec category. |
| RUSTSEC-2026-0370 | zvariant 4.2.0, same Linux zbus chain                         | Informational/unmaintained. Preserve all-target cargo-deny. Replace with zbus 6 wire codecs as soon as its dependency owners support them.                                                        |
| RUSTSEC-2026-0372 | zvariant_derive 4.2.0, Linux-only transitive procedural macro | Informational/unmaintained. Replace with zbus 6 derive APIs together with zbus and keyring/secret-service migration.                                                                              |

Official RustSec records as checked on 2026-10-27:

- https://rustsec.org/advisories/RUSTSEC-2026-0371.html
- https://rustsec.org/advisories/RUSTSEC-2026-0370.html
- https://rustsec.org/advisories/RUSTSEC-2026-0372.html

The exact three IDs are appended separately to the existing 10 ignores in codex-rs/deny.toml and codex-rs/.cargo/audit.toml. No global informational or unmaintained-policy setting, targets filter, bans, license policy, source severity, or ignore of other advisories changes. An accepted exception stays visible as an ignored advisory note, not an advisory-free result.

## Mandatory enforcement

scripts/codexdd-045-inherited-risk-guard.py is executed in the reusable .github/workflows/cargo-deny.yml, before the existing full all-target audit. It checks:

1. The current UTC time is strictly before 2026-11-20T00:00:00Z and this is still the 0.163.0 / CodexDD 0.4.5 integration, not an unapproved successor.
2. Exactly those three new IDs exist, with risk reason, expiry and package version documented, alongside the 10 unchanged historical IDs, in both advisory-ignore lists.
3. Windows and Linux Cargo metadata are derived from the actual locked source graph; all three packages are Linux-reachable and Windows-unreachable at their reviewed versions.
4. The latest RustSec advisory database still marks each ID as informational=unmaintained with no published fixed version. If the database changes or cannot be verified, CI fails closed.
5. Cargo-deny remains all-target with unknown Git/registry denial, immutable Git revision enforcement and protected Zstd, TLS, ZIP and ctutils security pins.
6. The particular extra trusted upstream Git source for the independent basic-toml fix is exact-commit-pinned, and its Cargo.lock identity is i18n-config only.

The workflow makes no changes to production, no relaxation of the unrelated crypto-bigint vulnerability, and no suppression of new RustSec advisories. Next integration or expiry requires fresh owner action.

## Separately reviewed pinned source for the basic-toml removal

RUSTSEC-2026-0374 (basic-toml) is **not** one of these exceptions. Its removal is staged in the exact Cargo-generated candidate (manifest Git blob d46347640b6ca6e43e96a12ee14eec894178af4f, Cargo.lock Git blob f19b0086eac09c6830c887be83db5ae4ae11587e) using an unpublished upstream i18n-config revision:

- Repository: https://github.com/kellpossible/cargo-i18n
- Exact revision: 2ce7b1dfe9a8cc224288bf50cbc2a67a6de863c4
- Windows/Linux locked dependency-resolution and standalone parser compilation: https://github.com/Takuetsu/codex-dd-astra/actions/runs/40533479354 (PASS).

Cargo-deny source enforcement is explicitly updated to allow only this repository URL; the additional guard guarantees that the source is used only for i18n-config at this immutable revision. This is not broad organization-wide Git trust. The upstream parser changed its public TOML deserialization-error type; CodexDD Windows secrets and credential recovery/failure tests are still outstanding.

## Not approved / unfinished release work

- The unrelated **RUSTSEC-2026-0387 crypto-bigint 0.5.5 vulnerability is Windows reachable and remains unexcepted**. It still blocks the all-target cargo-deny release gate.
- Native Windows secrets behavior, the exact-source integration and security audit, Linux D-Bus migration and other release checks remain pending.
- Draft PR #54 remains draft. This packet does not authorize the 21-stage Daniel-CL release suite, product version promotion, CI waiver, release merging or installation.

**Required audit language:** "Three owner-approved time-bounded Linux-only INFO maintenance exceptions; all-target cargo-deny is still mandatory, crypto-bigint remains a separate vulnerability blocker." Never label a test of this exception guard "advisory-free" or a production release PASS.

## Completed acceptance audit receipts — 2026-10-27

1. [Focused guard workflow #40537766373](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40537766373) **PASS**, exact source commit c44f8f4a355a47515beaa4ebf272706498afe968: policy scope, immutable Git source, review deadline, protected locked versions, Windows/Linux target reachability and all three current RustSec INFO/unmaintained metadata checks.
2. [Full, unmodified all-target cargo-deny workflow #40538049035](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40538049035) on commit b1fe0a2ad1694fa83399bec9a3656992ff42807a: scope guard **PASS**, bans **ok**, licenses **ok**, sources **ok**, advisories **FAILED on exactly one error** — RUSTSEC-2026-0387 for Windows-reachable crypto-bigint 0.5.5. There are duplicate-version and yanked-crate warnings, but no other advisory _errors_. The clean-worktree check also passed.
3. No cargo-deny errors appeared for the three specifically accepted Linux INFO advisories, and the removed basic-toml finding did not recur. This supports closure of the three agreed maintenance exceptions and resolver-level basic-toml remediation. It **does not** accept crypto-bigint, validate native secrets behavior, or make the full security gate green.

**Next gate requiring local validation:** focused Windows codex-secrets --lib tests on exact candidate branch and lock, followed by crypto-bigint major-line remediation. Keep full 21-stage release suite and production promotion on hold.
