# CodexDD roadmap

## 0.3.15 — GPT-5.1 Sol agent/model update

0.3.15 is reserved for incorporating the new GPT-5.1 Sol agent/model path and any compatibility work required by that update.

The Local Validation Orchestrator must not be implemented in parallel with 0.3.15. Its design and implementation should target the completed 0.3.15 behavior to avoid rework.

## 0.3.16 — Local Validation Orchestrator

### Goal

Move routine deterministic local validation from Daniel to CodexDD.

A normal CodexDD work packet should be able to complete implementation, run the appropriate local validation, repair bounded in-scope failures, and emit concise evidence without requiring Daniel to manually enter a sequence of PowerShell commands.

### Intended flow

1. Worker/Designer completes a bounded implementation packet.
2. CodexDD performs branch/worktree sanity checks.
3. CodexDD selects the appropriate validation profile.
4. CodexDD runs deterministic local shell commands directly.
5. If validation fails, CodexDD classifies the failure and may perform a bounded repair/retest loop.
6. CodexDD captures branch, SHA, test, build, lint, and worktree evidence.
7. CodexDD emits READY_FOR_VALIDATION or a concise blocked result.
8. Daniel remains the human gate for consequential actions.

### Validation profiles

Provide deterministic repo-owned validation profiles/scripts, with at least these conceptual levels:

- targeted: fastest tests relevant to the files/components changed in the current work packet.
- work-packet: targeted tests plus format/lint/build and nearby regression coverage.
- release: broad local pre-CI regression validation intended to catch failures before CI or soak.

Prefer stable repo scripts/recipes over ad-hoc model-generated command sequences where practical.

Conceptual Windows interface:

```powershell
.\scripts\codexdd-test-targeted.ps1
.\scripts\codexdd-test-workpacket.ps1
.\scripts\codexdd-test-release.ps1
```

The exact implementation should be chosen after the 0.3.15 compatibility review.

### Automatic validation stages

The orchestrator should be able to run, as applicable:

- branch/worktree sanity checks
- git status / diff sanity checks
- targeted unit/integration tests
- formatting and lint checks
- compile/build checks
- broader regression tests
- final clean/expected worktree verification
- SHA + branch + validation evidence capture

### Bounded repair loop

When a deterministic validation step fails:

1. classify the failure;
2. inspect only the relevant failure output/log region;
3. make an in-scope source/config/test fix when appropriate;
4. rerun the smallest affected validation;
5. expand back to the required validation profile after the focused test passes.

Automatic repair must be bounded. Initial target: no more than 2–3 automatic repair iterations per failure class/work packet. If the failure persists, stop and report rather than running indefinitely.

### Context and token discipline

Do not dump full compiler/test logs into model context by default.

- Save verbose logs to disk.
- Return concise structured summaries to the agent.
- On failure, surface only the relevant error/failure section plus a reference to the complete log.
- Use model reasoning for diagnosis/repair rather than routine command orchestration.

### Evidence contract

Successful validation should produce a compact result similar to:

```text
CodexDD validation: PASS

branch: <branch>
sha: <sha>

targeted:   PASS
unit:       PASS
build:      PASS
lint:       PASS
regression: PASS
git-tree:   EXPECTED/CLEAN

READY_FOR_VALIDATION
```

Failure results should identify:

- failed stage/test
- command/profile
- concise failure reason
- log/evidence reference
- repair attempts used
- whether the failure is in scope or requires escalation

### Human gate

The Local Validation Orchestrator must not silently remove explicit approval/workflow boundaries around:

- CI promotion where the project uses a human gate
- squash/merge
- production installation
- release publication
- destructive Git operations
- deployment or changes to Doug's machine or other external machines
- other production-impacting actions

### Acceptance criteria

- A normal work packet can complete implementation plus deterministic local validation without Daniel manually entering a sequence of shell commands.
- Validation profiles are repeatable and repo-owned.
- Failures can trigger bounded diagnose/fix/retest behavior.
- Runaway repair loops are prevented.
- Full logs are retained without flooding agent context.
- PASS/FAIL evidence includes branch/SHA and validation-stage results.
- Existing human approval gates remain intact.
- The feature is tested on Windows, the CodexDD target platform.
- Implementation is based on the completed 0.3.15 GPT-5.1 Sol integration rather than developed in parallel.

### Rationale

Daniel should not need to perform source edits or serve as CodexDD's shell-command executor for routine deterministic testing. CodexDD should own that work while preserving bounded execution, evidence, and human control over consequential actions.
