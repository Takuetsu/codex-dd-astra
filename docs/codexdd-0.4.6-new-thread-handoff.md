# CodexDD 0.4.6 — New Chat Activation / Exact Handoff

**Date:** 2026-11-05
**Purpose:** Paste the following prompt into a **new chat within the CodexDD project**. This is a scoped start to the 0.4.6 reliability milestone, not a request to merge, publish or install.

---

## Copy into new thread

Continue CodexDD **0.4.6 — Agent Reliability & Completion**. 0.4.5 is intentionally FROZEN; do not run its workflows, unfreeze its PRs or import its unfinished upstream/daemon/TUI changes.

The 0.4.6 milestone is owner-selected for restoring the usability of active CodexDD agents. Its priority order is:
1. P0: trustworthy end-to-end completion and evidence-bound `ready_for_owner_qa` / `repair_required` terminalization (DD-BL-005/007): no false READY on partial green tests, missing official validation, stale `exec-*` refs or unverified runtime/preview.
2. P0: correct bound Worker role, milestone-root vs nested Git worktree identity, active candidate SHA, native validation tool/profile readiness, and safe Node/Rust work_packet routing (DD-BL-006).
3. P0: prevent repeated no-progress adaptive continuations/model escalations on unchanged infrastructure, tool, provider or policy blockers while keeping productive continuations.
4. P0 bounded: support explicitly authorized non-source-changing operations/QA-preview and benchmark tasks without incorrectly forcing Implementation MechanicalValidation, with strict process/source/permission boundaries.
5. P1 conditional: reproduce and fix genuine in-capacity `agent thread limit reached` failures, provider `tool_choice` compatibility errors, and first-turn role-binding/preflight confusion. Do not blindly increase worker limits or change providers.

**Authoritative GitHub planning source:**
- Repo: `Takuetsu/codex-dd-astra`
- Development branch: `dd/codexdd-v0.4.6-agent-reliability` (refresh live HEAD).
- Production source baseline: `dd/astra-policy-v2` @ `b25550dcb2c6e84a84074b603086c9db6f32a63b`, version `0.4.4`.
- Triage checkpoint: `dd/codexdd-044-active-agent-stability-triage-20261105` @ `8cee2c8a7d6ef7ebfaa65569dcd86b3ad263de41`.
- Read `docs/codexdd-0.4.6-agent-reliability-design.md`, `docs/codexdd-0.4.4-active-agent-stability-triage-2026-11-05.md`, and `docs/codexdd-major-phase-work-packet-standard.md` from the current development branch.
- Read Google Drive **codex-dd — Upcoming Version Backlog** (November 4 entries) and **BREAKWATER — CodexDD Operational Guide** for current case evidence. Older backlog statements that version was unassigned/0.4.5 must finish first are superseded by my explicit 0.4.6 selection and freeze decision.
- Frozen 0.4.5 checkpoint: `a69ce2ee19a50b834bc116246f94940bef8b8806`, PRs #83–85 draft; 3F.94 environment blocked, 3F.95 test compile failed; 13 high-risk holds and 0/109 signoffs remain. No 0.4.5 source, CI, release or installation work.

**Begin NOW with Phase 3A, packet 3A.1, read-only audit and durable handoff:** inspect live branch/base identities, current 0.4.4 code/test ownership, saved incident evidence and missing installed-version data. Build a rigorously classified OBSERVED -> REPRO -> EXPECTED -> EVIDENCE -> OWNING PATH matrix; separate current-code behavior from historical/role/config issues. Verify that the 0.4.6 task-size and security boundaries are achievable before editing core/TUI. Commit the packet report as `docs/codexdd-0.4.6-packet-3a1-evidence.md` to the dedicated branch; report commit SHA, actual next packet and blockers. Only then start the next numbered packet under the major-phase rules. Do not silently treat the entire release as one run.

**Workflow constraints:** I expect you to handle GitHub source/docs/workflow implementation rather than having me manually edit files. Use small GitHub checkpoint commits and cheap local tests on **Daniel-CL via SSH** before any GitHub CI; no expensive CI for source-only preparation, no heavyweight cargo on Stonks or Doug. Keep worktrees under `E:\codexdd`. I launch CodexDD inside BREAKWATER milestone roots such as `D:\bw58`, not inside nested Git worktrees—respect this and verify the authorized target SHA without instructing me to change my launch convention. Do not modify live BREAKWATER/CV worktrees. Request my help only for indispensable concise Windows validation or consequential owner decisions; do not repeat questions already settled.

No new security privilege, unsafe PID kill, fake validation receipt, role bypass, release, merge, production install, or 0.4.5 unfreeze is authorized. 0.4.6 version bump is reserved for the later accepted candidate; production stays 0.4.4 until owner acceptance.

---

**Startup completion check:** 3A.1 read-only analysis -> committed SHA -> concise status and next packet. No build or manual operator command required at this instant.
