# CodexDD 0.4.3 — OpenAI Codex rust-v0.162.0 integration

**Status:** QUEUED — NOT ACTIVATED; reconnaissance and implementation have not started.

**Activation phrase:** `activate 0.4.3`

**Release intent:** CodexDD `0.4.3` (explicit owner-selected product version; upstream discovery must not allocate it).

**Target platform:** Windows only. **Validation host:** Daniel-CL, accessible to the operator through SSH from Stonks; no Remote Desktop is available.

## Objective and release scope

Advance the CodexDD fork's tracked OpenAI Codex upstream from `rust-v0.159.2` to the owner-selected **stable `rust-v0.162.0`** while preserving all CodexDD 0.4.2 functionality, the subsequently merged Windows elevated-SSH F2 hotfix, and existing security/fail-closed behavior.

Make **CodexDD 0.4.3** the dedicated upstream-integration release, using the safe, explicit 0.4.2 discovery → identity-bound preparation → local validation receipt → PR promotion workflow. This is **not** a new redesign of that orchestration.

Changes across the intervening 0.160.x and 0.161.x release lines must be reconciled as part of the full 0.159.2 → 0.162.0 delta. The 0.162.0 target is deliberate; do not silently substitute a newer tag if one appears.

## Verified planning anchors (2026-10-09)

- Fork: `Takuetsu/codex-dd-astra`; production branch: `dd/astra-policy-v2`.
- Last deployed source/runtime hotfix merge: `dc10de2b0240ebe51bba009bfa224fc305c9aa6a` (PR #48); installed binary `codexdd 0.4.2+gdc10de2b0240`.
- CodexDD version at planning: `0.4.2`, from `codex-rs/codexdd-version.txt`.
- Tracked upstream at planning: `rust-v0.159.2`, from `codex-rs/upstream-codex-release.txt`.
- Official upstream target: `openai/codex` tag `rust-v0.162.0`, published as a stable release on 2026-10-08.
- Annotated target tag object: `1f3f93473394b620b35580859b7e6864f7a9f948`.
- Target **peeled commit SHA**: `c1382380de69521303b416720a52f42d51af6248`. Use the commit SHA, **not** the annotated tag-object SHA, for candidate manifests and preparation inputs.
- The production branch will advance when this planning document and roadmap are committed. **On activation, resolve the then-current production HEAD and tree afresh.** The deployed binary SHA above is historical deployment evidence, not a permanently pinned preparation base.
- Upstream target tag, tracked tag, commit/tree identities, candidate-manifest freshness, and live production SHA must all be reverified before any preparation, promotion, or edit.

The target and release number are approved **planning scope**, not acceptance of the compatibility risks or authorization for unattended merge/install.

## Hard invariants

1. **Reconnaissance first.** Do not modify runtime, TUI, routing, upstream provenance, version files, or generated code before Phase 1 is inspected and explicitly accepted.
2. Preserve GPT-6/GPT-6.1 adaptive effort tiers, model preference and escalation policy, Worker/Designer binding, complexity floor, budget governance, Local Validation Orchestrator (LVO), trusted signals, interruption recovery, and resume/fork persistence.
3. Preserve the 0.4.2 upstream-orchestration safety contract: discovery is read-only; preparation is explicit and isolated; upstream version is separate from CodexDD product version; candidate tag/commit/tree/base identity is immutable; sensitive overlaps require semantic review; native local receipts bind promotion; ambiguous state fails closed.
4. Preserve the elevated-Windows SSH F2 repair: implicit startup chooses embedded mode without the expected-warning noise; failed elevation probes keep a diagnostic; explicit elevated daemon `start`/`restart`/`bootstrap` remain blocked. Do not bypass Windows ACL or process-breakaway restrictions to make a test pass.
5. Limit this release to the upstream integration and **necessary** compatibility fixes. Do not add unrelated adaptive features, Breakwater/Cycle Vengeance work, alternative model migration, or opportunistic daemon refactors.
6. Do not silently fast-forward production, auto-merge, publish releases, install on Daniel-CL, change external machines, or update the installed CodexDD version as a side effect of discovery/preparation.
7. Phase gates and release-shaped validation remain explicit; CI is not a substitute for applicable Windows-native validation.

The **one-time PR #48 waiver** of the live non-elevated detached shared-daemon smoke test did not verify that path. **Do not carry the waiver automatically into 0.4.3.** If the same SSH-only limitation persists, document the limitation and request a fresh release-scoped decision.

## Required workflow and numbered work packets

Follow `docs/codexdd-major-phase-work-packet-standard.md`: keep packets small, checkpoint exact Git SHAs, and stop at acceptance gates rather than collapsing reconnaissance, implementation, testing, and deployment into one step.

### Phase 1 — read-only reconnaissance and accepted plan

**Entry packet on `activate 0.4.3`: 1A.1.**

- **1A.1 — source, tag, and discovery baseline.** Resolve the live production HEAD/tree and clean-branch state; confirm tracked `rust-v0.159.2`; revalidate official `rust-v0.162.0` tag/peeled commit/tree; inspect the 0.4.2 discovery workflow and any existing candidate artifacts. Reject stale manifests from before the new planning-document commits. Record actual repository states; do not create a candidate or change code.
- **1A.2 — complete change/overlap reconnaissance.** Produce exact tree-diff inventories for fork customizations and upstream 0.159.2 → 0.162.0, including changed/generated files, textual conflicts, sensitive semantic overlap, upstream ancestry, Cargo/Rust/toolchain/version shifts, CI/Windows packaging changes, and security implications. Assess 0.160.x/0.161.x interim changes without narrowing the comparison to only 0.162.0 release notes.
- **1A.3 — integration contract and packet ownership.** Decide whether/how the 0.4.2 deterministic `automation/upstream-candidate-<tag>-<sha12>` branch and native validation-receipt contract will be used for the 0.4.3 implementation/release. Document the precise branch ownership, product `0.4.3` version-allocation point, conflict inventory, expected generated artifacts, validation plan, and stop/rollback gates.

**Gate 1:** Present evidence, risks, exact target identities, and an accepted source-integration plan. **No implementation until reconnaissance is accepted.**

### Phase 2 — bounded, explicitly approved candidate preparation

- **2A.1 — candidate manifest.** Use the 0.4.2 read-only upstream discovery tooling when appropriate; ensure the manifest binds the _current_ production SHA/tree, tracked 0.159.2 SHA/tree, target 0.162.0 peeled commit/tree, exact overlap inventory, and semantic-risk classification.
- **2A.2 — isolated transplant preparation.** Only after Gate 1 acceptance, explicitly prepare the target via `.github/workflows/upstream-prepare.yml` and its identity-bound inputs. Do not overwrite an existing candidate branch; stop on stale production/tag identity or textual conflict. No unattended PR.
- **2A.3 — preparation audit and checkpoint.** Verify the prepared tree/manifest/branch HEAD and failure reports; settle the authorized release-integration branch strategy without breaking 0.4.2 receipt/promotion requirements. Record the first durable checkpoint.

**Gate 2:** Candidate and ownership plan accepted. A clean textual transplant is **not** semantic approval.

### Phase 3 — source reconciliation by subsystem

Phase boundaries are intentionally split into numbered packets based on Phase 1 overlap evidence; do not assume every phase fits in one agent run.

- **3A — scaffold and ownership:** upstream baseline, immutable provenance, sensitive overlap owners and integration checkpoints.
- **3B — core/runtime:** adaptive policy/trusted signals, workflow lifecycle, persistence/resume/fork and app-server; separate source reconciliation, generated-artifact work, and narrow tests.
- **3C — TUI/Windows daemon:** terminal behavior, copy/paste and alternate-screen regressions, automatic embedded fallback for elevated SSH, F2 diagnostics, explicit elevated-daemon security guards, and other TUI overlap.
- **3D — model/catalog/status:** GPT-6/GPT-6.1 model capabilities, Astra/Sol/Luna adaptive tiers, `/status` reporting and CLI compatibility. No routing changes just because upstream has a newer model catalog.
- **3E — LVO/updater/version/build:** preserve local validation orchestration; 0.4.2 read-only upstream planner and manual preparation/promotion; update `codex-rs/upstream-codex-release.txt`, Cargo/protocol/generated resources, and explicitly assign `codexdd 0.4.3` **only on the accepted integration branch**. Keep native receipt/manifest/provenance identities consistent after every source commit.
- **3F — comprehensive source audit:** review every overlap, integration omission, unsafe behavior change, generated artifact drift, build/provenance delta, and outstanding item; checkpoint the exact candidate HEAD.

**Gate 3:** No unresolved required semantic overlaps; accepted provenance, versioning, and narrow Windows-target evidence.

### Phase 4 — Windows validation, CI, and production release

- Run repository-owned narrow checks then the LVO/work-packet validation where available. Use **cheap Daniel-CL SSH PowerShell gates** for operator-required native checks, not repeated long, manually authored scripts or source edits.
- Run the full relevant release validation profile once the integrated candidate is stable (including appropriate `RUST_MIN_STACK=16777216` for known Windows stack-overflow-prone Rust tests).
- Test the release-shaped operator experience from **elevated SSH**: `codexdd` startup without `--no-daemon`; **F2: No warnings**; `/status` confirms Adaptive Effort and Astra. Explicit elevated daemon actions must still be rejected.
- Address any non-elevated daemon acceptance gap explicitly; SSH-only access must not be misrepresented as a passing desktop check.
- Validate new 0.162.0 capabilities and regressions relevant to Windows, TUI, code mode, app-server, and LVO. Ensure configuration, credentials, session persistence, and protected modes do not regress.
- Only after local acceptance, create a native validated candidate receipt and use `.github/workflows/upstream-promote.yml` where its contract applies. PR/CI follows local gates.
- CI green, explicit owner-approved squash merge, Daniel-CL installation with backup, final elevated SSH F2/`/status` smoke, and soak are **separate, explicit owner gates**. Do not merge or install autonomously as part of discovering a newer tag.

## Execution/ownership contract

- **Assistant:** reconnaissance, design, GitHub-side source/code/docs changes, conflict resolution, test automation, reviews and durable work-packet commits.
- **Operator:** only short, meaningful native validation commands over SSH on Daniel-CL when required, acceptance/waivers, and consequential release approvals. No operator-authored Rust/PowerShell source patches, manual package assembly, or repeated remote task-scheduler troubleshooting.
- Prefer existing repository automation, LVO native receipts, and source audit to ad hoc CI/test scripts.
- No Remote Desktop assumption. If a gate truly needs an ordinary desktop, present the limitation and request a decision instead of generating a long workaround.
- Do not conflate CodexDD's product version `0.4.3` with upstream Codex `0.162.0`.

## New-chat activation handoff

When the operator starts a new chat with **`activate 0.4.3`**:

1. Read **this** design and `docs/codexdd-roadmap.md` from `dd/astra-policy-v2` and honor `docs/codexdd-major-phase-work-packet-standard.md`.
2. Resolve the **live** production HEAD/tree (not the last shipped runtime SHA), confirm product/tracked-upstream files and official target tag/commit, and inspect any existing 0.4.2 discovery manifest and candidate branch without assuming freshness.
3. Announce **CodexDD 0.4.3, Phase 1 packet 1A.1** and perform read-only reconnaissance immediately on GitHub.
4. Record a durable 1A.1 report and checkpoint; work autonomously on GitHub-side **reconnaissance only** until a meaningful acceptance decision is required. **Do not modify runtime/source code, run an upstream transplant, open a PR, or install anything yet.**
5. After Phase 1 is accepted, proceed with numbered packets and ask for Daniel-CL SSH commands only at the appropriate cheap native validation gate.

**Activation is not approval to change target versions, skip reconnaissance, merge, or install.**
