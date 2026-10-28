# CodexDD 0.4.5 — Packet 3F.25: restore upstream 0.163 TUI auth/history regression matrix

**DRAFT / NONRELEASE.** This packet addresses a missing test contract in one of the 15 original upstream/fork direct conflicts, not a release approval.

The current 0.4.5 TUI file `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs` retained CodexDD adaptive Worker state tests, but its dynamic-tool namespace and delegated-turn test still covered only two ChatGPT feature-rollout states. Official 0.163 expands the same test to **six** source-config/auth/history scenarios:

- ChatGPT with Daybreak feature enabled/disabled.
- API-key auth with the gated access program disabled/enabled.
- App-server `config/read` unavailable with JSON-RPC `-32601` and `-32600`.

The original fork test asserted a Daybreak turn program based solely on a rollout flag. The upstream contract instead checks actual auth and history capability, including whether `thread/start` transmits `daybreakEnabled`; this prevents accidental propagation of a gated program to API-key sessions and legacy servers. This edit ports the upstream exact six-case test matrix, test-only model provider URL and API-key fixture, request setup and expected program checks. It leaves the existing CodexDD Worker adaptive session assertions and all product runtime sources untouched.

The source change has eight bounded edits (one intentionally matches two assertions). `docs/upstream-candidates/codexdd-045-3f25-dynamic-auth-test-edits.json` records full old/new replacements, original fork Git blob `994d54b631493b3d0784e836c544c57a275e9985`, secured branch parent `ebb6975b7ee78e00299cfd3f9bd2f33fa5fc9789`, upstream 0.163 blob `062efa383da797e46f6aa56540cbf76a9c130353`, and new branch Git blob `c10799047d696edcafd24d994c84f1fd87fd6a26`. The new Python verifier reconstructs the entire new file from the immutable prior Git blob and asserts preserved CodexDD adaptive tests; the 109-owner matrix and two existing correction guards are reanchored.

**Validation gates:** (1) source-only Python reconstruction, matrix and previous correction guards; (2) native Windows **one test selector only** `dynamic_tool_requests_ignore_other_namespaces_and_dispatch_tui_namespace` across all six scenarios (no full 21-stage suite); (3) wider owner semantic signoff and final 21-stage source-frozen release gate later. Source-only verification does not imply native behavior passes. Secure Cargo, vendor, Bazel, deny policy and runtime markers are unchanged; optimized release packaging and full 109-path acceptance remain open. Do not merge, install or publish.
