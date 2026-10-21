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
