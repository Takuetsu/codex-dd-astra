# CodexDD 0.4.5 — Packet 3E.3: branch-bound Bazel module lock reconciliation

**Status: GITHUB ORACLE STAGED; RESOLVER AND LOCK ACCEPTANCE PENDING. NOT A RELEASE CANDIDATE.**

## Source and reason for GitHub-only execution

- Integration branch: `dd/codexdd-v0.4.5-upstream-0.163.0-integration`; reviewed parent `ad34e616df5ec19e6fae2e694041317a153c807c`.
- Official upstream target: `rust-v0.163.0`, commit `83086188ae0614bd882818ff1aa23f3194962012`.
- Previous Daniel-CL check at `f20d467917c638a938bb51533142140348c0c165` verified the new `runtime_package_combined` Bazel label can load; it did not verify dependency closure or the lock.
- Packet 3E.2 added official Bazel release-profile patches and Windows MSVC toolchain flags. The committed `MODULE.bazel.lock` is still the older fork blob `e5c69a07a0f5987c3cff38ff35e5ed5be3b91f1b`.
- The owner instructed that development-level manual diagnostics stop on Daniel-CL until final release validation. This is the explicit GitHub-only exception to normal local-first development; do not request operator commands.

## One-purpose bounded GitHub resolver

- New workflow `.github/workflows/codexdd-045-bazel-lock-reconcile.yml` triggers only when that workflow file is added/changed on the **0.4.5 integration branch**, not on every `MODULE.bazel` change. It has no release, install, merge, deploy or upstream-sync job.
- Before work, compare the source branch, parent ancestry, exact `MODULE.bazel` SHA, old lock SHA, Cargo manifest/lock, `deny.toml`, both unchanged release markers, the declared `rules_pkg`, official profile and process-wrapper patches, and clean tracked worktree.
- Bazel is configured with repository-pinned Bazelisk/Bazel 9.2.0. Run `bazel mod deps --lockfile_mode=update` against the **fork graph**, then separately `bazel mod deps --lockfile_mode=error` against its generated lock. Capture both full diagnostic logs.
- Use Python to verify parseable JSON and unchanged lockfile format, refuse unchanged lock, `git diff --check`, and prove that the **only** modified tracked file is `MODULE.bazel.lock`. Recheck branch HEAD immediately before pushing; permit only a fast-forward single-file lock commit on the integration branch.
- Minimal `contents: write` applies only to this isolated Actions job, used to write the reviewed lockfile. No secrets are read by the script; no fork-root permissions are granted to arbitrary PRs. On any failure, stop without a commit and upload the resolver logs.

## Acceptance and remaining work

- A green resolver job plus an inspected exact commit/diff establishes the bounded lockfile checkpoint. A Bazel lock refresh is **not** proof that Rust/Bazel binaries can compile, release archives can be built or live Windows startup behaves correctly.
- The `platforms` and `rules_cc` direct-dependency warnings need an independent review; do not silence them by turning off direct dependency checks.
- No new Cargo version, Rust code, release markers, `deny.toml` exemptions, production branch or external machine may be changed by the lockfile job.
- After lock acceptance, separately perform exact candidate/source audits, targeted native/release-profile tests and all remaining release security/conflict decisions. Only then consider a 21-stage Windows release test and installation.

_Operator action required now: none._
