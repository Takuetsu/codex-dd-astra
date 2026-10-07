# CodexDD roadmap

## Current production state

CodexDD **0.4.1** is complete and merged to production.

- production branch: `dd/astra-policy-v2`
- production/source SHA: `f45c5a119296d6e53c6429f9b101b3f52d91047e`
- production PR: #46
- upstream workspace: OpenAI Codex `0.159.2`
- target platform: Windows
- authoritative validation host: Daniel-CL

0.4.1 added GPT-6.1 Sol as a distinct adaptive tier between GPT-6 Sol and GPT-6 Astra while preserving the 0.4.0 lifecycle, validation, persistence, and fail-closed guarantees.

The merged 0.4.1 design and reconnaissance evidence remains in:

- `docs/codexdd-0.4.1-gpt-6.1-sol-design.md`
- `docs/codexdd-0.4.1-packet-1a1-model-catalog-reconnaissance.md`
- `docs/codexdd-0.4.1-packet-1a2-routing-surface-map.md`
- `docs/codexdd-0.4.1-packet-1a3-persistence-regression-map.md`
- `docs/codexdd-0.4.1-packet-1e-pre-validation-audit.md`

## 0.4.2 — automatic upstream-update orchestration

**Status: PHASE 2 SOURCE COMPLETE — DANIEL-CL VALIDATION PENDING.**

Feature branch:

`dd/codexdd-v0.4.2-automatic-upstream-update`

0.4.2 begins from the exact merged 0.4.1 production SHA:

`f45c5a119296d6e53c6429f9b101b3f52d91047e`

### Goal

Add a safe, explicit, CodexDD-owned workflow for discovering and preparing compatible OpenAI Codex upstream updates while preserving the rule that no upstream update is silently merged, installed, or deployed.

The workflow should reduce the manual work required to identify a new stable upstream, create the bounded update work packet, perform cheap mechanical checks, and surface evidence for human-controlled promotion.

### Non-goals / hard boundaries

- No unattended production merge.
- No unattended Daniel-CL installation or external-machine deployment.
- No silent bundling of unrelated CodexDD feature work into an upstream refresh.
- No automatic acceptance of an upstream version solely because a newer tag exists.
- No weakening of LVO, lifecycle, persistence, adaptive-routing, or fail-closed guarantees.
- No runtime behavior changes before reconnaissance identifies the complete update surface and acceptance criteria.

### Required order

1. Current upstream-sync mechanism and repository-surface reconnaissance.
2. Define stable-version discovery and compatibility policy.
3. Define bounded update-plan / work-packet generation.
4. Implement read-only discovery and dry-run planning.
5. Implement explicit preparation flow behind human-controlled promotion.
6. Add validation, status/evidence, persistence, and failure handling.
7. Windows validation on Daniel-CL.
8. CI, merge, install, and soak only after explicit human gates.

### Phase 1 result

Packets 1A.1–1A.3 are complete on the feature branch.

The existing six-hour sync already detects stable tags, reconstructs the CodexDD delta, pushes a branch, auto-bumps the next patch version, and opens a PR. 0.4.2 will replace that coupled behavior with a read-only scheduled planner plus explicit identity-bound preparation and promotion.

Phase 1 also established that product versioning must be decoupled from upstream discovery, local Daniel-CL/LVO evidence must precede PR/CI promotion, and exact tag/commit/tree plus overlap evidence must be durable candidate state.

The live reconnaissance candidate is OpenAI Codex `rust-v0.160.1`; it is not being integrated as part of Phase 1.

### Phase 2 result

Packets 2A.1–2B.2 are source-complete.

0.4.2 now separates the upstream lifecycle into:

1. read-only scheduled discovery and manifest generation;
2. explicit identity-bound candidate preparation;
3. Daniel-CL native validation receipt;
4. explicit receipt-bound PR promotion;
5. normal human-controlled CI/merge/install gates.

The old scheduled path can no longer allocate a CodexDD product version, push an integration branch, mutate GitHub Issues, or open a PR.

The release source identity is now `codexdd 0.4.2`.

Implementation record:

- `docs/codexdd-0.4.2-phase-2-implementation.md`

The next gate is cheap Daniel-CL validation before the normal work-packet profile.

### Working standard

Continue using the major-phase + numbered work-packet standard.

- Keep packets small enough for one agent run.
- Assistant handles source/code changes directly on GitHub.
- Daniel only runs meaningful Windows validation gates when requested.
- Prefer LVO-owned routine validation where the current runtime can exercise it.
- Native evidence outranks prose.
- Fail closed on ambiguous upstream identity, version, branch, or validation state.
- Consequential gates remain human-controlled: CI promotion, merge, installation, release publication, deployment, and external-machine changes.

## Preserved backlog

- Windows release packaging / tagged release automation originally explored in PR #13, except pieces explicitly reused by 0.4.2.
- Later CodexDD adaptive-policy enhancements not required for upstream-update orchestration.
- Lifecycle-aware complexity-floor and budget-aware de-escalation follow-ups not already completed.
- Other enhancement items previously captured in CodexDD planning material.
