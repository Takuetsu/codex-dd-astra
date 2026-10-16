# CodexDD 0.4.4: Windows Rusty V8 release preflight

The Windows release profile has 21 stages. Stage 21 builds codex-cli and
codex-code-mode-host in release mode. The code-mode host requires Rusty V8
with pointer compression and the V8 sandbox enabled. The default denoland
Rusty V8 GitHub release does not publish that flavor for Windows.

OpenAI Codex publishes the matching archive and Rust bindings in the GitHub
release tagged rusty-v8-v<version>. The repository already pins the release
manifest hashes at third_party/v8/rusty_v8_<version>_release_manifests.sha256.
The Windows preflight follows the same checksum trust chain as the existing
.github/actions/setup-rusty-v8 action.

## Before starting the detached Windows release task

Run the following in interactive PowerShell on the Windows build machine.
Use the release-candidate's clean, isolated worktree:

    powershell.exe -NoProfile -ExecutionPolicy Bypass -File E:\codexdd\.worktrees\codexdd-044-final-release\scripts\codexdd-prepare-rusty-v8.ps1 -Prepare

The preflight reads the pinned V8 version from codex-rs/Cargo.toml, selects
the host Windows target, downloads the V8 library archive and Rust bindings
into the Cargo target directory's codexdd-v8-artifacts cache, and verifies:

1. The release checksum manifest against the repository's trusted hash.
2. The archive and matching binding against that verified manifest.

No Python installation is required. An unsuccessful or unverified download
stops release validation; never substitute an upstream archive without the
required sandbox/pointer-compression feature combination.

## Targeted Stage 21 test before a full release run

Execute only the repository-owned wrapper:

    powershell.exe -NoProfile -ExecutionPolicy Bypass -File E:\codexdd\.worktrees\codexdd-044-final-release\scripts\codexdd-test-cli-release.ps1
    Write-Host "Stage 21 exit code: $LASTEXITCODE"

The wrapper re-verifies the cached assets offline. Then it sets
RUSTY_V8_ARCHIVE and RUSTY_V8_SRC_BINDING_PATH in its own process, and runs
the original locked Cargo release build for both binaries. The detached S4U
runner uses the same wrapper and requires no network connectivity.

Preserve prior failed run logs. A successful targeted Stage 21 result at a
new commit does not by itself establish a passing 21/21 release on that exact
commit. After exact-head GitHub CI and targeted Windows verification, execute
one detached full release profile in a fresh unique run directory. Keep the
PR draft and production installation unchanged until acceptance.
