# CodexDD 0.4.5 — Packet 3E.11: optimized native Windows CLI build

**Status: NONRELEASE OPTIMIZED CI REQUESTED; RESULTS OPEN.**

[Native Windows CLI debug build #39677741274](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39677741274) **PASSED** on SHA `8a3b31c77da4b7304692d881fe2d28df212663d5`: 2,846 Bazel actions in 1,822.098s, emitting `codex.exe` and `codex.pdb`. The prior smoke transition ran `-Copt-level=0` and no LTO. Same-SHA [source #39677741315](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39677741315) passed 456 static + 112 updater + 85 historical reconciliation = **653/653**.

The follow-on read-only Windows-2022 CI workflow runs a locked Bazel 9.2.0 optimized `cquery` first and compiles only `//codex-rs/cli:release_files` with `--config=release`, source/policy pins, preinstalled MSVC and `--jobs=2`; 90-minute bound and fail-fast checks for authentic EXE/PDB build receipts. Upload only raw logs.

**Not authorized:** combined runtime archive, EULA acceptance, signing, version marker promotion, merge, installation or detached 21-stage Daniel-CL acceptance. Four inherited INFO/unmaintained RustSec notices, semantic review of 109 overlaps and frozen source candidate remain separate open gates.
