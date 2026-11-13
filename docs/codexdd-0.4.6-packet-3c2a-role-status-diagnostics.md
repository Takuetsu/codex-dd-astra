# CodexDD 0.4.6 — 3C.2a: role and source-aware diagnostic status

**Opened:** 2026-11-13. **Status: WIP / focused Windows acceptance not yet run.** Owner packet: 3C.2 — TUI role-binding/status diagnostics.

## Scope

The existing first-nonblank `[adaptive_worker]` parser, immutable assignment preflight and Core native tool and profile admission are already implemented in earlier 3B.2 and 3C.1 packets; this slice must **not** quietly rebind an unbound/locked thread or invent tool availability.

In `ChatWidget::adaptive_effort_status_text` append four bounded, read-only operator diagnostics without changing the existing status fields or any authority:
- **Authorized source:** exact owner-assigned `source_root @ source_sha`, otherwise say no source pin and do not infer one from launcher cwd;
- **Pending prerequisites:** current gate ledger's unresolved native source profile, independent review and runtime health obligations with `PENDING`/`BLOCKED` labels, or explicit `Not specified by an authorized owner`;
- **Native tool required:** worker-role-specific Core tool name and explicit caveat that a historical LVO status is **not** proof that a current Core invocation/profile is available;
- **Next action:** first-message header syntax for unbound Workers; current pinned source work_packet; genuinely fresh independent Validator review; owner handoff when paused or blocked; never a fabricated READY or destructive reset.

Three deterministic tests exercise first-assignment diagnosis, source/Validator role-specific pending profile diagnostics, and cold-paused source READY refusal. A frozen source pin cannot be changed through this status view; this change is presentation only.

## Release hold

Scope is **diagnostic**, not evidence promotion, provider compatibility, source-orchestration rewrite or full UI redesign. Require Rustfmt and named 3C.2a tests on native Windows, plus negative provenance verification. No 0.4.6 production binary release until later owner-controlled gates.
