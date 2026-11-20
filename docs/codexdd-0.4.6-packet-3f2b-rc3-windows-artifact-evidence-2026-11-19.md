# CodexDD 0.4.6 — 3F.2b RC3 native artifact evidence and release holds

**Date:** 2026-11-19
**Exact implementation candidate:** `2acc2209b65f3186b00490b1c4b0cc444a636f4c` (`dd/codexdd-v0.4.6-rc3`)
**Native workflow carrier:** `2c8108f6eb5af1a0f1d51247f52610c94a966516` (`dd/codexdd-v0.4.6-3f1-audit`); the workflow explicitly checks out and asserts the implementation SHA, not the audit SHA.
**State:** **21-STAGE WINDOWS PROFILE PASSED; UNSIGNED VERIFICATION ARTIFACT UPLOADED; INDEPENDENT SECURITY, PR CI AND OWNER ACCEPTANCE OPEN. NOT RELEASE-READY.**
**Authority:** Evidence inventory only. Do not merge, install, tag, sign, distribute, or modify production/Breakwater/CV.

## Executed exact-source evidence

- [GitHub Windows run #44598888395](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44598888395) completed **success**. Its source/Rustfmt preflight and native Windows jobs both completed successfully. Workflow pins RC3 implementation `2acc2209` in both checkouts and verifies source/product `0.4.6` and upstream `rust-v0.162.0`.
- Raw Windows job log contains `CODEXDD_VALIDATION_JSON` **`profile=release,status=pass,exit_code=0,completed_stages=21`**, concluding in unchanged `cli-release-build` stage 21; no failed release stage. The script ran on Windows Server 2022 using the pinned Rust toolchain, DotSlash, checksummed V8 and separate nonproduction target.
- The same log contains **`profile=independent-review,status=pass,exit_code=0,completed_stages=6`**. This is independently prescribed *objective native regression testing*, **not independent author-disjoint source/security signoff**.
- Full native TUI adaptive check reported **279/279 passed**; `nextest` marked five cases `LEAK` (did not fail this profile). Record these as test-harness/process-leak diagnostic follow-up, not silently as zero warnings.
- The runtime-authority late-READY refusal regression and real app-server/Implementation -> distinct Validator -> fresh Core QA -> READY paths were exercised in the native job. This is evidence for those tests, not a universal security proof against all clients and RPC ingress.

## Uploaded evidence and hash/provenance boundary

- Run uploaded one explicitly unsigned, **verification-only** Actions artifact: [`codexdd-046-rc3-2acc2209-UNSIGNED-verification-only`](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44598888395/artifacts/14585635801), artifact ID `14585635801`, size **474,026,876 bytes**, GitHub upload digest **SHA-256 `1654702ac359a634bfd9c292008a5a572733cfe521451caa5c4d8cb06ad52fb4`**. GitHub records its expiration as **2026-11-26T23:33:30Z**.
- Workflow staging step copies required `codex.exe` and `codex-code-mode-host.exe`, available release-directory PDBs, and writes `provenance.json` with exact candidate SHA, 21/6 stage counts, and individual file SHA-256 values, then uploads through pinned `actions/upload-artifact`. Runner printed `RC3_UNSIGNED_EVIDENCE_HASHED_NO_PRODUCTION_RELEASE`.
- **Limits:** The observed GitHub digest covers the uploaded archive, not independently downloaded/rehashed local EXE/PDB bytes. This review did **not** download and inspect all archive entries or independently compare embedded per-file hashes, compiler binary identity, PDB completeness, signatures, signing chain, or install/rollback package. An artifact name and upload digest alone are not a distribution-ready provenance seal. **3F.2 independently verified deliverable: PENDING.**

## PR #88 blocking checks — classified, not waived

[Draft RC3 independent review PR #88](https://github.com/Takuetsu/codex-dd-astra/pull/88) remains OPEN/DRAFT/NO MERGE. Its native `PR fast / Windows adaptive validation` check passed, but required aggregate CI failed:

1. **Blob-size-policy:** job never reached a valid size decision: `openai/fence` startup received `Permission denied` and the subsequent local `check-clean-worktree` action was unavailable. Classification: runner/tool execution environment. Not a proven product blob-size violation and not an authorized waiver.
2. **Codespell:** exact finding `docs/codexdd-0.4.6-packet-3b1b-github-source-hardening.md:30`, `re-used` -> `reused`. Classification: precise text/style failure; fixing the candidate changes its source tree and demands re-evaluation of exact candidate identity.
3. **Repo checks:** `pnpm run format` flagged, among other paths, `docs/codexdd-0.4.6-packet-3a2-source-authority-map.md` and `docs/codexdd-0.4.6-packet-3b1b-github-source-hardening.md`. Classification: formatting mismatch against the PR merge tree; not proof of native behavioral failure, but still a failed required check.
4. **Cargo-deny:** `advisories FAILED, bans ok, licenses ok, sources ok`. Includes vulnerability and unmaintained advisories in locked dependencies; e.g. the report references SQLite bundled code and Hickory DNS. Classification: genuine unresolved security inventory requiring source/version/exposure analysis and reviewer disposition; do **not** suppress, ignore, or claim all are irrelevant merely because they may originate upstream.
5. Aggregate **CI required** fails because its dependencies are red. Do not use the unrelated successful exact-source Windows workflow to override failed PR checks.

## Required completion order, with no repeat four-hour build unless source changes

1. Preserve this exact RC3 implementation and its completed 21-stage evidence. Independently inspect artifact archive `provenance.json`, the two EXE hashes and PDB inventory, and compare against the pinned run and documented release requirements **before artifact signoff**.
2. Triage PR check failures into confirmed RC3 deltas vs inherited production/upstream issues; address narrow doc formatting/spelling and runner environment faults only with a change-impact plan. Resolve or formally disposition security advisories with a qualified, author-disjoint reviewer. Any candidate-source fix creates a new SHA and invalidates the old run as exact-current-SHA acceptance; do not falsely reuse the 21-stage green.
3. Perform independent semantic/security review of source authorization, exact candidate/worktree pinning, native Validator receipts, direct RPC negative paths, one-use READY/late rejection and Core-owned PID/HTTP, with recorded reviewer identity and disposition. CI green alone is not this signoff.
4. Complete owner-authorized **Daniel-CL** non-elevated real-agent/session smoke and restart/resume, representative project-like paired trials, then explicit installation/rollback decision. No early Breakwater coding pilot; owner-selected one-milestone ~one-week soak comes **after** release authorization.

**Decision:** 4F release-stage execution is **PASS** for exact RC3; 3F.2 artifact/PR security closure and Phase 4 owner QA are **OPEN**. The next meaningful work is artifact/hash audit and independent security review, not another blanket 21-stage CI rerun on unchanged RC3.
