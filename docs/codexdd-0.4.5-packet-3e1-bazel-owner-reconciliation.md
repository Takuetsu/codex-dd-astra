# CodexDD 0.4.5 — Packet 3E.1: atomic Bazel 0.163 packaging and Windows-MSVC owner reconciliation

**Status: SOURCE STAGED / TARGETED WINDOWS BAZEL VALIDATION PENDING. Not a release candidate.**

## Scope and anchored provenance

- Branch: `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.
- Parent checkpoint: `8f04cf10cfbeddcd687a57dfdc5c7d1b8d247461` (core integration-test compilation green).
- Official baseline: `c1382380de69521303b416720a52f42d51af6248` (`rust-v0.162.0`).
- Official target: `83086188ae0614bd882818ff1aa23f3194962012` (`rust-v0.163.0`).
- Protected fork production: `b25550dcb2c6e84a84074b603086c9db6f32a63b` (CodexDD 0.4.4).
- This packet owns only Bazel definitions, release-archive declarations, the associated MSVC Bazel config, and the existing Bazel CI workflow. No Rust/Cargo code, version markers, installer, CI release profile or security policy changed.

## Reviewed integration details

1. Add the official 0.163 `bazel/rules/runtime_package.bzl` rule as a coherent dependency: it owns `release_binary`, `runtime_archive`, the release/debug transition, symbol staging and packaging. Add its export to `bazel/rules/BUILD.bazel`.
2. Merge official `codex-rs/BUILD.bazel` targets `runtime_package_combined`, `runtime_package_primary`, `runtime_package_app-server`, `runtime_package_helpers` and `runtime_package_build_mode`, **preserving** the fork-only `codexdd-version.txt` export.
3. Stage the official unchanged-fork release outputs in `codex-rs/app-server/BUILD.bazel`, `code-mode-host/BUILD.bazel`, `responses-api-proxy/BUILD.bazel`, `windows-sandbox-rs/BUILD.bazel`, `windows-sandbox-service/BUILD.bazel` and `bwrap/BUILD.bazel`. The fork's existing `codex-rs/cli/BUILD.bazel` already has upstream-compatible release outputs; leave its protected `CODEXDD_VERSION_FILE` build-script wiring **untouched**.
4. Bring `.bazelrc` to official 0.163 identity (the fork and official 0.162 files had identical blob SHA `ea06d44080c082913fe7fe2ce537d8e42c70728d`). This introduces required Windows MSVC host/toolchain settings and consistent release-profile configuration; do not copy just the workflow flag without its definition.
5. Update the fork's Bazel CI workflow **only** to add official 0.163 native-Windows manual-dispatch allowance and the `--config=ci-windows-msvc` native test flag. Retain all four fork-owned 90-minute CI timeouts, the existing native 40-minute timeout, and current matrix/security settings. This file is a reviewed merge, not an upstream overwrite.

## Focused validation and hard stops

- Stage-only inspection checks: `codex-rs/cli/BUILD.bazel` retains version-file wiring, 90-minute CI windows retain four entries, all package rules/aliases and matching Bazel config coexist, release markers remain 0.4.4 / rust-v0.162.0.
- **Next required operator gate:** on Daniel-CL at the latest exact branch SHA, run `bazelisk query --lockfile_mode=off '//codex-rs:runtime_package_combined'` before broad Bazel builds; using `--lockfile_mode=off` avoids local lockfile rewriting during this diagnostic. Record the exit code and test only this section on failure. A query is graph loading, not binary compilation, lockfile acceptance, release-profile validation or authorization to publish.
- Do **not** trigger expensive Bazel workflows or the 21-stage Windows release suite before this local gate. If the local Bazel toolchain is unavailable, document the blocker and request explicit approval for a GitHub-only diagnostic rather than adding a throwaway workflow.
- The original 15 upstream direct-conflict dispositions, remaining 0.4.5 native runtime tests, four inherited cargo-deny unmaintained advisories, release-profile and live F2/daemon/resume evidence remain **open**. No advisory exemption is created.
- Continue only after focused validation. Product release and tracked upstream markers must remain 0.4.4 / rust-v0.162.0 until separate release acceptance.

## Packet 3E.1a — Bazel module dependency correction (2026-10-21)

**Status: DECLARATION STAGED / DANIEL-CL GRAPH QUERY PENDING.**

- The first Daniel-CL query on staged source `1c70e0448f0ef99f79ec02e28cf1c49cd2c5bc76` executed Bazelisk 1.29.0 / Bazel 9.2.0 successfully, then failed during package loading in `bazel/rules/runtime_package.bzl:5:6`: `No repository visible as '@rules_pkg' from main repository`.
- The upstream `rust-v0.163.0` `MODULE.bazel` at Git blob `d946666bec0b47facc9c5eb07294919bf071226b` declares `bazel_dep(name = "rules_pkg", version = "1.0.1")`, while the original official `rust-v0.162.0` and fork production `MODULE.bazel` share blob `3fd011a3c08e065e7faad79858425d8fd02cb7d1` and do not. The new `runtime_package.bzl` has upstream blob `ba4cdad58f916f04602109c7b4d2512e83da5485`, which imports the missing module. Add **only that exact upstream dependency declaration**, adjacent to `rules_platform`, without changing fork-specific rules/patches.
- The separately reported `platforms@1.0.0 -> 1.1.0` and `rules_cc@0.2.18 -> 0.2.20` version messages were **warnings**, not the blocking missing-repository error. This packet does not change their version constraints.
- Use read-only `bazelisk query --lockfile_mode=off '//codex-rs:runtime_package_combined'` for the next local preflight; the committed `MODULE.bazel.lock` remains the fork-owned old resolver lock **and is not accepted for release** until a separate deterministic Bazel lock regeneration/verification packet. Do not import upstream's unrelated lockfile wholesale or silently mutate the operator's working tree.
- Official 0.163's other MODULE additions (release build-script patch and Windows CRT/toolchain flags), plus replacement of the fork's grouped build-script argument patch, require their own careful source+native validation packet and are **not** accepted by this missing-dependency fix.
- Source guards and product markers remain unchanged; this does not trigger a GitHub release workflow, build, long suite, deployment, or merge.
