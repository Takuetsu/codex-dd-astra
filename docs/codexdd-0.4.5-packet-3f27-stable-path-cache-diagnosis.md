# CodexDD 0.4.5 — 3F.27: isolate Windows PATH drift in Bazel action-cache keys

**NONRELEASE.** Previous 3F.26 two-runner experiment [#40732792493](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40732792493) compiled the tiny synthetic Windows EXE/PDB fixture successfully on both runners, and the second restored a 34 MB Bazel action cache successfully. But both reported `16 processes: 13 internal, 3 local` and no action reuse: the deliberate final cache-hit gate failed.

Comparing both MSVC-initialized runner PATHs revealed 98 entries each with **exactly one changed entry**: `C:\hostedtoolcache\windows\Ruby\3.3.12\x64\bin` versus `...\Ruby\3.3.13\x64\bin`. The repository `.bazelrc` explicitly passes the full inherited PATH into both `build:ci-windows-msvc --action_env=PATH` and `--host_action_env=PATH`. It is a plausible action cache key mismatch, not yet proven.

**3F.27 cheap two-runner diagnostic:** keep the exact prior source/security pins, Bazel/MSVC configuration, and synthetic EXE/PDB target. Remove **only** the one GitHub-hosted Ruby directory from PATH in each build process before invoking Bazel, retaining MSVC, SDK, Git, Bazelisk and all other installed tool entries. The seed job writes the SHA256 fingerprint of the normalized PATH to a job output; the independent runner must have the same fingerprint or fail before building. The seed completes normally and saves a new isolated disk/repository cache namespace `045-3f27`; the second job must show an actual Bazel disk/remote action hit (not merely GitHub cache transport).

This is a nonrelease cache-key feasibility experiment only. Success does not establish cross-run capacity for 2,858 optimized Rust actions or release-profile thin-LTO, and cannot authorize an expensive optimized build without further review. Do not merge, install, alter Cargo/security policy or promote 0.4.4 production markers. Final optimized CLI/archive, 109 semantic owners, and 21-stage Daniel-CL suite remain pending.

## Final evidence — 2026-10-28

**PASS, scoped to cross-run Bazel action-cache reuse of the synthetic fixture.** The exact-source Windows workflow [#40735428604](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40735428604) completed successfully:

- Seed job **122982555165**: 16 actions, **13 internal / 3 local**; clean cache keys seeded, all Bazelisk/disk/repository post-action caches saved successfully.
- Independent Windows restore job **122984344499**: all three hosted GitHub caches restored, including **~34 MB Bazel disk cache** and **~112 MB repository cache**; 16 actions, **13 internal / 3 disk cache hits**, zero local build actions in the Bazel process summary.
- Both seeded/consumed the identical SHA256 of the normalized action PATH: `7e3457c3218c6933b8b30de65061ccf56c951619862e7d4f38ed9f2be086908e`. Each test runner removed only its unique hosted Ruby path; MSVC, SDK, Bazelisk, Git, and the remainder of PATH stayed intact.
- The prior [3F.26](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40732792493) had identical synthetic test configuration except environment normalization/cache namespace and showed **3 local** actions again on the consumer. This strongly supports inherited hosted-tool PATH drift as the action-key obstacle, without requiring any repository-wide toolchain or security-policy changes.

**Acceptance recorded:** `SMALL_WINDOWS_MSVC_BAZEL_CACHE_REUSE_VERIFIED` on nonrelease fixture only. The unchanged candidate security pins, official upstream target, optimized release mode, packaging rule and production 0.4.4 markers are not promoted.

**Still blocked:** optimizing/packaging real `//codex-rs/cli:release_files` (2,858-action dependency graph; earlier thin-LTO attempt cancelled at exactly 90 minutes), confirming cache storage quotas and repeatability on real Rust outputs, 109-path semantic approval and the source-frozen 21-stage Daniel-CL release suite. A small action-cache proof cannot be treated as a successful optimized Rust compiler/linker run. Any next real optimized attempt must be explicitly budgeted, isolated and nonrelease, with cheap source-only and local checks first.
