# CodexDD 0.4.1 packet 1A.2 — adaptive routing surface map

## Status

Reconnaissance complete. This packet records the accepted routing design before runtime implementation.

GPT-6.1 Sol is a distinct adaptive tier. It does not replace the existing GPT-6 Sol identity.

Proposed CodexDD family identity:

- enum: `AdaptiveFamily::Sol61`
- persisted/operator label: `sol61`
- status label: `Sol 6.1`
- model: `gpt-6.1-sol`

Ordering:

`Luna < Sol < Sol61 < Astra`

## Why a distinct tier

The current runtime intentionally keeps cheaper routes available and crosses family boundaries only with trusted capability justification. Mapping the existing `Sol` family directly to GPT-6.1 Sol would silently make every current Sol route more expensive/stronger and would erase the proven GPT-6 Sol rung.

A new family preserves 0.4.0 behavior for lower-cost paths while adding a controlled step before Astra.

## Current routing surfaces

The authoritative pure ladder is `codex-rs/tui/src/adaptive_policy.rs`.

Supporting route selectors are:

- `adaptive_complexity.rs` — implementation starting floors;
- `adaptive_budget.rs` — independent Validation quality floor and mechanical-validation route;
- `adaptive_controller.rs` — failure/unfinished-turn pressure reductions;
- `chatwidget/adaptive_trusted_signal.rs` — trusted capability-report gate at family boundaries;
- `chatwidget/adaptive_effort.rs` — family parsing/model mapping, fresh Worker routing, durable labels, status;
- `chatwidget/adaptive_admission.rs` — effective model/effort synchronization before successor admission.

## Accepted automatic ladder

The current lower and upper families remain intact. GPT-6.1 Sol is inserted between GPT-6 Sol and GPT-6 Astra:

| Order | Route          |
| ----: | -------------- |
|     1 | Luna Low       |
|     2 | Luna Medium    |
|     3 | Luna High      |
|     4 | Sol Low        |
|     5 | Sol Medium     |
|     6 | Sol High       |
|     7 | Sol 6.1 Low    |
|     8 | Sol 6.1 Medium |
|     9 | Sol 6.1 High   |
|    10 | Astra Low      |
|    11 | Astra Medium   |
|    12 | Astra High     |
|    13 | Astra XHigh    |
|    14 | Astra Max      |

GPT-6.1 Sol supports XHigh and Max, but those are intentionally omitted from the automatic CodexDD ladder. Once a task has justified stronger reasoning beyond Sol 6.1 High, CodexDD should spend that escalation on the frontier Astra family rather than add two more intermediate attempts.

`ultra` remains outside the ladder because it is a Codex multi-agent orchestration selection rather than a normal single-model reasoning rung.

## Starting preference

Existing semantics remain unchanged: every adaptive run begins at Luna Low unless a role-specific floor applies. `starting_family` remains preference metadata and does not bypass lower rungs.

`sol61` becomes a valid parseable preference so status/persistence/operator input can represent the family, but preference alone still does not skip directly to it.

## Complexity pressure

Accepted implementation floors:

| Complexity    | 0.4.0      | 0.4.1           |
| ------------- | ---------- | --------------- |
| Routine       | Luna Low   | Luna Low        |
| Standard      | Luna High  | Luna High       |
| Complex       | Sol Low    | Sol Low         |
| Architectural | Sol Medium | **Sol 6.1 Low** |

Rationale: Architectural is the only complexity class that should proactively buy the newer workhorse capability. Complex work remains on the proven lower-cost Sol floor. No complexity class starts directly on Astra.

## Quality / budget pressure

Accepted independent Validation floors:

| Budget mode | 0.4.0     | 0.4.1              |
| ----------- | --------- | ------------------ |
| Conserve    | Luna High | Luna High          |
| Balanced    | Sol Low   | Sol Low            |
| Surplus     | Sol High  | **Sol 6.1 Medium** |

Rationale: budget surplus is explicitly intended to buy review quality. Moving only Surplus review to Sol 6.1 Medium makes the new workhorse useful in a high-value independent-review path without raising normal Balanced spend.

## Mechanical validation

Unchanged:

| Budget mode | Route       |
| ----------- | ----------- |
| Conserve    | Luna Low    |
| Balanced    | Luna Medium |
| Surplus     | Luna High   |

Mechanical validation is deterministic low-cost verification and must not spend into Sol, Sol 6.1, or Astra by itself.

## Worker / Designer behavior

Designer behavior remains unchanged. The Designer supplies bounded role/scope and acceptance criteria; it does not select model family/effort.

Worker routing:

- Implementation: initial reconnaissance remains Luna Low; accepted complexity then applies the table above.
- Validation: uses the quality-review table above.
- Repair: retains the normal cheap start and existing trusted escalation path.
- Mechanical validation inside Implementation: retains the Luna-only table.

No new Worker role is required.

## Failure and unfinished-turn pressure

The existing safety rule remains unchanged:

- unfinished/failure pressure may increase effort inside a family;
- a family jump requires the existing trusted capability-report boundary;
- inserting Sol61 creates two additional family boundaries: Sol High -> Sol 6.1 Low and Sol 6.1 High -> Astra Low;
- native failure pressure alone does not authorize either boundary.

This preserves the runaway-escalation guard.

## Route synchronization

Successor admission already validates both:

- effective model == `pending.route.family.model()`;
- effective effort == the pending adaptive effort.

Adding the new family/model mapping therefore requires no admission-protocol redesign.

## Implementation surface established by 1A.2

Runtime files expected to change:

- `codex-rs/tui/src/adaptive_policy.rs`
- `codex-rs/tui/src/adaptive_complexity.rs`
- `codex-rs/tui/src/adaptive_budget.rs`
- `codex-rs/tui/src/chatwidget/adaptive_effort.rs`

Targeted tests/fixtures expected to change:

- adaptive policy/controller tests for the expanded ladder;
- complexity-floor tests;
- budget-route tests;
- family parse/model/status tests;
- trusted-signal boundary tests where the next family after Sol is currently Astra;
- persistence/resume tests for the new family;
- Worker/Designer guide routing tables.

No change is planned to mechanical-validation policy, LVO profiles, trusted signal transport, or admission semantics.
