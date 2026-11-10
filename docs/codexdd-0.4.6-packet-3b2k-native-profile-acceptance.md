# CodexDD 0.4.6 — Packet 3B.2k: actual repo-owned Windows validation profiles

**Scope:** Native profile availability and contract acceptance in disposable test checkouts; no project source mutation, role relaxation, 0.4.5 thaw, production install, release, or branch merge.
**Starting checkpoint:** `d4b1f261a794a21032e9cb80f07b033b2c3a144b` on `dd/codexdd-v0.4.6-agent-reliability`.
**State:** Audit complete; native script execution **PENDING** on a new exact commit.

## Evidence and boundaries

- [#42890342163](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42890342163) passed both new native LVO Git-timeout and authorized nested source/SHA regressions at `80a2098543df51a45e424a3f13342d61573aa64f`. 3B.2j is closed for these exact focused tests.
- The separate preflight identity call `codex_git_utils::collect_git_info` is already **bounded**: `info.rs` uses 5-second `GIT_COMMAND_TIMEOUT`; `git_process.rs` uses `tokio::time::timeout`, `kill_on_drop` and Windows JobObject subprocess-tree ownership. Existing `git_process_tests.rs` includes descendant-cleanup regressions. **This is a source audit, not a new Windows execution of those Git-utils tests.** Do not duplicate this code or claim broad runtime proof.
- The real CodexDD Rust repo has `scripts/codexdd-test-targeted.ps1`, `codexdd-test-workpacket.ps1`, and `codexdd-validation-common.ps1`. Its targeted profile has three actual stages: clean Git diff, `cargo nextest` Core adaptive regression tests, `cargo nextest` TUI adaptive regressions. It emits contract-v1 profile/stage events and records executable exit codes.
- Read-only live source review of **private** `Takuetsu/breakwater` `main` confirms real **Node/TypeScript** repo-owned `codexdd-test-targeted.ps1` and `codexdd-test-workpacket.ps1` with contract-v1 native stages (targeted 4; work-packet 7), package scripts for focused validation, lint, build and profile tests, and Node >=20.19.0. **Availability/source contract only; no BW Node profile execution yet.** No private source, code, credential or private paths may be copied into CodexDD's repository or its public Actions artifacts.

## Bounded native Rust profile smoke (one outcome)

On an explicit opt-in source-matched `windows-2022` job, use the repository-pinned Rust/V8 and `cargo-nextest` setup. Run the unmodified actual `scripts/codexdd-test-targeted.ps1` in the disposable **CodexDD repository checkout**. Verify actual `CODEXDD_VALIDATION_JSON` `profile_begin`, three corresponding `stage_begin` / `stage_end` pairs and a matching final `profile_end` (status `pass`, exit 0, `completed_stages=3`); compare native script exit and declared stage exit codes. Preserve log as job output only, not an artificial release or user-facing LVO PASS.

If the environment lacks a required tool, the profile must emit an observable ERROR and the job must be red; do not substitute fake PASS or silently edit the targeted profile. Stop repeated identical runs after two attempts without new evidence. This **proves the actual Rust profile command contract on a native isolated Windows checkout**, *not* the live Core tool invocation/role/Worker handoff, **not** Node profile completion, and **not** a full work-packet or 21-stage release gate.

## Remaining hard requirements

1. Execute BREAKWATER's actual repo-owned Node profile against a correct isolated Windows checkout with owner-controlled repository access and current source credential. Do not run against live game worktrees; isolated native receipt must be checked before Node acceptance.
2. Verify real Core `run_codexdd_validation` tool invocation and correct first-user Implementation Worker admission/persisted source snapshot using the exact scoped candidate. Independent Validation/Repair roles remain denied the Implementation-only runner.
3. Complete actual separate TUI/app-server restart smoke, Phase 3B.3 durable gate evidence, 3B.4 progress/refusal taxonomy and 3C–4 release/owner gates.

**Never equate a green source/Rustfmt or synthetic fixture run with actual Rust/Node profile acceptance.**
