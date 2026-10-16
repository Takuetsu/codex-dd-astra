# CodexDD 0.4.4 Windows Rusty V8 release preflight

The 21-stage Windows release profile builds `codex-cli` and
`codex-code-mode-host` in Stage 21. The code-mode host requires Rusty V8
with pointer compression and sandbox enabled. The default `denoland/rusty_v8`
release does not publish that Windows combination.

OpenAI Codex publishes the required archive and matching Rust bindings under
the `rusty-v8-v<version>` GitHub release tag. This repository pins the
release-manifest checksums in
`third_party/v8/rusty_v8_<version>_release_manifests.sha256`.
The preflight follows the same verification chain as
`.github/actions/setup-rusty-v8`.

## Prepare before the detached Windows release run

Use an interactive PowerShell session on the Windows build machine, with the
isolated validation checkout pinned to the release candidate:

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File E:\codexdd\.worktrees\codexdd-044-final-release\scripts\codexdd-prepare-rusty-v8.ps1 -Prepare
```

The preflight reads the pinned V8 version from `codex-rs/Cargo.toml` and
selects the Windows target. It downloads the V8 archive, matching Rust
bindings, and release checksums into a cache under Cargo's target directory.

1. Verify the downloaded checksum manifest against the repository's trusted hash.
2. Verify both the archive and Rust bindings against that manifest.

Python is not required. If an asset is unavailable or fails verification,
stop release validation. Do not substitute an upstream archive without the
required sandbox and pointer-compression features.

## Validate Stage 21 only

Run the repository-owned Stage 21 wrapper before repeating the full suite:

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File E:\codexdd\.worktrees\codexdd-044-final-release\scripts\codexdd-test-cli-release.ps1
Write-Host "Stage 21 exit code: $LASTEXITCODE"
```

The wrapper re-verifies the cached assets offline, exports
`RUSTY_V8_ARCHIVE` and `RUSTY_V8_SRC_BINDING_PATH` for its Cargo subprocess,
and runs the unchanged locked release build. The detached S4U task does not
need access to the GitHub release.

Preserve logs from earlier failed runs. Passing Stage 21 separately does
not establish an exact-head 21/21 release pass. After targeted validation
and exact-head GitHub CI pass, run the entire detached 21-stage release
profile with a fresh plan and log directory. Do not merge or install before
release acceptance.
