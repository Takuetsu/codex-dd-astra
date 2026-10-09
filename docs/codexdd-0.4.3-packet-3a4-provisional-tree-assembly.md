# CodexDD 0.4.3 — packet 3A.4: provisional upstream source-tree scaffold

**Status:** Approved Gate 2 WIP source scaffold; **not a clean cherry-pick, conflict resolution, compilable build, candidate, or release.** A deliberately temporary nonpromotable source-tree assembly step after 3A.1–3A.3 ownership auditing.

## Preconditions

- Production: dd/astra-policy-v2 at 4828e3b4232781963594cfaaebdc49c5531de8b1; tree f486f0b7ec5a909849246ee91f01af098c08421a.
- Tracked upstream: rust-v0.159.2 at ff6aec96948b70d94983af2641a6b67c94faeff5; tree 406dfdd5c68f303a3a8d04f32b3965b3b0ca0361.
- Target: rust-v0.162.0 at c1382380de69521303b416720a52f42d51af6248; tree 4899ef4940a8bc7fdf7873aa0d3f438085b8163f.
- Independent full recursive tree comparison reproduced 257 fork customization paths, 2,507 target-changed paths and 136 overlaps, with no file/directory conflicts in the intended overlays. See [the complete 3A.3 ownership matrix](codexdd-0.4.3-packet-3a3-ownership-audit.md).

## Provisional assembly algorithm

1. Start with the exact target official upstream Git tree for rust-v0.162.0.
2. Compute all 257 production custom paths relative to tracked 0.159.2 using Git SHA, mode and type. Overlay their current production blobs and modes, or preserve their deletion. This retains fork-owned adaptive routing, LVO, Windows daemon F2 guards, workflows, configuration and provenance.
3. Retain all prior 3A documentation/checkpoints from this isolated integration branch, without pulling an obsolete planning branch into its ancestry.
4. Create a WIP source-tree commit with only the prior integration checkpoint as parent. Do not touch production, deterministic candidate branch, version files or a PR.

**Important:** All 136 overlapping files temporarily retain their production/fork blob. This is **not** Git's clean 3-way merge and does **not** incorporate the upstream edits to these paths. The 22 observed textual conflicts and 114 other overlapping files **all remain unresolved / pending semantic integration**. The original discovery remains state blocked_transplant_conflict and transplant_dry_run=conflict. Nothing in this scaffold may be used to claim a clean candidate.

## Required narrow integrity verification

- Verify the complete staged path-to-Git-SHA/mode/type map against the exact formula: target tree plus all production customization deltas plus current branch-specific documentation.
- Confirm the 2,371 upstream-only changed paths retain their target 0.162.0 blob/mode/type. Confirm all 136 overlap paths retain their fork version and remain explicitly unreviewed.
- Verify source branch parentage, production still unchanged, and absence of the canonical candidate branch.
- Confirm product version remains 0.4.2 and provenance remains rust-v0.159.2 until later accepted 3E.3. This is a WIP source baseline, not a completed upstream switch.
- No compilation, Windows test, native release PASS, or promotion permission is inferred from Git tree checks.

## Next source work and gates

- 3B through 3E must reconcile all 136 path overlaps (including all 22 textual conflicts and 12 formerly unclassified high-risk paths), with per-file semantic review and focused tests.
- Generated SDK/schema/compressed exports must be built by generators from reconciled source, not edited manually.
- Implement and have the owner approve a separate tooling-only additive conflict-reconciliation bridge **before** canonical candidate publication. The original v1 clean-only preparer/promoter remain fail-closed.
- Re-anchor the source baseline, discovery and any future attestation if the production branch advances through a tooling merge.
- Require Gate 2B, exact canonical HEAD, typed attestation, sensitive semantic review, native release-profile PASS, and owner merge/install approval separately.

This document is a WIP engineering checkpoint and is not a preparation artifact.
