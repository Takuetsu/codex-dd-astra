# CodexDD 0.4.5 — Packet 1A.3: automatic-upstream-update architecture and reconciled integration contract

**Date:** 2026-10-17  
**Status:** **PACKET 1A.3 DESIGN CHECKPOINT COMPLETE; PHASE 1 GATE 1 OPEN — SOURCE IMPLEMENTATION NOT YET AUTHORIZED BY THIS DESIGN ALONE.**  
**Release:** CodexDD `0.4.5` targeting exactly OpenAI Codex stable `rust-v0.163.0`.  
**Feature / planning branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-automation`.  
**Production:** `dd/astra-policy-v2` remains CodexDD `0.4.4` at `b25550dcb2c6e84a84074b603086c9db6f32a63b` as verified on 2026-10-17.  
**First two Phase 1 packets:** [1A.1 discovery](codexdd-0.4.5-packet-1a1-upstream-0.163.0-reconnaissance.md) and [1A.2 exact 109-path ledger](codexdd-0.4.5-packet-1a2-overlap-security-and-automation-audit.md).

This is the **proposed acceptance contract**, not evidence that the candidate was built, source conflicts resolved, CI passed, or installed machines updated. This checkpoint creates documentation only, on an isolated feature branch; existing production and registered workflows are untouched.

## 1. Verified identity and why 0.163.0 cannot self-promote

| Identity | Pinned value |
| --- | --- |
| Production release source commit | `b25550dcb2c6e84a84074b603086c9db6f32a63b` |
| Production tree | `196252ceb8982ff52d2af3867ee3cd754bca5564` |
| Current CodexDD product identity | `0.4.4` |
| Tracked upstream tag / commit | `rust-v0.162.0` / `c1382380de69521303b416720a52f42d51af6248` |
| Target official stable tag | `rust-v0.163.0` |
| Target annotated tag object | `129b8ec7ee6f4511557a4856279f529025c25af4` |
| Target peeled **commit** | `83086188ae0614bd882818ff1aa23f3194962012` |
| Target upstream tree | `e91f335e7a03e1d14fa1e5fc7fc7620cdfc60842` |
| Canonical eventual candidate ref | `automation/upstream-candidate-rust-v0.163.0-83086188ae06` |
| Reconciled candidate source provenance | To be bound only after accepted source changes, native validation and exact commit/tree freeze |

The existing **read-only scheduled** `.github/workflows/upstream-sync.yml` was still functioning as of [run 39045820878](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39045820878), 2026-10-17 15:36 UTC; it uploaded another `rust-v0.163.0` discovery artifact. Earlier run [38967580282](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282) supplied the originally reviewed blocked manifest. There is no `0.163.0` canonical candidate branch.

The immutable discovery reports **356 fork customization paths**, **1,314 upstream-changed paths**, **109 exact overlaps**, **97 paths classified sensitive**, **12 unclassified overlaps**, and **15 direct transplant conflicts** (13 text and two generated compressed schema blobs). Seven overlapping files were modified by CodexDD 0.4.4. A dry-run transplant returned `blocked_transplant_conflict`. **Do not relabel this original evidence as clean, rewrite its bytes, or pass it to the v1 clean-only prepare lane.**

Any new production or upstream ref must be re-resolved at implementation time. The target cannot move from 0.163.0 without an explicit separate release-scope decision.

## 2. Critical additional discovery: v2 manual reconciliation already exists

The Phase 1 audit initially focused on the 0.4.2 **v1** discovery / clean prepare / promotion flow. There is already a stronger, previously merged **v2** reconciled-conflict toolchain from [PR #50](https://github.com/Takuetsu/codex-dd-astra/pull/50), used to prepare and validate [0.4.3 PR #52](https://github.com/Takuetsu/codex-dd-astra/pull/52).

| Existing lane | Authoritative entrypoints | Current authority |
| --- | --- | --- |
| v1 discovery | `.github/workflows/upstream-sync.yml`, `.github/scripts/upstream_sync.py` | Scheduled/read-only; immutable blocked or clean evidence |
| v1 clean preparation | `.github/workflows/upstream-prepare.yml` | Explicit manually dispatched preparation; fails on conflicts, absent-ref lease |
| v1 validated PR promotion | `.github/workflows/upstream-promote.yml` | Explicit manual dispatch; identity- and Windows-receipt-gated PR only |
| **v2 manually reconciled publication** | `.github/workflows/upstream-reconcile-publish.yml`, `.github/scripts/upstream_reconciliation_stage.py`, `.github/scripts/upstream_reconciliation_git.py`, `.github/scripts/upstream_reconciliation_v2.py` | Owner-only dispatch; original blocked v1 artifact/digests and signed-off source snapshot; builds a metadata-only candidate, guarded absent-ref publication |
| **v2 manually reconciled promotion** | `.github/workflows/upstream-reconcile-promote.yml`, `.github/scripts/upstream_reconciliation_promotion.py`, `.github/scripts/upstream_reconciliation_receipt.py` | Owner-only dispatch; exact v2 source/sidecar/receipt plus native **release-profile PASS** and semantic acceptance; PR only |
| v2 Windows receipts | `scripts/codexdd-validate-upstream-candidate.ps1` -> `scripts/codexdd-validate-upstream-candidate-v2.ps1`, `scripts/codexdd-v2-receipt.ps1` | Head/source/provenance-bound native PASS; no generic shell command or GitHub CI can substitute |

**Reuse v2.** Do not add an undocumented conflict bypass to the v1 `candidate_state` or present a clean-looking v1 manifest for 0.163.0. The v2 manifest must retain the truthful `manual_conflict_reconciled_review_required` state, immutable original blocked artifact SHA-256, exact ordered 15-path conflict evidence, 109-path overlap set, source snapshot SHA/tree, semantic-review sidecar, and Windows full-release receipt.

### v2 reuse blockers verified in actual source

The v2 workflow and helper files currently **hardcode 0.4.3 / rust-v0.162.0** rather than being universal updater components:

- `upstream-reconcile-publish.yml`: fixed `rust-v0.162.0`, expected candidate ref, v0.4.3 source/staging refs, manifest path, artifact name, and concurrency group.
- `upstream-reconcile-promote.yml`: fixed `rust-v0.162.0`, candidate ref, v0.4.3 source ref, review-PR body/title and concurrency group.
- `.github/scripts/upstream_reconciliation_promotion.py`: fixed 0.162.0 target SHA and `ACCEPT_CODEXDD_0.4.3_RECONCILED_<source>` acceptance token.
- `.github/scripts/upstream_reconciliation_v2.py`: fixed 0.4.3 integration source branch check.
- `.github/scripts/upstream_reconciliation_git.py` and `upstream_reconciliation_cli.py`: default 0.4.3 source refs.
- The `codexdd-043-v2-reconciliation-tests.yml` test workflow is specifically keyed to a historical 0.4.3 branch.

**Generalize only through an explicit new owner-pinned version of these contracts with regression fixtures proving all 0.4.3 evidence remains valid.** Treat literal source/staging refs as untrusted input unless matched against a validated, owner-accepted release plan. No runtime `eval`, arbitrary Git ref execution, caller-selected shell or unsafe path interpolation. Preserve old receipts and manifests byte-exact.

### GitHub Actions PR-creation permission is a real operational constraint

During 0.4.3, the exact v2 promotion workflow's prerequisite checks passed, but its `gh pr create` step was rejected by GitHub's Actions-token policy: **“GitHub Actions is not permitted to create or approve pull requests.”** The owner subsequently created review-only PR #52 through a separately authenticated route.

Therefore **do not assume that adding `pull-requests: write` makes scheduled PR creation work**. Respect repository policy. Where GitHub refuses automatic PRs, publish a status summary and an exact branch/evidence pointer and leave PR creation to the connected owner-authorized GitHub account or to a separately approved, sufficiently scoped GitHub App. Never alter repository Actions security settings or attempt to evade this policy silently.

## 3. Two lanes, one immutable identity protocol

### Lane A: routine low-risk release discovery and guarded auto-preparation (new 0.4.5 feature)

`SCHEDULED_DISCOVERY` (existing; 6 hours, read-only) -> `AUTO_ELIGIBILITY_RECHECK` (new, no writes) -> `PUBLISH_ISOLATED_CANDIDATE` (new, narrowly credentialed) -> optional **draft, explicitly UNVALIDATED** PR, only if platform policy permits -> **STOP** at local Windows and owner gates.

**A release is auto-preparation eligible only if every following gate passes**, evaluated from canonical Git objects at the *write decision*, not from a mutable or caller-uploaded JSON:

1. Tag is a strictly newer official stable `rust-vX.Y.Z`; match the canonical upstream annotated/lightweight tag ref, **peeled commit** and tree. No prerelease, tag substitution, unexpected object type or changed tag OID.
2. Current production HEAD/tree and tracked-upstream tag/SHA/tree are re-resolved and agree with the recorded proposal; no partially completed/dirty source, abandoned job, or moved branch.
3. Exact recursive `git ls-tree -rz` diff proves **zero overlapping paths**, including binary, generated, Python/TS protocol, config, build, CLI/daemon, nested repository/path-mode changes; both overlap inventory and policy classifier must be complete. Unknown path types **fail closed**.
4. Synthetic full customization transplant is clean and produces a reproducible candidate tree; upstream changes contain **no protected compatibility, security, network, credentials, permission, Worker/tool API, TUI/Windows daemon, release dependency, schema-generation or version-provenance changes** that require semantic review.
5. Dependency resolution does not roll back an existing pinned security fix, including the CodexDD 0.4.4 `rustls 0.23.46` and `noyalib-serde-yaml 0.0.57`. No invalid lockfile, generated artifact drift, unresolved security advisory, or cargo/protocol ambiguity. A full safe dependency allowlist must be documented; unknown dependency deltas block.
6. The publish target is the deterministic `automation/upstream-candidate-<stable-tag>-<sha12>` branch. No preexisting candidate ref, duplicate active PR, canceled run leaving a partial commit, ambiguous concurrent release or stale parent. Publish with **absent-ref compare-and-swap**; never force overwrite a candidate.
7. All cheap **safe-to-run** static checks and policy/reproducibility tests pass. A clean transplant or green Linux check is **not** semantic or Windows runtime proof. Stop if executing untrusted upstream code would be required under write credentials.

**Zero overlap is necessary but not sufficient.** Even with zero overlap, cross-file API changes may break CodexDD. The first implementation should default to **report-only** for unfamiliar or protected upstream deltas; enabling publication for a reviewed tiny safe class is a separate acceptance checkpoint. No blanket automatic source conflict resolution.

An automatically opened draft PR, where authorized, must carry prominent **UNVALIDATED / NEEDS_WINDOWS_AND_OWNER_REVIEW** status and exact provenance. It may never invoke v1 `upstream-promote.yml` or v2 promotion prematurely, mark a release ready, self-approve, enable auto-merge, tag, install or distribute binaries.

### Lane B: upstream with conflicts or semantic risk (0.163.0 now)

`READ_ONLY_DISCOVERY` -> `BLOCKED_RECONCILIATION_PLAN` -> packetized isolated source resolution -> strict v2 original-evidence and conflict-sidecar attestation -> explicit v2 candidate publication -> native Daniel-CL release-profile receipt -> separately accepted PR/CI -> explicit owner merge/install/deploy.

For 0.163.0 specifically: auto-planner writes an actionable **blocked** status/report, not a fake green branch. Resolve the 13 text conflicts and regenerate two compressed artifacts, then review all 109 overlap paths and indirect 0.163.0 behavior. Preserve the original 15-path blocked report **unchanged** and attach an explicit owner-reviewed resolution/evidence entry for each path in v2.

## 4. Permission, execution and trust boundaries

- **Discovery:** `contents: read` only. Canonical refs fetched from `openai/codex`. Artifact is evidence, **not an authorization token**.
- **Policy evaluation:** trusted scripts pinned to CodexDD's approved production revision. Input is untrusted until canonical official tag/commit/tree and production SHA/tree have been re-derived; recompute every delta and eligibility rule locally, compare exact canonical JSON/hash and record the decision reason.
- **Write job:** distinct job/workflow with minimum `contents: write` only, fixed trusted workflow checkout, no `pull_request_target` executing contributed code, no `workflow_run` artifact escalation, no credentials persisted in upstream checkout, and **no execution of candidate/upstream scripts while write tokens or other secrets are available**. Before writing, repeat identity checks and compare-and-swap remote absent-ref requirements; report any conflict without retries that force a ref.
- **Optional draft PR:** only a separately constrained `pull-requests: write` capability if repository Actions policy expressly permits. If denied, preserve candidate and exact evidence; do not change permissions or claim success. Draft metadata must be bound to the candidate SHA and carry missing native gates.
- **Reconciliation:** v2 exact reviewed source, original evidence and registered semantic sidecar. Human-authored acceptance bound to exact source SHA/tree is not inferred from a model answer, a dry run, a test-only fixture or an artifact alone.
- **Windows acceptance:** Daniel-CL native tests and v2 `release` receipt remain authoritative; separate final 21-stage monitored acceptance and live F2/daemon/resume-fork smoke. GitHub CI comes after cheap local checks, not as a routine trial-and-error substitute.
- **Release:** explicit owner-controlled final PR review/merge, release publication, installer/backups, Daniel-CL then Stonks/Doug deployment. A bot must never write directly to `dd/astra-policy-v2`.

**Threat model to test:** malicious/moved official tag, tag-object versus peeled-commit confusion, user-controlled path escaping `docs/upstream-candidates/`, duplicate JSON keys and noncanonical artifact hashes, spoofed run ID or ref, untrusted text injected into GitHub Actions shell/YAML, stale production during run, branch-race, PR race, unrecognized overlap, generated binary conflict, poisoned Cargo dependencies, historical immutable evidence overwritten, forged PASS receipts, cancelled/rerun job causing repeated writes, and `[skip ci]` on final candidate.

## 5. Explicit decision table and expected statuses

| Input condition | Planned auto-controller state | Writes allowed? | Next step |
| --- | --- | --- | --- |
| No newer stable tag | `up_to_date` | No | Stop and log |
| Tag/prerelease/ref identity cannot be verified | `blocked_identity` | No | Fix canonical resolution |
| Production/tracked source moved or stale artifact | `blocked_stale_base` | No | Fresh discovery, never reuse old hashes |
| Any transplant conflict | `blocked_transplant_conflict` | No | V2 reconciled manual path |
| Any overlap (classified or unclassified) | `manual_semantic_review_required` | No unattended preparation | Risk ledger and owner-reconciled evidence |
| No overlap, but protected upstream files / dependencies / unsafe ancestry changed | `manual_semantic_review_required` | No unattended preparation | Review source/API/security delta |
| Eligible low-risk, clean and all admission tests green | `auto_preparation_ready` (new policy decision, not a reinterpretation of v1 state) | Publish *isolated branch only* after identity recheck | Optional draft PR only where permitted; wait for Windows |
| Existing ref/PR or a concurrent candidate | `blocked_duplicate` | No | Idempotent report existing exact identity |
| Transient process/infrastructure failure | `blocked_incomplete` | No | Preserve diagnostic, no false PASS |
| Reconciled 0.163.0 after full source review | v2 `manual_conflict_reconciled_review_required` | **Owner-gated** v2 publish only | Native 21-stage Windows receipt and review-only promotion |

The v1 `preparation_ready` state alone is **not** sufficient for the new auto-publish decision: today its `sensitive_overlap_count == 0` check could miss the twelve currently unclassified overlaps. Add an independent auto-eligibility evaluator with a default-deny policy. Preserve v1/v2 historical receipt contracts and never relabel the original blocked 0.163.0 discovery.

## 6. Git branch ownership and conflict-safe release route

- **Development / packet checkpoint:** `dd/codexdd-v0.4.5-upstream-0.163.0-automation`, the existing feature branch. Docs and accepted updater changes belong here, in numbered packets. Every packet ends in an exact Git SHA checkpoint; assistant owns GitHub source edits.
- **Isolated reconciled upstream source:** use a separate `dd/codexdd-v0.4.5-upstream-0.163.0-integration` branch, **created only after Gate 1 acceptance** from a re-resolved approved baseline/updated tooling. Track the precise source snapshot commit and tree and freeze after all semantic resolutions. Do not mix speculative automation work into a source freeze.
- **V2 metadata/staging:** use a new 0.4.5 exact-identity owner-controlled staging branch (e.g. `dd/codexdd-v0.4.5-v2-attestation-staging`) derived from the then-current production SHA, containing only the required original blocked evidence and v2 attestation paths. Do **not** reuse 0.4.3 staging or receipts.
- **Canonical v2 candidate:** publish exactly once as `automation/upstream-candidate-rust-v0.163.0-83086188ae06` after source and sidecar acceptance. A previous candidate ref at that name means stop and inspect, never overwrite automatically.
- **PR:** retain `dd/astra-policy-v2` as base. Source/manifest/sidecar/receipt and final tested candidate SHA must match; real GitHub required checks cannot be replaced by historical `[skip ci]` runs.
- **Daniel-CL worktrees:** when operator action is needed, create worktrees **inside `E:\codexdd`**, with independent paths and no source-edit tasks delegated to Daniel.

The historical v2 tooling currently hardcodes 0.4.3 refs. The 0.4.5 implementation must parameterize and validate exact owner-pinned ref inputs, or introduce separately pinned 0.4.5 workflows, without changing previously merged/accepted 0.4.3 evidence. Preserve immutable manifest SHA-256 and original conflict digest as first-class validation inputs.

## 7. Numbered implementation and integration work packets

### Phase 2 — safe automatic-update enhancement (after Gate 1)

| Packet | Exact source ownership | Acceptance evidence |
| --- | --- | --- |
| **2A.1** | `.github/scripts/upstream_sync.py`, `test_upstream_sync.py`: add 12-path classifier coverage and default-deny overlap policy without corrupting historical v1 manifests | Positive/negative tree-Git fixtures for all 12 gaps; 0 overlaps versus unclassified overlap; stable/prerelease, mode/rename and immutable hashes |
| **2A.2** | Pure, auditable independent auto-eligibility planner and tests under `.github/scripts/` | Deterministic states; protected paths, indirect-risk denylist, security downgrade, stale source/tag, conflict and race matrix; no writes |
| **2A.3** | New **read-only, report-only** scheduled evaluation (possibly integrating into discovery); privilege and workflow security tests | Can run repeatedly without Git writes/PRs; summary includes exact reason and candidate key; no candidate-script execution |
| **2B.1** | New least-privilege gated preparer for a reviewed low-risk admission class; reuse v1 deterministic candidate branch, original provenance and absent-ref lease | Exact checkout/commit/tree, double-check after build, cancellation/race/duplicate tests; no PR/merge/install |
| **2B.2** | Optional draft-PR/reporting adapter; honor repository Actions-token restrictions | Status is unvalidated; explicit failure fallback if `gh pr create` is forbidden; no automatic CI-waiver or merge |
| **2B.3** | Preserve v1/v2 receipt contracts, test workflow permissions/branch paths, audit docs, and durable SHA checkpoints | No regression to existing v1/v2 parser or owner-only release controls; cheap Daniel-CL checks precede final CI where source changed |

Do **not** enable a privileged scheduled writer before 2A.1–2A.3 tests and a separately reviewed safe admission policy have passed. The first useful 0.4.5 automation feature can be enhanced **read-only risk triage**, which is safe even for 0.163.0.

### Phase 3 — actual upstream 0.162.0 → 0.163.0 source integration

| Packet | Owner/subsystem | Explicit mandatory checks |
| --- | --- | --- |
| **3A.1** | Provenance, exact 0.163 tag/commit/tree, current production re-anchor, version and 0.4.4 security-preserved Cargo dependency reconciliation | Keep **`rustls 0.23.46`**, `noyalib-serde-yaml 0.0.57`, and `quinn-proto 0.11.18` unless separately accepted; review new upstream `hyper 1.11.1` and `rmcp 3.5.1` |
| **3A.2** | Bazel/CI/toolchain and generated output ownership | Reconcile `.github/workflows/bazel.yml`, Cargo workspace/lockfile and release packaging; no silent Windows build/test policy regression |
| **3B.1** | Rust app-server protocol sources, TypeScript/JSON/Python exports and two compressed `.json.zst` artifacts | Regenerate from reviewed source-of-truth; preserve CodexDD `turn/adaptiveRuntimeSignal` and new upstream notification/read-state contracts |
| **3B.2** | App-server/core thread, permission/network security | Network proxy permission-before-DNS, `Host` header and credential alias security, side-thread replies, API-key gates, network approvals and policy overrides |
| **3B.3** | History normalization / persistence / resume-fork | Keep CodexDD missing-tool-output abort diagnostics and upstream inherited/retained context; restore managed permission state safely |
| **3C.1** | Protected tool registry, direct tool mode, Native LVO and 0.4.4 fixes | No bypass of reconnaissance no-write, Mechanical Validation no-edit, terminalization restrictions or genuine HEAD-bound receipt on cancellation/parallel calls |
| **3C.2** | Worker binding, adaptive signals, model catalog / Daybreak | Preserve GPT-6.1 Sol / adaptive policy and `/status`; upstream model additions must not silently reroute tiers |
| **3D.1** | TUI/Windows daemon/startup and `/config`, `/keymap` | Preserve F2 elevated embedded startup and explicit elevated daemon blocking; cover managed-daemon plus adaptive-family startup args |
| **3D.2** | TUI paste, steering, Esc/Code Mode cancellation, side threads | Windows Ctrl+V and Shift+Insert, key chords, correct pending-thread attribution, Worker interruption and no fake terminal PASS |
| **3E.1** | Reusable v2 manifest/sidecar/attested staging and pinned workflow generalization | All 15 original conflicts and all 109 overlaps have source-bound review evidence; no rewriting original blocked bytes or 0.4.3 v2 receipts |
| **3E.2** | V2 candidate publication, version/reporting and generated provenance | Source freeze; explicitly assign CodexDD `0.4.5` / tracked `rust-v0.163.0` only in accepted integration; absent-ref candidate and clean local worktree |
| **3F.1** | Full overlap and security/source audit | Resolve every critical finding; attest all 109 paths plus indirect semantics, build/security deltas, source/manifest hashes and failed-run history |

Significant packets must be split further if they exceed the normal five-to-ten-file implementation boundary; no single massive `3B` or `3D` run.

### Phase 4 — release validation and deployment (owner controls)

1. Assistant commits accepted small GitHub source packets. Daniel-CL performs **only relevant cheap local tests** when asked; if a test fails, correct source on GitHub and rerun the failing section only. No repetitive full suite.
2. Run the appropriate native work-packet and targeted profiles as supported, with actual trusted ToolCall/receipt provenance; capture immutable source/candidate identities.
3. When source is frozen, run **one** complete SSH-safe detached/monitored Windows 21-stage release suite on Daniel-CL (including known Windows stack-size/ConPTY guard conditions). A GitHub Ubuntu job cannot replace this Windows profile.
4. Obtain the v2 exact-candidate release-profile PASS receipt, owner-accepted semantic review, then run exact-HEAD GitHub required CI; no `[skip ci]` as a release shortcut.
5. Separately obtain owner approval for PR/merge, release publication, backup/installation on Daniel-CL, interactive elevated F2 and `/status`, explicit elevated-daemon deny check, ordinary/non-elevated startup where testable, resume/fork, and Stonks/Doug deployment.

## 8. Gate 1 decision record and immediate handoff

**Reconnaissance is complete for Phase 1 (1A.1–1A.3)** and the default safety decision is **NO-GO for unattended 0.163.0 integration, auto-PR promotion, automatic product version changes or installation**. This design recommends **GO for a narrowly scoped, read-only 2A.1 classifier and denial-policy implementation only after the architecture is accepted**. Automated publishing remains a later, separately risk-checked permission gate.

**Outstanding explicit acceptance boundaries:**
- Owner/source-architecture acceptance for Phase 1 before runtime/workflow edits or manual conflict reconciliation.
- Later owner acceptance of any privileged scheduled writer, any exceptional semantic risk, complete v2 source/sidecar review and release, merge or deployment.

**Next executable packet after Phase 1 acceptance:** `2A.1`, classifier coverage and fail-closed auto-eligibility groundwork; no Daniel-CL action before a source-changing packet reaches a meaningful cheap validation gate.

**Checkpoint requirements:** verify this document on `dd/codexdd-v0.4.5-upstream-0.163.0-automation`, record the exact branch SHA, and leave `dd/astra-policy-v2` untouched.

## Evidence links

- [Packet 1A.1](codexdd-0.4.5-packet-1a1-upstream-0.163.0-reconnaissance.md)
- [Packet 1A.2 full 109-path ledger](codexdd-0.4.5-packet-1a2-overlap-security-and-automation-audit.md)
- [Discovery run October 17](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39045820878)
- [Blocked original manifest artifact](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282/artifacts/12075916353)
- [Upstream stable 0.163.0](https://github.com/openai/codex/releases/tag/rust-v0.163.0)
- [Historical v2 reconciliation tooling](https://github.com/Takuetsu/codex-dd-astra/pull/50)
- [Historical v2 integration / Actions PR-creation blocker](https://github.com/Takuetsu/codex-dd-astra/pull/52)
- [Work-packet standard](codexdd-major-phase-work-packet-standard.md)
