# CodexDD 0.4.5 — Packet 3E.5: Windows Bazel zstd-sys patch refresh

**Status: WINDOWS MSVC RELEASE-GRAPH ANALYSIS GREEN; NATIVE COMPILATION OPEN. NONRELEASE.**

## Exact preceding native failure

[Windows release analysis run #39628795223](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39628795223), from commit `7384da0b29e57c48e763b3794ec69ed67d41053f`, passed source identity, hosted-MSVC initialization, Bazel 9.2.0 and `bazel mod deps --lockfile_mode=error`. Its configured `//codex-rs/cli:release_files` query **failed**, before reaching combined runtime archive analysis, while fetching `zstd-sys 2.1.2+zstd.1.5.7`. Bazel could not apply the fork's `patches/zstd-sys_windows_msvc_include_dirs.patch` final visibility hunk (`CONTENT_DOES_NOT_MATCH_TARGET`, near old line 210). This is a patch/source compatibility issue, **not** Rust compilation, a license issue, or a failure of the strict Bazel dependency lock.

## Smallest reviewed patch change

- The current published `zstd-sys 2.1.2+zstd.1.5.7` `build.rs` already protects the visibility flag with `if hide_symbols() && !target_is_msvc()`; the original fork patch tries to replace an unconditional flag which no longer exists. Retain the **two original proven-applicable hunks** that define target and add native MSVC include paths for `zstd/lib/{compress,decompress,dictBuilder,legacy}` and `zstd/contrib/seekable_format`. Remove only the now-obsolete third visibility hunk, without overriding upstream visibility or changing the dependency graph.
- This is restricted to the repository patch file, read-only CI pin/trigger and source-only regression tests. Do **not** change `MODULE.bazel`, `MODULE.bazel.lock`, `Cargo.toml`, `Cargo.lock`, `deny.toml` or any runtime/release markers without a fresh separately reviewed resolver decision.
- The same existing Windows-2022 workflow checks the immutable new patch blob, existing locked module, MSVC `cli:release_files` and then combined runtime archive release-profile `cquery`; any failure stops there and uploads logs. No EULA acceptance, build, deployment or merge. If the patch bytes require a Bazel module-lock refresh, fail closed and review it separately before rerunning.

The four inherited INFO/unmaintained RustSec advisories remain RED. A successful `cquery` would prove configuration analysis only, not binary creation, a full 21-stage Daniel-CL receipt or release authority.

## Native Windows configured graph closeout — October 21, 2026

[CI #39630905182](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39630905182) passed at exact source SHA ce1b8acf5ebbbf22d642d9f1494d46450afc279b: pinned source/security and production markers, strict Bazel lock, and Windows MSVC configured targets for CLI release files (1243 packages / 57185 targets) and combined runtime archive (41 packages / 14842 targets). [Source guard #39630905234](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39630905234) passed 442/442 static, 112/112 guarded updater, and 85/85 historical reconciliation checks (639 total). There was **no native compilation**, EULA acceptance, branch write or release. The next bounded packet compiles only the zstd-sys crate under hosted Windows MSVC. Four inherited INFO/unmaintained all-target cargo-deny findings remain RED, and the frozen release/21-stage Daniel-CL suite remains unapproved.
