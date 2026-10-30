# CodexDD 0.4.5 — 3F.57, source-bound CLI/config/persistence owner review

NONRELEASE. Continued formal 109-path reviews independently from the running 3F.56 Windows package build. No shipping Rust/Bazel/Cargo modifications.

## Seven more exact upstream reviews

At original 0.163 fork source SHA `72cd09d3f52c076d445078758af20489ccdb4e74`, each of the following seven paths now has a machine-verifiable per-hunk reverse source reconstruction against `openai/codex@rust-v0.163.0`:

- `codex-rs/cli/src/main.rs`: exactly four lines of additive adaptive startup forwarding. Upstream CLI dispatch retains its original behavior; elevated/embedded/daemon startup still needs exact-source native user-flow review.
- `codex-rs/cli/tests/daemon_startup.rs`: replaces the obsolete elevated F2 warning expectation with `/status` observation, additionally rejects creation of a shared daemon PID on elevated resume/fork. Native elevated/normal environment acceptance is outstanding.
- `codex-rs/config/src/config_toml.rs`: additive startup Worker role/config plus paired Daybreak decoding. Native authority/trust/bad-role parsing still outstanding.
- `codex-rs/rollout/src/policy.rs`: makes private `RolloutItem::WorkflowState` durable but excludes adaptive runtime signal events from model-visible journal; preserves old boolean persistence API atop upstream optional durable representation.
- `codex-rs/rollout/src/recorder.rs`: state DB must be consulted for DB-only queries, including an empty filter; private workflow-state durability present.
- `codex-rs/history/src/rollout_payload.rs`: adds symmetric `CodexDdWorkflowState` wire variant and owned/borrowed conversion without replacing upstream variants.
- `codex-rs/thread-store/src/types.rs`: adds restored private workflow-state field to thread store snapshot and constructor, without removing upstream members.

The exact source bytes/hunks are pinned in `docs/upstream-candidates/codexdd-045-3f57-cli-state-reversible-source-review.json`; `test_codexdd_045_3f57_cli_state_semantics.py` reconstructs and rehashes all seven upstream Git blobs and verifies negative/authority-adjacent policy text.

## Formal status update

The 3F.55 signoff ledger now records **44 evidence-submitted, 65 source/behavior queued, 0 formally accepted** paths. These seven are source-evidence-ready only; they still require native tests and explicit accountable owner decisions. The canonical owner matrix remains `109 PENDING_OWNER_REVIEW / NOT_ACCEPTED`. The final 21-stage release profile remains blocked by design.

This packet contains *no product code changes* and must not be confused with the 3F.56 Windows bundle run (an independent, unsigned developer distribution build based on earlier exact optimized source). Production remains CodexDD 0.4.4.
