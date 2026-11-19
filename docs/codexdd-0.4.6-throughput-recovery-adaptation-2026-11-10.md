# CodexDD 0.4.6 — Workflow adaptation from BREAKWATER throughput recovery (2026-11-10)

**Type:** Process/governance-only checkpoint. No runtime changes, test waivers, release/install/merge, new BREAKWATER work, or frozen 0.4.5 work.
**CodexDD implementation branch at review start:** `347482a4929ccf0354b6307214543261d2f5003d` (`dd/codexdd-v0.4.6-agent-reliability`). Production 0.4.4 remains unchanged.
**Source authorities:** BREAKWATER `AGENTS.md` updated by commit `f90dc870d0443fbac565a882a63034529f8d12c6`, merged on `main` by PR #96 / commit `95b49d711e50e610cee453fef640eeae2b207fc3`; CURRENT 2026-11-10 top entries in Google Drive `BREAKWATER — Project Operations and Governance` and `BREAKWATER — AI Execution Control Plane`. Earlier staged Workflow V3 (2026-10-15) remains its validation-cadence basis.

## Transferable lessons, applied here

1. **One outcome, not one long conversation:** CodexDD packets now define one testable agent-reliability result or evidence-backed defect. A compact assignment card identifies exact authority/SHA, owning subsystem, expected behavior, proof, non-goals and STOP condition. Prior large chat histories and broad exploratory changes are not task authorization.
2. **Classify evidence first (A/T/E/U/B):** Before changing source, distinguish confirmed in-scope product/security/persistence failures (A), test/oracle/fixture/harness failures (T), environment/tool/profile/permission failures (E), missing evidence (U), and unrelated backlog (B). Keep prior red evidence durable.
3. **Two-attempt no-progress breaker:** If two same-objective attempts yield no new discriminating evidence, do not repeat GitHub Actions or the same diagnostic. Stop the loop, report the missing fact, and choose a different bounded measurement or explicit owner decision. Reproducing a failure with new stage evidence is useful progress, not a reason to suppress further diagnosis.
4. **Staged proof:** Focused cheap local tests and necessary cross-boundary negatives during implementation/repair; source-bound evidence reuse; fresh scoped independent review before acceptance; preliminary early real-agent smoke only in an owner-authorized disposable environment. Final whole-candidate Windows release, security, daemon/resume and full regression gates remain mandatory at the final release decision. Do not repeatedly pay for the full suite per routine source correction.
5. **Real delivery metrics:** Count functioning task completion, accurate BLOCKED instead of false READY, fixed P0 failures, owner interventions and repeated unproductive loops. GitHub job count, tokens, commits and long reports do not constitute usability.

## Specific application to current 3B.2i-e blocker

**Last observed exact-head Windows run:** GitHub Actions #42825784600, development branch HEAD `347482a4929ccf0354b6307214543261d2f5003d`. Source preflight, Rustfmt and existing focused regressions passed. The real-process candidate/cold-resume case failed during candidate Git preflight with `SETUP_BLOCKED: candidate Git preflight exceeded 20 seconds`. The trace shows persistence permit acquired and Core state mutex released before the Git task; no successful source promotion, checkpoint flush, real-process resume acceptance or release can be claimed.

**Classification:** A product-path timeout is observed, but root causation between a specific Git subprocess, fixture/worktree, Windows environment and blocking-task cancellation remains **U/E pending discrimination**. Do not call it a proven particular Git command defect merely because the timeout expired.

**Next bounded discriminating packet:** Instrument the individual read-only Git child steps (reported canonical root, HEAD, porcelain status, ancestry) with sanitized start/completion/error timing and an actual child-process deadline/cleanup. Distinguish a hanging subprocess from a slow async runtime; a timeout around `spawn_blocking` alone does **not** kill an already running Git child. In a disposable checkout, reproduce with one focused test and complete any feasible cheap Daniel-CL test before dispatching another broad GitHub run. If the environment cannot support a targeted local test, document the exact blocker and obtain the GitHub-only exception required by existing CodexDD policy. Keep safety/security/persistence checks fail-closed.

## Explicitly not adopted from BREAKWATER

- Z-A0.58's **standard Codex CLI only** executor restriction; it applies only to that BREAKWATER milestone. The CodexDD project must not silently rewrite its own runtime identity or claim CodexDD is presently fit for BW58.
- Z-A0.58's Owner QA pause, game milestones, ports, playable-game proof, gate activation and frozen BW56 conditions as authorization for CodexDD work.
- Any waiver of CodexDD's Daniel-CL-first focused validation, immutable Worker scope/SHA, independent Validation/Repair roles, native release profile, 21-stage Windows gate when applicable, security/daemon/F2 tests, staged promotion or explicit owner signoff.

**Changes made by this packet:** `AGENTS.md` and `docs/codexdd-major-phase-work-packet-standard.md` (process only). The source branch retains all current 0.4.6 numbered phase requirements; this packet does not close 3B.2i-e or authorize the next game gate.
