# CodexDD 0.4.5 — 3F.41A: Windows optimized release-archive inventory (source only)

**NONRELEASE; no optimized Rust compilation.** The bounded 3F.28/3F.31 small-library Rust cache proof is GREEN, while historical 3F.28 CI was red due only to a stale stdout-only verifier. The full optimized CLI `//codex-rs/cli:release_files` had previously timed out close to 2,858 Bazel actions in 90 minutes. Do **not** repeat that cold action graph from this inventory audit.

## Actual current Bazel target source

The release plan is pinned to these exact Git blobs:
- `codex-rs/BUILD.bazel`: `843e9fe095e0982df2043bb17c7da8e6e1c95025`
- `bazel/rules/runtime_package.bzl`: `ba4cdad58f916f04602109c7b4d2512e83da5485`
- `codex-rs/cli/BUILD.bazel`: `5dd2b8b9629275fb2c392c59134ea95a3ce317d5`
- `.bazelrc`: `b7698b896e8d0830b60477999d330b115cf5ee8e`.

`//codex-rs/cli:release_files` includes an optimized CLI executable plus the required **one PDB**, using a `_release_profile` transition with opt compilation, thin LTO and 4 codegen units. `_runtime_binary_files_impl` errors if its required Windows `pdb_file` output group is missing or multiple. `pkg_tar` uses `allow_duplicates_with_different_content = False`, and its archive is unsigned/unstripped. These are *source-level contracts*, not verified optimized runtime artifacts.

`//codex-rs:runtime_package_combined` (and `//codex-rs:runtime_package` alias) currently includes the CLI, app-server, code-mode-host and two Windows sandbox helpers (command-runner and sandbox-setup). The **Windows sandbox service** lives in `//codex-rs:runtime_package_helpers`, and the **responses-api proxy** is only in `//codex-rs:runtime_package_primary`, not the combined target. This may reflect intentional separately distributed optional components, but must be resolved with release-distribution design **before labeling combined as a complete standalone Windows install**. Do not silently add extra executables or omit them from customer-facing package promises.

The release-binary `PDB` contract and combined file inventory are enforced by a cheap 3F.41A source-only guard. Source identity is frozen and no release configuration or binary contents are modified. Next: decide which distribution artifact installs/supplies the optional service and proxy, prove cached optimized Rust work scales beyond the small library, then build the actual EXE/PDB and combined archive once. Inspect PDBs and archive members against an explicit Windows shipping manifest before running the final 21-stage source-frozen suite.

## Separate native evidence

[3F.39](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40760413516) **GREEN** (8/8 TUI selectors). [3F.40B](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40765870072) **GREEN** (8/8 core selectors, 10 Rust cases). All **109** owner semantic signoffs remain `PENDING_OWNER_REVIEW`/`NOT_ACCEPTED`. Draft PR #58 and production 0.4.4 remain unchanged.
