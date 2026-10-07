# CodexDD roadmap

## 0.4.0 — Enhancement line

CodexDD 0.4.0 begins from the soaked 0.3.14 production baseline:

- production branch: `dd/astra-policy-v2`
- baseline SHA: `06465383abb1acf7c74c0a84b00726853b74fdc4`
- baseline version: `0.3.14`
- target platform: Windows

0.4.0 is the next enhancement release line. It should not be treated as another patch-level whack-a-mole release.

### Working standard

Larger 0.4.0 work follows the CodexDD major-phase + numbered work-packet standard.

Each work packet should remain bounded enough for one agent run, with explicit acceptance criteria and a clean handoff. Local cheap validation should happen before expensive CI. Upstream synchronization, feature implementation, and release validation should remain separate work packets rather than being silently bundled together.

### First enhancement — Local Validation Orchestrator

The first planned 0.4.0 enhancement is the Local Validation Orchestrator.

#### Goal

Move routine deterministic local validation from Daniel to CodexDD.

A normal CodexDD work packet should be able to complete implementation, run the appropriate local validation, repair bounded in-scope failures, and emit concise evidence without requiring Daniel to manually enter a sequence of PowerShell commands.

#### Intended flow

1. Worker/Designer completes a bounded implementation packet.
2. CodexDD performs branch/worktree sanity checks.
3. CodexDD selects the appropriate validation profile.
4. CodexDD runs deterministic local shell commands directly.
5. If validation fails, CodexDD classifies the failure and may perform a bounded repair/retest loop.
6. CodexDD captures branch, SHA, test, build, lint, and worktree evidence.
7. CodexDD emits READY_FOR_VALIDATION or a concise blocked result.
8. Daniel remains the human gate for consequential actions.

#### Validation profiles

Provide deterministic repo-owned validation profiles/scripts with at least these conceptual levels:

- **targeted** — fastest tests relevant to the files/components changed in the current work packet.
- **work-packet** — targeted tests plus format/lint/build and nearby regression coverage.
- **release** — broad local pre-CI regression validation intended to catch failures before CI or soak.

Prefer stable repo scripts/recipes over ad-hoc model-generated command sequences where practical.

Conceptual Windows interface:

```powershell
.\scripts\codexdd-test-targeted.ps1
.\scripts\codexdd-test-workpacket.ps1
.\scripts\codexdd-test-release.ps1
```

The exact implementation should be selected during the 0.4.0 reconnaissance/design packet.

#### Automatic validation stages

The orchestrator should be able to run, as applicable:

- branch/worktree sanity checks
- git status / diff sanity checks
- targeted unit/integration tests
- formatting and lint checks
- compile/build checks
- broader regression tests
- final clean/expected worktree verification
- SHA + branch + validation evidence capture

#### Bounded repair loop

When a deterministic validation step fails:

1. classify the failure;
2. inspect only the relevant failure output/log region;
3. make an in-scope source/config/test fix when appropriate;
4. rerun the smallest affected validation;
5. expand back to the required validation profile after the focused test passes.

Automatic repair must be bounded. Initial target: no more than 2–3 automatic repair iterations per failure class/work packet. If the failure persists, stop and report rather than running indefinitely.

#### Context and token discipline

Do not dump full compiler/test logs into model context by default.

- Save verbose logs to disk.
- Return concise structured summaries to the agent.
- On failure, surface only the relevant error/failure section plus a reference to the complete log.
- Use model reasoning for diagnosis/repair rather than routine command orchestration.

#### Evidence contract

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

#### Human gate

The Local Validation Orchestrator must not silently remove explicit approval/workflow boundaries around:

- CI promotion where the project uses a human gate
- squash/merge
- production installation
- release publication
- destructive Git operations
- deployment or changes to Doug's machine or other external machines
- other production-impacting actions

#### Acceptance criteria

- A normal work packet can complete implementation plus deterministic local validation without Daniel manually entering a sequence of shell commands.
- Validation profiles are repeatable and repo-owned.
- Failures can trigger bounded diagnose/fix/retest behavior.
- Runaway repair loops are prevented.
- Full logs are retained without flooding agent context.
- PASS/FAIL evidence includes branch/SHA and validation-stage results.
- Existing human approval gates remain intact.
- The feature is tested on Windows, the CodexDD target platform.

### Upstream refresh policy for 0.4.0

Do not silently combine an upstream Codex jump with an unrelated enhancement packet.

If 0.4.0 adopts a newer upstream stable release, perform that integration as its own numbered work packet with its own reconnaissance, compatibility review, cheap local validation, CI, and soak evidence. Select the exact upstream stable target when the packet is activated.

### Preserved backlog

The following items remain valid but are not implicitly part of the first 0.4.0 implementation packet:

- Windows release packaging / tagged release automation originally explored in PR #13.
- Any later upstream Codex stable refresh beyond the current 0.159.2-based production line.
- Other enhancement backlog items already documented in CodexDD planning material.

### Activation

When 0.4.0 work is explicitly activated, branch implementation from the frozen 0.3.14 production baseline above and begin with the first bounded reconnaissance/design work packet for the Local Validation Orchestrator.
