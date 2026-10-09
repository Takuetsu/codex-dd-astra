# CodexDD 0.4.3 — packet 2A.1 executed fixed-target discovery and Gate 2

**Status:** 2A.1 read-only execution COMPLETE; upstream candidate preparation BLOCKED, **state remains `blocked_transplant_conflict`**. Conflict-recovery Gate 2 not yet accepted. No candidate, version bump, source transplant, production merge, installation, or waiver.

## Exact evidence

- Production `dd/astra-policy-v2` commit: `4828e3b4232781963594cfaaebdc49c5531de8b1`
- Production tree: `f486f0b7ec5a909849246ee91f01af098c08421a`
- Product: CodexDD `0.4.2`; tracked upstream: `rust-v0.159.2` at `ff6aec96948b70d94983af2641a6b67c94faeff5` (tree `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361`)
- Selected fixed target: `rust-v0.162.0` at `c1382380de69521303b416720a52f42d51af6248` (tree `4899ef4940a8bc7fdf7873aa0d3f438085b8163f`)
- Read-only GitHub Actions manual discovery: [run 37960772252](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252), completed successfully, source HEAD equal to the production commit above.
- GitHub evidence artifact: `upstream-candidate-rust-v0.162.0-37960772252` (artifact ID `11630164177`, 2,684-byte ZIP). Its `manifest.json` and `transplant-conflicts.txt` were downloaded and inspected independently.
- Original `manifest.json` SHA-256: `59a9e5fbcace88827dc1edfd38abb0b6b45c8d41c7ff956d7af0ed94e9988ee3`.
- Original `transplant-conflicts.txt` SHA-256: `c3456c3e93130126f8f6f6bc1366bdafcc0c11835244ba70c31b91a564a450a4`.
- Manifest contract `1`, candidate key `rust-v0.162.0@c1382380de69521303b416720a52f42d51af6248`, `ancestry_status=diverged`, merge base `06971ec9aad037d7c32b7466031fbb8b3b407103`.
- Observed inventory: **257** fork customization paths, **2,507** changed upstream paths, **136** overlaps, **124** sensitivity-labeled overlaps; the original source tree is different from the pre-merge 256-path snapshot.
- Actual synthetic transplant result: `conflict`; manifest state: `blocked_transplant_conflict`; **22 exact unresolved path names**. The [transcribed report](codexdd-0.4.3-packet-2a1-executed-transplant-conflicts.txt) preserves the 22 paths in the artifact's original order; the authoritative original bytes remain in the GitHub Actions artifact. The complete historical 22-conflict forecast is now confirmed against the new production base.
- Workflow success means **discovery execution succeeded**; it does **not** imply a clean source transplant or a prepared/promotable candidate.

## Gate 2 — explicit conflict recovery decision needed

The 0.4.2 clean-only `upstream-prepare.yml` must **not** be invoked while the actual transplant conflicts. Do not relabel the original manifest as clean or `manual_semantic_review_required`. Do not create `automation/upstream-candidate-rust-v0.162.0-c1382380de69` or bypass `upstream-promote.yml`. Proposed next lane, subject to Gate 2 approval:

1. Recheck live production and tag identities immediately before writes. Create or refresh the **isolated**, auditable `dd/codexdd-v0.4.3-upstream-0.162.0-integration` branch without touching production or canonical candidate.
2. Begin 3A.1–3A.3 scaffold, full path ownership, immutable conflict evidence, and **separate typed** resolution-attestation design. Segment core/app-server (3B), Windows/TUI/F2 (3C), model/status (3D), updater/build/provenance (3E), full audit (3F).
3. If a promotion/reconciliation contract extension is needed, first implement and validate it as a **separate tooling-only PR**, independently owner-gated before any merge. Existing clean-only, semantic review and Windows-native release-profile safeguards stay in place.
4. Recheck exact production baseline after any prerequisite tooling merge; never move an obsolete manifest forward by rewriting its identity fields.
5. Require explicit canonical-candidate publication acceptance, semantic review and exact SHA-256-bound native Windows receipt before any promotion, release or installation.

**No Gate 2 authorization is recorded in this checkpoint.** This document is evidence and a decision boundary, not permission for upstream source integration.
