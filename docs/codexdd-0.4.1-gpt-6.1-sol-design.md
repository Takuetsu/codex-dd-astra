# CodexDD 0.4.1 — GPT-6.1 Sol integration

## Activation state

CodexDD 0.4.1 starts from the merged CodexDD 0.4.0 production SHA:

`013ab39780bd0adfb503f75742bb90f23e04d330`

Feature branch:

`dd/codexdd-v0.4.1-gpt-6.1-sol`

Platform: Windows.

Upstream workspace remains OpenAI Codex `0.159.2`.

Reconnaissance packets 1A.1-1A.3 are complete and accepted. Runtime implementation packets 1B-1D are complete. Packet 1E source audit is complete, and the Daniel-CL targeted profile is green. The next gate is the broader work-packet profile before PR/CI.

## Goal

Add GPT-6.1 Sol to the CodexDD adaptive model/effort workflow while preserving the routing, lifecycle, validation, persistence, and operator-evidence behavior proven in 0.4.0.

The automatic upstream-update enhancement is not part of 0.4.1.

## Work packets

### 1A.1 — Model catalog and capability reconnaissance — COMPLETE

Identify the exact GPT-6.1 Sol model identifier, display/family metadata, supported reasoning-effort levels, capability metadata, and the current upstream model source of truth.

Do not change adaptive routing in this packet.

### 1A.2 — Adaptive routing surface map — COMPLETE

Map every current route that can select, compare, persist, render, or reject model families:

- starting preference
- complexity routes
- escalation routes
- failure pressure
- quality pressure
- budget modes
- Worker/Designer behavior
- mechanical validation

Produce an explicit routing table for where GPT-6.1 Sol belongs.

### 1A.3 — Persistence, status, and regression impact map — COMPLETE

Trace:

- `/status`
- workflow snapshots
- resume/fork restore
- app-server protocol/schema if affected
- snapshots/tests containing model-family assumptions
- compatibility with persisted 0.4.0 state

### 1B — Model identity integration — COMPLETE

Make GPT-6.1 Sol representable in the minimum authoritative model layer required by 1A.

### 1C — Adaptive routing integration — COMPLETE

Implement the accepted routing table in bounded sub-packets rather than one large change.

### 1D — Status and persistence integration — COMPLETE

Update status/persistence only where the reconnaissance proves it is required.

### 1E — Full implementation audit — PRE-VALIDATION AUDIT COMPLETE

Audit the complete 0.4.1 change surface before release validation.

## Windows validation phase

Before PR/CI, prove on Daniel-CL:

- intended routes can select GPT-6.1 Sol
- routes that should remain on lower model families do so
- supported effort levels route correctly
- budget modes behave as designed
- failure/quality pressure behavior remains correct
- Worker/Designer behavior remains correct
- mechanical validation retains its intended low-cost route
- `/status` is accurate
- resume/fork behavior is correct
- LVO targeted/work-packet/release profiles stay green
- release candidate starts normally on Windows

## New-thread activation

Use this instruction in the next project thread:

`Activate CodexDD 0.4.1 from dd/codexdd-v0.4.1-gpt-6.1-sol. Start with packet 1A.1 GPT-6.1 Sol model catalog/capability reconnaissance. Do not change runtime routing until reconnaissance is accepted. Continue GitHub-side work autonomously until a meaningful Daniel-CL validation gate is reached.`

## Order after 0.4.1

After GPT-6.1 Sol integration is complete and proven, return to the automatic upstream-update enhancement.
