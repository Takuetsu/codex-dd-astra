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

## Work-packet sizing

Each work packet should normally cover one coherent subsystem, one generator/repair task, or approximately 5-10 meaningful implementation files.

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

Implementation validation before PR/CI is operator-mediated and runs on the Windows target machine, normally Daniel-CL, through PowerShell/SSH.

- The assistant defines small, explicit validation packets and gives the operator the exact PowerShell commands to run.
- The operator runs those commands on Daniel-CL and returns the output before the next validation step proceeds.
- Prefer cheap, targeted local checks first, then broaden only after earlier gates pass.
- Phase-level validation and Phase 4 pre-install validation should be robust enough to expose integration defects before GitHub CI, but they still run locally through this operator-mediated loop.
- Do not create temporary GitHub Actions workflows merely to perform implementation-phase validation.
- GitHub CI is reserved for the later PR/CI stage after the local validation gates are green.
- Windows-target validation on Daniel-CL is authoritative for pre-PR implementation testing unless a packet explicitly requires another environment.
- Phase 4 release-shaped smoke must exercise the actual operator launch context, not only non-interactive CLI flags. For Windows releases that support automatic shared-daemon startup, test a normal interactive launch from the Administrator PowerShell used in production, verify safe embedded fallback when elevated daemon creation is intentionally refused, and separately verify that explicit elevated daemon creation remains blocked.
- Heavy Rust resume/fork tests on Windows may require `RUST_MIN_STACK=16777216`; if a test fails only with `STATUS_STACK_OVERFLOW`, rerun the unchanged test with that stack before classifying it as a runtime defect.

This policy keeps the human owner in the validation loop, catches Windows-specific defects before CI, and avoids spending GitHub runner time on iterative development failures.

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

## Current 0.3.14 application

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
