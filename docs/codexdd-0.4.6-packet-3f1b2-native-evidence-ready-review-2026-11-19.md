# CodexDD 0.4.6 — 3F.1b2 native evidence and READY boundary review

**Date:** 2026-11-19
**Exact candidate implementation SHA:** `8818334c5cf2e22154fea4c0cfcd25a8baf66f55`, intentionally unchanged
**Audit status:** **Source trace documented; independent review/approval is still required.**
**Historical native evidence:** [Windows RC1 #44539233887](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44539233887); CLI version, status snapshot, native Implementation -> independent Validation -> fresh Core-owned QA -> READY, 8/8 3B.4 and one cold-ready negative passed.
**Security-review scope:** Static read of native tool-issuer, TUI registry, gate ledger and terminal-time host callback. Does NOT assert a complete all-files independent semantic/security review.

## Core native evidence generation

1. `core/src/tools/handlers/codexdd_validation.rs:697–792`: `emit_validation_result` constructs the app-server `DynamicToolCallItem` after a native profile result. `success` requires `summary.status == pass` **and** exit code zero. Core chooses selected canonical source root; tool identity is `run_codexdd_validation` or `run_codexdd_independent_validation` by profile, and emits a started then completed event with exact call-id/run-id/source SHA.
2. `tui/src/adaptive_evidence.rs:684–848`: `record_from_item_completion` accepts a `ThreadItem::DynamicToolCall` and `native_source_provenance` requires namespace None, exact built-in tool name/profile, completed with `success: true`, `result: pass`, absolute root, 40-hex SHA, nonempty bounded run ID and positive completion clock. A plain command/MCP PASS cannot supply a native receipt.
3. `tui/src/adaptive_evidence.rs:186–407`: the registry treats conflicting duplicate evidence IDs as unusable; source-gate proof must match source turn, Implementation role, authorized scope, canonical root, exact SHA, native run ID/profile and minimum completion time. Independent review requires a **different** Validation thread from original Implementation, Validation role, exact source and current native independent-review run/turn.
4. `tui/src/chatwidget/adaptive_evidence.rs:18–102`: the actual `item/completed` handler binds Worker identity from the locked assignment, registers evidence, and calls source/independent gate observers; stale ledger refs are re-resolved at terminal time rather than trusted as a persistent green flag.
5. `tui/src/adaptive_gate_prerequisites.rs:94–223, 237–437`: the gate manifest must include exactly one independent validation requirement; checkpoints persist requirements/blockers **without** source or independent successful receipts. Restored gates are pending; the registry must revalidate every receipt. Owned runtime health is always pending until fresh host QA.

## READY admission and one-use Core health callback

- `tui/src/chatwidget/adaptive_trusted_signal.rs:119–267` consumes a terminal-bound signal for the matching source turn. Before requesting a Core health recheck it checks successful evidence refs, independent Validation thread, exact owner scope and pinned root/SHA, and **exactly one pending approved runtime port**; it then consumes the signal, discards pending model attempts, generates a fresh request UUID and sends `AppEvent::ReviewOwnedRuntimeHealthAtQa`.
- `tui/src/chatwidget/adaptive_trusted_signal.rs:628–725` requires locked distinct Validation Worker, matching source ledger, independent native review evidence present in READY signal refs, and no other outstanding source/independent/blocked gate. A generic success or status message is insufficient.
- `core/src/tools/handlers/codexdd_owned_runtime.rs:30–107,165–238` accepts only a Core exec session identifier from the model; immutable owner config provides command argv and port. `core/src/unified_exec/owned_runtime_health.rs:141–197` confirms Windows OS listener PID ownership before and after bounded loopback HTTP 200.
- `tui/src/chatwidget/adaptive_trusted_signal.rs:746–857` matches response to the pending request UUID, original/Validation thread IDs and terminal turn, checks nonzero PID, exact owner port, source root/SHA, fresh Core observation age <=5s, and reruns ledger admission before latching one-use `ReadyForOwnerQa`. It does not persist Core health as a reusable PASS.
- `tui/src/session_state.rs:439–469,536–544` restores checkpoint gates as pending, clears historical `ready_for_owner_qa` and suspends unverified legacy Ready. This is a protection against replayed historical rollout metadata.
- `app-server/src/request_processors/turn_processor.rs:227–414`: `thread/workflowState/update` accepts a `SetReadyForOwnerQa` rollout marker without itself calculating Core native receipts; the *live TUI admission path* provides the stronger gate. Important residual check: separately prove by a **direct app-server RPC negative test** that a synthetic persisted marker cannot be misinterpreted as current native READY by other clients or after resume. The current TUI restore code clears it, but that does not prove every client is safe.

## Observed validation, limitations and outstanding checks

- RC1 #44539233887 passed actual Windows end-to-end D02 and retained 3B.4, plus cold-resume negative. This directly tests the TUI/host gate on the exact candidate, **not** every hostile RPC ingress.
- **No confirmed bypass** found in the reviewed native producer/registry/terminal/cold-resume code paths. This is a bounded source observation, **not** independent security approval.
- **3F.1b open source attack surfaces:** direct `thread/workflowState/update` and `thread/injectItems` callers; source-root changes between trust and tool use (TOCTOU); collision/replay of independent review run ID across turns, process PID reuse and fast port rebinding; app-server-only/third-party client READY presentation; resource-lock replacement during D01; abandoned/stale QA request callback; authentication and permissions on transport. Review and targeted negatives required before signoff.
- **Independent reviewer requirement:** a separate qualified person/agent, with a distinct identity and no authorship conflict, must review the exact 8818334c5cf2e22154fea4c0cfcd25a8baf66f55 source diff against production, 0.4.5 freeze exclusions, these risk questions and all P0 paths. A CI green or the present assistant's own audit cannot serve as their approval.
- **No unauthorized deployment:** do not promote candidate, tag, publish installer, change daemon, modify BREAKWATER/CV or claim `OWNER_QA_PASS` from these source checks.

## Decision

**3F.1b2 native evidence/READY trace checkpoint complete. 3F.1 independent security signoff OPEN.** Finish independent review and concrete negative test/exception dispositions before accepting 3F.1. The candidate SHA is intentionally fixed.
