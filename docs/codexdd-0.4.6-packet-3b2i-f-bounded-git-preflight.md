# CodexDD 0.4.6 — Packet 3B.2i-f: bounded Git preflight and focused-CI cadence

**State:** Source checkpoint for targeted validation, **not** a validated functional repair or milestone acceptance.
**Parent commit:** `2ea8ec853f5d2f7bec4457bcf9b12b6bbf8617de` on `dd/codexdd-v0.4.6-agent-reliability`.
**Prior evidence:** #42825784600 on `347482a4929ccf0354b6307214543261d2f5003d`: source preflight, Rustfmt and existing focused tests passed; real app-server candidate test failed before promotion with `SETUP_BLOCKED: candidate Git preflight exceeded 20 seconds`. Trace proved the persistence permit was acquired and the Core state mutex was released. The specific child process causing the delay remains **unverified**.

## Single outcome and owning layer

**Outcome:** Every read-only Git operation in Core's explicit candidate-SHA verifier is bounded, reports the exact failing stage, and does not retain a hung direct Git child after timeout. Never accept or promote a candidate on an incomplete Git check.

**Owning layer:** `codex-rs/core/src/session/worker_authority.rs` for subprocess inspection; `codex-rs/core/src/session/thread_settings.rs` for the outer async safety budget.

**Classification:** U/E (precise command still unknown, potential subprocess/piped-output stall), with a confirmed user-visible authorization timeout. Do not claim a particular Git command or root cause without a new discriminating observation.

**Scope:** Replace `Command::output` and `Command::status` in this *candidate verifier only* with child polling, per-step deadlines, direct-child kill/wait and bounded scratch-file stdout. Keep Git commands and comparison semantics: exact canonical root, exact HEAD, clean porcelain status (including untracked edits), ancestor of previously pinned SHA. Disable interactive credential prompting. Output is neither written into the source worktree nor logged with user path or changes. A failed or slow Git step returns `SETUP_BLOCKED` including `root`, `head`, `status`, or `ancestry`; no persistence/promotion/READY follows.

**Bounds:** 5 seconds for metadata/ancestry operations, 12 seconds for full untracked status, 35 seconds for the outer asynchronous task; no new unbounded blocking subprocess. An outer timeout does not guarantee cancellation of an unstarted blocking task, and terminating the direct child does not prove every descendant was killed; test/process-level evidence remains required.

**Focused proof:** One explicit child-process timeout/reap negative test `codexdd_046_3b2p_*`. Existing candidate dirty/unbound/lineage tests, Core-to-TUI candidate synchronization, and the real-process candidate/cold-resume test remain acceptance requirements. On an isolated throwaway Linux Git checkout, the four *unchanged* inspection commands with `--no-optional-locks` succeeded; that is a command-compatibility smoke, not a Windows Rust compile or app-server run.

## Changed CI trigger policy

Development pushes now perform only exact-source contract preflight and Rustfmt. The expensive Windows Core/TUI/app-server compile, regressions and real-process cold-resume test run only on an explicitly dispatched workflow (`workflow_dispatch`) after relevant focused validation and the existing Daniel-CL/local-first or approved GitHub-only exception gate. This removes the previous automatic 45–70 minute Windows compile after every repair commit but does not delete or waive the CI gate.

The manual full workflow includes the new `codexdd_046_3b2p_` timeout negative test. A skipped Windows job on a development push is **PENDING/NOT RUN**, not a PASS.

## Next step and STOP conditions

1. Review this exact source and the cheap source/Rustfmt checks; correct any source or formatting issue without running broad Windows CI.
2. Obtain focused Windows evidence in an approved isolated environment. Verify the new timeout case and real app-server candidate handoff, checking whether the next log names `root`, `head`, `status`, or `ancestry`, or whether it passes. Only then approve a deliberate final scoped Windows CI dispatch.
3. If the same objective produces no new discriminating evidence in two attempts, stop and request a different observation/environment or owner decision rather than scheduling another identical run.
4. Remove the temporary 3B2e probe warnings after root cause and real-process acceptance are established.

**Non-goals:** No repairs to BREAKWATER or Cycle Vengeance, no source reset/checkout, no privilege broadening, no loosening of native source/Worker authority, no freeze thaw of 0.4.5, no production merge/install/release. The independent Windows/daemon/F2/Node/Rust final release gates remain separate.

## Owner-requested focused Windows CI invocation (2026-11-10)

The owner requested native Windows tests of the bounded Git subprocess and actual app-server candidate/cold-resume path. The connected GitHub tools can inspect/rerun old jobs but cannot invoke `workflow_dispatch` on the new source revision; no assistant-controlled SSH connection to Daniel-CL is available in this session. Prior owner direction explicitly kept this phase's integration work on GitHub. This is a narrowly scoped **GitHub-only test execution**, not a new release/production validation exception.

To avoid a throwaway workflow or the existing 40–70-minute full-suite push loop, the existing 3B.1 workflow now has a reusable, opt-in `windows-3b2-native-focused` job. It runs only when a development branch commit contains the exact marker `[3b2-focused-native]`, after source preflight and Rustfmt. The job compiles and runs exactly the `codexdd_046_3b2p_` Core bounded-child regression and `codexdd_046_3b2n_` real-process app-server candidate/cold-resume acceptance, on `windows-2022` in a disposable checkout with repository-pinned Rust/V8 setup. The broad `windows-rust` job remains manually gated; no release or installation.

If the first test passes but the real-process test fails, inspect the exact stage-specific `SETUP_BLOCKED` message before changing runtime code. A green focused result is necessary evidence but not the final Windows, security or independent Validator acceptance. Do not confuse source-format success, skipped Windows jobs or a dry-run receipt with native PASS.
