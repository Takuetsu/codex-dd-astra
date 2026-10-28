# CodexDD 0.4.5 — Packet 3E.8: rename-safe Bazel Cargo dependency aliases

**Status: PROPOSED SOURCE AND NARROW WINDOWS TARGET COMPILE; NONRELEASE; CI RESULT PENDING.**

## First real compiler failure

[Windows CLI debug-smoke #39636959563](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39636959563) ran on exact commit `93fc25a1585533452ccf9f6d21925bbeddd5579b` and failed after 2,763 Bazel actions / roughly 25 minutes while compiling `//codex-rs/skills:skills`. All five initial E0433 errors were `serde_yaml` unresolved in `codex-rs/skills/src/parser.rs` (lines 36, 50, 57 and 168). Windows MSVC setup and the lock/module resolution passed. The earlier green `zstd-sys` native dependency compile remains separate valid evidence; this CI is RED for CLI.

## Root cause and narrowly scoped fix

`codex-rs/skills/Cargo.toml` already contains `serde_yaml = { workspace = true }`. The accepted `codex-rs/Cargo.toml` maps that local Rust name to `noyalib-serde-yaml =0.0.57` via `package =`; `Cargo.lock` is security-pinned and unchanged. The Bazel `codex_rust_crate` macro uses generated `@crates//:defs.bzl` `all_crate_deps()` but **omits the matching generated `aliases()` mapping**, so Bazel doesn't supply `--extern serde_yaml` for the renamed package. This is a Cargo/Bazel rule wiring gap, not missing application code or permission to change parser or package.

The canonical `rules_rs` crate-universe usage supplies `aliases = aliases()` alongside `deps = all_crate_deps()`. Apply that generated naming map to the common library, binary, unit-test and integration-test targets in `defs.bzl`, preserving the exact dependency selection for each target. No changes to `MODULE.bazel`, locks, Cargo manifests, `deny.toml`, application Rust sources or upstream/product version markers.

Before repeating the 25-minute CLI build, compile **only** `//codex-rs/skills:skills` under the hosted Windows 2022 MSVC toolchain, Bazel 9.2.0, unchanged strict module lock and a maximum two concurrent local jobs, upload only logs, and stop at first failure. The smoke job does not build optimized binaries, publish, install or accept any security exception. The four inherited INFO/unmaintained RustSec advisories remain blocking pending owner review.

## Generated helper signature correction — October 21, 2026

[Focused Windows alias run #39650561844](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39650561844) stopped during Bazel package analysis, before Rust compilation. The pinned **rules_rs 0.0.96** generated helper at `@crates//:defs.bzl` is `def aliases(package_name = None)`, so the old rules_rust documentation's `normal = True, normal_dev = True` keyword options are invalid. All generated `aliases` calls must be **zero-argument** `aliases()` in the fork, while preserving their existing matching `all_crate_deps` dependency scopes. Update only the helper call syntax and its static negative regression. The prior static contract passed **647/647** source/updater/v2 tests at commit `c5536a3022345fa7182f74e2e58f7d2238fa4075`; that is not an accepted compiler receipt. Re-run only `//codex-rs/skills:skills` with unchanged Windows source/security gates before repeating any expensive CLI build.

## 3E.8c: narrower alias mapping after exact Bazel transition-cycle evidence

[Windows skills build #39651082149](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39651082149) failed **before Rust compilation** with a Bazel analysis cycle in `//codex-rs/protocol:protocol -> network-proxy -> windows-sandbox-rs -> protocol` across differing configurations. All seven blanket `aliases()` additions from the initial packet had inadvertently changed first-party target analysis throughout the workspace. This means a workspace-wide generated alias map is **not safe for this pinned rules_rs and the current Windows release graph**.

Return `defs.bzl` to the accepted pre-packet implementation everywhere except a new **optional `lib_aliases` argument** to `codex_rust_crate`; when omitted, it is the original empty alias mapping. Only `codex-rs/skills/BUILD.bazel` opts in, by filtering the package's official generated `aliases()` to exactly one value `serde_yaml` and failing closed if no unique pinned mapping exists. The unchanged `skills/Cargo.toml` already maps `serde_yaml` to `noyalib-serde-yaml =0.0.57`. This is a **targeted Bazel rule wiring correction**: no Rust, Cargo, MODULE, lockfile, toolchain, version or security exception changes. Re-run only the pinned Windows `codex-skills` compile. Earlier 647/647 static tests did not prove Bazel graph acyclicity.

## 3E.8d: accepted native skills alias leaf and CLI resume — October 21, 2026

- The [focused Windows skills alias build #39660443013](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39660443013) **SUCCEEDED** at exact Git SHA `66f5543286b9b8df1cb80e49c9ca3f3bda77c99e`. It completed **1,281 Bazel actions**, produced the native `//codex-rs/skills:skills` Rust library and emitted the authentic `Build completed successfully` receipt. The original five `serde_yaml` E0433 errors were absent. The [source guard #39660442971](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39660442971) also passed **450/450 source**, **112/112 updater** and **85/85 historical v2** tests (647 total) on the same SHA.
- First prior failures #39657370820 and #39657370864 stopped on a stale skills source identity and test expectation, not compilation. Commit `66f5543` fixed only those two guards without changing the accepted Cargo lock or security policy. The positive native leaf build is a **new** passing receipt, not a claim that the prior full CLI compilation passed.
- **Next controlled action:** update the existing read-only nonrelease Windows `//codex-rs/cli:release_files` debug smoke to require the exact **green** `defs.bzl` blob `f49bfc9be69d03d60a02e8aec6974a6fae9e200c` and `codex-rs/skills/BUILD.bazel` blob `9f4e136ec76fc298e32ccb94fd0800b1f5656c76`; stop at first compiler failure and upload diagnostics only. Neither the 21-stage native Windows release suite nor optimized packaging is authorized here. Four inherited INFO/unmaintained all-target notices still block production acceptance.

## 3E.8e: downstream core-plugins alias reached by CLI retry — October 21, 2026

[Windows CLI retry #39663703563](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39663703563) failed on `core-plugins` `serde_yaml` after resolving `codex-skills`. [Source guards #39663703585](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39663703585) passed 648/648 on source SHA `cc89a756b6516ba1cf8f1a6bd53fd1900066bc8f`. Continue in the separate [3E.9 core-plugins alias packet](codexdd-0.4.5-packet-3e9-core-plugins-bazel-alias.md) with a bounded one-library Windows check. Do not broaden the dependency alias map or start another long CLI build before that focused gate passes.
