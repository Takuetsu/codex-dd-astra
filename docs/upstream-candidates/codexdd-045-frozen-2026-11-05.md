# CodexDD 0.4.5 release freeze — 2026-11-05

**OWNER HOLD / NONRELEASE. Do not merge, install, promote, or restart expensive 0.4.5 CI.**

Active agent usability on 0.4.4 takes priority. Preserve all source, draft PRs, test artifacts, failing logs, semantic review holds and 0/109 pending formal owner approvals. Do not infer approval from a green source audit.

| Workstream | Checkpoint | Result at freeze |
| --- | --- | --- |
| 3F.93 (PR #83) | `b8effbfd79bb638d5df980a6a962ee2e6f85559d` | Windows process/cold-resume 3/3 GREEN, nonrelease |
| 3F.94 (PR #84) | `9bf7bbb467009f67fb8372fb155e7e2db342c66a` | 0/2 accepted; hosted Windows job object denies required daemon breakaway (Win32 5). Source hold audit #42097465527 GREEN; task attempts 267011 not executed |
| 3F.95 (PR #85) | `0d9effb538142fc7c0fb9f3a4dffca6d451c527b` (test snapshot) | Windows run #42099189133 FAILED after app-server compile; TUI test has E0308 UserMessage type and E0277 anyhow error conversion, real TUI process test not run |

3F.95 workflow is frozen by removing its push trigger and adding unconditional disabled job guards. Reverting those guards needs a separate owner-approved unfreeze. 3F.94 native expensive CI already gated on a dedicated disposable Windows runner and explicit opt-in. Do not modify or repurpose these branches while hotfixing.

**Release holds:** production 0.4.4; 4 Core + 9 TUI high-risk owner reviews; 0/109 formal owner signoffs. No 0.4.5 release readiness or installation.

**Stability response:** start an isolated 0.4.4-targeted hotfix from current production `dd/astra-policy-v2`; record installed versions/observed agent failures, prioritize reproducible active-agent blockers, preserve project worktrees; cheap focused Daniel-CL local validation before GitHub CI, then deployment with rollback and owner-confirmed live smoke.

**Unfreeze prerequisite:** recover active agents first, port verified hotfixes forward with focused behavior tests, revisit 3F.94 dedicated-runner acceptance, correct/validate 3F.95, and complete the unwaived owner reviews/approval process.
