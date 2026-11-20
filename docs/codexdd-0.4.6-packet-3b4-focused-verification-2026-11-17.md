# CodexDD 0.4.6 — Packet 3B.4 focused verification

**Verified:** 2026-11-17  
**Development branch:** `dd/codexdd-v0.4.6-agent-reliability`  
**Exact source:** `d8f7a64400ff404ef98d676c24dce0f2338161b0`  
**Native evidence:** [Windows Actions #44138512034](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44138512034) — completed **success**  
**Disposition:** **FOCUSED IMPLEMENTATION GREEN; FULL 0.4.6 / INTEGRATED ACCEPTANCE OPEN.**

## Verified native checks

The exact-SHA GitHub run passed source/contract preflight, Rustfmt, and the `3B.4 Windows no-progress cold-resume persistence and negative controls` job. Native Cargo enumeration reported **8 tests**, and execution reported **8 passed, 0 failed, 0 ignored** for `codexdd_046_3b4_`. The same job separately passed five one-test checks: source-pinned READY cold-resume refusal, one-retry controller behavior, genuine native activity clearing the no-progress streak, control-only lookups not masking an Implementation stall, and legacy history rollout round-trip.

The eight dedicated checks cover:

- **C01:** A transient infrastructure refusal permits at most one same-route retry; an unchanged second refusal blocks, without model-family escalation.
- **C02/C04:** Authentication, quota, environment, permission, dependency, unavailable model, owner decision, authorization/scope, and unknown failures block rather than buying extra capability.
- **C03:** Twelve consecutive Implementation turns with accepted native command evidence remain on the authorized route; the no-progress watchdog and unfinished-turn pressure reset on native activity.
- **C05:** Persisted bounded no-native-progress pressure, spent retry budget, last counted turn and unfinished-turn pressure survive cold resume without restoring native evidence, pending permits, or an Owner QA PASS. Replayed turns, over-limit counters, malformed/foreign-role checkpoints and inconsistent replay markers refuse; older checkpoints with absent optional fields remain readable.

These are executable source-matched Windows regressions, not an end-to-end owner workflow or a numerical performance acceptance result.

## Remaining constraints

1. The `AdaptiveGateLedger` continues to require genuinely revalidatable Core evidence for source, independent Validator and owned runtime-health gates. None of the 3B.4 checks grants that authority.
2. An ordinary successful command is not by itself proof of a changed _applicable_ evidence digest. The C01–C05 focused checks do not establish every causal distinction in the full 3A.3 progress-digest contract. This must remain an integrated acceptance check, not an inferred PASS.
3. No ten-pair comparison against equal-access ordinary Codex has been completed; no token, wall-clock, intervention or completion target is claimed.
4. D01/D02/D03 must be reviewed on one final source SHA alongside the retained 3C safety regressions. D02 still has no production Core-owned health receipt, and the final release gates are open.
5. Conditional P1 E/F/G issues remain bounded diagnostic holds until source-matched supporting evidence exists.

## Next execution

Continue with **D02**: tie a genuinely owned process, exact approved port, fresh HTTP 200 and source/Worker identity to a Core-issued, independently revalidated ephemeral runtime-health receipt. Until that producer exists, keep `OwnedRuntimeHealth` PENDING. Before any positive integration, preserve wrong-PID, wildcard listener, stale health, foreign process, wrong-port, wrong-SHA and cold-resume refusal behavior.

Do not merge, install, bump the 0.4.4 production version, touch BREAKWATER/CV, or count focused 3B.4 success as 0.4.6 release acceptance.
