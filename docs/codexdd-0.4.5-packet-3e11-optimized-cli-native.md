# CodexDD 0.4.5 — Packet 3E.11: optimized native Windows CLI build

**Status: NONRELEASE OPTIMIZED CONFIGURATION PASSED; SHIPPING BINARY UNFINISHED AT JOB TIME LIMIT. NOT A RELEASE CANDIDATE.**

[Native Windows CLI debug build #39677741274](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39677741274) **PASSED** on SHA `8a3b31c77da4b7304692d881fe2d28df212663d5`: 2,846 Bazel actions in 1,822.098s, emitting `codex.exe` and `codex.pdb`. The prior smoke transition ran `-Copt-level=0` and no LTO. Same-SHA [source #39677741315](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39677741315) passed 456 static + 112 updater + 85 historical reconciliation = **653/653**.

The follow-on read-only Windows-2022 CI workflow runs a locked Bazel 9.2.0 optimized `cquery` first and compiles only `//codex-rs/cli:release_files` with `--config=release`, source/policy pins, preinstalled MSVC and `--jobs=2`; 90-minute bound and fail-fast checks for authentic EXE/PDB build receipts. Upload only raw logs.

**Not authorized:** combined runtime archive, EULA acceptance, signing, version marker promotion, merge, installation or detached 21-stage Daniel-CL acceptance. Four inherited INFO/unmaintained RustSec notices, semantic review of 109 overlaps and frozen source candidate remain separate open gates.

## Actual optimized build outcome (October 21-22, 2026)

- [Run 39682732793](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39682732793) at integration SHA d2cdc47b19253aee0b99c7de1cc410b521479856 passed exact Cargo/Bazel source, MSVC SDK, security-policy and unreleased 0.4.4 identity preflights. Its 90-minute Windows-hosted limit elapsed and GitHub cancelled the build action. This is not a source compiler error.
- The uploaded raw artifact [12389566974](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39682732793/artifacts/12389566974) contains optimized-cquery.log and cli-release-files.log. The optimized cquery was successful in 307.033 seconds: 1,243 packages loaded, 57,184 targets configured, one release_files target analyzed. No load/analysis errors.
- The optimized build reached [2,845 / 2,846] actions after compiling dependencies and spent more than 1,263 seconds of repeated status on the final codex-rs/cli:codex Rust binary before the runner cancelled the job. No Rust compiler error or optimized EXE/PDB success receipt was recorded. Do not claim optimized CLI or package validation passed.
- Unlike debug smoke build #39677741274, optimized compilation enables the shipping Rust release profile including ThinLTO and linked-symbol treatment. Its cost exceeds the current cold hosted-Windows 90-minute budget. Repeating the identical job without a change in resources, caching or validation location is wasteful. Do not weaken the shipping optimization profile just to turn CI green.
- Preserve this evidence as a build-performance constraint. Candidate preflight should check exact Bazel release configuration and source/policy pins cheaply; the complete optimized archive and binary verification belongs in the accepted, resource-appropriate frozen release validation plan (including detached Daniel-CL validation when authorized). Capture authentic EXE/PDB receipts, not only an analyzed graph.
- The separate all-target cargo-deny gate remains RED for four inherited INFO/unmaintained advisories; nonrelease scratch-policy feasibility is evaluated independently by codexdd-045-advisory-decision-preview.yml without changing codex-rs/deny.toml or accepting any exception.
- No product marker promotion, release artifact, installer, production merge, external machine update, full 21-stage Windows release acceptance or automated upstream publication was performed.
