# CodexDD 0.4.5 — 3F.59 nine private persistence, rollout and memory owner source reviews

**NONRELEASE. Source-only evidence, not native acceptance or a production upgrade.**

The 3F.59 packet documents nine additional per-path, exact `openai/codex@rust-v0.163.0` reversible source deltas from the source-frozen reviewed candidate `72cd09d3f52c076d445078758af20489ccdb4e74`:

- `external-agent-migration/src/sessions/append_tests.rs`: regression verifies untrusted imported histories cannot exert private workflow-state authority. Also removes two obsolete upstream test fixture response fields; native import/schema compatibility is still required.
- `history/src/lib.rs`: versioned private workflow-state records, a validation receipt snapshot, and changed developer-message attribution comment. Requires old/new wire compatibility and trust review.
- `history/src/tests.rs`: private workflow-state round trip with identity exclusion, new wire-variant inventory.
- `memories/write/src/phase1.rs`, `memories/write/src/rollout_input.rs`: private workflow metadata is not materialized into model memory.
- `rollout-trace/src/protocol_event.rs`: adaptive runtime trust signals are excluded from generic event trace treatment.
- `rollout/src/search.rs`: private workflow state omitted from user-visible history search.
- `state/src/extract.rs`, `state/src/runtime/threads.rs`: private state does not appear as user-visible conversation messages, and runtime match branches remain exhaustive.

The machine-readable hunk/reconstruction manifest is `docs/upstream-candidates/codexdd-045-3f59-persistence-memory-upstream-deltas.json`, with source-blob tests in `test_codexdd_045_3f59_persistence_memory_semantics.py`.

**Formal 109-path owner status:** 53 source-evidence-submitted, 56 require additional source or behavioral review, and **0 formally accepted**. Each new entry still requires exact-source native history roundtrip, external import authority rejection, private metadata non-leakage, restart/fork persistence, and accountable owner signoff. The original 109-path matrix remains `NOT_ACCEPTED` until the release transaction.

**Windows packaging (separate completed gate):** [3F.56 green #41073817590](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41073817590) produced seven unsigned developer EXEs and seven PDBs; 3F.58 recorded 14 unique SHA-256s and the archive digest. This does not authorize production installer signing or distribution.

No Rust runtime, dependency graph, production marker or installer changes in 3F.59.
