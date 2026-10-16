# CodexDD 0.4.5 — Packet 1A.1: upstream 0.163.0 and automatic integration reconnaissance

**Date:** 2026-10-16  
**Status:** PHASE 1 / PACKET 1A.1 CHECKPOINT — source reconnaissance complete; Gate 1 semantic/architecture acceptance pending.  
**Owner-selected CodexDD release:** 0.4.5  
**Owner-selected upstream target:** OpenAI Codex official stable \`rust-v0.163.0\`  
**Repository:** \`Takuetsu/codex-dd-astra\` (Windows only)  
**Documentation/feature branch:** \`dd/codexdd-v0.4.5-upstream-0.163.0-automation\`  
**Production branch:** \`dd/astra-policy-v2\`  

This is a planning/reconnaissance checkpoint, **not** an upstream integration or 0.4.5 build. Do not change the product version, tracked upstream baseline, runtime, production branch, release artifacts, or installed machines as a consequence of this packet.

## Owner intent and scope

Improve CodexDD's 0.4.2 safe upstream-update orchestrator so that new stable Codex releases require less manual work: scheduled detection should lead to evidence-rich, isolated preparation and mechanical checks where safe, and to a draft integration PR when safe. Do **not** silently overwrite CodexDD customizations or treat a clean textual transplant as semantic proof. Keep native Windows acceptance, owner review, production merge, publication, installation, and deployment as separate human-controlled gates.

CodexDD **0.4.5** is also the requested integration vehicle for official stable **\`rust-v0.163.0\`**. Product and upstream version numbers remain independent; do not advance the upstream target implicitly to a newer release.

## Verified production and official upstream identities

| Identity | Value |
| --- | --- |
| Production HEAD when activated | \`b25550dcb2c6e84a84074b603086c9db6f32a63b\` |
| Production tree | \`196252ceb8982ff52d2af3867ee3cd754bca5564\` |
| Product identity in production | \`0.4.4\` |
| Tracked upstream stable tag | \`rust-v0.162.0\` |
| Tracked upstream peeled commit | \`c1382380de69521303b416720a52f42d51af6248\` |
| Tracked upstream tree | \`4899ef4940a8bc7fdf7873aa0d3f438085b8163f\` |
| Target upstream stable tag | \`rust-v0.163.0\` |
| Target official annotated tag object | \`129b8ec7ee6f4511557a4856279f529025c25af4\` |
| Target upstream peeled **commit** | \`83086188ae0614bd882818ff1aa23f3194962012\` |
| Target upstream tree | \`e91f335e7a03e1d14fa1e5fc7fc7620cdfc60842\` |
| Recorded merge base | \`ac9b5b8380517ded445b09dd3196d8d9e2ba3c59\` |
| Upstream-vs-fork ancestry | \`diverged\` |

OpenAI published the stable upstream 0.163.0 release on **2026-10-15**: https://github.com/openai/codex/releases/tag/rust-v0.163.0 . The upstream tag object was verified from the canonical \`openai/codex\` Git ref and peeled to a **commit** (not a tag object).

Production product identity \`0.4.4\` and upstream identity \`rust-v0.162.0\` were read directly from the source files on the live default production branch. Production HEAD/tree were resolved from the live Git ref and commit object, not copied from older design notes. PR #53 (0.4.4 DD-BL-006) was merged on 2026-10-16 at this production HEAD. Re-resolve HEAD/tree before any later candidate preparation: a documentation commit or other production update invalidates an earlier manifest's production identity.

## Actual scheduled discovery evidence

- Workflow: \`.github/workflows/upstream-sync.yml\`; read-only permissions, GitHub Actions schedule \`17 */6 * * *\` and manual dispatch.
- Most recent verified run: https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282
- Run result: **SUCCESS** on 2026-10-16, with successful immutable candidate manifest and dry-run steps.
- Artifact: \`upstream-candidate-rust-v0.163.0-38967580282\` (artifact ID \`12075916353\`).
- Artifact manifest identity: \`rust-v0.163.0@83086188ae0614bd882818ff1aa23f3194962012\`.
- Candidate state: **\`blocked_transplant_conflict\`**; synthetic transplant dry run: **\`conflict\`**.
- This is **successful detection** and **correct fail-closed refusal**, not a successful integration or candidate publication. No auto-PR/merge/install occurred.

### Exact tree-delta inventory from the immutable discovery manifest

| Measure | Exact count |
| --- | ---: |
| CodexDD customization paths relative to tracked upstream | 356 |
| Official upstream changed paths from 0.162.0 to 0.163.0 | 1,314 |
| Path overlaps | 109 |
| Sensitive overlaps (union of classified paths) | 97 |
| Unmerged paths in synthetic transplant dry run | 15 |

Sensitivity categories overlap; their counts must **not** be summed to calculate the 97 unique sensitive paths:

- TUI runtime: 44; core runtime: 29; app-server runtime: 10.
- Persistence/resume/fork: 12; worker lifecycle: 6.
- Generated protocol: 9; configuration: 3; build/release CI: 3; status rendering: 2.

The 15 direct textual/transplant conflicts are:

1. \`.github/workflows/bazel.yml\`
2. \`codex-rs/Cargo.lock\`
3. \`codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst\`
4. \`codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst\`
5. \`codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts\`
6. \`codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts\`
7. \`codex-rs/core/src/context_manager/history_tests.rs\`
8. \`codex-rs/core/src/context_manager/normalize.rs\`
9. \`codex-rs/core/src/tools/registry.rs\`
10. \`codex-rs/tui/src/app/startup.rs\`
11. \`codex-rs/tui/src/app/tests/session_lifecycle_requests.rs\`
12. \`codex-rs/tui/src/app/tests/startup_defaults_tests.rs\`
13. \`codex-rs/tui/src/app_server_session/workspace_roots_tests.rs\`
14. \`codex-rs/tui/src/chatwidget/interaction.rs\`
15. \`codex-rs/tui/src/lib.rs\`

The conflict count is not a measure of integration difficulty on its own: **97 sensitive overlap paths require semantic review even if textually conflict-free**.

## Verified existing orchestration boundary (0.4.2 baseline, still present in 0.4.4)

- **Discovery:** \`.github/workflows/upstream-sync.yml\`, scheduled six-hour or manually dispatched; repository read-only, canonical stable tag/SHA resolution, immutable manifest, overlap classification, synthetic dry run, artifact; no branch write or PR.
- **Preparation:** \`.github/workflows/upstream-prepare.yml\`, manual \`workflow_dispatch\` with exact accepted production and upstream identities; guarded deterministic candidate branch publication, conflict/staleness/duplicate checks.
- **Validation:** \`scripts/codexdd-validate-upstream-candidate.ps1\` supplies local, candidate HEAD/manifest-bound native receipt; cheap Daniel-CL validations before broad CI.
- **PR promotion:** \`.github/workflows/upstream-promote.yml\`, manual \`workflow_dispatch\` requiring native validation receipt plus explicit accepted semantic-review evidence for sensitive overlaps.
- **Production controls:** No automatic product version change, candidate PR on discovery, squash merge, install, release publication, or deployment.

The existing workflow detected 0.163.0 automatically, **but correctly cannot promote this conflict-bearing and semantic-risk-heavy change** under its safety contract.

## Compatibility/risk map for the next reconnaissance packets

1. **0.4.4 and LVO:** Preserve trusted \`run_codexdd_validation\` work-packet receipts, independent Validation Worker behavior, Implementation Worker readiness gates, and bounded BLOCKED terminal handling from merged PR #53.
2. **Adaptive effort, model catalog and Worker state:** Preserve GPT-6/GPT-6.1 routing, Sol/Astra policy, complexity-floor and Worker binding, \`/status\`, budget governance and provenance. Do not inherit upstream capability flags or unsupported catalog changes unreviewed.
3. **Windows/TUI and daemon:** Preserve elevated SSH F2 safe embedded startup, explicit elevated daemon prohibitions and privilege boundaries; review upstream 0.163.0 \`/config\`, \`/keymap\`, \`/daemon\`, paste, side-conversation, and cancellation changes in overlapping TUI files.
4. **Core/app-server, permissions and resume/fork:** Review follow-up steering, tool lifecycle, interrupted Code Mode, persisted permission/approval restoration, IPC event handling and schema changes for security, compatibility, and state isolation.
5. **Generated artifacts and build:** Regenerate and reconcile schema/precomputed TypeScript and JSON, lockfile/toolchain and Bazel changes using repository-owned generators; neither blanket \`--ours\` nor blanket \`--theirs\` is an acceptable conflict-resolution policy.
6. **Full upstream delta:** Include stable \`0.162.1\` fixes on the path to \`0.163.0\`; do not rely solely on the release notes or the 15 conflict paths to define complete coverage.

## Proposed safe 0.4.5 automation direction — NOT YET APPROVED OR IMPLEMENTED

- Keep scheduled read-only discovery as an independent unprivileged step, with immutable manifest and retention.
- Design a separate bounded, write-scoped auto-candidate controller that **re-derives and verifies** the official tag/peeled SHA, production tree, ownership and manifest identity before writing; do not trust arbitrary data or code from a lower-privilege workflow/artifact.
- Only automatically create/update a uniquely identified isolated candidate/draft PR for a **provably eligible** conflict-free update with fully admissible validation evidence. Introduce deterministic deduplication, throttling, audit links, and fail-closed handling of races, incomplete runs and changed production/tag identities.
- For conflict or semantic-risk-heavy releases such as \`0.163.0\`, emit an actionable evidence report and stop; do **not** claim a tested branch exists or auto-approve overlapping runtime changes. Draft PR creation must not imply passing Windows release validation or accepted semantic review.
- Reserve native Windows-local verification, semantic decisions, final GitHub CI, production merge, product version allocation, publishing, installing on Daniel-CL/Stonks/Doug and deployment for explicit acceptance.
- Do not add a scheduled privileged workflow until the trust boundary and conditions are reviewed and the scope is accepted.

## Numbered packet plan and gates

**Phase 1 (reconnaissance, source read-only):**

- **1A.1 (this checkpoint):** verify production identity, official stable tag and SHA/tree, scheduled evidence and existing updater boundaries; enumerate observed conflicts. **COMPLETE**.
- **1A.2:** inspect each of the 15 conflicts plus the full 109 overlap paths by subsystem; map 0.163.0 security/API changes, schemas/build generation and 0.4.4 regressions; identify genuinely automatable versus semantic-review-only paths.
- **1A.3:** approve the architecture for any auto-preparation/draft-PR path, job privileges/identity checks, eligibility and blocked-result reporting, ownership of 0.163.0 integration branches and numbered work packets.

**Gate 1:** accept reconnaissance, exact immutable target and new trust-boundary rules before any upstream candidate preparation or runtime/workflow source edits.

**Phase 2 (implementation, after Gate 1):** packetized parser/eligibility and regression tests; guarded auto-candidate orchestration; duplicate and stale-state handling; audit/PR reporting; narrow GitHub-side source checkpoints. Preserve manual promotion path and stop on conflicts.

**Phase 3 (upstream source integration, after Gate 1):** isolate target \`rust-v0.163.0\`, reconcile generated/build and 15 text conflicts, core/app-server/TUI/adaptive/Worker/LVO/Windows semantics and 97 sensitive overlaps; update tracked upstream provenance and explicit CodexDD product version only on accepted integration branch; review full 1,314 upstream-path delta; audit each numbered subsystem packet.

**Phase 4 (release):** cheap Daniel-CL SSH native checks first; targeted/profile tests; final monitored Windows 21-stage release suite; exact-HEAD CI; separate owner-controlled merge, release, local install/backups, F2/\`/status\`, and downstream Stonks/Doug deployment. No unattended installation.

## Checkpoint contract

- The \`0.4.5\` feature branch is intentionally documentation-only at this checkpoint and rooted in exact production SHA \`b25550dcb2c6e84a84074b603086c9db6f32a63b\`.
- No candidate branch, upstream cherry-pick, production change, workflow dispatch, PR, or local Windows test is warranted for packet 1A.1.
- Next executable packet: **1A.2 — conflict and semantic delta reconnaissance**, with separate durable evidence/checkpoint.
- Re-verify production and tag identities before any later promotion, as scheduled evidence may become stale.

## Evidence links

- Official upstream release: https://github.com/openai/codex/releases/tag/rust-v0.163.0
- CodexDD merged updater: https://github.com/Takuetsu/codex-dd-astra/pull/47
- CodexDD merged 0.4.4: https://github.com/Takuetsu/codex-dd-astra/pull/53
- Exact scheduled discovery run: https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282
- Discovery manifest artifact: https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282/artifacts/12075916353
- Existing planning precedent: \`docs/codexdd-0.4.3-upstream-0.162.0-design.md\`; repository work-packet standard: \`docs/codexdd-major-phase-work-packet-standard.md\`.
