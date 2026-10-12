# CodexDD 0.4.3 — Packet 3E.3 version/provenance and promotion preflight

**Status: BASELINE AUDIT BEFORE 3E.3 — PRODUCT/UPSTREAM MARKERS SUBSEQUENTLY ALLOCATED ON INTEGRATION; RELEASE PROMOTION REMAINS BLOCKED.** This preflight is not Gate 3 acceptance, a candidate manifest, local validation, or authorization to publish.

## Independently verified source identities (2026-10-10)

- Live production branch `dd/astra-policy-v2`: `4828e3b4232781963594cfaaebdc49c5531de8b1`, unchanged at this audit.
- Source integration branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`. The next broad executable regression run is [#38082627362](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38082627362), launched on `a99d079fe88a07c5a9a82ffdb676c810d48fcd12`. Its result was **pending** when written.
- Official target: `rust-v0.162.0`, peeled commit `c1382380de69521303b416720a52f42d51af6248`; original discovery remains `blocked_transplant_conflict` with 22 recorded conflict paths.
- Isolated branch `codex-rs/codexdd-version.txt` still contains `0.4.2` and `codex-rs/upstream-codex-release.txt` still contains `rust-v0.159.2`. This is intentional *unallocated* release identity, not a claim the integration source still matches upstream 0.159.2.
- `codex-rs/Cargo.toml` has the **upstream workspace** version `0.162.0`. Its separately generated lockfile was aligned against official 0.162.0 external crates in [#38015689709](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38015689709). Do not conflate this Cargo package version with CodexDD product `0.4.3`.
- `codex-rs/cli/tests/version_reporting.rs` currently asserts `codexdd 0.4.2+g`, and the read-only Phase 3F CI identity assertion deliberately checks the two pre-allocation marker values. Both need coordinated updates when 3E.3 is executed.

## Narrow future 3E.3 allocation packet

Once the integrated source/behavioral test gate is accepted, allocate version identity **only on the isolated integration source**, in a bounded change:

1. Set product version marker to `0.4.3` and tracked upstream release marker to `rust-v0.162.0`, preserving distinct semantics.
2. Reconcile the CLI version-reporting test and any source-development CI identity assertions with the allocated product/provenance values; verify native CLI `--version` and provenance output at the appropriate test profile.
3. Preserve the existing Cargo workspace version and upstream dependency resolutions; only regenerate `Cargo.lock` using pinned Cargo if a manifest change actually changes the lock identity. Never manually edit lock checksums or generated protocol exports.
4. Re-run source/build checks and verify the final resolved source still matches the exact intended upstream target plus audited fork deltas. Any post-test source modifications invalidate older validation evidence for affected behavior.
5. Freeze and record an exact source snapshot Git commit/tree **before** metadata-sidecar creation; do not reuse a receipt from before the version/provenance allocation.

**No version change was executed by this preflight**, and no released binary should report 0.4.3 on its basis.

## Blocking candidate/promotion contract gap (do not bypass)

- The original v1 `.github/workflows/upstream-prepare.yml` is **clean-transplant-only**. The authoritative 22-conflict discovery must remain blocked rather than relabeled `preparation_ready`.
- The current `.github/workflows/upstream-promote.yml` accepts only v1 candidate manifests; `scripts/codexdd-validate-upstream-candidate.ps1` explicitly rejects non-v1 manifest contract versions. Therefore the manually reconciled development branch cannot be passed directly to that promoter.
- The deterministic canonical candidate branch `automation/upstream-candidate-rust-v0.162.0-c1382380de69` is **absent** as of this audit; the integration branch is not a substitute.
- Follow `docs/codexdd-0.4.3-packet-3a2-reconciliation-contract.md`: implement a **separate, additive v2 conflict-recovery tooling-only branch/PR** from the *then-live* production base, with fail-closed typed sidecar/manifest/receipt parsing, SHA/tree identity checks, publish-once candidate handling, and positive/negative tests. Merge that tooling PR only following explicit owner acceptance.
- A tooling merge moves production HEAD and invalidates any historical current-base discovery/receipt for publication. Rerun pinned read-only discovery against the new base, re-anchor the resolved source and v2 attestation, and only then publish a canonical candidate once. Required native Windows release-profile PASS and owner-reviewed semantic evidence remain separate prerequisites to PR promotion.
- The existing `dd/codexdd-v0.4.3-upstream-recovery-tooling` branch is an earlier **v1 fixed-target discovery** branch, not an implemented v2 bridge. Do not repurpose or mistake it for the required new tooling gate.

## Gate boundaries

Current read-only CI checkpoints establish partial Linux source/execution compatibility, **not** final Windows-native F2/elevation/daemon safety, updater/LVO native receipt behavior, or release authorization. Daniel-CL remains reserved for short, meaningful SSH/PowerShell validation when the source and tooling gates make it useful; production, installed binaries, and external machines remain unchanged.


## Executed 3E.3 isolated allocation and fresh validation

After [628 targeted Rust tests passed](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38084373403), and after [3E.1 upstream/LVO compatibility smoke #38089309300](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38089309300) passed 23 Python regressions plus native PowerShell 5.1 parsing and pass/fail/infrastructure fixtures, explicit CodexDD product/upstream identity was allocated on the source branch **only**.

- Exact atomically committed source allocation: [`cde952708d4fb441e8c3c27223107c416920a8ab`](https://github.com/Takuetsu/codex-dd-astra/commit/cde952708d4fb441e8c3c27223107c416920a8ab), changing exactly six paths: `codex-rs/codexdd-version.txt` from 0.4.2 to **0.4.3**, `codex-rs/upstream-codex-release.txt` from rust-v0.159.2 to **rust-v0.162.0**, the CLI `--version` test assertion, the `/copy` status fixture's visible product version, and the two temporary source-development workflow identity guards.
- `codex-rs/Cargo.toml` remains the official upstream Cargo workspace version 0.162.0; no Cargo manifest, lockfile, dependency checksum, generated schema or runtime routing changed in this allocation commit. The lock remains the earlier verified official 0.162.0 dependency graph.
- **Fresh retest #38089445440:** [expanded 628-test runtime workflow](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38089445440), pinned to the allocation commit, started and not yet accepted as green when documented.
- **Fresh 3E.1 smoke #38089445427:** [Windows LVO / Python updater compatibility](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38089445427) **success**, pinned to the same allocation commit and verifying existing v1/LVO byte-identical sources remain untouched.
- **New 3E.3 executable CLI/core smoke #38089596594:** [`--version`, core LVO handler and app-server trusted adaptive signal](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38089596594) launched from subsequent workflow-only commit `d416159e77123bc88659ceff4d1ce933d20b25ef`; outcome pending when documented.
- The distinct nonproduction v2 tooling-only branch `dd/codexdd-v0.4.3-v2-reconciliation-tooling` was created at production SHA `4828e3b4232781963594cfaaebdc49c5531de8b1` for the required 22-conflict contract and is **not** a candidate or authorized promoter. Initial strict read-only evidence parser source `cf17f0203bcb9d5be1d24027f05cec39480b9446` has a separate CI gate. No tooling merge, release receipt or canonical candidate exists merely because that branch exists.

**Required acceptance still outstanding:** inspect final refreshed CI conclusions; implement/test owner-reviewable v2 conflict reconciliation and promotion bridge on a separate tooling PR; rebind source snapshot and original discovery to a new current production base after any approved tooling merge; execute the native Windows release profile and elevated F2/daemon/SSH checks; collect an identity-bound receipt and explicit owner authorization before any PR/production promotion. Installed production remains 0.4.2.
