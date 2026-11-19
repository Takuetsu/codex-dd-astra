# CodexDD 0.4.6 — 3F.2a prebuild artifact/provenance audit

**Date:** 2026-11-19
**Pinned candidate SHA:** `8818334c5cf2e22154fea4c0cfcd25a8baf66f55` (`dd/codexdd-v0.4.6-rc1`)
**State:** **SOURCE PROVENANCE VERIFIED; RELEASE ARTIFACT AUDIT PENDING — NO CANDIDATE RELEASE PACKAGE EXISTS YET.**
**Do not merge, install, sign, distribute, tag or release.**

## Verified native source and build inputs

- [RC1 Windows run #44539233887](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44539233887): all three jobs successful. The exact candidate CLI test reports product `0.4.6+g...`; version source `codex-rs/codexdd-version.txt=0.4.6`. The TUI status snapshot reports `codexdd: 0.4.6 (dev)` (development metadata, not a release designation).
- Distinct pinned upstream identity is `rust-v0.162.0`; workspace Cargo version `0.162.0`. Frozen 0.4.5 code is not an authorized 0.4.6 release input.
- `.github/actions/setup-rusty-v8/action.yml`: Codex-built Windows V8 archive and binding are checksum checked against the repository-pinned release manifest and artifact checksums, including bounded network retries.
- `scripts/codexdd-prepare-rusty-v8.ps1`: the official native release helper has **offline verification by default**. Initial `-Prepare` may download into the isolated Cargo target cache only after validating trusted manifest and both artifact SHA-256 values; the later release build independently rechecks cached bits.
- `scripts/codexdd-test-release.ps1`: defines 21 sequential validated stages through the shared contract-v1 `CODEXDD_VALIDATION_JSON` record. It fails closed on missing tools, a failing stage or invalid repository markers and does **not** itself publish a release.
- `scripts/codexdd-test-app-server-release.ps1`: builds and verifies a real `test_stdio_server.exe` helper before app-server library tests, avoids falsely green tests with a missing fixture.
- `scripts/codexdd-test-cli-release.ps1`: compiles `codex-cli` and `codex-code-mode-host` with `cargo build --release --locked`; fails on unavailable verified V8 bits. Its successful build is **not** packaging, signing, provenance sealing or public release.
- Full 21-stage profile requires native Windows, pinned Rust toolchain, cargo-nextest, rustfmt/clippy, DotSlash for native children, a clean checkout, verified Windows V8 archive/bindings, and required helper binaries.

## Actual artifact evidence

[RC1 run #44539233887](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44539233887) returned **zero uploaded GitHub Actions artifacts**. It generated disposable local test binaries on the runner only. No release-cli EXE/PDB bundle, per-file SHA-256 manifest, signed binary, provenance attestation, installer, rollout package, or real installed-runtime receipt has been audited. Repository releases endpoint returned no release package for this candidate. Therefore **3F.2 artifact audit cannot be marked PASS yet.**

## Proposed final acceptance checks (not receipts)

1. **21-stage exact-candidate full Windows profile:** execute the actual `scripts/codexdd-test-release.ps1` unmodified on a clean nonproduction Windows 2022 runner/worktree, using repository-pinned dependencies and `codexdd-prepare-rusty-v8.ps1 -Prepare` only for a first verified cache. Capture profile contract-v1 `profile_end: pass`, `completed_stages: 21`, all stage names and zero errors; no substitution with partial tests.
2. **Independent objective review:** execute `scripts/codexdd-test-independent.ps1` on exact source under a separate native validation job or approved peer reviewer. A scripted green result is not a security signoff.
3. **Artifact identity:** after successful release build, record paths and SHA-256 for `codex.exe`, `codex-code-mode-host.exe` and any available PDBs in an isolated staging directory. If PDBs or complete package requirements are absent, record `PENDING` rather than pretend to have a distributable bundle.
4. **No transport/promotion side effects:** build must be nonproduction and unsigned; no production installs, tags, releases, game modifications, or automatic daemon starts. Keep untrusted upload artifacts separate from approved signed releases.
5. **Separate Windows owner gate:** medium-integrity Daniel-CL app/session/agent smoke, true process restart and installed runtime QA, >=10 representative paired performance tests where feasible and owner signoff remain required by the milestone. GitHub Actions success does not complete Phase 4.

## Next action

Start the isolated Windows prerequisites + actual release-profile CI using **exact source `8818334c5cf2e22154fea4c0cfcd25a8baf66f55`**, not the documentation-audit branch HEAD, and do not label an artifact as distribution-ready without independent security approval and hash/provenance review.
