# CodexDD 0.4.3 — candidate release harness repair (nonproduction)

**Status: PATCH PREPARED; CHEAP NATIVE FIXTURE PENDING; FROZEN SOURCE AND CANONICAL CANDIDATE UNCHANGED.**

## Observed Daniel-CL release blockers

1. At the canonical v2 candidate `c092b451c189bbda03bdae292889fccbd1ab07ed`, strict read-only preflight passed and four canonical discovery/sidecar SHA-256 hashes matched, but Windows PowerShell 5.1 `Get-Command python` returned both installed Python and a WindowsApps alias. Passing the resulting `.Source` array to the call operator failed before release stages.
2. After temporarily isolating the real Python interpreter, native release stage 1 `diff-check` **passed**, and stage 2 `rust-format-check` returned **1** after ~52 seconds. The output included repeated upstream `imports_granularity = Item` nightly-only warnings. The previous exact pinned Rust 1.95.0 formatter command `cargo fmt --all -- --check` passed with zero hunks on Daniel-CL. The release/workpacket command was inconsistent: `cargo fmt -- --config imports_granularity=Item --check` forces an unstable override. This is a command discrepancy; the exact differing source hunks were **not** captured in the screenshot, so it is not yet proven that only the override caused the exit.
3. Git for Windows with `core.autocrlf` materialized the four canonical metadata files as CRLF, violating byte-exact preflight. Direct Git blob restoration and index renormalization were needed even though SHA-256 hashes of committed content were correct.

## Scope of repair

- Both PowerShell release profiles now use **`cargo fmt --all -- --check`**, matching the earlier already-passing pinned-toolchain check. The repository's upstream `codex-rs/rustfmt.toml` is not altered. Existing nightly-only warnings, if any, are not suppressed to fabricate success.
- The v2 launcher resolves a **single real** Python executable and rejects the WindowsApps alias; it still runs exactly the same strict v2 Git/source/discovery preflight before a release profile and again afterwards.
- New `.gitattributes` entries require LF checkout of candidate manifest, reconciliation JSON, and original conflict-list files on Windows. Their **committed canonical contents remain unchanged**.
- A cheap synthetic Windows PowerShell fixture exercises alias/valid/missing interpreter selection, identical stable formatting commands and LF attributes; it performs **no cargo build**.

These changes touch only local validation tooling, fixture, attributes and this review document. **No product Rust/SDK code changes.** The canonical candidate is **not** automatically updated: its immutable source SHA, v2 staged metadata, and release receipt provenance must be refreshed in a separately verified corrective release-candidate revision. No old PASS can be silently transferred to a new commit. Publication, production merge and installation remain separate downstream gates.

## Actual Daniel-CL checkpoint — 2026-10-11 local time

The operator fetched the isolated `dd/codexdd-0.4.3-native-release-harness-fix` at `eaa14c7123d64a512fa6d9acb1bf8ba003d433e6` into `E:\\codexdd-043-harness-check`, ran `scripts/codexdd-native-release-harness-tests.ps1`, and observed **`CODEXDD_NATIVE_RELEASE_HARNESS_TEST PASS`**. This is a narrow PowerShell fixture pass, not a release PASS.

The subsequent real `cargo fmt --all -- --check` **failed**, first reporting a diff in `external-agent-migration/src/sessions/append.rs` around line 224. A later manually entered `Write-Host 'CODEXDD_043_HARNESS_AND_FORMAT_PASS'` printed a success-looking label **after the exception**; it is expressly **not valid validation evidence**.

A one-token lexical-formatting fix at [`10e26a282ca0553cd45a9236be1122723d67dff5`](https://github.com/Takuetsu/codex-dd-astra/commit/10e26a282ca0553cd45a9236be1122723d67dff5) removes the optional leading pattern separator `|` before the standalone `RolloutItem::WorkflowState(_) => return None` arm. The active fail-closed authority behavior and native-history handling are unchanged. This patch was isolated to **one Rust source line** and **has not yet passed** a follow-up Daniel-CL formatter check. Earlier 3/3 migration unit regressions were green on the preformat source and remain historical, not a new check of this exact blob.

## Next gate

Run cheap Windows PowerShell fixture and stable `cargo fmt --all -- --check` in an isolated checkout of this repair branch. If green, reconcile source Git objects, make an explicit updated source freeze and rebuild all SHA-bound metadata; only then use the strict candidate-bound native release profile. Do not rerun all 21 stages against the known-broken original script.
