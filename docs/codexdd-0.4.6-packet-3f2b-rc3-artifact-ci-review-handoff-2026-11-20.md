# CodexDD 0.4.6 — RC3 artifact and CI audit, independent security review handoff
**Audit date:** 2026-11-20
**Candidate:** `dd/codexdd-v0.4.6-rc3` @ `2acc2209b65f3186b00490b1c4b0cc444a636f4c`
**Production base:** `dd/astra-policy-v2` @ `b25550dcb2c6e84a84074b603086c9db6f32a63b`
**Outcome:** **PARTIAL / RELEASE HOLD.** Successful exact-candidate native Windows profile, but repository merge CI, artifact byte-level verification, non-author security approval and owner real-host acceptance remain OPEN. No release, waiver, merge, tag, signing or installation authority.

## Evidence matrix

| Check | Result / direct evidence | Disposition |
|---|---|---|
| Candidate source identity | Exact 40-character candidate SHA above; PR #88 HEAD matches. | PASS for source identity |
| RC3 exact Windows release profile | [run #44598888395](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44598888395), workflow head `2c8108f6eb5af1a0f1d51247f52610c94a966516` is **the audit-workflow commit**, whose checkout explicitly pins **candidate** `2acc2209...`. Both the source/rustfmt job and 21-stage native/independent objective job concluded success. Its workflow asserts release contract `profile=release,status=pass,completed_stages=21` and independent native `profile=independent-review,status=pass,completed_stages=6`. | PASS as a GitHub-hosted native test, **not** security signoff or host QA |
| TUI integration | Same passing job includes real Windows app-server executable, late-READY rejection regression and entire adaptive suite before full release profile. | PASS for tested exact candidate |
| Verification artifact existence | [run #44598888395 artifact list](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44598888395): artifact ID `14585635801`, name `codexdd-046-rc3-2acc2209-UNSIGNED-verification-only`; size `474026876` bytes; created 2026-11-19 23:34:14 UTC, expiry 2026-11-26 23:33:30 UTC. GitHub zip artifact digest `sha256:1654702ac359a634bfd9c292008a5a572733cfe521451caa5c4d8cb06ad52fb4`. | PASS for GitHub metadata only |
| File-level artifact hashes / manifest | Workflow makes a `provenance.json` manifest with individual SHA-256 digests for `codex.exe`, `codex-code-mode-host.exe` and available PDBs. **Actual archive contents were not retrieved or independently hashed in this audit.** The GitHub artifact ZIP digest is **not** a substitute for per-binary digest verification. Verify manifest contents against downloaded bytes before distribution or installation. | **PENDING** |
| Signing/attestation/installer | The artifact is explicitly unsigned, nonproduction, verification-only; no signed distribution attestation, installer or owner-approved rollout proven. | **PENDING** |
| Blocking PR checks for candidate | [blocking-ci run #44598846102](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44598846102) on candidate SHA fails: Codespell, cargo-deny, Blob size policy, repo-checks/Prettier and dependent `CI required`. `PR fast / Windows adaptive validation` passes. | **FAIL / MUST DISPOSITION** |
| Individual merge-gate diagnosis | Codespell `Codespell` step fails; cargo-deny `Run cargo-deny` fails; Blob size policy `openai/fence` and clean-worktree steps fail before blob scanning; repo-checks `Prettier` step fails. Detailed error text/annotations not retrieved; **cannot yet distinguish candidate changes, inherited baseline or tooling/environment** for the failing families. | **BLOCKED: obtain logs, local reproductions and targeted fixes; no blanket waiver** |
| Source-snapshot audit | [run #44656642598](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44656642598) fails. Inspected audit-only workflow `.github/workflows/codexdd-046-rc3-source-snapshot-audit.yml` checks out obsolete `5cf06f...` and has a malformed multiline Python expression/heredoc. This is broken **audit workflow source**, not evidence RC3 core fails. | **FAIL / repair cheap audit on audit branch, rerun and document** |
| Independent non-author review | [draft PR #88](https://github.com/Takuetsu/codex-dd-astra/pull/88) is open/unmerged; zero submitted reviews and no requested reviewer as of audit. The connected owner `Takuetsu` is the candidate author and cannot provide independent approval. | **PENDING** |
| Owner Windows acceptance | Non-elevated Daniel-CL real agents, daemon/process restart, paired scenarios, rollback backup and owner signoff have no verified acceptance receipts in this audit. | **PENDING** |

## Independent reviewer work packet — exact RC3, not self-signoff

Request a reviewer **distinct from the author/account `Takuetsu`**, with source access, to assess exact candidate-vs-production diff and read the prior [3F.1 inventory](https://github.com/Takuetsu/codex-dd-astra/blob/dceecbec6b065ced7bd36f0e63e40e918a0919bf/docs/codexdd-0.4.6-packet-3f1a-source-security-inventory-2026-11-19.md). Priority findings:

1. User-origin/first Worker authorization, canonical target SHA/root/scope, clean-worktree checks, durable flush, restart/fork/cold-resume guardrails, and TUI/direct app-server RPC ingress.
2. Core-issued LVO / independent Validator receipt identity, thread/turn/source/scope/run pairing, replay/forgery/duplicate and self-attestation prevention.
3. Single-use `READY_FOR_OWNER_QA`, late refused READY no-successor logic specifically fixed by `2acc2209`, legitimate bound terminal transitions, Core-owned OS PID and fresh loopback health, PID/port reuse, process ownership and TOCTOU.
4. Actual 21 + 6 native stage source receipts, remaining PR blocking-CI failures, pinned Windows Rust/V8 dependencies, per-binary provenance, no frozen 0.4.5 leakage.
5. Required Daniel-CL live smoke, backup/rollback and manual owner promotion gates.

Reviewer must provide file/line and proof for findings, and explicitly APPROVE only after all gates accepted; otherwise CHANGES_REQUESTED. An owner-authored checklist or the native `independent-review` test profile is **not** an independent human/security review.

## Next actions / stop rules
1. Capture exact errors for the four failing merge-gate check families, classify new/inherited/environmental, fix minimum root causes without changing security policy, then rerun only appropriate focused checks before full required CI.
2. Repair the cheap source-snapshot workflow on the **audit branch**, preserving RC3 code SHA.
3. Obtain/download GitHub artifact contents before expiry; verify `provenance.json` and each listed EXE/PDB SHA-256; record an immutable verification receipt. Do not call ZIP digest per-file verification.
4. Assign a qualified reviewer with GitHub access (not the author); keep draft PR #88 not-ready-for-merge until explicit approval.
5. Perform medium-integrity real-agent Daniel-CL acceptance separately; do not install or activate Breakwater soak without owner authorization.

**Audit conclusions: full artifact/CI clearance NOT achieved, and the passing 21-stage profile cannot override the other failures.**
