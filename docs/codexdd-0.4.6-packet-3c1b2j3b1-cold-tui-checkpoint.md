# CodexDD 0.4.6 — 3C.1b.2j3b1: cold embedded app-server + TUI gate checkpoint

**Opened:** 2026-11-13. **Parent:** 3C.1b.2j3a source-origin replay refusals **ACCEPTED** at `25d0ebc5db9af7fc2af07d968a633033317b5d12` ([Actions #43440575225](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43440575225), 2 passed). **Status:** focused native Windows validation PENDING. 3C.1b.2j3, 3C.1 and the 0.4.6 release remain OPEN.

## Specific evidence gap

The TUI's `restore_persisted_workflow_state` and `AdaptiveGateLedger::restore_pending` already refuse to treat saved gate requirements as live native PASS. Earlier acceptance covered durable Core first-user Worker identity on a restarted process (3C.1b.2g), and the App rejected duplicate source-origin handoffs (j3a), but the combination had not verified **a freshly spawned embedded app-server mapping a persisted TUI Validator gate checkpoint after process restart**, followed by source drift and removed-root refusal.

This bounded packet does **not** change production source code or create fictional first-user Core authority. It uses a disposable persisted TUI `ThreadAdaptiveWorkflowState` as an existing saved Validator checkpoint; the authoritative initial Core first-user origin and true native receipt production remain separately proven by earlier native real-process tests. The new test is for the *cross-process TUI checkpoint rehydration boundary*.

## Exact source-pinned test

`codexdd_046_3c1b2j3b1_real_cold_resume_keeps_gates_pending_and_refuses_source_drift` in `codex-rs/tui/src/app_server_session.rs`:

1. Create a disposable Git checkout, commit a source SHA, and a separate ephemeral Codex home/rollout with a saved independent Validator thread.
2. In the **first real embedded app-server process**, persist an existing source-bound Validation Worker checkpoint with original owner-required `work_packet` plus independent-review gates and a historical `ready_for_owner_qa` terminal; do not fabricate any source/independent PASS receipt.
3. Stop the first process; spawn a **second embedded app-server process** and invoke actual `thread/resume` and TUI session-state mapping. Require same thread, exact source/root/scope/role, **both gates pending**, empty evidence registry, no successor/signal/origin resurrection, and the prior READY downgraded to paused/no terminal.
4. Stop again, advance the checkout to a different HEAD, start another fresh process and resume. The TUI may display the old authorized pin for historical inspection, but it must never promote it to current HEAD or READY, and the existing native source preflight must refuse validation of the changed checkout.
5. Stop again, remove the worktree and start a final process. A missing original owner root must refuse gate-checkpoint restoration, not recover trusted Validator authority.

A source-contract preflight + Rustfmt + one Windows `codex-tui --lib` test using the exact `[3c1b2j3b1-native]` marker is required for acceptance. All processes and Git state are isolated disposable test fixtures. No production binaries or game worktrees are changed.

## Still open after j3b1

**3C.1b.2j3b2:** full coupled real App event loop / genuine original first-user source → Validator app-server cold restart including separate Core-native receipts and complete no-duplicate READY admission, plus any actual process/replay state defects exposed. This is required before closing the whole 3C.1b.2j3 boundary. Remaining 3C.2, 3B.4, 3D–3F and Phase 4 native Windows release gates must finish before owner-authorized installation and the planned one-milestone, approximately one-week BREAKWATER soak.
