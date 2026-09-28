# CodexDD 0.3.8 Mechanical Validation Admission

CodexDD 0.3.8 repairs the Mechanical-validation command gate introduced in 0.3.7. The lifecycle and routing model from the 0.3.7 Worker/Designer Integration Guide remains authoritative; 0.3.8 changes which bounded validation commands can pass the no-source-edit gate and how an admission refusal is classified.

## Why 0.3.8 exists

A live BREAKWATER Z-A0.51C soak reproduced a tooling defect after a bound Implementation Worker correctly entered Mechanical validation. The Worker could not run its normal Node/TypeScript proof commands, and common Windows command batches containing read-only Git inspection were rejected before execution.

The defect was in CodexDD command admission, not in the BREAKWATER product candidate.

## Node package-manager validation

Mechanical validation now recognizes validation-oriented scripts invoked through:

- `npm` / `npm.cmd`
- `pnpm`
- `yarn`

Supported script families are intentionally bounded to validation/build naming conventions such as:

- `test` and `test:*`
- `typecheck` / `type-check`
- `check`
- `lint`
- `build`
- `verify`
- `validation:*`
- formatting scripts whose name explicitly contains the check form, such as `format:check`
- project-specific check suffixes such as `maps:check`

Unknown script names still fail closed.

Known mutation or long-running script names remain blocked, including names containing `fix`, `write`, `watch`, `update`, `generate`, `publish`, `release`, `clean`, `dev`, `serve`, or `preview`.

Validation arguments that explicitly request common write/update behavior such as `--fix`, `--write`, `--update`, snapshot updates, or watch mode are also blocked.

This is a command-admission boundary, not a claim that arbitrary repository scripts are intrinsically safe. Mechanical validation does not blanket-authorize arbitrary `npm run` / `pnpm run` / `yarn run` scripts.

## Windows read-only Git and command batching

Mechanical validation continues to inherit the reconnaissance-safe read-only Git policy, including:

- `git status`
- `git status --short`
- safe `git diff`
- `git diff --check`
- `git rev-parse`

0.3.8 also accepts literal `&&` chains when every individual command is independently allowed. A single-`&` operator, `||`, redirection, command substitution, unsafe pipeline stage, or any disallowed command causes the whole chain to fail closed.

For example:

```text
git status --short && git diff --check
npm run typecheck && git status --short
```

are admitted, while:

```text
npm run test && Set-Content probe.txt changed
```

is rejected before execution.

## Admission rejection is policy, not capability failure

A Mechanical-validation gate refusal is now emitted through the tool lifecycle as a blocked policy outcome. It is not an executed command failure.

Adaptive failure pressure already treats declined/blocked command evidence as cancellation rather than native execution failure. This keeps a rejected source-writing command from masquerading as a model-capability failure and from arming the automatic two-failure capability escalation path.

The Worker should still respond to a legitimate gate refusal by either choosing an allowed validation command or, when source changes are genuinely required, reporting:

```text
kind=implementation_work
```

and ending the turn before editing.

## BREAKWATER proof commands

The live reproduction commands that motivated 0.3.8 are covered by regression tests, including:

```text
npm run test -- src/tests/capabilityExpression.test.ts
npm run format:check:changed
npm run typecheck
npm run lint
git diff --check
git status
```

Source-writing counterparts remain blocked.

## Unchanged 0.3.7 lifecycle

0.3.8 does not change the lifecycle routes:

| State | Routing rule |
| --- | --- |
| Reconnaissance | Luna Low, read-only |
| Implementation | Accepted complexity floor or stronger trusted route |
| Mechanical validation / Conserve | Luna Low |
| Mechanical validation / Balanced | Luna Medium |
| Mechanical validation / Surplus | Luna High |

Complexity persistence, `implementation_work` re-entry, review floors, capability guards, successor protections, workflow terminals, and owner authority are unchanged.
