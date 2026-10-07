# CodexDD roadmap

## Current production state

CodexDD **0.4.0** is complete, merged, installed, and post-install smoke validated on Daniel-CL.

- production branch: `dd/astra-policy-v2`
- production/source SHA: `013ab39780bd0adfb503f75742bb90f23e04d330`
- installed identity: `codexdd 0.4.0+g013ab39780bd`
- upstream workspace: OpenAI Codex `0.159.2`
- target platform: Windows
- authoritative validation host: Daniel-CL

0.4.0 delivered the Local Validation Orchestrator (LVO):

- native bounded `run_codexdd_validation` runner;
- repository-owned `targeted`, `work_packet`, and `release` profiles;
- protected mechanical-validation lifecycle integration;
- native same-turn receipt authority for Implementation handoff;
- bounded two-cycle repair / targeted-retest / work-packet loop;
- durable repair and operator state across resume/fork;
- extended `/status` LVO evidence;
- Windows-authoritative release validation and daemon/startup proof.

Release proof completed before merge/install:

- final work-packet: 6/6 stages PASS;
- Phase 4 packets 4.1–4.6 PASS;
- release profile: 21/21 stages PASS;
- PR #45 CI: all required checks green;
- PR #45 squash merged;
- post-install `/status` showed CodexDD 0.4.0 and clean initial LVO state;
- explicit elevated daemon start remained blocked;
- final post-install smoke ended with `CODEXDD_0_4_0_POST_INSTALL_SMOKE_PASS`.

The merged 0.4.0 implementation/design evidence remains in:

- `docs/codexdd-0.4.0-local-validation-orchestrator-design.md`
- `docs/codexdd-0.4.0-validation-profile-contract.md`
- `docs/codexdd-0.4.0-phase-3f-audit.md`
- `docs/codexdd-0.4.0-phase-4-validation.md`

## 0.4.1 — GPT-6.1 Sol workflow integration

**Status: READY FOR RECONNAISSANCE / NEW THREAD.**

Feature branch:

`dd/codexdd-v0.4.1-gpt-6.1-sol`

0.4.1 begins from the exact merged 0.4.0 production SHA:

`013ab39780bd0adfb503f75742bb90f23e04d330`

0.4.1 is active on this branch. Packets 1A.1-1A.3 completed model-catalog, routing, persistence, and regression reconnaissance before runtime changes. Implementation now adds a distinct GPT-6.1 Sol tier between GPT-6 Sol and GPT-6 Astra while preserving the established 0.4.0 lifecycle/LVO guarantees.

### Goal

Add **GPT-6.1 Sol** to the CodexDD adaptive model/effort workflow without weakening the routing, budget, lifecycle, validation, persistence, or fail-closed guarantees established through 0.4.0.

### Required order

1. GPT-6.1 Sol reconnaissance/design.
2. Model catalog / identity integration.
3. Adaptive routing integration.
4. Status and persistence integration where required.
5. Focused Windows validation.
6. Broad release validation / CI / install / soak.
7. Only after 0.4.1 is complete, begin the automatic upstream-update enhancement.

The automatic upstream-update enhancement must **not** be mixed into 0.4.1.

### 0.4.1 first packet

Start with a bounded reconnaissance/design packet covering:

- exact GPT-6.1 Sol model identifier(s) exposed by the current Codex model catalog;
- supported reasoning-effort levels and any capability metadata CodexDD depends on;
- every current adaptive routing ladder where GPT-6.1 Sol may belong;
- complexity-floor behavior;
- failure-pressure escalation;
- quality-pressure escalation;
- budget-aware routing/de-escalation;
- Worker/Designer role behavior;
- mechanical-validation routing;
- `/status` model/effort rendering;
- resume/fork persisted model state;
- tests/snapshots that encode model-family assumptions;
- compatibility with the still-frozen upstream `0.159.2` line.

Do not edit runtime routing until reconnaissance identifies the complete change surface and acceptance criteria.

### Working standard

Continue using the major-phase + numbered work-packet standard.

- Keep packets small enough for one agent run.
- Assistant handles source/code changes directly on GitHub.
- Daniel only runs meaningful Windows validation gates when requested.
- Prefer LVO-owned routine validation after the new runtime is capable of exercising it.
- Native evidence outranks prose.
- Fail closed on ambiguous model identity/routing state.
- Do not mix an upstream Codex refresh into GPT-6.1 Sol integration.
- Consequential gates remain human-controlled: CI promotion, merge, installation, release publication, deployment, and external-machine changes.

## Later enhancement — automatic upstream updates

After 0.4.1 GPT-6.1 Sol integration is complete and proven, return to the automatic upstream-update enhancement.

That work must be its own release/packet line and preserve the existing rule that upstream synchronization is explicit, independently validated, and never silently bundled with unrelated adaptive-routing work.

## Preserved backlog

- Windows release packaging / tagged release automation originally explored in PR #13.
- Later upstream Codex stable refresh beyond the frozen 0.159.2 line.
- Lifecycle-aware complexity-floor and budget-aware de-escalation follow-ups not already completed.
- Other enhancement items previously captured in CodexDD planning material.
