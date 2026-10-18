# CodexDD 0.4.5 — Packet 3C.8c: CLI version, adaptive startup and Windows F2 coverage

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Fifteen exact official 0.163.0 CLI Rust source/test paths plus three reviewed fork-specific merges staged. Thirteen **source-only** offline static tests added, not run on Daniel-CL. No **native Rust** compilation, Windows elevated-SSH regression or release receipt exists.

## Reviewed three-way source merges

The original production CodexDD 0.4.4 differs from official rust-v0.162.0 in exactly three of the changed CLI paths. Its original deltas were reconstructed byte-for-byte before transplanting only those hunks onto full pinned official rust-v0.163.0 at `83086188ae0614bd882818ff1aa23f3194962012`. Each patch has a unique anchoring context; no full-file overwrite or default-approval broadening occurred.

- `codex-rs/cli/BUILD.bazel`: retains `CODEXDD_VERSION_FILE` wiring so Bazel builds reflect the CodexDD product, not upstream codex alone.
- `codex-rs/cli/src/main.rs`: forwards the optional launcher-assigned `adaptive` value into the interactive `TuiCli` configuration, without changing approval, fallback or startup policy.
- `codex-rs/cli/tests/daemon_startup.rs`: retains the Windows elevated-start embedded-mode/F2 hotfix regressions, using an initial `/status` readiness signal, verifying no shared daemon PID, while retaining normal restrictive-job guidance.

The exact original, official, and merged Git blob IDs for those three paths are pinned in `.github/scripts/test_codexdd_045_3c8c_cli_windows.py`. The other fifteen modified/new CLI sources and fixtures were byte-identical between production 0.4.4 and official rust-v0.162.0 and now exactly match official 0.163.0.

## Pending validation and release gates

3C.4–3C.6 source validation passed 106/106 on Daniel-CL. 3C.7 (41 tests), 3C.8a (11), 3C.8b (12), 3C.8c (13) are staged but still unrun. The CLI source tree is not accepted until native compiler checks and the Windows elevated-SSH/startup regressions pass, alongside the app-server/daemon dependencies and generated protocol schemas.

**Cargo.lock**, native Rust integration, full 21-stage Windows release suite, GitHub CI and final upstream provenance still require source-graph completion and release acceptance. Automatic upstream publication stays hard-disabled. Production markers remain 0.4.4 / rust-v0.162.0, and no merge, artifact shipment or version bump is authorized.
