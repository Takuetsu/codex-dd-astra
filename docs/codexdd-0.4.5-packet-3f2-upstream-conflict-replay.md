# CodexDD 0.4.5 — Packet 3F.2: original 0.163 direct-conflict replay

**Status: EXACT 15-PATH GITHUB REPLAY PASS; SOURCE IDENTITY LEDGER PINNED; NATIVE SEMANTIC DISPOSITION OPEN. NOT A RELEASE CANDIDATE.**

- Branch source starts from `6b52453d963d85810945ca71d3d7833a1f8b91a6`. No production mutation, release marker promotion, policy waiver, or Daniel-CL diagnostics are authorized by this packet.
- Upstream-prepare normally creates a synthetic customization commit with *CodexDD production tree* and *parent official prior release*, then cherry-picks that customization onto the *new official upstream release*. This packet performs the corresponding three-way `git merge-tree --write-tree --merge-base=<old> <new> <production>` in a GitHub hosted read-only checkout, with explicit baseline `c1382380de69521303b416720a52f42d51af6248`, target `83086188ae0614bd882818ff1aa23f3194962012`, production `b25550dcb2c6e84a84074b603086c9db6f32a63b`.
- The workflow `.github/workflows/codexdd-045-original-conflict-recon.yml` uses `contents: read`, fetches only the two pinned official release commits with Git's blob filter, does not commit, does not write branch refs and never publishes. It captures raw Git merge-tree stdout/stderr and an explicit list/count artifact; it validates non-mutating source identity and fails closed if the report isn't parseable or isn't the expected *15* original direct conflicts. A discrepancy means more reconnaissance, not permission to change the expected count.
- Earlier [3F.1](codexdd-0.4.5-packet-3f1-overlap-source-inventory.md) measured **109** four-way overlapping files and independently identified **22 historic 0.4.3 conflict paths** (only **13** of those changed upstream in 0.163). Those sets must not be substituted for the 15 direct conflicts from the original 0.163 customization transplant.
- After this diagnostic is green, inspect the exact original roster and assign each path a clear disposition: upstream semantic intent, CodexDD-protected behavior, merged source blob and applicable focused native Windows evidence. A blob comparison or successful `git merge-tree` command is **not** itself such a release receipt.
- After the one-shot query has concluded, remove its workflow and record the evidence in a separate reviewed GitHub source checkpoint. No user action is required.

## Reproduced direct-conflict roster and source-only disposition — 2026-10-21

- [Read-only GitHub run #39613477624](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39613477624) **PASSED**. Exact pinned official Git commits were fetched without write access. The source-only `git merge-tree --write-tree --name-only --no-messages --merge-base` simulation returned exit code **1** (the expected merge-conflict result), then emitted exactly **15** ordered unmerged paths; the workflow validated the count and clean tracked source state. Neither a source checkout nor production was mutated.
- The exact 15-path immutable per-file SHA ledger is `docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-original-direct-conflicts.json`, with the four Git blob identities, nonrelease historical CI evidence, and each path's **still-required native/semantic gate**.
- The original conflict categories are: **one Bazel workflow, one Cargo lockfile, four generated protocol artifacts, three core Rust paths and six protected TUI Rust paths**. All fifteen integrated blobs differ from the 0.4.4 fork; this source distinction is independently verified against the 109-overlap manifest by `.github/scripts/test_codexdd_045_3f2_direct_conflicts.py`.
- Historical nonrelease GitHub runs are referenced strictly as limited previous evidence, **not** a passing exact frozen 0.4.5 release candidate. Notably registry-focused Windows runtime, app-server security, schema generator drift, TUI/core test compilation and targeted Bazel package-dependency loading have green historic receipts, but the full native Windows release suite and inherited RustSec advisory decisions remain **open**.
- The diagnostic workflow `.github/workflows/codexdd-045-original-conflict-recon.yml` is removed with this checkpoint after its successful one-time replay. This is a read-only workflow retirement, not revocation of a release candidate that never existed.

### Exact fifteen original paths

- `.github/workflows/bazel.yml`
- `codex-rs/Cargo.lock`
- `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst`
- `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst`
- `codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts`
- `codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts`
- `codex-rs/core/src/context_manager/history_tests.rs`
- `codex-rs/core/src/context_manager/normalize.rs`
- `codex-rs/core/src/tools/registry.rs`
- `codex-rs/tui/src/app/startup.rs`
- `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs`
- `codex-rs/tui/src/app/tests/startup_defaults_tests.rs`
- `codex-rs/tui/src/app_server_session/workspace_roots_tests.rs`
- `codex-rs/tui/src/chatwidget/interaction.rs`
- `codex-rs/tui/src/lib.rs`

### Gate interpretation

**Source integrity:** every original direct conflict is now pinned to a reviewed nonproduction integration blob; the underlying three-way replay and 109 overlap inventory are independently reproduced. **Semantic behavior:** still requires the per-path native evidence listed in the ledger, security approval, freeze and final SHA-bound Windows release suite. **All-target cargo-deny:** still RED for four inherited INFO/unmaintained advisories, no exemptions. **Version markers:** still 0.4.4 / rust-v0.162.0. Do not merge, package, release, install or auto-update from this receipt.

## 3F.2a source-guard fixture correction

- Initial read-only [source-guard run #39614440088](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39614440088) failed 1 of 432 checks **only** because the newly written source-strategy allowlist excluded its own intended `manual_security_merge` value for `core/src/tools/registry.rs`. All pinned blob-path/provenance tests passed; the registry source was not changed.
- The test now explicitly admits `manual_security_merge` but asserts the registry's exact classification and still-**NOT_ACCEPTED** release gate, ensuring the correction does not falsely mark security behavior passed. No Cargo/Bazel/module/policy/runtime/marker changes in this follow-up.
