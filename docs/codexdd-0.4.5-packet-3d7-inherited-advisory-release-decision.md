# CodexDD 0.4.5 — Packet 3D.7: inherited advisory maintenance decision

**Status: NONRELEASE REVIEW COMPLETE; ADVISORY ACCEPTANCE OPEN / RED.**

- Integration branch: \`dd/codexdd-v0.4.5-upstream-0.163.0-integration\`
- Reviewed branch source: \`16af9fcf2b6583838112e386e0fafc2790ab8410\` (2026-10-20)
- Accepted Windows-resolved \`codex-rs/Cargo.lock\` Git blob: \`a8eb8a076276cb3db5674b33c67d743b1c3d21fc\`
- Product/released upstream markers remain **0.4.4 / rust-v0.162.0**; the integrated 0.163 sources are **not released**.
- This packet is read-only security/remediation analysis. It changes no runtime code, dependency manifest, lockfile, audit policy, version marker, external installation, or release receipt.

## Verified gates and distinction between findings

1. The exact Windows ZIP 9 security run [#39479991587](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39479991587) passed **six of six** native extraction/fallback regressions at \`16af9fcf\`; the added BZIP2-method test proves failed archive extraction does not replace a pre-existing plugin snapshot or recorded SHA.
2. Earlier Windows source/compiler/TUI checks passed on the source-equivalent \`f10b862d\`. Prior focused ZIP, Windows registry/Guardian/CLI/TUI runtime and schema audits passed on the unchanged dependency graph at \`c6c65a7f\`. Re-run exact-HEAD gates when freezing a candidate; do not infer a complete release suite PASS.
3. The unmodified cargo-deny audit [#39467522626](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39467522626) reported **advisories FAILED, bans ok, licenses ok, sources ok**. The resolved \`zip 9.0.5\` graph removed the previous three ZIP advisories and the BZIP2 license rejection. It retains **four inherited INFO/unmaintained notices** and one **yanked spin warning**. No blanket ignore was added.
4. The target-graph recon [#39468311160](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39468311160) distinguished Windows from Linux dependency reachability; this is evidence about reachability, **not** a replacement for the all-target cargo-deny policy.

## Maintenance candidates, compatibility and decision

| Advisory / package | Accepted graph and applicability | Current technically sound path | Release treatment |
| --- | --- | --- | --- |
| [RUSTSEC-2026-0374](https://rustsec.org/advisories/RUSTSEC-2026-0374.html) — \`basic-toml 0.1.10\` | **Windows reachable:** \`codex-secrets -> age 0.12.1 -> i18n-embed-fl 0.10.1 -> i18n-config 0.4.8 -> basic-toml\` | Upstream \`i18n-config\` maintenance fix replacing \`basic-toml\`, or a separately reviewed, pinned and behavior-tested narrowly scoped replacement. No same-major maintenance release removing the dependency was found at this review. | **OPEN; no waiver.** A risky secrets-library replacement or parser downgrade is not a compatible drop-in fix. |
| [RUSTSEC-2026-0371](https://rustsec.org/advisories/RUSTSEC-2026-0371.html) — \`zbus_names 3.0.0\` | **Not Windows reachable** through the current locked workspace graph; Linux via \`zbus 4.4.0\` | Standalone names types were incorporated into \`zbus 6\`. Migration requires coordinated dependency-owner/API work. | **OPEN in all-target audit.** No silent Windows target pruning. |
| [RUSTSEC-2026-0370](https://rustsec.org/advisories/RUSTSEC-2026-0370.html) — \`zvariant 4.2.0\` | **Not Windows reachable**; Linux via \`zbus 4.4.0\` | Wire types were incorporated into \`zbus 6\`. Check \`codex-tui\`, \`keyring\`, and \`secret-service\` compatibility before any migration. | **OPEN in all-target audit.** |
| [RUSTSEC-2026-0372](https://rustsec.org/advisories/RUSTSEC-2026-0372.html) — \`zvariant_derive 4.2.0\` | **Not Windows reachable**; transitive Linux \`zvariant\` dependency | Derive functionality migrated to \`zbus 6\`. Treat as part of the coordinated Linux D-Bus migration, not a standalone version bump. | **OPEN in all-target audit.** |
| Yanked warning — \`spin 0.9.8\` | Through \`flume 0.12.0\` and \`heapless 0.7.17\`; warning, not an advisory error | Prior CI produced a byte-exact compatible \`spin 0.9.9\` *preview* lock blob \`c121f97894bee98b48eab6302751723052a99caa\`. | Optional narrow future cleanup. **Uncommitted/unaccepted; do not trade security-pin regression for removing a warning.** |

### Why disabling default features or blindly downgrading will not resolve the Windows finding

- \`age 0.12.1\`'s default Cargo feature set is already empty, so \`default-features = false\` is not a demonstrated path to removing its non-optional transitive localization macro dependency.
- Published \`i18n-embed-fl 0.10.1\` has a mandatory \`i18n-config ^0.4.7\` dependency. \`i18n-config 0.4.8\` directly depends on \`basic-toml ^0.1\`; the release history says v0.4.7 switched *from* \`toml\` *to* \`basic-toml\`.
- Therefore a forced resolution to pre-0.4.7 \`i18n-config\` would violate the declared semver requirement and is not an accepted maintenance remediation. Likewise, replacing \`age\`, crypto primitives, or its parser without dedicated behavior/source review is outside the current nonrelease packet.

### Why a Linux-only zbus upgrade is not a safe automatic lockfile fix

- RustSec explicitly marks the standalone names, variant, and derive crates as unmaintained after their functionality moved into \`zbus 6\`. These are **maintenance notices, not proof of a disclosed exploit**.
- The integration declares \`zbus = "4.4.0"\` under \`cfg(target_os = "linux")\` in \`codex-tui\`; \`codex-keyring-store\` includes \`secret-service = "4"\` and Linux-specific keyring backend features. Changing those owners to major-version 6 requires API and feature compatibility work outside a simple \`cargo update\`.
- Windows-only reachability is strong platform evidence for a reviewed risk disposition, but it does **not** convert the repository's unchanged *all-target* cargo-deny FAIL into a green result.

## Explicit release decision choices (no decision silently taken)

**Path A — remediation:** Accept a narrowly scoped, tested upstream/parser maintenance fix for \`basic-toml\`, and separately review the coordinated Linux-only zbus/secret-service/keyring migration. Require Cargo-generated lock diff provenance, retained \`rustls 0.23.46\`/\`quinn-proto 0.11.18\`/\`noyalib-serde-yaml 0.0.57\` security pins, source guards, focused native Windows behavior checks, and the exact unmodified cargo-deny audit before claiming the dependency gate green.

**Path B — explicitly reviewed, time-bounded inherited-risk disposition:** Record each specific advisory's severity, target reachability, transitive ownership, compensating controls, named reviewer, an expiry/review checkpoint, and an actionable removal trigger. Only after a distinct owner security acceptance may narrowly scoped policy exceptions be implemented and re-audited. Any such result must be labeled **accepted exceptions**, never **advisory-free**. Do not grandfather unknown or future advisories or change all-target policy implicitly.

**Current state: neither path has been accepted.** No policy exceptions, release overrides, broad CI waiver, merge/install authorization, or auto-publication are in effect.

## Remaining 0.4.5 acceptance gates

1. Resolve the four inherited notices by Path A or explicitly decide and document Path B. Existing source/security expectations must remain fail-closed in the meantime.
2. Reconfirm exact candidate source/lock/provenance and schema drift, then all required focused Windows native runtime checks for adaptive Worker, Guardian, registry, approvals, daemon, CLI F2, plugin ZIP and app-server.
3. Freeze exact candidate SHA, run the standard **detached SHA-bound 21-stage native Windows release suite** on Daniel-CL only after preflight acceptance, and inspect both monitor output and authentic receipt. Preserve the established location/monitoring contract; no long suite under a fragile SSH session.
4. Only after CI/release promotion gates and explicit release acceptance, advance 0.4.5 markers, merge, package, install, and verify on Windows. Automatic upstream discovery does **not** mean unattended promotion.

No operator action on Daniel-CL is required for this **documentation-only** checkpoint.
