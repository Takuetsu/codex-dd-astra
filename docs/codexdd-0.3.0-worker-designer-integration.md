# CodexDD 0.3.0 Worker/Designer Integration Guide

This guide is the integration contract for project Designers, milestone planners, and Worker prompts that use CodexDD 0.3.0.

CodexDD 0.3.0 separates four pressures that must remain independent:

1. **Complexity pressure** chooses the bounded implementation floor.
2. **Failure pressure** reacts to objective failed attempts.
3. **Quality pressure** creates independent validation for nontrivial work.
4. **Budget pressure** adjusts review spend from live account usage.

The goal is to spend inexpensive capacity on reconnaissance and routine work, spend stronger models where they improve quality, and preserve the existing anti-runaway escalation guards.

Authoritative implementation: PR #24, merged as `95717424a5dd3c609968b09b8ea8cc2830e8363e`.

## Designer responsibilities

A Designer should define:

- the Worker role;
- the exact authorized scope;
- requirements and acceptance criteria;
- project-specific constraints and invariants;
- the objective evidence needed to consider the assignment complete.

A Designer should **not** choose the implementation model family or reasoning effort. Avoid instructions such as:

- "Use Terra Medium."
- "Start on Sol."
- "Escalate to Astra if this is difficult."
- "Run /new validation when finished."

CodexDD owns those routing decisions.

## Required first-assignment header

The first user-supplied Worker assignment must begin with this authority header:

```toml
[adaptive_worker]
role = "implementation"
authorized_scope = "<exact bounded milestone/task scope>"

```

The blank line after the header is required. Assignment prose follows after that blank line.

Supported roles are:

- `implementation`
- `validation`
- `repair`

`authorized_scope` must be non-empty and narrowly describe what the Worker is allowed to do.

The first non-blank line must be exactly `[adaptive_worker]`. The first assignment locks Worker assignment state. If an ordinary prompt is submitted first, that thread is not retroactively converted into a bound Implementation Worker. A malformed authority header fails closed.

### Recommended Designer prompt

```text
[adaptive_worker]
role = "implementation"
authorized_scope = "BREAKWATER Z-A0.41: implement the bounded session-state persistence change and its required tests only."

Implement this bounded assignment.

Requirements:
- preserve behavior outside the authorized scope;
- update the required tests;
- do not broaden the refactor beyond what is necessary.

Acceptance criteria:
- the requested behavior is implemented;
- affected tests pass;
- no unrelated files are changed.

Follow the CodexDD adaptive workflow exactly.
```

Do not tell the Worker to immediately edit a specific file. The first bound Implementation turn has a reconnaissance responsibility.

## Operator/session prerequisite

Adaptive mode must be enabled for the working session. The operator may select a preference with `/adaptive luna|terra|sol|astra`.

The preference is metadata. Every adaptive run still begins at **Luna Low**; selecting Terra, Sol, or Astra does not bypass the cheap first attempt.

Use `/adaptive status` to inspect live state.

## Phase 1: Luna Low reconnaissance

On the first Luna Low turn of a bound Implementation Worker, the Worker should:

1. inspect the relevant code and problem;
2. make no implementation edits;
3. estimate the implementation surface;
4. submit the structured `complexity` signal;
5. end the turn and allow CodexDD to continue automatically.

The structured complexity report contains:

- `estimated_files`
- `cross_module`
- `public_api_or_data_model`
- `persistent_state_or_serialization`
- `concurrency_or_async`
- `build_release_or_toolchain`
- `uncertain_root_cause`
- `broad_test_surface`

Designers should describe the real work accurately and let the Worker classify it. Do not pre-select a complexity class or write prompts intended to game the classifier.

## Complexity classes and implementation floors

CodexDD deterministically maps reconnaissance to these bounded implementation floors:

| Complexity | Implementation floor |
| --- | --- |
| Routine | Luna Low |
| Standard | Luna High |
| Complex | Terra Low |
| Architectural | Terra Medium |

Complexity pressure can raise the initial implementation floor only. It cannot start work directly on Sol or Astra.

After the complexity signal is accepted, CodexDD continues the **same bounded Worker assignment** at the authorized floor. The Designer/operator should not restart the task or manually change the model.

## Phase 2: implementation

The Implementation Worker continues the existing authorized scope at the selected floor.

Normal project requirements still apply. CodexDD routing does not expand the Worker's authority, milestone ownership, or allowed change surface.

When the bounded implementation is complete, the Worker reports the appropriate structured workflow terminal. Implementation and Repair completion use `ready_for_validation`.

Final-answer prose is not authoritative; the runtime uses the structured adaptive signal.

## Phase 3: independent quality review

For **Standard, Complex, and Architectural** work, an accepted `ready_for_validation` terminal automatically creates a fresh, independently bound Validation Worker in the same working tree.

Routine work does not automatically spend an additional validation allocation.

The normal 0.3.0 path therefore does **not** require the Designer or operator to run `/new validation`.

The Validation Worker receives the same authorized scope and is instructed to review independently rather than assume the implementation is correct. The review checks areas such as:

- coupling and duplication;
- abstraction quality;
- architectural fit;
- unintended API/state/concurrency effects;
- unnecessary complexity;
- test quality and objective validation.

A green Validation Worker reports `ready_for_owner_qa` or the repository-handoff alias supported by the workflow. An evidence-backed blocker reports `repair_required`.

Project-specific Owner QA and repository-handoff governance remain project responsibilities.

## Budget-aware review routing

CodexDD derives its budget mode from live account rate-limit windows. Designers should not estimate quota state in prompts.

The most restrictive known window wins. Stale or incomplete window data does not manufacture surplus and falls back to Balanced behavior.

Independent review routes are:

| Budget mode | Validation/review floor |
| --- | --- |
| Conserve | Luna High |
| Balanced | Terra Low |
| Surplus | Sol Low |

Surplus capacity is intentionally spent on **independent review quality first**, rather than simply inflating implementation cost.

## Failure pressure remains separate

Failure pressure is not the same thing as complexity, quality, or budget pressure.

Existing anti-runaway behavior remains authoritative:

- objective native failures drive failure pressure;
- budget surplus does not authorize arbitrary active-Worker family jumps;
- complexity does not authorize arbitrary mid-attempt escalation;
- crossing a model-family boundary remains guarded by the trusted capability-report path;
- pause/off/user interruption remains authoritative.

Do not add Designer instructions that attempt to bypass these guards.

## Designer anti-patterns

Avoid:

```text
Use Terra Medium and implement this immediately.
If it fails, jump to Sol.
Use Astra for the review.
When done, run /new validation.
```

Prefer:

```text
[adaptive_worker]
role = "implementation"
authorized_scope = "<exact bounded scope>"

Implement this bounded assignment against the stated acceptance criteria.
Follow the CodexDD adaptive workflow exactly.
```

The Designer supplies the problem definition. CodexDD supplies the route.

## Existing project migration checklist

Projects such as Breakwater or Cycle Vengeance should review their Designer, milestone, Worker, AGENTS.md, workflow, and prompt-template instructions and update anything that:

- omits the `[adaptive_worker]` first-assignment header;
- leaves `authorized_scope` vague or unbounded;
- hard-codes model family or reasoning effort;
- instructs the first Luna turn to edit immediately;
- manually creates the normal Validation Worker;
- attempts to infer or manually spend quota;
- conflates failure pressure with complexity or budget pressure;
- bypasses project-specific Owner QA or repository handoff.

Preserve project governance that does not conflict with this contract.

## Live status expectations

Before the first bound assignment, `/adaptive status` may show:

```text
Enabled: yes
Current: Luna Low
Budget mode: Balanced
Complexity: None
Implementation floor: None
Worker role: Unspecified
Worker binding: Awaiting first assignment
```

After reconnaissance, a nontrivial assignment may show, for example:

```text
Current: Terra Medium
Budget mode: Balanced
Complexity: Architectural
Implementation floor: Terra Medium
Attempt: 2
Worker role: Implementation
Worker binding: Bound
```

That transition is expected and does not represent runaway failure escalation.

## Source-of-truth files

The 0.3.0 integration behavior is implemented primarily in:

- `codex-rs/tui/src/adaptive_budget.rs`
- `codex-rs/tui/src/adaptive_complexity.rs`
- `codex-rs/tui/src/adaptive_policy.rs`
- `codex-rs/tui/src/adaptive_worker.rs`
- `codex-rs/tui/src/chatwidget/adaptive_effort.rs`
- `codex-rs/tui/src/chatwidget/adaptive_runtime_bridge.rs`
- `codex-rs/tui/src/chatwidget/adaptive_signal_transport.rs`
- `codex-rs/tui/src/chatwidget/adaptive_trusted_signal.rs`
- `codex-rs/core/src/tools/handlers/adaptive_signal.rs`

When documentation and runtime behavior disagree, treat the current merged runtime and its regression tests as authoritative and update this guide.
