# CodexDD 0.3.7 Worker/Designer Integration Guide

CodexDD 0.3.7 extends the 0.3.2 Worker/Designer contract with a lifecycle-aware Implementation floor. The accepted complexity class remains authoritative for source-changing implementation work, but it no longer pins the Worker to that floor after implementation is complete and only mechanical validation remains.

The 0.3.2 guide remains the baseline for Worker binding, first-turn reconnaissance, complexity classification, independent review, failure pressure, and model-family capability guards. This document defines the 0.3.7 lifecycle additions.

## Implementation lifecycle

A bound Implementation Worker now has three observable lifecycle states:

1. **Reconnaissance** — Luna Low, read-only, until the structured complexity report is accepted.
2. **Implementation** — source-changing work at the accepted complexity floor or a stronger route justified by existing trusted pressure.
3. **Mechanical validation** — tests/checks/builds/diff/evidence after source-changing implementation is complete.

The complexity classes and source-changing floors remain unchanged:

| Complexity | Implementation floor |
| --- | --- |
| Routine | Luna Low |
| Standard | Luna High |
| Complex | Sol Low |
| Architectural | Sol Medium |

An Architectural task therefore still receives Sol Medium while it is doing architectural reasoning or source edits.

## Entering mechanical validation

When source-changing implementation is complete and the remaining work is mechanical validation, the Implementation Worker reports:

```text
kind=mechanical_validation
```

The signal carries no evidence refs or diagnostic note. The Worker ends that turn after reporting it.

CodexDD accepts the transition only for a bound Implementation Worker with an accepted complexity class, with no unfinished-turn pressure and no native failure pressure on that transition turn.

After acceptance, CodexDD starts the successor attempt at the budget-aware mechanical route:

| Budget mode | Mechanical validation route |
| --- | --- |
| Conserve | Luna Low |
| Balanced | Luna Medium |
| Surplus | Luna High |

Mechanical validation never spends into Sol or Astra by itself.

This is a deliberate de-escalation from the implementation floor. It is not a downgrade of the task's accepted complexity class; the class remains persisted so CodexDD can restore the correct implementation floor if source changes become necessary again.

## Mechanical validation gate

Mechanical validation is not permission to edit source at a cheaper model.

Core marks mechanical-validation turns and enforces a source-edit gate before tool execution. The phase permits:

- read/search/diff inspection already permitted by the reconnaissance read-only classifier;
- bounded test/check/build commands such as `cargo test`, `cargo check`, `cargo build`, and `cargo clippy`;
- `cargo fmt --check` but not a formatting write;
- selected equivalent test/build commands for supported toolchains.

Source-editing tools and commands are blocked. For example, `apply_patch`, `Set-Content`, `Remove-Item`, `git add`, arbitrary Python write scripts, `cargo run`, and plain `cargo fmt` are not authorized in mechanical validation.

If validation reveals that source changes are required, do not work around the gate.

## Returning to implementation

A mechanically validating Implementation Worker that discovers required source changes reports:

```text
kind=implementation_work
```

and ends the turn before editing.

CodexDD then restores the successor route to at least the previously accepted complexity floor. If the current route is already stronger because of trusted capability/failure handling, that stronger route is preserved rather than reduced.

For example:

```text
Architectural + Conserve
Sol Medium implementation
-> mechanical_validation
Luna Low mechanical validation
-> test reveals source repair is required
-> implementation_work
Sol Medium implementation (or stronger if already justified)
```

After the source repair is complete, the Worker can report `mechanical_validation` again.

## Failure and capability safeguards

Lifecycle de-escalation does not replace or bypass the existing adaptive controller.

- A mechanical-validation transition is rejected while native failure pressure is live on that turn.
- Unfinished pressure must be zero before entering mechanical validation.
- Mechanical work can still accumulate normal unfinished/failure pressure.
- Crossing Luna -> Sol or Sol -> Astra because of capability pressure still requires the existing trusted capability-report path.
- `implementation_work` may restore a previously authorized complexity floor without a new capability report because the complexity authorization already exists.
- A stronger route that was already justified is not discarded when source-changing work resumes.
- Pause, user interruption, workflow terminals, successor admission, and independent review behavior remain authoritative.

## Workflow terminal remains unchanged

Mechanical validation is still part of the bound Implementation assignment.

When the assignment is fully complete, report:

```text
kind=ready_for_validation
```

For Standard, Complex, and Architectural work, the normal fresh independent Validation Worker handoff remains unchanged. Validation/review floors remain budget-aware and independent of the Implementation lifecycle.

## Status expectations

An Architectural Implementation Worker in active source-changing work can show:

```text
Budget mode: Conserve
Complexity: Architectural
Implementation phase: Implementation
Implementation floor: Sol Medium
```

After a successful mechanical transition in the same budget state:

```text
Budget mode: Conserve
Complexity: Architectural
Implementation phase: Mechanical validation
Implementation floor: Luna Low
```

The complexity class remains Architectural; only the active lifecycle floor changes.

## Designer guidance

Designers should continue to define the bounded problem, authority, constraints, and acceptance criteria. They should not manually select the lifecycle route or model effort.

Do not instruct a Worker to stay on Sol for tests simply because the implementation was Architectural. Do not instruct a Worker to edit source while in mechanical validation. The Worker should use the structured lifecycle signals and let CodexDD route the successor turn.

The preferred assignment still begins with:

```toml
[adaptive_worker]
role = "implementation"
authorized_scope = "<exact bounded task>"
```

Then state the requirements and acceptance criteria and instruct the Worker to follow the CodexDD adaptive workflow.

## Source of truth

The 0.3.7 lifecycle behavior is implemented primarily in:

- `codex-rs/tui/src/adaptive_budget.rs`
- `codex-rs/tui/src/adaptive_complexity.rs`
- `codex-rs/tui/src/adaptive_policy.rs`
- `codex-rs/tui/src/chatwidget/adaptive_effort.rs`
- `codex-rs/tui/src/chatwidget/adaptive_runtime_bridge.rs`
- `codex-rs/tui/src/chatwidget/adaptive_trusted_signal.rs`
- `codex-rs/tui/src/chatwidget/adaptive_admission.rs`
- `codex-rs/tui/src/app/thread_routing.rs`
- `codex-rs/core/src/tools/handlers/adaptive_signal.rs`
- `codex-rs/core/src/tools/registry.rs`

When documentation and runtime behavior disagree, the merged runtime and regression tests are authoritative.
