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
| Source-snapshot audit | [run #44656642598](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44656642598) fails. Inspected audit-only workflow `.github/workflows/codexdd-046-rc3-source-snapshot-audit.yml` checks out obsolete `5cf06f...` and has a malformed multiline Python expression/heredoc. This is broken **audit workflow source**, not evidence RC3 core fails. | **PASS (repaired audit-only workflow)**: [run #44758443292](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44758443292) succeeded at audit SHA `d7f0ebe433b9229813adbb301db72947307f198e`, still testing pinned RC3 source. Previous failure retained as diagnostic evidence. |
| Independent non-author review | [draft PR #88](https://github.com/Takuetsu/codex-dd-astra/pull/88) is open/unmerged; zero submitted reviews and no requested reviewer as of audit. The connected owner `Takuetsu` is the candidate author and cannot provide independent approval. | **PENDING** |
| Owner Windows acceptance | Non-elevated Daniel-CL real agents, daemon/process restart, paired scenarios, rollback backup and owner signoff have no verified acceptance receipts in this audit. | **PENDING** |

## 2026-11-20 exact blocker log classification (no waivers)

Direct job logs for [blocking-ci #44598846102](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44598846102) were inspected:

- **Codespell**: one finding, `docs/codexdd-0.4.6-packet-3b1b-github-source-hardening.md:30`, `re-used` -> `reused`.
- **Prettier**: 50 source-file warnings (49 Markdown files and one `.github/workflows/codexdd-046-3e2-candidate-smoke.yml`). The 0.4.3 frozen Prettier baseline checker passed and must not be bypassed. A source-only staging branch `dd/codexdd-v0.4.6-rc3-ci-repair` is used to format without mutating the SHA that passed the Windows full native profile.
- **Blob policy**: old `openai/fence@e6b39c80... # pin@v0.10.0` failed **before checkout**: `Fence setup failed ... /run/fence-launcher/.../action/bin/fence: Permission denied`; the subsequent missing clean-worktree action was a consequence of skipped checkout. The source diff's blob sizes were **not** checked. Staged upgrade to publisher's signed-off immutable Action commit `7cba9082252803421a0e433f29198de6d0063221 # pin@v0.13.1` needs an actual successful rerun, not a policy waiver.
- **cargo-deny**: 42 `error[vulnerability]` entries and a handful of warnings for currently locked crates, including `crypto-bigint`, `hickory-proto`, `hickory-resolver`, `openssl`, `libsqlite3-sys`, `quinn-proto` and others. Crucially **`codex-rs/Cargo.lock` Git blob `0c913b46674d3593ee84f1fee5508d16e33efae5` and `codex-rs/deny.toml` blob `ea7fdc149fc880c6b36d75c29851641034289a4e` are byte-identical between the 0.4.4 production baseline and exact RC3**. These are *inherited, newly reported security exposures*, not 0.4.6 newly introduced dependencies, but they remain real. Do **not** silence cargo-deny, expand ignore lists, or alter policy until a qualified security reviewer and the owner approve a risk decision. Repairing the transitive graph needs an explicitly scoped dependency migration and new source-bound validation; it cannot be certified by the existing RC3 21-stage build receipt.

**Release hold remains mandatory:** this is a CI diagnosis and isolated hygiene repair, not a claim that the four required check families passed.

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
2. DONE: repaired and reran cheap source-snapshot workflow successfully on the **audit branch** at `d7f0ebe4`, preserving RC3 code SHA.
3. Obtain/download GitHub artifact contents before expiry; verify `provenance.json` and each listed EXE/PDB SHA-256; record an immutable verification receipt. Do not call ZIP digest per-file verification.
4. Assign a qualified reviewer with GitHub access (not the author); keep draft PR #88 not-ready-for-merge until explicit approval.
5. Perform medium-integrity real-agent Daniel-CL acceptance separately; do not install or activate Breakwater soak without owner authorization.

**Audit conclusions: full artifact/CI clearance NOT achieved, and the passing 21-stage profile cannot override the other failures.**

## 2026-11-20 three-family CI repair results (staging only)

**Draft PR:** [#89](https://github.com/Takuetsu/codex-dd-astra/pull/89), `dd/codexdd-v0.4.6-rc3-ci-repair` @ `61c0fb8909ec454c823d8354687e5283ef762430` against original RC3 `2acc2209...`, **DO NOT MERGE**.

**Exact PR check run:** [blocking-ci #44764297392](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44764297392), four required check families:

| Family | Final job result | Evidence / effect |
|---|---|---|
| Codespell | **PASS** | One typo corrected in docs. |
| repo-checks (including Prettier) | **PASS** | 49 docs + one CI smoke YAML formatted with repository formatter; earlier 0.4.3 frozen baseline still enforced. [one-shot formatter #44763720708](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44763720708) passed source-only scope assertions. |
| Blob size policy | **PASS** | Pinned publisher Fence v0.13.1, supported Ubuntu 26.04; unchanged 512,000-byte actual diff scanner. First [focused Fence/blob run #44764011900](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44764011900) passed; normal PR check now also PASS. Earlier Ubuntu 24.04 Fence failed startup; no bypass or `continue-on-error`. |
| cargo-deny | **FAIL — SECURITY HOLD** | Same 42 `error[vulnerability]` advisories as original RC3; bans/licenses/sources passed. `Cargo.lock` and `deny.toml` are byte-identical to production 0.4.4. Requires separately authorized dependency migration plus validation or explicit, qualified owner/security risk review; NO blanket ignore/waiver. |

Standard PR-fast Windows adaptive validation was started by the normal `blocking-ci` workflow and may finish separately; **this is not the already-passing 21-stage release profile**, which was neither changed nor rerun.

**Exact code delta:** GitHub compare `2acc2209...61c0fb89` showed 51 paths changed **only in docs and two CI YAML files**, zero `codex-rs` / executable / dependency lockfile paths. Original full native release proof remains pinned to original RC3, and PR #88 remains draft at its original head. The staged hygiene patch is not independent security approval, full CI green, or authority to update that candidate, install or soak.

