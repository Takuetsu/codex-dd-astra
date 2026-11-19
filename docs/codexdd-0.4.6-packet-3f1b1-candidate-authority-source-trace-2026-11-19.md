# CodexDD 0.4.6 — 3F.1b1 source trace: candidate SHA authority and durable handoff

**Date:** 2026-11-19  
**Candidate under review:** `8818334c5cf2e22154fea4c0cfcd25a8baf66f55` (the exact 3E.2 Windows green RC1).  
**Audit branch:** `dd/codexdd-v0.4.6-3f1-audit`, documentation only.  
**Disposition:** **STATIC SOURCE CALL-CHAIN REVIEW CHECKPOINT; NOT AN INDEPENDENT SECURITY SIGNOFF, RELEASE TEST OR AUTHORITATIVE OWNER QA.**

## Security claim examined

A candidate Git SHA must not become a new trusted Worker source through a model message, delegated turn, active-turn steering, concurrent settings change, stale source/root, unsuccessful storage write, or a replayed/modified role. Promotion must require explicit original user intent, exact original Implementation scope and live same-thread persistence. The Core/TUI must continue checking fresh native source evidence after a promotion.

## Exact observed source call chain

| Boundary | Reviewed source and guard | Observed disposition |
| --- | --- | --- |
| User input shape | `core/src/session/turn_input.rs:322–369` parses only the first standalone text item containing `[codexdd_candidate]`; `content.len() == 1`, direct-user `parent_turn_id == None` and allowed trigger, rejects SubAgent/Internal sessions | Candidate instructions from delegated/internal inputs are explicitly rejected |
| Turn admission | `core/src/session/turn_input.rs:432–448, 567–700, 761–770` routes candidate `StartOrSteer` into `start_if_idle`, checks the reserved active-turn state, refuses `NotIdle`, forbids mixed settings changes and rejects candidate `Steer` outright | No obvious direct path for a candidate to modify a running Worker |
| Candidate identity | `core/src/session/worker_authority.rs:30–92, 254–343` validates two exact 40-hex SHAs, a durable Implementation source-changing Worker, canonical selected worktree, observed HEAD, clean `--porcelain=v1` with untracked files and descendant ancestry | Source/current-git checks are present; cannot infer approval from Git HEAD alone |
| Native persistence | `core/src/session/thread_settings.rs:47–112` acquires the shared persistence permit, requires a live thread, performs bounded blocking Git preflight, writes prospective same-thread `ThreadSettingsApplied`, awaits append and `flush`, then applies the immutable candidate and emits a Core-owned event | On append/flush errors the in-memory source update path is not reached |
| Non-escalation after flush | `core/src/session/worker_authority.rs:349–380` checks previous/candidate SHA and exact role/scope/root/task kind against the original snapshot before updating `binding.source_sha` | Source change only; role and privileges are not intentionally expanded |
| Runtime credential and checkpoint | `core/src/session/turn_context.rs:1275–1288` only sets Worker config on a full turn context when Core holds a durable accepted binding; `app-server/src/request_processors/turn_processor.rs:242–350` rejects malformed/oversized gate checkpoint or incorrect thread/role/scope/root/SHA | A self-reported TUI gate checkpoint alone cannot establish native Core trust |

## Relevant tests present in source, not yet independent RC1 acceptance

- `core/src/session/turn_input_tests.rs`: `codexdd_046_3b2j_candidate_submission_is_standalone_and_explicit`, `codexdd_046_3b2j_unbound_candidate_does_not_consume_turn`, `codexdd_046_3b2j_candidate_cannot_steer_an_active_turn`.
- `core/src/session/worker_authority.rs`: `codexdd_046_3b2i_descendant_preflight_does_not_rebind`, `codexdd_046_3b2i_rejects_unbound_wrong_sha_and_dirty_candidate`, `codexdd_046_3b2m_successor_snapshot_survives_cold_restore_without_reassignment`, `codexdd_046_3b2i_candidate_promotion_preserves_role_and_scope`.
- The 3E.2 RC1 Windows smoke **did not separately invoke these candidate-authorization-specific test filters**. It proved CLI version, TUI status, D02 real fixture, 3B.4 persistence and a cold-READY replay refusal. Do not relabel these source declarations as tested release-profile results.

## Residual risks requiring independent review

1. Trace the complete authorization request from app-server ingress through the same-thread store and TUI re-pinning; inspect rejection on real daemon restart, fork, delegated sessions and malformed `ThreadSettingsApplied`. Static read alone does not prove event provenance.
2. Confirm there is no exploitable TOCTOU gap between source HEAD/preflight, settings flush and the native tool's eventual source check under concurrent Git changes. Candidate preflight alone is insufficient evidence; the native validation tool must recheck exact source.
3. Ensure no test/setup code or alternate API mutates `first_worker_authority` without the user-origin and durable gates, particularly after recovery from a failed flush.
4. Preserve all exact-source negative controls and independent Core/TUI receipt checks in the final 21-stage native release; re-run targeted candidate authority tests on the final candidate where necessary. A green `3E.2` smoke is not a `3F.1` signoff.

## Decision

**No confirmed direct-owner candidate authorization bypass was established by the reviewed paths.** This is a first-pass static *observation*, not a complete independent security assurance. No runtime code change, CI rerun, release, 0.4.5 transplant or process operation is justified by this checkpoint. The next review slice is native evidence/Validator ingress and one-use READY terminal routing, with explicit independent reviewer signoff held open.
