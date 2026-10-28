# CodexDD 0.4.5 — Packet 3F.11: current-source 109-overlap semantic review start

**Status: 109/109 current file identities inspected; 7/7 post-receipt Rust formatter deltas reconstructed byte-exactly. SEMANTIC REVIEW OF ALL 109 NOT COMPLETE. NOT A RELEASE CANDIDATE.**

## Source boundary and immutable receipts

- Source-only branch: `dd/codexdd-v0.4.5-109-overlap-semantic-review`, branched from the crypto-integrated nonrelease candidate. No source implementation, production marker, installed build, Cargo dependency, advisory policy, or production branch was modified by this packet.
- Original four-way overlap source receipt: `docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json`, pinned official 0.162 and 0.163, fork 0.4.4, reviewed integration snapshot `f6fb89c098da28626c7c072a40d5e9b7306c8866`.
- Original direct-conflict receipt: `docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-original-direct-conflicts.json`, exactly **15 direct conflicts** distinct from 109 overlapping files.
- [Read-only current-source inventory #40580764913](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40580764913) **PASS**, source `9293c43d63b0f5bea602da0fae1fe914a34533b6`: all 109 paths exist and none reverted to production 0.4.4; **100** blobs still match the historical reviewed receipt, **9** differ. An artifact records every changed blob and the 15 remaining direct-conflict gate descriptions. This is source integrity, not behavioral acceptance.

## Disposition of the nine source changes since the frozen review

**Seven purely mechanical formatter transformations independently verified.** Each historical `before_blob` was fetched by immutable Git SHA and each current file by its exact source branch, with all recorded edits applied in order and all expected replacement sites unique; full reconstructed output compared byte-for-byte with current source and Git blob IDs. **7/7 files, 12/12 recorded replacement hunks exact**:

| Path                                                  | Current reviewed blob                      | Reconstruction edits |
| ----------------------------------------------------- | ------------------------------------------ | -------------------: |
| `codex-rs/rollout/src/search.rs`                      | `d222871cab7b8e55dcd1dec8faef7e9cf4918796` |                    1 |
| `codex-rs/tui/src/app/startup.rs`                     | `3b111def463e2f52b4786b7a6db8c6e3aa7954f7` |                    1 |
| `codex-rs/tui/src/app/tests.rs`                       | `03febfb8e58c93e975f1f240195576a0f1080e08` |                    2 |
| `codex-rs/tui/src/app.rs`                             | `7d9da0b198cfac44c0c06933c622b7c1515a613b` |                    1 |
| `codex-rs/tui/src/app_event.rs`                       | `5f43579e23f87a94786ae9803df8dba40060a84e` |                    1 |
| `codex-rs/tui/src/chatwidget/settings.rs`             | `bb747507416e7712fe4ff5832446783c0127994e` |                    2 |
| `codex-rs/tui/src/chatwidget/tests/slash_commands.rs` | `bec8d92caae4cdff17b7f06923bd4cd9ee358a52` |                    4 |

Source for exact old/new transformation receipts: `docs/upstream-candidates/codexdd-045-format-followup.json`, formatter job `121909294810`. The two additional formatted files `core/src/tools/registry_tests.rs` and `core-plugins/src/startup_sync_tests.rs` were not part of the 109 original overlaps; they remain covered by the original nine-file reconstruction test. **Do not infer runtime security acceptance solely from formatter provenance.**

**Two approved dependency changes from #1**:

- `codex-rs/Cargo.toml` — current blob `cfebd9b784ef9ff027b274aa9f7101c9c2496613`.
- `codex-rs/Cargo.lock` — Cargo-generated, current blob `13bc15ff844e93dfaf7777c73bb7313b35bb3b81`.
- Original all-target `cargo-deny` [#40577886527](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40577886527) **PASS**; complete protected crypto/voice owner graph and static regression evidence are documented in `docs/codexdd-0.4.5-packet-3f10-crypto-owner-closure.md`. A passing source/graph audit does **not** prove every runtime integration across all 109 paths.

## Original 15 direct-conflict owners and focused evidence

1. **Three security-sensitive core Rust conflicts**: history/normalize/registry; [Windows native 15/15 tests #39615406741](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39615406741) previously passed at its historical source SHA, including post-hook write policy, trusted signals and cancellation.
2. **Six TUI Rust conflicts**: startup, session lifecycle, workspace roots, chat interaction and TUI entry; [Windows native 25/25 #39618752181](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39618752181) previously passed at its historical source SHA, including elevated Windows startup, adaptive isolation, daemon and guarded key handling.
3. **Four generated protocol artifacts**: stable/experimental compressed schema and two TypeScript notification types; require final frozen candidate schema generation and drift checks, not just historical blob equivalence.
4. **One Bazel workflow**: must preserve CodexDD 90-minute CI bounds and reconcile upstream Windows MSVC release packaging atomically with Bazel dependencies; native release-package acceptance is a separate gate.
5. **One Cargo lockfile**: crypto/security-owner #1 remediation must remain locked and all-target audited across exact final candidate; no dependency waiver is allowed.

## All 15 original direct-conflict source deltas accounted for on the current branch

The [actual 109-overlap inventory artifact #40580764913](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40580764913) includes exact Git IDs for all 15 authentic 0.163 direct-conflict paths. Comparing each current blob to the separate frozen direct-conflict receipt gives the following **source-only** disposition:

- **13/15** are byte-identical to their original reviewed `f6fb89c...` source: Bazel workflow, the four generated schema/TypeScript artifacts, all three core Rust conflicts, and five of six TUI Rust conflict files. No post-receipt source change in these 13.
- **1/15 TUI startup source** (`codex-rs/tui/src/app/startup.rs`) changed only from recorded formatter blob `29065c6c0c90c010a53bf065c95278312712a90e` to `3b111def463e2f52b4786b7a6db8c6e3aa7954f7`; that unique source edit was reconstructed exactly from the 3F.6 formatter receipt, with no other byte changes.
- **1/15 dependency lock** (`codex-rs/Cargo.lock`) changed to the secure, Cargo-generated `13bc15ff844e93dfaf7777c73bb7313b35bb3b81` as part of completed focused crypto remediation; the unchanged original cargo-deny and exact Windows native tests are green. This is intentional functional dependency migration, **not** a format-only edit.

**Therefore: 15/15 original direct conflicts have current source provenance reconciled.** This resolves _source drift accounting only_, not path-by-path semantic acceptance or native Bazel/schema/release signoff. No historical `NOT_ACCEPTED` release gate has been removed or bypassed.

## Remaining semantic review contract for 109 paths

- 100 unchanged historical reviewed identities still need explicit _owner behavior_ classification, not another blob identity acceptance claim. Prior targeted source/runtime tests can be cited only within their historical SHA and precise behavior boundaries.
- Review by safety-critical owner: **core authority and history**, **TUI adaptive/Worker/daemon**, **app-server/protocol + generated SDKs**, **rollout/state/memory**, and **Windows/Bazel packaging**. Preserve both upstream 0.163 changes and CodexDD fail-closed authorization for every touched contract; run only the affected focused regressions before expensive release-wide work.
- Original 15 merge-conflict ledger remains a separate review/signoff set; do not replace it with the historical 0.4.3 conflict roster of 22 or claim 15/15 semantics accepted from one static report.
- Final release acceptance still requires frozen-source Windows native security and behavioral checks, schema drift, Bazel packaging, detached Daniel-CL 21-stage suite, release CI and explicit approval. No auto-publish or installed change here.

**Next #2 packet:** prioritize the nine direct-conflict core/TUI source contracts with their existing focused historical receipts and map the remaining overlapping app-server/protocol/rollout and Bazel owners to their exact native test selectors. Keep any correction small and targeted, preserving source/authorization invariants.

## 3F.12/3F.13 current secure-graph native semantic receipts — October 27, 2026

- [Exact Windows core/TUI owner run #40583499451](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40583499451) **PASS 40/40**: 15 core authorization/context/history tests and 25 TUI adaptive, startup/F2, managed-daemon, approval, workspace and key-interaction tests; secure `crypto-bigint 0.7.10` resolver pinned.
- [Exact Windows app-server/history/rollout run #40598678934](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40598678934) **PASS 14/14**: six app-server verification/admission, four history serialization/fork-state, and four rollout StateDbOnly/filter regressions, on the same reviewed secure Cargo graph.
- [Stable and experimental schema + Python SDK generation #40598386099](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40598386099) **PASS**: exact audited graph regenerated protocol/config artifacts and SDK with no drift.
- The original **nine core/TUI Rust conflicts** now have secure-graph focused native evidence; four generated-protocol original conflicts have source-bound Linux generation evidence. The secure `Cargo.lock` has a focused all-target cargo-deny and native runtime receipt. The **Bazel workflow/release-package original conflict remains not accepted** until optimized Windows packaging and release-profile verification.
- [Audited formatter-only run #40603996779](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40603996779) reconciled 68 Python and four Bazel layout changes without touching the 109 original-overlap source blobs, Cargo/deny/vendor, or version markers. The one-shot write-capable workflow was deleted after use.

These receipts substantially advance behavior-class review; they **do not establish full 109/109 semantic acceptance**, a live cloud authentication session, a native Windows shipping archive, a final frozen-source 21-stage release suite, or production promotion. Historical 0.163 static fixtures still pin older source/Cargo identities and require an explicit reanchor rather than quiet test bypass.
