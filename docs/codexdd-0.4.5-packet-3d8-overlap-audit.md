# CodexDD 0.4.5 — Packet 3D.8: exact three-way source overlap inventory

**Status: BLOB-LEVEL RECONNAISSANCE VERIFIED; FIVE PRESERVED-FORK PATHS REQUIRE EXPLICIT DISPOSITION; SEMANTIC AND ORIGINAL DIRECT-CONFLICT REVIEW OPEN. Not a release candidate.**

## Source identity and reproducibility

Read-only Git tree comparison at the 0.4.5 integration checkpoint used:

- Official upstream baseline `rust-v0.162.0`: `c1382380de69521303b416720a52f42d51af6248`.
- Official target `rust-v0.163.0`: `83086188ae0614bd882818ff1aa23f3194962012`.
- Original production CodexDD 0.4.4: `b25550dcb2c6e84a84074b603086c9db6f32a63b`.
- Integration checkpoint after the rollout-order merge: `89307e9136c8747db43031adb4d56499ddf52a54`; predecessor used for full-tree comparison was `4edca883f02efa80636c9c8de508d8cbd9e86603`. The only intervening relevant source changes were `codex-rs/rollout/src/recorder.rs` and its test.
- Compute overlap per Git blob path: official 0.162 and official 0.163 differ **and** original production and official 0.162 differ. Missing paths must count as distinct from present paths. Compare exact Git object hashes, not only filenames or line similarity.

| Directory | Upstream-changed paths | Fork-changed paths | Overlaps | Integration classification at this packet |
| --- | ---: | ---: | ---: | --- |
| `codex-rs/` | 1,266 | 207 | **106** | 102 integration-specific blobs; **4** still byte-identical to old fork |
| `.github/` | 17 | 45 | **1** | **1** intentionally retained fork Bazel CI file, review still open |
| `sdk/` | 4 | 2 | **2** | 2 reviewed generated output differences; generator drift CI separately green |
| **Total** | | | **109** | **104 changed/reconciled blob identities; 5 unchanged-fork overlaps** |

All currently reviewed 44 fork-sensitive TUI source paths in `.github/scripts/test_codexdd_045_3c9c_tui_upstream.py` have distinct pinned `REVIEWED_FORK` hashes versus their original production hashes. The source-guard workflow was green at `f10b862d` and again at [run #39491671832](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39491671832) following the rollout edit. This **closes the 44-path TUI file-identity review inventory**, not every TUI behavioral, 109-overlap semantic, or original 15-conflict release attestation. Older 3C.9f documentation reporting only 24/44 reviewed is a historical checkpoint, not the live blob inventory.

## Five upstream/fork overlaps still preserving the old fork file verbatim

1. `codex-rs/BUILD.bazel`: production adds `codexdd-version.txt` to `exports_files`, but upstream 0.163 introduces packaging settings and `runtime_archive` targets. **Do not paste the upstream target into this fork alone**: `bazel/rules/runtime_package.bzl` is not present on this integration branch, and the current app-server Bazel BUILD does not expose the required `:release_files` alias. Reconcile Bazel build rules/dependencies atomically, preserve the CodexDD version export, and validate targets before claiming this overlap resolved.
2. `codex-rs/core/tests/suite/scenarios.rs`: old fork removed an unused `ReasoningEffort` import; upstream now adds substantial 0.163 coverage. Rebase the fork-only change on the full upstream test file and check whether the import is newly needed; run targeted integration-test compile before a release-suite run.
3. `codex-rs/core/tests/suite/unified_exec.rs`: original fork includes the trusted adaptive evidence-ID regression, and 0.163 adds a separate command-origin metadata persistence regression. Both must survive together. Preserve their distinct names and security intent; do not replace the entire fork file.
4. `codex-rs/history/src/tests.rs`: production contains CodexDD-specific workflow-state/Guardian history regressions; upstream 0.163 adds typed `ResponseItem::Message` optional fields and `ContentItem::OutputText` metadata. Apply only source-compatible constructor/pattern changes to the existing tests and run the native `codex-history` tests.
5. `.github/workflows/bazel.yml`: CodexDD intentionally carries increased **90-minute** Bazel CI timeouts. Upstream 0.163 allows manual native-Windows runs and adds the `--config=ci-windows-msvc` release-compile setting. Review and integrate those independently without silently replacing the fork's timeouts.

The previous sixth verbatim-fork overlap, `codex-rs/rollout/src/recorder.rs`, was reconciled in [commit `89307e91`](https://github.com/Takuetsu/codex-dd-astra/commit/89307e9136c8747db43031adb4d56499ddf52a54). It now evaluates `ThreadListRepairMode::StateDbOnly` **before** shortcutting an empty CWD filter, preserving upstream closed-DB errors and the fork's `WorkflowState` item. A new native regression exercises this ordering. [Windows rollout focused run #39491672011](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39491672011) is required to pass before marking behavioral acceptance; it was still running at this packet's creation.

## What this inventory proves — and what it does not

- **Proven:** The four-way Git-blob inventory exactly reproduces the 109 original overlap count across the three repository directories; it distinguishes committed merged identities from unchanged old fork paths. It also disproves stale earlier packet claims that 20 TUI fork paths still lack reviewed blob identities.
- **Not proven:** Every merge contains every desirable upstream semantic change; a changed SHA is not a correctness receipt. The original **15 direct-conflict records** are not replaced by this 109-file classification and still require an explicit path-by-path disposition. Existing 0.163 source/static/native checks remain independently binding.
- **Blocked:** No final 21-stage Windows release suite, release marker promotion, production merge, automatic upstream publisher, installation or artifact release while source conflicts, focused tests or the four unmaintained advisories remain unaccepted.
