# CodexDD 0.4.6 — Packet 3B.2i-h: bounded TUI Git inspection on embedded resume

**Type:** NONRELEASE source checkpoint, awaiting exact-source formatting and native regression proof.
**Source parent:** `d844289af99142f6cdc4ea0922b9ea5b8b233d45`.
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.

## Outcome and observed gap

Previous 3B.2i-g source `ca0374329233a216206b3b38c6b080dced526080` passed all three focused Windows tests in [#42868360986](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42868360986): 3B.2p bounded Core Git child, 3B.2n native app-server cold restart/second persisted candidate, and 3B.2q embedded TUI pin reconciliation, one pass each. That run did **not** prove that the **TUI** Git child itself is bounded.

In the TUI, `preflight_mechanical_target` used `Command::output()` for read-only root/head/status inspection, and `reconcile_core_candidate_preflight` used `Command::status()` for ancestry. The 20-second `tokio::time::timeout(spawn_blocking(...))` on the resume path could return while a stalled Git child kept running; initial TUI preflight had no corresponding outer timeout. This is a confirmed boundedness **source-design gap**, not proof of an observed real-world hang in the TUI.

## Changes

- `codex-rs/tui/src/adaptive_worker.rs`: use one bounded Git child runner for TUI root, HEAD, full-untracked status and ancestry; null stdin/stderr, scratch-file stdout with a 1 MiB cap, interactive credential prompting disabled, `--no-optional-locks`, short polling, direct child kill/wait on deadline. No process-tree kill or automatic promotion is claimed.
- Metadata/ancestry budget 5 seconds per step; full status budget 12 seconds. On launch, output, command failure or timeout: `SETUP_BLOCKED`; no new TUI pin or validation PASS.
- Regression `codexdd_046_3b2r_stalled_tui_git_child_is_bounded_and_reaped` uses a five-second synthetic process with a 150ms budget, asserting stage-specific timeout and confirmed direct-child termination.
- `codex-rs/tui/src/app_server_session.rs`: increase only the **outer async guard** to 35 seconds, above the per-step combined 27-second maximum. A failed check retains the original TUI source pin. Existing wrong-root, dirty, wrong-scope, fork, remote and descendant rules remain unchanged.

## Acceptance / STOP

1. Source-identity check and `cargo fmt -- --config imports_granularity=Item --check` on exact branch HEAD. If red, correct formatting before any costly native dispatch.
2. Cheapest available relevant Windows focused test for new `codexdd_046_3b2r_`; revalidate existing `codexdd_046_3b2q_` and Core/app-server cross-boundary selectors on the **same exact source**, using only an authorized narrow test invocation.
3. Record actual test counts, source SHA, link and failure classification. A compilation-only success or skipped Windows job is not executable acceptance. Stop after two noninformative attempts, not repeated broad CI.
4. Later separate work remains: actual live two-program TUI + app-server process restart smoke, Node/Rust repo-owned LVO profiles, independent Validator and exact-source wrong-thread/flush negatives; Phase 3B.3 gate provenance and 3B.4 progress/circuit-breaker; final owner-controlled Windows 21-stage/daemon/F2/security/distribution and release signoff.

**Boundaries:** no source checkout/reset, worktree mutation, permission relaxation, model escalation, 0.4.5 thaw, release, merge, installation or change to production 0.4.4. The 3B.2i-h source commits are **not yet native validated**.

## Native acceptance verified — 2026-11-10

**PASS (focused 3B.2i-h); exact candidate `b5c99d3d4af3828f3c590ea3122e9d4a0b4cdde3`.** [GitHub Actions run #42878341161](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42878341161) completed successfully. Source-contract preflight and Rustfmt passed before Windows tests. Windows job `130048575651` executed and passed exactly four focused regression selectors:

| Selector | Observed native Windows result |
| --- | --- |
| `codexdd_046_3b2p_stalled_git_process_is_bounded_and_reaped` | 1 passed; 0 failed; 0 ignored |
| `codexdd_046_3b2n_real_app_server_candidate_and_cold_resume` | 1 passed; 0 failed; 0 ignored |
| `codexdd_046_3b2q_embedded_resume_reconciles_only_authorized_clean_descendant` | 1 passed; 0 failed; 0 ignored |
| `codexdd_046_3b2r_stalled_tui_git_child_is_bounded_and_reaped` | 1 passed; 0 failed; 0 ignored |

The new 3B.2r filter additionally enumerated exactly one test before executing it. Windows TUI 3B.1 broader compile/regression job was **skipped by design**, not counted as PASS. The preceding `c5eea24f` and `5411cc80` Rustfmt failures were corrected in this green source; those red results are historical and do not confer acceptance. Changes to this evidence file alone need no additional Rust or Windows CI.

**Scoped disposition:** 3B.2i-h focused source/format/native Windows acceptance is **COMPLETE**. Direct child termination/reap is proven for the synthetic regression; process-tree cleanup and every real-world preflight corner case are not proven. Core/TUI authority, permissions, and installed version unchanged. Phase 3B.2 is still open, as are the independent Windows Node/Rust native LVO profiles, true separate-program TUI/app-server restart acceptance, independent Validator, and official release gates. No production install, merge, 0.4.5 thaw, version bump, or release authorized.
