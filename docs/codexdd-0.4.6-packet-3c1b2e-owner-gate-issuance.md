# CodexDD 0.4.6 — Packet 3C.1b.2e: owner-authorized gate issuance

**Date:** 2026-11-12. **Parent:** `d2131bd604f32a7181d9aad840b50a2fab1bf3e4` (3C.1b.2d contract acceptance recorded).
**State:** SOURCE IMPLEMENTATION ONLY. Native Windows CI, real independent Core tool PASS, full app-server/TUI handoff and 0.4.6 release acceptance remain **PENDING**.

## Purpose and authority

The 3C.1b.2c handoff required an exact owner-specified `AdaptiveGateLedger`, but did not create one when a real user supplied the initial `[adaptive_worker]` assignment. This packet adds the missing **one-time owner-origin issuance path** while preserving fail-closed READY. No ledger can be inferred from a model summary or generic command success.

The **optional** `required_gates` field is a bounded TOML array of inline tables in externally configured `adaptive_worker` or the very first user-origin `[adaptive_worker]` header. Omission is supported for legacy/non-QA assignments but **never** creates a ledger or authorizes READY. The owner must explicitly name every required gate before protected work. Later prompts, generated handoffs, or replayed checkpoints cannot add, remove or reinterpret them.

Example source-change owner assignment (replace both illustrative root and SHA with a real clean approved worktree and its exact 40-character HEAD):

```toml
[adaptive_worker]
role = "implementation"
authorized_scope = "owner-approved single candidate task"
source_root = 'D:\approved\candidate-worktree'
source_sha = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
task_kind = "source_change"
required_gates = [{ kind = "source_profile", profile = "work_packet" }, { kind = "independent_validation" }]
```

For a first user message, include a **blank line and a nonempty task instruction** after the table. This example is *not* authorization for any real candidate.

Accepted requirement kinds: `{kind="source_profile",profile="targeted"|"work_packet"|"release"}`, `{kind="owned_runtime_health",ports=[nonzero,unique,owner-approved TCP ports]}`, and `{kind="independent_validation"}`. The independent gate must appear **exactly once**. A source-change task needs at least one `work_packet` or `release` gate; targeted alone cannot substitute. Extra explicitly approved gates are retained, not silently removed.

An `operations_preview` task requires an owned-runtime-health gate and an independent gate, no source-profile gate, and an exact clean source root/SHA. Runtime-health gates stay **PENDING** until separately authenticated owned listener/HTTP evidence exists in 3D; port values or source-only PASS never satisfy health. A `benchmark` manifest is SETUP_BLOCKED until a typed native benchmark evidence contract exists.

## Immutable binding and validation

- Gate fields are typed and reject unknown inline-table keys. Empty, weakened or ambiguous manifests, missing profile/ports, unsupported profiles, duplicate independent gates, invalid/duplicate listener ports, missing pin, wrong role and excess gate counts are SETUP_BLOCKED before starting protected work.
- The exact original owner assignment must bind an **Implementation** Worker with a bounded scope and source root/SHA. Time-bounded read-only Git checks confirm the selected canonical worktree, exact HEAD and no staged/unstaged/untracked changes. No ancestor repo fallback or mutation.
- Startup-configured Workers receive the pending ledger **before thread/start**, and first-message Workers receive it after the first accepted user-origin assignment. The same converter implements both paths. Route changes preserve it; later prompts cannot overwrite it.
- Fresh independent Validation Workers inherit existing requirements only on an authenticated `ReadyForValidation` source handoff. Source and independent receipts are rechecked against worker role/scope, worktree, SHA and run ID. Durable checkpoints preserve only requirements/blockers, never historical PASS or READY.
- Missing manifests remain unissued; source-only test success never satisfies an owned-runtime requirement.

## Focused native regression

The `[3c1b2e-native]` Windows workflow requires source preflight, Rustfmt, one typed `codex-config` parser test, five TUI owner-issuance/refusal tests, and three prior 3C.1b.2c handoff regressions. Coverage: exact manifest + startup binding, first-turn immutability, pending runtime-health behavior, weakened/forged-field refusal, and wrong SHA/foreign Git root/dirty checkout refusal.

**Nonclaims:** These are source/unit fixtures, not a full real six-stage independent profile through Core on a separate candidate, real app-server two-Worker acceptance, live runtime health proof, full Windows release suite or installed 0.4.6. BREAKWATER/Cycle Vengeance and frozen 0.4.5 are untouched.

## Next

After native CI, execute real source-pinned disposable-worktree Core/TUI/app-server acceptance with two separate Workers, including negative stale/mismatched/dirty evidence paths and an authentic independent native PASS before calling 3C.1 complete. Finish the full remaining 0.4.6 development and release validation before the approximately seven-day single-milestone BREAKWATER soak.
