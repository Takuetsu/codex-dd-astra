# CodexDD 0.4.5 — Packet 3E.2: Bazel rules_rust release profile and Windows argument handling

**Status: SOURCE CHECKPOINT STAGED / NATIVE WINDOWS BAZEL VALIDATION PENDING. This is not a release candidate.**

## Accepted preceding local diagnostic

- On Daniel-CL, `bazelisk query --lockfile_mode=off '//codex-rs:runtime_package_combined'` at integration commit `f20d467917c638a938bb51533142140348c0c165` completed successfully and printed the exact target label; Bazel 9.2.0 / Bazelisk 1.29.0. Two module-version warnings (`platforms 1.0.0 versus 1.1.0` and `rules_cc 0.2.18 versus 0.2.20`) remained nonblocking. No build/test/runtime or release-profile proof follows from a query.
- Worktree diagnostics showed only untracked existing worktree directories under `E:\\codexdd`; there was no evidence of modified tracked source. Do not clean or delete those directories.

## Identity and one-owner scope

- Fork production CodexDD 0.4.4: `b25550dcb2c6e84a84074b603086c9db6f32a63b`; upstream tracked release marker `rust-v0.162.0`.
- Reviewed upstream Codex 0.163: `83086188ae0614bd882818ff1aa23f3194962012`. Upstream `MODULE.bazel` Git blob `d946666bec0b47facc9c5eb07294919bf071226b`.
- Parent integration checkpoint: `f20d467917c638a938bb51533142140348c0c165` with previously loaded `rules_pkg 1.0.1`.
- Integrate precisely two official upstream patch files: `patches/rules_rust_release_build_script_profile.patch` blob `a849512e9a383d55eb44d11d8f646e0ff2ecf525`, and `patches/rules_rust_process_wrapper_param_file.patch` blob `969357eaf1f707962bf9550eb639b6058369244d`.
- Reconcile `MODULE.bazel` and `patches/BUILD.bazel` together. Keep fork-added windows patch series, `rules_rs 0.0.96`, `rules_pkg 1.0.1`, release markers, prior CodexDD CLI version-data Bazel wiring, and `MODULE.bazel.lock` intact.

## Semantic owner decisions

1. Adopt official 0.163 release-build-script profile handling from `rules_rust_release_build_script_profile.patch`. The release transition already staged in 3E.1 selects `cargo-release-profile`; this patch maps Rust opt/debug profile settings into native Cargo build-script environment without dropping necessary ABI/SDK flags.
2. Adopt official 0.163 `rules_rust_process_wrapper_param_file.patch`, which writes wrapper arguments into a multiline response file and teaches the wrapper to read them before parsing flags. It supersedes the fork's older grouping-only mitigation for Windows command-line length. The old patch file and BUILD export remain present for forensic review but are **not active** in MODULE; do not stack both competing modifications to the same wrapper argument list.
3. Configure `MSVC_RUSTC_FLAGS` for `aarch64-pc-windows-msvc` and `x86_64-pc-windows-msvc` in both `extra_exec_rustc_flags` and `extra_rustc_flags`. This aligns native and cross build scripts with Cargo's static CRT expectations; it does not authorize an install or an alternate Windows target.
4. Deliberately defer the unrelated upstream `osx.libraries` addition, `codex_release_0.162.1` test archive update, and `libsqlite3-sys` CPU-intrinsics annotation for separate owner review. Do not copy the entire upstream MODULE or its lockfile.

## Required narrow validation and follow-ups

- First local gate: from the integration checkout on Daniel-CL, verify exact branch SHA and run **only** `bazelisk query --lockfile_mode=off 'deps(//codex-rs:runtime_package_combined, 1)'`. This causes Bazel to load the new rule wiring and immediate package labels, without building or running tests, and must not update the local MODULE lockfile. Record the output and stop on the first error.
- Static source asserts before the checkpoint: two patch blobs match official upstream identities, the fork-specific Windows direct-link, skip-temp and execroot patches remain in MODULE, old group patch is no longer applied, CRT flags are wired to both Rust toolchain halves, and the version marker remains 0.4.4.
- After a green local query, a **separate packet** must regenerate `MODULE.bazel.lock` deterministically and prove it is consistent with the full fork MODULE and all patch identities. Do not accept the upstream lockfile wholesale. This is not yet a lockfile-format, compile, package-archive or release acceptance.
- Native Windows release-profile compilation, live F2/daemon/resume checks, the 15 original upstream conflict dispositions, the four inherited advisory decisions, security acceptance, detached 21-stage release suite and production merge remain independently blocked.
