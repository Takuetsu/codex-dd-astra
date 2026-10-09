# CodexDD roadmap

## Current production state (2026-10-09)

**CodexDD 0.4.2 is deployed and operational, including the elevated Windows SSH F2 warning hotfix.**

- Production branch: `dd/astra-policy-v2` in `Takuetsu/codex-dd-astra`.
- Last deployed **runtime source/merge SHA**: `dc10de2b0240ebe51bba009bfa224fc305c9aa6a` (F2 PR #48).
- Daniel-CL installed executable reports: `codexdd 0.4.2+gdc10de2b0240`.
- Product identity: `0.4.2`; tracked OpenAI Codex upstream: `rust-v0.159.2`.
- Platform: Windows only; authoritative validation host Daniel-CL; access from Stonks is **SSH only** (no RDP).
- Installation and production `/status` verified; F2 from elevated SSH showed **Warnings → No warnings**, without `--no-daemon`.
- The live non-elevated shared-daemon startup/attachment test was **explicitly waived for PR #48 only** due to restrictive scheduled-task job behavior; it remains unverified, not passed.
- **Important:** Planning-document commits may advance the GitHub production HEAD beyond the deployed runtime SHA. Always re-resolve the live production HEAD/tree before a new candidate manifest, feature branch, or upstream preparation. Do not assume that the deployed binary SHA is the current branch tip.

### Recently completed milestones

- **0.4.0:** Local Validation Orchestrator (LVO), merged as PR #45.
- **0.4.1:** GPT-6.1 Sol tier and adaptive routing/catalog integration, merged as PR #46.
- **0.4.2:** Read-only automatic upstream discovery with explicit identity-bound preparation, native local validation receipts, and manual promotion; merged as PR #47 (`8336ca046af71cbf032c76d933ca371336f92673`).
- **0.4.2 F2 hotfix:** Elevated Windows SSH automatic embedded startup without the misleading warning; merged as PR #48 (`dc10de2b0240ebe51bba009bfa224fc305c9aa6a`) and **verified after production installation**.

Historical implementation, provenance and validation material is preserved in:

- `docs/codexdd-0.4.1-gpt-6.1-sol-design.md`
- `docs/codexdd-0.4.2-automatic-upstream-update-design.md`
- `docs/codexdd-0.4.2-phase-2-implementation.md`
- `docs/codexdd-0.4.2-phase-3-source-audit.md`
- `docs/codexdd-elevated-windows-f2-hotfix.md`

## NEXT: CodexDD 0.4.3 — OpenAI Codex rust-v0.162.0 integration

**Status: QUEUED — NOT ACTIVATED.** Begin only when the operator starts a new chat with **`activate 0.4.3`**.

**Authoritative design / activation handoff:**

`docs/codexdd-0.4.3-upstream-0.162.0-design.md`

### Objective

Upgrade the fork's tracked OpenAI Codex version **from `rust-v0.159.2` to the user-selected stable `rust-v0.162.0`**, integrating required compatibility changes across the intervening 0.160.x and 0.161.x release lines. Release as **CodexDD `0.4.3`** only after normal source, Windows, CI, and owner acceptance gates.

Official target planning identity (reverify at activation):

- Upstream repository: `openai/codex`.
- Stable tag: `rust-v0.162.0` (published 2026-10-08).
- Annotated tag object: `1f3f93473394b620b35580859b7e6864f7a9f948`.
- Peeled target **commit**: `c1382380de69521303b416720a52f42d51af6248`.
- No silent target change to 0.162.1 or a later release; require the operator's explicit scope approval for retargeting.

### Working plan

1. **Phase 1: Read-only reconnaissance.** Start **packet 1A.1** on activation. Reconfirm live production HEAD/tree, upstream target identity and tracked upstream, inspect current 0.4.2 planner artifacts, and map exact source/semantic overlaps through packets 1A.1–1A.3. **No runtime/source edits or upstream preparation before reconnaissance acceptance.**
2. **Phase 2: Safe candidate preparation.** Reuse the 0.4.2 read-only `upstream-sync.yml`, explicit `upstream-prepare.yml`, immutable manifest and ownership checks. Avoid stale/duplicate candidate branches. Coordinate release-specific Git branch/manifest/receipt identity with the existing `automation/upstream-candidate-<tag>-<sha12>` contract.
3. **Phase 3: Source integration and audits.** Divide into 3A (scaffold), 3B (core/runtime), 3C (TUI/Windows F2), 3D (model/catalog/status), 3E (LVO/updater/version/build/provenance), 3F (full source audit), with numbered, checkpointed work packets.
4. **Phase 4: Windows acceptance and release.** Native cheap Daniel-CL/LVO validation before broad release profile, then candidate HEAD-bound receipt, `upstream-promote.yml`-compatible PR/CI, explicit squash merge, install with backup, elevated SSH `F2`/`/status` smoke and soak.

Preserve GPT-6.1 Sol/Astra policy, adaptive state, LVO, 0.4.2 read-only discovery and human-controlled promotion, F2 elevated SSH embedded fallback, Windows daemon privilege boundaries, and resume/fork persistence. Do not include Breakwater or Cycle Vengeance work in this release.

### Owner / assistant workflow

- The assistant handles GitHub-side design, code changes, integration, and automation; the operator does **not** manually edit CodexDD source or assemble packages.
- The operator provides only short, meaningful Windows validation commands over SSH to Daniel-CL when needed. Prefer LVO-native receipts and existing repo profiles, not repeated ad hoc scripts or reliance on RDP.
- Use `docs/codexdd-major-phase-work-packet-standard.md`. Significant phases contain numbered work packets and durable Git SHA checkpoints.
- Manual approval is required for reconnaissance acceptance, sensitive semantic-risk decisions, any test waiver, CI/PR promotion, merge, installation and deployment.
- The F2 hotfix's previous non-elevated daemon test waiver **does not automatically extend to 0.4.3**.

### New-chat activation contract

When the operator says **`activate 0.4.3`**:

1. Read the 0.4.3 design and this roadmap from the live production branch.
2. Re-fetch the current production SHA, tracked upstream and official 0.162.0 target, and check for newer/stale planning artifacts.
3. Start **Phase 1 / packet 1A.1** immediately as read-only GitHub reconnaissance. Prepare a durable report and stop at the acceptance gate.
4. Do not change runtime/routing/source, prepare upstream, open a PR, merge, or install until the applicable later gates are explicitly accepted.

## Preserved backlog

- Windows release packaging / tagged release automation originally explored in PR #13, except pieces explicitly reused by 0.4.2.
- Later CodexDD adaptive-policy enhancements not required for upstream-update orchestration.
- Lifecycle-aware complexity-floor and budget-aware de-escalation follow-ups not already completed.
- Other enhancement items previously captured in CodexDD planning material.
