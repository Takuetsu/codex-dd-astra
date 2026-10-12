# CodexDD 0.4.3 — Gate 3 release-repair source freeze revision

**Status: NARROW POST-CANDIDATE REPAIR VALIDATED BY DANIEL-CL FORMAT + POWERSHELL FIXTURE; CANDIDATE SUPERSESSION PREPARATION ONLY.**

## Existing owner's execution instruction and scope

The owner explicitly instructed the assistant to continue all CodexDD GitHub source/release work without repetitive permission requests, stopping only when necessary for local Daniel-CL tests. This is a record of that **standing operational authorization**, not an independently signed semantic approval of unrelated source changes. The original [Gate 3 scoped owner source freeze](codexdd-0.4.3-gate3-source-freeze-acceptance.md) applied to the previously frozen source snapshot `a02e316fd4d8d5b338cedff6815ac68ddcd46c79`; this revision **does not retroactively pretend that approval covered new code**.

## Exact change boundary

- Previously published canonical release candidate: [`c092b451c189bbda03bdae292889fccbd1ab07ed`](https://github.com/Takuetsu/codex-dd-astra/commit/c092b451c189bbda03bdae292889fccbd1ab07ed). That candidate passed the native v2 Git/discovery preflight but **failed** the native 21-stage profile on stage 2 (`rust-format-check`) with exit code 1. No 21-stage native release receipt exists for it.
- New product-source repair baseline: [`fe0e375ab422273bab29b6c4e5e3aa45825f47b4`](https://github.com/Takuetsu/codex-dd-astra/commit/fe0e375ab422273bab29b6c4e5e3aa45825f47b4), tree `5a7e31fe366076bb2624908123a8b4c396d78244`. **Only one Rust source file differs** from the earlier approved source: `codex-rs/external-agent-migration/src/sessions/append.rs`, Git blob `47bc6fb417cfdc1e3415cf5f8f0aa1ea8b44be7b` -> `e58ec916846877370fd31d40b1b7d508aa412cfa`. One optional Rust match pattern leading `|` was removed; the `RolloutItem::WorkflowState(_) => return None` reject arm and all surrounding branch logic are unchanged.
- The remaining modified files are **validation-only**, not product-runtime changes: `.gitattributes` (LF evidence checkout), `scripts/codexdd-validate-upstream-candidate-v2.ps1` (single usable Python executable), release/workpacket PowerShell format commands now `cargo fmt --all -- --check`, a synthetic launcher fixture, a resolver helper, and the corresponding technical review document.
- Five read-only 0.4.3 CI workflow product-source fingerprint checks are reanchored to the **exact post-repair executable product source** at `fe0e375ab422273bab29b6c4e5e3aa45825f47b4`. These workflow-only changes should not trigger broad CI ahead of native candidate qualification.

## Actual cheap local evidence

- On Daniel-CL, the isolated `E:\\codexdd-043-harness-check` checkout at `eaa14c7123d64a512fa6d9acb1bf8ba003d433e6` emitted `CODEXDD_NATIVE_RELEASE_HARNESS_TEST PASS`. The same checkout initially failed stable `cargo fmt --all -- --check` in `append.rs`, near line 224; the later manually typed success banner **was not** a PASS.
- The subsequent formatting-only correction at `10e26a282ca0553cd45a9236be1122723d67dff5` (included in the repair baseline above) was checked out at `fe0e375ab422273bab29b6c4e5e3aa45825f47b4` on Daniel-CL. The screenshot confirms `cargo fmt --all -- --check` completed without entering the throw branch and then printed `CODEXDD_043_FORMAT_PASS`.
- The prior `external_agent_migration` appended-history 3/3 targeted unit check passed on the pre-format blob. Its success is **bounded to the earlier source**; no post-format migration unit rerun or 21-stage native PASS is claimed.
- No production executable, installation, version marker, official Rust 0.162.0 upstream pin, generated SDK source, or original 136-path conflict classification was changed by the repair.

## Release continuation without silent evidence reuse

1. Keep the originally blocked 136 overlaps/22 textual conflicts and original discovery artifact SHA-256 hashes immutable. The source delta remains one updated overlap Git blob; the remaining 135 retain prior audited source hashes.
2. Freeze the **actual integration branch HEAD** that includes this revision and exact CI guard changes as the new source snapshot; amend a freshly staged v2 four-file metadata package. The prior staging and the original failed candidate SHA remain historical evidence, **not successful release candidates**.
3. Candidate revisions must be bound to an exact new source commit/tree and new v2 sidecar SHA-256. Do not accept the old candidate's preflight or failed native profile as if they were passing checks for the replacement.
4. All subsequent code/tests remain subject to the distinct release gates: native Daniel-CL candidate 21-stage PASS, proper provenance receipt, final GitHub CI/PR review, production merge, and installation.
5. The canonical candidate branch naming contract is fixed by the v2 tool. If a failed candidate must be superseded, use a documented exact-SHA conditional update with the prior SHA preserved in this review, never an unguarded or silent ref overwrite. Do not mutate current production.

**This review documents the owner-directed technical remediation; it does not manufacture a native release receipt or silently approve future code changes.**
