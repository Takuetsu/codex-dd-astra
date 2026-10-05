# CodexDD 0.4.0 validation profile contract

**Status:** 3B.1 implementation contract

**Contract version:** 1

**Platform:** Windows

CodexDD repositories opt into Local Validation Orchestrator execution with these conventional scripts:

```text
scripts/codexdd-test-targeted.ps1
scripts/codexdd-test-workpacket.ps1
scripts/codexdd-test-release.ps1
```

The 0.4.0 runtime implementation must treat these scripts as deterministic repo-owned validation profiles, not arbitrary model-authored shell.

## Invocation

Canonical Windows invocation:

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\codexdd-test-workpacket.ps1
```

The same form applies to `targeted` and `release`.

A profile accepts no arbitrary command text.

## Profiles

### targeted

Purpose: smallest deterministic CodexDD regression set used for focused validation and retest.

Current self-host stages:

1. `git diff --check`;
2. canonical nextest environment + `cargo nextest run --no-fail-fast -p codex-core --lib adaptive`;
3. canonical nextest environment + `cargo nextest run --no-fail-fast -p codex-tui --lib adaptive`.

The scripts reproduce the root `just test` environment directly on Windows:

- `CODEX_REPO_ROOT=<repository-root>`;
- `RUST_MIN_STACK=8388608`;
- `NEXTEST_PROFILE=local`.

This avoids recursively entering the repository's Windows `just` shell adapter from inside a validation PowerShell process while preserving the same nextest settings.

### work-packet

Purpose: normal post-implementation Local Validation Orchestrator gate.

Current self-host stages:

1. `git diff --check`;
2. Rust formatting check using the repository's canonical `cargo fmt` configuration;
3. scoped `cargo clippy --tests` for `codex-core` + `codex-tui`;
4. adaptive core nextest;
5. adaptive TUI nextest;
6. locked debug CLI build.

This is the default profile intended for ordinary Implementation mechanical validation.

### release

Purpose: broad explicit pre-CI/pre-install regression gate.

Current self-host stages cover:

- diff and Rust formatting;
- workspace Clippy;
- history and rollout;
- protocol/state/thread-store;
- core;
- app-server;
- full TUI library suite;
- locked release CLI build.

The release profile is intentionally expensive and must not be selected silently by an ordinary Implementation Worker.

## Exit codes

Profile process exit codes are part of contract version 1:

| Exit code | Meaning |
| --- | --- |
| `0` | Every stage passed. |
| `1` | A validation command ran and returned a nonzero native exit code. This is a candidate product/test failure. |
| `2` | Profile contract or execution infrastructure failed, for example an invalid repository root or missing executable. This is not automatically source-repair authority. |

Profiles stop at the first failed/error stage.

## Structured event stream

Machine-readable events are written to stdout as one line each:

```text
CODEXDD_VALIDATION_JSON <compact-json-object>
```

Every object contains:

- `contract_version` = `1`;
- `event`.

Defined events:

### profile_begin

Fields:

- `profile`;
- `stage_count`.

### stage_begin

Fields:

- `profile`;
- `stage`;
- `ordinal`;
- `stage_count`;
- `executable`;
- `arguments`.

### stage_end

Fields:

- `profile`;
- `stage`;
- `ordinal`;
- `status` = `pass | fail | error`;
- `native_exit_code`;
- `duration_ms`;
- `error_type` / `message` when applicable.

### profile_end

Fields:

- `profile`;
- `status` = `pass | fail | error`;
- `exit_code`;
- `duration_ms`;
- `completed_stages` when applicable;
- `failed_stage` when applicable;
- `error_type` / `message` for profile-level contract errors.

Normal native command output may appear between event lines. 3B.2 must retain the full process output in the runtime log while parsing only prefixed event lines into the compact validation result.

## Safety and determinism requirements

A conforming profile:

- is repository-owned and reviewable;
- contains a fixed stage list;
- does not accept arbitrary shell text from the model/runtime;
- uses deterministic local commands;
- returns the contract exit code categories above;
- does not perform merge, install, release publication, deployment, or destructive Git actions;
- does not silently edit source;
- should use check/lint/test/build commands that leave source content unchanged;
- may create ordinary build/test artifacts outside tracked source content.

The Local Validation Orchestrator runtime must still execute the profile through Codex's existing permission, sandbox, cancellation, and tool lifecycle infrastructure.

## Full-log ownership

The profile itself streams native stdout/stderr and structured events. Native stderr is output evidence, not by itself an infrastructure exception; the native process exit code determines PASS/FAIL unless executable resolution or process launch itself fails.

3B.2 owns persistent log capture. It must store the complete output outside the validated source worktree, under bounded CodexDD runtime storage, and return only compact stage/error evidence to the model by default.

## Contract smoke

`scripts/codexdd-validation-contract-tests.ps1` exercises the three process outcome classes on Windows:

- success with native stderr -> 0;
- native validation failure with stderr -> 1;
- execution/infrastructure error -> 2.

This smoke does not run the expensive CodexDD validation profiles.

## 3B.1 boundary

This contract and the self-hosting scripts do not make validation automatic.

3B.2 adds the bounded runtime runner. 3C.1 integrates that runner into protected mechanical validation and makes a current work-packet PASS receipt a prerequisite for `ready_for_validation`.


### CodexDD self-host formatting scope

The CodexDD self-host `work-packet` and `release` profiles run the canonical Rust formatting check directly:

```text
cargo fmt -- --config imports_granularity=Item --check
```

The repository-wide `scripts/format.py --check` also checks Just and Bazel/Starlark sources. On the frozen 0.3.14 production baseline those unrelated formatter groups are not clean in Daniel-CL's current formatter environment, even though the LVO packets do not modify those files. They therefore are not used as the routine LVO packet gate. This does not authorize formatting drift in files changed by a packet; each repo-owned profile is responsible for deterministic formatting checks appropriate to its owned source surfaces.
