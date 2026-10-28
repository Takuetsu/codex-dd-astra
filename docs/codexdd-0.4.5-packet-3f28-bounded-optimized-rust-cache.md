# CodexDD 0.4.5 — Packet 3F.28: bounded real Rust optimized-profile cache test

**NONRELEASE, source-frozen and cheap relative to the 2,858-action CLI build.** The 3F.27 [Windows cache experiment](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40735428604) passed: both Windows runners had the same normalized PATH SHA256 and the second reused all **3** cacheable actions (3 disk cache hits, zero local), after its predecessor 3F.26 found changed hosted Ruby PATH entries breaking action keys. 3F.25 native authentication/history and 3F.24 GovCloud/adaptive startup owner tests also passed.

## Scope and rationale

Move from synthetic EXE/PDB packaging fixture to one **real existing CodexDD Rust workspace library**, `//codex-rs/utils/elapsed:elapsed` (Cargo `codex-utils-elapsed`), which has no nonstandard Rust dependencies. This is source code from the secured 0.4.5 candidate, not a generated/fake Rust compilation. Limit targets to this one library—never `//codex-rs/cli:release_files` or `//codex-rs:runtime_package`. Build on the pinned MSVC Windows 2022 host with exact Bazel 9.2.0/1.28.1 and `--config=release`, retaining the repository's `--compilation_mode=opt` and `rules_rust thin-LTO` flags, plus `runtime_package_build_mode=release`. This tests actual optimized Rust action reuse; no debug-profile substitution or weakened crypto/dependency/security policy.

Use the same two *independent* runner sequence as 3F.27, with the one explicitly identified hosted Ruby directory removed from each Bazel process PATH. Enforce identical SHA256 fingerprints and run-level/commit identity. Pin the Rust BUILD.bazel, Cargo.toml and `src/lib.rs`, along with Cargo.lock, secure workspace manifest, MODULE.bazel, packaging rule, deny.toml and AWS vendor revision. Verify unchanged `codexdd-version.txt=0.4.4` and `upstream-codex-release.txt=rust-v0.162.0` (production markers deliberately not promoted while upstream 0.163 remains in draft review).

## Time, size and evidence limits

- Each GitHub Windows job caps at **20 minutes**; the native Bazel process gets a **660-second soft timeout** so a failed compilation can still leave cache diagnostics and let GitHub cache post-actions finish, unlike the 90-minute cancelled release build.
- Cache namespace is isolated to `codexdd045-3f28-realrust` and version `045-3f28`; explicit action-disk cache size must be nonzero and at most **512 MiB** on each runner. No compiler binary, EXE, PDB, keys or production artifact is uploaded—only short diagnostic logs.
- The first runner must show successful real Rust target compilation using `--config=release` (`opt` and `thin` settings). The second independently restores the same source/cache, verifies identical normalized PATH, and must show **at least one actual Bazel disk-cache hit and zero local execution actions**.
- If target compilation, release setting, cache budget, source identity or cross-run action reuse fails, mark **NOT ACCEPTED**; do not rerun the expensive optimized CLI automatically.

## Limits even if GREEN

A green 3F.28 would prove only small-library optimized Rust cache-key compatibility and storage feasibility, not capacity for large Rust LLVM thin-LTO outputs, final optimized CLI executable/PDB, merged multi-binary Windows archive, source-owner signoff for all 109 overlapping files, or the final 21-stage Daniel-CL release suite. The earlier full optimized CLI build [#40693489522](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40693489522) timed out at exactly 90 minutes; no cold repeat is authorized. 0.4.5 remains draft and production stays at 0.4.4.
