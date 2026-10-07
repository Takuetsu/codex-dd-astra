# CodexDD Worker/Designer Integration Guide — Current

**Status:** CURRENT for the CodexDD 0.4.0 source line.

The source product identity is **CodexDD 0.4.0**. Installed workstations may remain on an earlier production build until the normal release/install gate completes, so verify the active runtime with `codexdd --version` before making version-specific claims.

The runtime and its regression tests are authoritative if this prose falls behind.

## Responsibility split

The Designer supplies the Worker role, exact bounded `authorized_scope`, requirements/acceptance criteria, project invariants, and objective completion evidence.

The Designer does **not** choose the Worker model family or reasoning effort, pre-classify complexity, guess quota state, direct arbitrary escalation, or treat final prose as workflow authority.

CodexDD owns adaptive routing inside the bound role/scope. Project governance still owns product scope, Owner QA, acceptance, merge, and closure.

## Required first assignment

A fresh bound Worker begins with:

```toml
[adaptive_worker]
role = "implementation"
authorized_scope = "<exact bounded task scope>"

```

Assignment prose follows the required blank line.

Supported roles are `implementation`, `validation`, and `repair`. The first nonblank content must be `[adaptive_worker]`; scope and assignment body must be nonempty; malformed authority headers fail closed. Current runtime also normalizes bracketed-paste framing without weakening this contract.

## Fresh-Worker discipline

A Worker role is a context boundary.

- Do not repurpose an authoring Worker as the independent validator.
- Do not repurpose a validator as the repair author.
- Use a fresh Worker for a new role or separately bounded repair class when project governance requires independence.
- Durable authority belongs in project records and repository state, not Worker prose.

## Adaptive pressures

CodexDD keeps four pressures separate:

1. **Complexity pressure** chooses the bounded implementation floor.
2. **Failure pressure** reacts to objective failed attempts.
3. **Quality pressure** allocates independent review/validation.
4. **Budget pressure** adjusts review and mechanical-validation spend from live rate-limit information.

Describe the real work accurately and let the runtime route it.

## Initial implementation reconnaissance

The first turn of a bound Implementation Worker is protected reconnaissance. The Worker inspects repository state, makes no source edits, estimates the implementation surface, reports the trusted complexity signal, and then continues at the resulting floor.

The runtime enforces the read-only reconnaissance boundary at tool dispatch.

Current complexity inputs include estimated files, cross-module impact, public API/data-model impact, persistent state/serialization, concurrency/async, build/release/toolchain, uncertain root cause, and broad test surface.

| Complexity    | Implementation floor |
| ------------- | -------------------- |
| Routine       | Luna Low             |
| Standard      | Luna High            |
| Complex       | Sol Low              |
| Architectural | Sol Medium           |

Complexity may raise the starting floor; it does not authorize arbitrary mid-attempt jumps and never starts directly on Astra.

## Implementation lifecycle and mechanical validation

CodexDD distinguishes source-changing implementation from protected mechanical validation.

After source-changing work is explicitly complete, mechanical validation may de-escalate below the complexity floor. Current budget-aware routes stay within Luna:

| Budget mode | Mechanical-validation route |
| ----------- | --------------------------- |
| Conserve    | Luna Low                    |
| Balanced    | Luna Medium                 |
| Surplus     | Luna High                   |

Mechanical validation may run bounded tests/checks/builds and read-only inspection but does **not** authorize source edits. The runtime tool gate enforces this. If evidence requires more source work, use the trusted lifecycle transition back to implementation rather than bypassing the gate.

### Local Validation Orchestrator — 0.4.0

CodexDD 0.4.0 makes normal post-implementation local validation runtime-owned rather than model-authored or operator-typed.

During protected mechanical validation, the Implementation Worker uses the native `run_codexdd_validation` tool with one fixed repository-owned profile:

- `work_packet` — normal post-implementation gate;
- `targeted` — focused retest after an admitted repair;
- `release` — explicit broad pre-CI/pre-install validation; never silently selected for an ordinary work packet.

The runner resolves conventional repository scripts, executes them through Codex's existing local execution/sandbox path, records full logs outside the source worktree, and emits a compact native receipt. It does not accept arbitrary model-generated shell text.

For Implementation handoff, `ready_for_validation` requires a successful same-turn native `work_packet` receipt for the current candidate. After any admitted repair, the same mechanical-validation turn must first pass `targeted` and then `work_packet`.

Repair authorization is also evidence-bound:

- the initial automatic repair must originate from a failed `work_packet` run;
- infrastructure or contract failures do not authorize source edits;
- at most two source-changing repair cycles are admitted for the active work packet;
- a third repair request deterministically blocks the workflow for owner review;
- changing failed stages does not reset the repair budget.

The existing `/status` surface reports the latest LVO profile/result, native run ID, candidate branch/SHA, failed stage, active failure fingerprint, repair budget, deterministic next validation step, and log path. Model prose is never validation authority.

## Completion and trusted terminals

Final-answer prose is not workflow authority.

Implementation and Repair completion use `ready_for_validation`.

Validation completion uses:

- `ready_for_owner_qa` for green evidence; or
- `repair_required` for an evidence-backed candidate defect.

The runtime accepts the trusted structured adaptive signal, not prose that merely sounds complete.

## Independent validation

For eligible nontrivial `ready_for_validation` work, CodexDD can create/bind a fresh independent Validation Worker according to runtime policy. The normal path does not require a Designer to hard-code `/new validation`.

Project governance may still require an independent validator when its gate is stricter than generic CodexDD policy.

Validation should verify the exact candidate, inspect independently, run required proof, remain read-only for code/tests by default, and return `ready_for_owner_qa` or `repair_required`. A validator finding a defect does not become the repair author when independence is required.

## Budget-aware review

CodexDD derives budget mode from live rate-limit windows. Do not encode guessed quota state in prompts.

| Budget mode | Independent-review floor |
| ----------- | ------------------------ |
| Conserve    | Luna High                |
| Balanced    | Sol Low                  |
| Surplus     | Sol High                 |

Budget surplus is spent on review quality before being treated as permission for implementation escalation.

## Failure/capability escalation

Failure pressure remains separate from complexity, quality, and budget pressure.

Current guardrails include objective-failure-driven pressure, no arbitrary complexity/budget model jumps, trusted capability-report gating at family boundaries, and authoritative pause/off/user interruption.

Do not write prompts intended to bypass these guards.

## Validation terminalization — 0.3.13

CodexDD 0.3.11–0.3.13 hardened Validation completion.

In current 0.3.13, a dedicated Validation terminalization turn is used when conclusive evidence exists but the native terminal still must be committed. On that turn:

- only trusted `report_adaptive_signal` is authorized;
- unrelated tools are blocked;
- existing native evidence references are reused;
- exactly one trusted terminal is expected: `ready_for_owner_qa` or `repair_required`.

This supersedes older operator workarounds for manually chasing a missing Validation terminal.

## Worker completion footer

CodexDD 0.3.10+ records cumulative Worker active-turn duration and renders a runtime-owned completion/DONE footer for terminal Workers. The footer is operational evidence; it does not replace project acceptance, source-control rules, or the trusted workflow terminal.

## Windows / PowerShell validation

CodexDD 0.4.0 repositories opt into deterministic local validation with repository-owned PowerShell profiles such as:

- `scripts/codexdd-test-targeted.ps1`;
- `scripts/codexdd-test-workpacket.ps1`;
- `scripts/codexdd-test-release.ps1`.

Routine Implementation mechanical validation should use the native LVO runner rather than reconstructing those commands manually. Protected mechanical validation still permits bounded read-only/diagnostic operations and does not grant a general source-edit bypass.

If a protected-phase command is rejected, treat it as tooling evidence and do not weaken the protection merely to continue editing. Operator-run PowerShell remains appropriate for explicit owner validation gates, release-shaped environment checks, or transitional validation of a candidate runtime that is not installed yet.

## Operator guidance

Normal use:

1. launch the installed `codexdd`;
2. start a fresh Worker context;
3. submit the bound first assignment;
4. let CodexDD own adaptive routing;
5. inspect runtime status only for diagnosis;
6. trust structured terminals over prose.

Avoid routine instructions that manually choose model/effort, preselect complexity, force `/adaptive` family preferences, manually create the normal validator when runtime/project policy already supplies one, relaunch the app between ordinary Worker contexts, or rerun green work merely to manufacture a terminal.

## Version verification and source truth

Before version-specific diagnosis, run `codexdd --version` and distinguish the **installed launcher/binary identity** from a **source checkout HEAD** that may be newer.

Primary runtime source areas include:

- `codex-rs/tui/src/adaptive_budget.rs`
- `codex-rs/tui/src/adaptive_complexity.rs`
- `codex-rs/tui/src/adaptive_policy.rs`
- `codex-rs/tui/src/adaptive_worker.rs`
- `codex-rs/tui/src/chatwidget/adaptive_effort.rs`
- `codex-rs/tui/src/chatwidget/adaptive_runtime_bridge.rs`
- `codex-rs/tui/src/chatwidget/adaptive_signal_transport.rs`
- `codex-rs/tui/src/chatwidget/adaptive_trusted_signal.rs`
- `codex-rs/core/src/tools/handlers/adaptive_signal.rs`
- `codex-rs/core/src/tools/registry.rs`

When this guide and merged runtime/tests disagree, runtime/tests win and this guide should be updated.

## Version-history boundary

This is replacement-style CURRENT guidance. Release-era quirks and retired workarounds belong in release commits, issues/backlog records, or historical guides.

`docs/codexdd-0.3.2-worker-designer-integration.md` is preserved as historical 0.3.2 documentation and is not current operator authority.
