# CodexDD Major-Phase + Numbered Work-Packet Standard

This document defines the required execution structure for larger CodexDD releases and integrations.

## Core rule

A major phase such as `3A`, `3B`, `3C`, `3D`, `3E`, `3F`, or `Phase 4` is a planning and ownership boundary. It is **not** assumed to fit inside one agent run, one chat response, or one uninterrupted execution window.

All substantial major phases must be divided into numbered work packets before or during execution:

- `3B.1`
- `3B.2`
- `3B.3`
- ...
- `3C.1`
- `3C.2`
- ...

The numbered work packet is the normal executable unit.

## Why this is required

Large single-run phases create unnecessary risk from:

- chat or stream timeouts;
- connector/runtime limits;
- oversized tool batches;
- partial implementation with unclear recovery state;
- difficult rollback and review;
- excessive context growth before a durable checkpoint exists.

The workflow therefore optimizes for short, recoverable implementation slices rather than maximizing the amount of work attempted in one run.

## 2026-11-10 process adaptation: evidence-first throughput

**Applies to CodexDD work-packet execution, not runtime privilege.** BREAKWATER's approved November 10 delivery-throughput amendment and October 15 staged Workflow V3 establish these transferable process improvements. The specific BREAKWATER Z-A0.58 pause and normal-Codex-only executor are **not** CodexDD policies. The 0.4.6 milestone, its P0 requirements, 0.4.5 freeze and all owner-reserved promotion gates stay unchanged.

### Compact one-outcome packet card

Before editing or dispatching a costly validation, record only the information necessary to execute a narrow task:

| Field | Required content |
| --- | --- |
| Outcome | One observable agent behavior, or one confirmed defect class to repair |
| Identity and authority | Owner-authorized milestone/role, exact repository + branch + base/candidate SHA and environment |
| Owner | Source subsystem or owner path; links to current governing contract and test |
| Acceptance and focused proof | Observable acceptance criterion, focused tests, essential security/persistence negative controls, any required independent scope |
| Exclusions | Adjacent feature/cleanup/history explicitly not included |
| Stop and handoff | Evidence that should stop the packet, blocker classification, missing fact and next permitted action |

Keep the card brief. Cite durable source/Drive records; never paste long prior-worker narratives as current authority. An already authorized packet may perform routine implementation decisions inside its scope, but may not cross an owner-reserved release, installation or scope gate.

### Failure classification before repair

- **A — Product:** reproducible in-scope behavior, security, source-binding or persistence failure against a current acceptance criterion. Fix the owning layer and add focused regression proof.
- **T — Test:** stale/wrong assertion, oracle, fixture, driver or test-harness defect. Repair test ownership only after product causation is reasonably excluded; never quietly convert a real product failure to T.
- **E — Environment:** runner, dependency, transport, access, profile, timeout or permission failure. Collect a discriminating environment observation and fix only the authorized setup layer.
- **U — Unproven:** insufficient evidence or conflicting hypotheses. Do not infer product causation; obtain one smallest discriminating observation or report BLOCKED.
- **B — Backlog:** unrelated future defect or enhancement. Preserve evidence for a separately authorized packet; do not absorb it into the current gate.

Write OBSERVED -> EXPECTED -> smallest REPRO -> exact source/runtime identity -> EVIDENCE -> OWNING PATH -> classification/hypothesis -> smallest CHANGE -> PROOF. Red/INCOMPLETE evidence stays red until the exact failed criterion is re-established.

### Circuit breaker and validation cadence

- Never re-run an identical failing suite, Git status, environment setup or CI run merely to generate another receipt. Reuse valid, source-bound successful evidence when the source, profile, scope and environment facts still match.
- **After two inconclusive/no-new-evidence attempts on the same objective, stop that loop.** Capture what is proven, what remains unknown, and one genuinely different diagnostic/owner decision. An evidence-producing failure or material changed source/environment is progress, not an automatic count against the limit. Do not bypass a safety-critical blocker.
- During implementation and bounded repair: use the cheapest meaningful local focused proof, relevant changed formatting/compile checks and security/persistence negative controls. Repair-specific regressions are more useful than repeated full-project CI. Keep independent nonauthor verification fresh and scoped before accepting a packet/gate.
- When an actually runnable isolated candidate is ready, an **owner-authorized** early real-agent Windows usability smoke may be labeled PRELIMINARY. Do not label it whole-milestone READY, replace independent verification, or modify live game worktrees.
- Defer full integrated official Windows profiles, broad E2E/regression/security, native daemon/resume/F2, distribution provenance and owner release signoff until their recorded final gate. They remain **mandatory**, not optional. The Daniel-CL first/focused-before-GitHub CI policy later in this document still governs the testing environment and any GitHub-only exceptions.
- Measure accepted working behaviors, closed P0 defects, safe blocked outcomes, unproductive repetitions and validation effort. Do not optimize the number of commits, worker launches, test minutes or model-effort claims as if these were user value.

## Work-packet sizing

Each work packet should normally deliver **one observable outcome or one confirmed repair class** within a coherent subsystem. Approximately 5-10 meaningful files is a safety guideline, **not** a throughput goal or justification for padding a packet.

A packet should be split again when any of the following is true:

- it spans multiple independent subsystems;
- it mixes implementation, generated-artifact repair, broad testing, and release work;
- it is likely to require many sequential connector/tool calls;
- it has a large semantic-conflict surface;
- it cannot reasonably reach a durable checkpoint before an execution timeout;
- recovery after interruption would require reconstructing significant unstored state.

There is no minimum packet size. Reliability is more important than keeping packet counts low.

## Required packet lifecycle

Every numbered work packet follows this lifecycle:

1. **Scope**

   - State the exact files, subsystem, conflict set, generator, or validation target owned by the packet.
   - Explicitly identify work deferred to later packets.

2. **Reconnaissance**

   - Verify the current branch/head and prior checkpoint.
   - Reconfirm relevant upstream/base anchors when applicable.
   - Inspect the packet's conflict or dependency surface before editing.

3. **Implementation**

   - Perform only the packet's authorized work.
   - Do not opportunistically absorb unrelated later-phase work.

4. **Narrow validation**

   - Run or inspect the smallest meaningful checks for the packet.
   - Full release validation remains a later dedicated packet/phase unless the packet explicitly owns it.

5. **Durable checkpoint**

   - Commit the completed packet to the feature branch.
   - Verify the resulting branch/head SHA.
   - Do not rely on uncommitted or in-memory state as the handoff.

6. **Stop and report**
   - Report the packet result, checkpoint SHA, unresolved items, and next packet.
   - Stop after the checkpoint instead of automatically consuming the next packet.

The next packet begins in a new execution turn unless there is a clear reason to continue and sufficient runtime margin remains.

## Checkpoint policy

A completed packet must end at a recoverable Git checkpoint.

For larger releases:

- checkpoint commits are expected and intentional;
- WIP checkpoint commits are acceptable when clearly labeled;
- production branches remain untouched until the release workflow authorizes merge/install;
- a timeout after a verified checkpoint is treated as an interrupted conversation, not lost implementation work;
- branch/head verification is required before resuming after any timeout or stream failure.

When a packet cannot be completed safely, checkpoint only coherent work and label the remaining work explicitly.

## Major-phase completion

A major phase is complete only when all of its numbered packets are complete and the phase-level exit conditions are satisfied.

For example:

- `3B.1` can complete while Phase `3B` remains open.
- Phase `3B` closes only after `3B.1`, `3B.2`, `3B.3`, etc. are complete and the 3B validation/ownership audit passes.

Do not use "Phase 3B complete" to mean "the current 3B packet completed."

## Separation of implementation and validation

For large releases, prefer distinct packets for:

- source integration;
- generated artifacts;
- subsystem-specific tests;
- full phase audit;
- build/provenance repair;
- broad pre-install validation.

This is specifically intended to prevent the old pattern where one long run attempted implementation, generation, testing, repair, and release preparation before creating a checkpoint.

## Local validation execution policy

### Owner directive (2026-10-11): cheap Windows local validation before final GitHub CI

This is the default CodexDD testing workflow for 0.4.3 and future releases:

1. **Assistant handles GitHub changes.** The assistant owns code, design, documentation and GitHub-side fixes. Before asking for validation, establish the exact integration-branch commit SHA. The operator does not manually edit source.
2. **Run cheap checks locally first.** For every source-changing work packet, prefer the smallest meaningful read-only/status, formatting, targeted compilation or relevant regression check on **Daniel-CL over SSH**. The aim is to find inexpensive failures before consuming GitHub CI runs; do not make full builds or release suites the default.
3. **Use LVO when applicable.** An installed, compatible Local Validation Orchestrator can run repository-owned `targeted` and `work_packet` profiles and provide authoritative receipts. If an unreleased candidate cannot run through that local orchestrator, the assistant provides concise, copy-pasteable, non-destructive PowerShell commands and the operator runs them on Daniel-CL over SSH. Do not assume RDP or claim the assistant has executed local commands.
4. **Local fix/retest loop.** When a check fails, the assistant diagnoses it, changes the source on GitHub and asks only for the affected cheap local check again. Do not use temporary GitHub Actions workflows for routine trial-and-error or repeatedly push CI-only fixes when a feasible local check exists.
5. **GitHub Actions is the final CI gate.** Launch/request broad GitHub CI only after relevant local checks pass for the intended commit. For a CI failure, inspect logs, distinguish infrastructure from a product defect, fix narrowly, then rerun applicable cheap Daniel-CL validation **before** another intentional CI run.
6. **Native release acceptance is separate.** At the proper release gate, obtain the actual Windows candidate-bound release-profile receipt and live F2, elevated/non-elevated daemon and resume/fork evidence; GitHub tests and CI fixtures do not replace those gates. Production merge, install and release publication still require explicit owner authorization.
7. **Exception must be justified and approved.** Only when the necessary diagnostic cannot reliably run on Daniel-CL may the assistant propose a narrowly scoped, read-only GitHub-only oracle. Document the specific local blocker and obtain explicit owner approval before dispatching it. Remove temporary diagnostic workflows when no longer needed.
8. **No unnecessary test work.** Documentation-only changes require only appropriate diff/identity review, not a Rust build or a new CI workflow.

This directive supersedes older informal GitHub-first test habits without changing the source reconnaissance, semantic acceptance, candidate publication, local release-profile, and separate human promotion gates. Keep the production branch unchanged during unapproved runtime integration.

CodexDD 0.4.0 changes routine pre-PR implementation validation from an operator-mediated shell loop to Local Validation Orchestrator ownership on the Windows target when the repository provides the fixed validation-profile contract.

- During a bound Implementation Worker's protected mechanical-validation phase, the native `run_codexdd_validation` runner owns ordinary local validation.
- Ordinary completed implementation uses the repository-owned `work_packet` profile. After an admitted source-changing repair, `targeted` must pass before `work_packet` is rerun.
- Handoff authority comes from current native validation receipts, not model prose or manually reconstructed command sequences.
- The operator is no longer the routine command typist for work-packet validation. Manual PowerShell is reserved for explicit owner gates, release-shaped environment checks, bootstrap validation of a candidate runtime that is not installed yet, or a diagnosed runner/infrastructure fallback.
- Windows-target evidence on Daniel-CL remains authoritative for CodexDD pre-PR and pre-install release validation unless a packet explicitly requires another environment.
- Prefer the repository-owned validation profiles over ad hoc test command lists. The `release` profile is an explicit broad gate and must never be silently selected by an ordinary Implementation Worker.
- Infrastructure/contract failures do not authorize opportunistic source edits. Repair admission must follow the bounded native-evidence policy.
- Narrow temporary GitHub Actions oracles are **exceptional**, require a documented Daniel-CL blocker and explicit owner approval as specified above, and never substitute for the authoritative Windows validation gate.
- Normal GitHub PR/CI remains a later promotion stage after the required local Windows gates are green.
- Consequential actions remain human-governed: merge, production install, release publication, deployment, destructive Git operations, and changes to external machines are not implied by an LVO PASS.
- Phase 4 release-shaped smoke must exercise the actual operator launch context, not only non-interactive CLI flags. For Windows releases that support automatic shared-daemon startup, test a normal interactive launch from the Administrator PowerShell used in production, verify safe embedded fallback when elevated daemon creation is intentionally refused, and separately verify that explicit elevated daemon creation remains blocked.
- Heavy Rust resume/fork tests on Windows may require `RUST_MIN_STACK=16777216`; if a test fails only with `STATUS_STACK_OVERFLOW`, rerun the unchanged test with that stack before classifying it as a runtime defect.

The 0.4.0 development branch itself may still require operator-run profile scripts while the new runtime is being bootstrapped and proven before installation. That bootstrap exception does not revert the post-0.4.0 operating model: once the LVO-capable runtime is active, routine work-packet validation belongs to the orchestrator.

## Relationship to the 3A -> 3F + Phase 4 workflow

The existing major-phase structure remains valid:

- `3A` - reconnaissance/scaffold/ownership map
- `3B` - core/runtime integration
- `3C` - TUI/adaptive lifecycle integration
- `3D` - status/model/catalog integration
- `3E` - version/provenance/build/upstream-sync repair
- `3F` - complete implementation audit
- `Phase 4` - robust pre-install validation
- soak/install/release steps follow their existing authorization rules

The change is that each major phase is now a container for one or more numbered work packets. No major phase is assumed to fit in one agent run.

## Historical 0.3.14 application

For CodexDD 0.3.14, Phase 3B is split as follows:

- **3B.1 - Core/runtime integration**

  - Reconcile the Phase 3B handwritten/runtime overlap set onto upstream `rust-v0.159.2`.
  - Preserve CodexDD adaptive workflow behavior.
  - End with a WIP/checkpoint commit.

- **3B.2 - Generated precomputed exports**

  - Regenerate the stable and experimental app-server precomputed `.json.zst` exports from the merged schema sources.
  - Verify source/artifact consistency.
  - Stop and checkpoint.

- **3B.3 - Phase 3B targeted validation**
  - Audit the complete 59-path Phase 3B ownership set.
  - Run the narrow relevant compile/tests.
  - Fix only Phase 3B defects.
  - Checkpoint and close Phase 3B only when its exit conditions pass.

Phase 3C then begins with `3C.1`, not with an attempt to complete all of 3C in one run.

## Default for future larger releases

This major-phase + numbered work-packet model is the default CodexDD release workflow for larger upgrades, upstream integrations, and multi-subsystem feature releases.

If a future phase is genuinely small enough to complete safely in one run, it may contain a single packet such as `3D.1`. The packet/checkpoint discipline still applies.
