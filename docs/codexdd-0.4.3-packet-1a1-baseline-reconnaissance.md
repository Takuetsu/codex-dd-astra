# CodexDD 0.4.3 packet 1A.1 — source, tag, and discovery baseline

**Status:** COMPLETE — read-only reconnaissance checkpoint; Phase 1 Gate 1 **NOT ACCEPTED**.
**Date:** 2026-10-09 (UTC).
**Packet scope:** identity, production/discovery baseline, existing candidate evidence and safety boundaries. **No runtime/source/provenance/version edits, upstream preparation, PR, release or installation are authorized by this packet.**
**Working documentation branch:** `dd/codexdd-v0.4.3-phase1-recon`; **not** the upstream preparation candidate branch.
**Frozen production anchor:** `dd/astra-policy-v2` at `55139187f31d044d9f413245c38ba2da058cab2e`, tree `614e1e9cfb1b9749f3a57fa21f48848d7d7d7153`.

## 1. Live production versus deployed runtime

The branch anchor above was independently re-resolved through the live Git reference and commit/tree API before the documentation branch was created. The last installed Daniel-CL runtime remains `codexdd 0.4.2+gdc10de2b0240`, associated with hotfix merge `dc10de2b0240ebe51bba009bfa224fc305c9aa6a` (PR #48). **Do not use that deployed SHA as the current upstream preparation base.**

The production branch is **four commits ahead** of the deployed merge; Git comparison shows changes confined to four Markdown planning/hotfix files:
- `docs/codexdd-0.4.2-automatic-upstream-update-design.md`;
- `docs/codexdd-0.4.3-upstream-0.162.0-design.md`;
- `docs/codexdd-elevated-windows-f2-hotfix.md`;
- `docs/codexdd-roadmap.md`.

Remote Git branches identify committed trees, not a mutable Windows worktree. Production has a resolvable commit/tree; no active PR was returned by the open-PR search at this checkpoint. No claim is made about a local checkout's cleanliness on Daniel-CL.

Live identity-file reads on production:
- `codex-rs/codexdd-version.txt`: **0.4.2**;
- `codex-rs/upstream-codex-release.txt`: **rust-v0.159.2**.

**Neither identity file is to change during Phase 1.**

## 2. Official tracked and selected target tag verification

Source of authority: Git references and annotated-tag objects in the official `openai/codex` repository (not fork-local tags). Both selected tags exist as official annotated Git tags. The values below are *peeled commit IDs*, not tag-object IDs.

| Identity | Tracked baseline | Owner-selected target |
| --- | --- | --- |
| Official tag | `rust-v0.159.2` | `rust-v0.162.0` |
| Annotated tag object | `8b9fa496bbf2c47aebd62e85a080b9a522a455b5` | `1f3f93473394b620b35580859b7e6864f7a9f948` |
| Peeled commit | `ff6aec96948b70d94983af2641a6b67c94faeff5` | `c1382380de69521303b416720a52f42d51af6248` |
| Commit tree | `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` | `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |

The target tag object points to the expected commit. The target commit timestamp is 2026-10-08 16:56:07 UTC. **0.162.0 is a fixed owner-selected scope, not a moving “latest stable” target.** Compare the complete tracked 0.159.2 → 0.162.0 tree delta in packet 1A.2, including interim 0.160.x and 0.161.x behavior. Do not silently retarget to later releases.

## 3. Existing 0.4.2 planner and discovery evidence

Inspected:
- `.github/workflows/upstream-sync.yml` — manual/six-hour **read-only** discovery (`contents: read`), canonical official tag query, exact recursive tree comparison, synthetic-delta **dry run**, artifact upload; no branch push, PR creation, version allocation, merge or installation.
- `.github/scripts/upstream_sync.py` — exact tag/commit candidate identity, stable-tag-only eligibility, deterministic candidate branch, Git tree-delta and sensitivity classification.
- `.github/workflows/upstream-prepare.yml` — distinct explicit manual prepare, identity revalidation, duplicate-branch prevention, isolated transplant; not invoked.
- `.github/workflows/upstream-promote.yml` and `scripts/codexdd-validate-upstream-candidate.ps1` — native HEAD/manifest-bound Windows receipt and manual PR-promotion gates remain in force, not invoked.

Most recent completed discovery run found in repository Actions:
- Run: [37936059803](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37936059803), successful, started 2026-10-09 13:19:48 UTC.
- Artifact: `upstream-candidate-rust-v0.162.0-37936059803` (ID `11617648968`), created 13:20 UTC.
- Manifest **production SHA `dc10de2b0240ebe51bba009bfa224fc305c9aa6a`**, tree `96728fa701c370d76c768b5727b05b889084daf9`.
- Candidate key: `rust-v0.162.0@c1382380de69521303b416720a52f42d51af6248`; `ancestry_status=diverged`, merge base `06971ec9aad037d7c32b7466031fbb8b3b407103`.
- *Historical discovery-snapshot counts:* **255** fork-customization paths, **2,507** upstream-changed paths, **136** overlapping paths, **124** sensitive overlap paths.
- *Historical discovery-snapshot dry run:* `transplant_dry_run=conflict`, `state=blocked_transplant_conflict`, with **22** textual conflict paths.
- Representative conflicts: `codex-rs/app-server-daemon/src/backend/windows.rs`, `codex-rs/app-server-daemon/src/lib.rs`, `codex-rs/core/src/config/mod.rs`, `codex-rs/rollout/src/policy.rs`, `codex-rs/tui/src/daemon_startup.rs`, `codex-rs/tui/src/app/thread_routing.rs`, `codex-rs/tui/src/app_server_session.rs`, and generated protocol/TypeScript/precomputed files.
- Searches found **no existing branch for `rust-v0.162.0`** or expected deterministic candidate `automation/upstream-candidate-rust-v0.162.0-c1382380de69`. The old `automation/upstream-sync-rust-v0.155.0` branch is unrelated historical state.

**Critical staleness:** the above manifest predates the current production HEAD `55139187...`. Its exact `production_sha`/`production_tree` does **not** match production now, so the 0.4.2 fail-closed preparation contract forbids using it for a new candidate, despite both upstream tags still resolving correctly. The counts and conflict list are useful **reconnaissance leads, not a current candidate acceptance receipt**. Refresh the manifest against the live base in packet 1A.2/Phase 2 as appropriate; do not reuse its production SHA.

## 4. Initial risk and preservation map

1. **Windows daemon / elevated SSH F2:** upstream-overlapped Windows daemon and TUI startup code must preserve 0.4.2's accepted embedded fallback and explicit elevated daemon guards. The one-time PR #48 non-elevated detached-daemon waiver does not carry forward.
2. **Core adaptive/Worker/session:** upstream-changed core, rollout, thread/session and app-server state surfaces must preserve GPT-6/GPT-6.1/Astra policy, trusted signals, LVO authority, resumes/forks and worker lifecycle; textual conflict resolution alone is inadequate.
3. **Generated schema/build:** protocol JSON/Zstandard/TypeScript exports, `Cargo.lock`, CI and toolchain differences require tracked regeneration and narrow validation rather than hand-editing binary generated resources.
4. **Release integrity:** keep CodexDD product `0.4.3` allocation distinct from upstream `rust-v0.162.0`; no autonomous PR/merge/install; Windows-native Daniel-CL evidence is authoritative.
5. **Baseline movement:** the roadmap itself warns planning commits advance production. Before any candidate preparation, re-resolve the production commit/tree and remake all identity-bound evidence.

## 5. Packet conclusion and next gates

**Packet 1A.1 is complete as reconnaissance. Gate 1 is not yet ready for owner acceptance.**

- **Next packet 1A.2:** re-run *current-base* exact fork/upstream tree-delta and conflict/sensitive-ownership inventory; inspect upstream 0.160.x/0.161.x/0.162.0 runtime changes, Windows daemon/F2, generated artifacts, toolchain/Cargo and CI. Separate semantic hazards from textual transplant conflicts.
- **Then packet 1A.3:** define exact deterministic candidate/branch ownership, source integration packet splits, manifest/receipt continuity, product-version allocation point, rebase/stale-state/rollback rules, and a bounded Windows validation strategy.
- **Gate 1:** present completed 1A.1–1A.3 evidence and risks to owner for **explicit acceptance before any source changes or candidate preparation**.

No test pass, candidate readiness, production installation, compatibility acceptance, or release permission is asserted by this documentation checkpoint.
