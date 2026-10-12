# CodexDD 0.4.3 — Packet 3B.3c: app-server handler and event source reconciliation

**Status: SOURCE MERGE CHECKPOINT, NOT NATIVE-VALIDATED.** Source integration is isolated and nonpromotable. Upstream target remains official rust-v0.162.0, peeled commit c1382380de69521303b416720a52f42d51af6248. Production baseline remains 4828e3b4232781963594cfaaebdc49c5531de8b1. Original fixed-target discovery remains blocked_transplant_conflict; canonical candidate unpublished.

## Committed source reconciliation

- `5709bdfaa82d287b84d110334c2dd6a598382c01`: upstream protocol/src/lib.rs and protocol.rs plus rollout-trace/protocol_event.rs with retained fork-only trusted adaptive signal types, turn triggers and trace filtering.
- `063aa82114b19355be44d3133abd4cbe0a031e80`: upstream app-server/notification_media.rs with the internal adaptive notification no-media passthrough.
- `1cad55d5596734f51924162ebc50804e27e69741`: upstream app-server/message_processor.rs, request_processors.rs and thread_state.rs with fork workflow-state request dispatch, protocol imports and turn-scoped adaptive signal accumulation.
- `bf60b2b26ffb6b5378687ee75e7b2e605927d9e8`: upstream app-server/request_processors/turn_processor.rs with explicit thread workflow-state persistence handler.
- `e1418fe23441f2652db1d19f66e17028d8849a3f`: upstream app-server/bespoke_event_handling.rs with fork adaptive event envelope binding to event turn and server conversation IDs, live notification, trusted turn-scoped replay before terminal completion and existing regression test.

## Security and integration requirements

These are GitHub source-only merges, **not an assertion of compilation, behavioral correctness, runtime signal authorization or schema consistency**. In particular, review the trust origin and caller permissions of `thread/workflowState/update`, event turn attribution, duplicate replay idempotence and turn terminalization; do not assume typed DTOs grant authorization. Run narrow app-server Rust tests after source tree integration and enforce Windows release gates.

The 3B.3 handwritten source overlap set is now reconciled at an initial source level. **3B.4 remains unstarted**: run repository-owned code generators from fully reconciled protocol/config sources; check generated JSON schemas, TypeScript and Python bindings and `.json.zst` precomputed exports for drift. Do not manually patch binary compressed artifacts or misrepresent source-derived regenerated files without a real generator execution.

No Daniel-CL action requested yet. No production merge, product version bump, upstream release-file bump, CI acceptance, v2 sidecar publication or installation.
