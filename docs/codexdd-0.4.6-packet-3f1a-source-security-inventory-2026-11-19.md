# CodexDD 0.4.6 — 3F.1a candidate source/security audit inventory (not signoff)

**Date:** 2026-11-19  
**Result:** 3F.1a read-only source identity, file manifest and high-risk boundary *triage* complete; 3F.1b exhaustive semantic review and 3F.1c independent signoff **OPEN**.  
**Immutable release-candidate source:** `8818334c5cf2e22154fea4c0cfcd25a8baf66f55` on `dd/codexdd-v0.4.6-rc1`.  
**Comparison baseline:** `b25550dcb2c6e84a84074b603086c9db6f32a63b` (`dd/astra-policy-v2`), still production 0.4.4.  
**Shelved 0.4.5 freeze:** `a69ce2ee19a50b834bc116246f94940bef8b8806`; separate branch history with production as common ancestor, not the RC1 ancestor. Ancestry alone is *not* proof of zero overlapping source content.  
**Audit-only branch:** `dd/codexdd-v0.4.6-3f1-audit`. No edit to candidate, development, production, or installed app.  
**Supporting manifest:** [all changed paths with provisional risk triage](codexdd-0.4.6-packet-3f1a-diff-manifest.json).

## Packet card and verified preflight

- **Outcome:** Establish the exact RC1-vs-production change boundary and prioritize source security/semantic review. Keep release candidate pinned and its validation receipts intact.
- **Owner/source:** GitHub `Takuetsu/codex-dd-astra`; released production base `b25550dcb2c6e84a84074b603086c9db6f32a63b`; exact RC1 `8818334c5cf2e22154fea4c0cfcd25a8baf66f55`. The compare is a *diff inventory*, not approval of code behavior.
- **Observed delta:** 465 development commits ahead, 0 behind; **165 changed paths** in the GitHub comparison, **90 codex-rs paths excluding snapshot files**, **87 Rust paths**, **65 documentation paths**. Risk-priority buckets (heuristic, may overlap across semantics): P0-high-security-review 34; P1-build-provenance-review 10; P2-general-semantic-review 33; P2-test-coverage-review 23; P3-documentation-review 65.
- **Quality gate:** [RC1 Actions #44539233887](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44539233887) completed successfully at this exact source: product/upstream provenance, Rustfmt, Windows codex-cli `--version`, TUI status snapshot, native D02 independent-Validator/live health/one-use READY, 8/8 3B.4 and cold-resume refusal. Candidate checkout stayed clean.
- **Acceptance scope:** These are targeted fixture proofs, **not** 3F audit clearance, a full 21-stage release, real Daniel-CL usage, or an installed app-owner health receipt.

## First-pass read-only security boundary observations

The first-pass inspected the definitions/signatures and relevant portions of 13 high-impact source paths (see manifest) plus the pinned V8 CI action. Findings are *present source controls*, not independent proof that all call paths enforce them:

1. **Worker/candidate authorization:** `core/src/session/worker_authority.rs` validates exact 40-character previous/candidate SHAs; the candidate must match the original Implementation Worker, canonical clean worktree, descendant HEAD, and persist/flush a snapshot preserving role, scope, root, and task kind before granting the new source pin. `core/src/session/turn_input.rs` is a high-priority follow-up for checking user-origin and idle-only call-site enforcement.
2. **Owner-approved runtime observation:** `core/src/tools/handlers/codexdd_owned_runtime.rs` parses only `session_id` from a model tool call. The authorized port/executable argv come from the original Implementation/operations-preview gate. `core/src/unified_exec/owned_runtime_health.rs` matches the Windows listener's OS PID, requests loopback `GET /health` with bounded time/size, requires HTTP 200, and rechecks ownership. There is no trusted release receipt from simply observing health.
3. **Independent evidence/READY:** `tui/src/adaptive_gate_prerequisites.rs` checks exact scope/root/SHA, pending port admission, and an independent Core review reference; READY requires no outstanding gate. Deeper review must trace *every* native event ingress and caller before accepting the independent-verification and one-use guarantees.
4. **Benchmark bounds:** `tui/src/adaptive_benchmark.rs` requires immutable owner-approved 2/4 commands, a clean source preflight, scratch-only exclusive lock, chronological nonoverlapping Core event pairs, separate process/item IDs and one-use admission. Source/measurement provenance across all callers remains to be independently reviewed.
5. **V8 provenance:** `.github/actions/setup-rusty-v8/action.yml` extends the production action with bounded transport retries; trusted release manifest SHA verification and two artifact checksum checks remain present. Inspect Windows artifact provenance again during 3F.2.

**No new confirmed source/security violation was established by this bounded static first pass.** This is **not** a finding that the whole diff is secure, complete, or authorized for promotion.

## Required 3F.1b/3F.1c stop gates

- Trace original user-origin/idle-only candidate change through `turn_input.rs`, app-server request routing, flush/restart/fork, and TUI; test or document any bypass **without weakening role, scope, source, sandbox, permission or validation protections**.
- Trace model tool registration, `run_codexdd_validation`, independent review, native receipt matching and all READY transition call-sites. Prove wrong-root/wrong-worker/stale replay refusal still applies at the final candidate SHA; avoid passing on copied/reforged metadata.
- Trace D01/D02 process and command ownership end to end, including safe cancellation, PID reuse, hostile listener/port, non-200 responses, and benchmark lock replacement; never grant a privileged daemon manager or synthetic green evidence.
- Verify the full candidate change set against the **frozen 0.4.5 feature hold**: branch ancestry is distinct, but potential overlap must be assessed by semantic/source review. Do not import shelved daemon/updater/TUI work by assumption.
- Obtain an independent, scope-bound reviewer/owner signoff for 3F.1, and separately complete 3F.2 artifact/CI provenance, 4.1 real Windows agent smoke, 4.2 representative paired completion/efficiency acceptance, and 4.3 exact-candidate native 21-stage release with owner promotion authorization.

## Conclusion and next step

**3F.1a COMPLETE — inventory/triage only. 3F.1 overall OPEN.** Take the 34 P0-priority paths from the manifest into bounded 3F.1b reviews. Reuse the green RC1 source evidence; do not run CI for this documentation-only branch or treat a later audit-document commit as if it were the tested candidate's binary SHA. When independent review is ready, secure an explicit reviewer signoff **before** starting release promotion.
