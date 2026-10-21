# CodexDD roadmap

## Active development — CodexDD 0.4.5 (2026-10-21)

CodexDD 0.4.5 integrates the owner-selected OpenAI Codex `rust-v0.163.0` with guarded upstream-discovery improvements. GitHub production remains 0.4.4 / `rust-v0.162.0`, and all current work is nonrelease on `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

- **Native semantics:** exact 15-path upstream conflict ledger in [3F.2](codexdd-0.4.5-packet-3f2-upstream-conflict-replay.md); 109 overlapped paths under audit; **15/15** Windows core filters green in run 39615406741; **25/25** Windows TUI filters green in run 39618752181 ([3F.4](codexdd-0.4.5-packet-3f4-native-tui-conflict-regressions.md)).
- **Updater integration:** import Phase 2 read-only discovery, default-deny eligibility and disabled candidate/draft-PR adapters from `dd/codexdd-v0.4.5-upstream-0.163.0-automation` at `7495858766cf628d480fbac3d6323329884092f3`. The original six packets passed 112/112 targeted Daniel-CL Python tests; the integrated [GitHub source-guard run #39624128524](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39624128524) independently passed **432/432 static** plus **112/112 updater** checks (28 original discovery, 69 guarded modules, 15 v2/privilege audit) on exact SHA `48f7ec292e51d59c358c6c763729cd8a9e9f6f7d`. Existing scheduled discovery remains **read-only**. Automatic candidate branch/PR publishing stays **hard-disabled** with an empty safe-path allowlist. See [2B.3](codexdd-0.4.5-packet-2b3-contract-audit.md) and [1A.3](codexdd-0.4.5-packet-1a3-automation-and-integration-contract.md).
- **Reconciled v2 contract compatibility:** [3F.5](codexdd-0.4.5-packet-3f5-v2-release-targets.md) passed **439 source, 112 updater and 85 historical v2 regression checks** in [CI #39626573120](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39626573120) on exact code SHA `8e8ff2df11268326453a6fde3265aa9beea9849d`. Owner-controlled 0.4.5 candidate workflows and SHA-bound release evidence are not yet staged.
- **Outstanding release decisions:** the four inherited INFO/unmaintained RustSec notices remain unresolved in the all-target cargo-deny gate ([3D.7](codexdd-0.4.5-packet-3d7-inherited-advisory-release-decision.md)). Native Bazel release-profile packaging, exact frozen candidate and detached 21-stage Daniel-CL acceptance, manual merge and installation are separate gates. Do not advance version markers or deploy yet.

---

## Current DD-BL-006 priority — CodexDD 0.4.4 (2026-10-12)

- **0.4.3 upstream integration:** PR #52 merged at `d8ad52cc223362a03f4dc5b40ec51695b3c48936` with tracked upstream `rust-v0.162.0` and product identity `0.4.3`. This GitHub merge does not itself prove a later production installation.
- **0.4.4 activated, IN DEVELOPMENT:** fix DD-BL-006, the BREAKWATER Node/TypeScript Mechanical Validation profile blocker. Canonical issue: `codex-dd — Upcoming Version Backlog` in Google Drive. Dedicated CodexDD branch: `dd/codexdd-0.4.4-dd-bl-006`; durable working design/checkpoints: [`docs/codexdd-0.4.4-dd-bl-006-reconnaissance.md`](codexdd-0.4.4-dd-bl-006-reconnaissance.md).
- Scope: active Git-worktree profile discovery, fail-closed LVO error evidence and blocked Worker termination, narrow Node command policy, and explicit Node contract integration on isolated BREAKWATER tooling branch. Preserve no-source-edit Mechanical Validation and SHA/run-bound native evidence.
- **Validation not yet accepted.** Source branches are not release candidates until cheap Daniel-CL native checks, Node contract/runtime proof, and final CI pass. No automatic merge or install.
- The historical 0.4.2/0.4.3 roadmap sections below are retained as release planning history; their earlier `NEXT` / `QUEUED` labels are superseded by this entry.

---

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
