# CodexDD 0.4.3 — work packet 3A.1: integration scaffold and pinned provenance

**Status:** COMPLETE — Gate 2 conflict-recovery authorization accepted on 2026-10-09. This packet establishes the isolated source-integration branch and the evidence boundary; it **does not** claim a prepared candidate, resolved transplant, accepted conflict resolution, Windows validation, or releasable source.

## 1. Scope and ownership

- Source branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`, **created from the then-live production HEAD**, not from an old Phase 1/2 planning branch.
- Production: `dd/astra-policy-v2` at `4828e3b4232781963594cfaaebdc49c5531de8b1`, tree `f486f0b7ec5a909849246ee91f01af098c08421a`.
- Canonical candidate **reserved, absent**: `automation/upstream-candidate-rust-v0.162.0-c1382380de69`.
- This packet owns only the baseline/identity record, integration-branch scaffold, and traceability to read-only discovery. It does not own source transplants or a promotion contract.
- Machine-readable read-only observation: [3A.1 pinned provenance](codexdd-0.4.3-packet-3a1-baseline-provenance.json). It is **not** a v1 candidate manifest and must **never** be passed to a v1 candidate promoter.
- Planning evidence remains separately checkpointed on `dd/codexdd-v0.4.3-phase2-readonly-planning` at `73d0d2e88ad3fb0d3ed018ebe34f92cf29e2c795` ([executed discovery report](https://github.com/Takuetsu/codex-dd-astra/blob/73d0d2e88ad3fb0d3ed018ebe34f92cf29e2c795/docs/codexdd-0.4.3-packet-2a1-executed-discovery-and-gate2.md), [22 conflict paths](https://github.com/Takuetsu/codex-dd-astra/blob/73d0d2e88ad3fb0d3ed018ebe34f92cf29e2c795/docs/codexdd-0.4.3-packet-2a1-executed-transplant-conflicts.txt)). This source branch is intentionally independent of that older planning branch.

## 2. Independently verified exact identities

| Authority | Verified identity |
| --- | --- |
| Product on production | CodexDD `0.4.2` |
| Tracked upstream | Official annotated tag `rust-v0.159.2`, tag object `8b9fa496bbf2c47aebd62e85a080b9a522a455b5` |
| Tracked peeled commit | `ff6aec96948b70d94983af2641a6b67c94faeff5` |
| Tracked tree | `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` |
| Selected upstream | Official annotated tag `rust-v0.162.0`, tag object `1f3f93473394b620b35580859b7e6864f7a9f948` |
| Selected peeled commit | `c1382380de69521303b416720a52f42d51af6248` |
| Selected tree | `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |
| Candidate key | `rust-v0.162.0@c1382380de69521303b416720a52f42d51af6248` |

GitHub official `openai/codex` annotated refs and tag objects independently confirm that these are the **peeled commits**, not the tag-object SHAs. Target selection is pinned to `0.162.0` regardless of a newer available official release.

## 3. Immutable dry-run failure and conflict scope

- Original read-only discovery: [GitHub Actions run 37960772252](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252), successfully dispatched from production HEAD `4828e3b4232781963594cfaaebdc49c5531de8b1`.
- Original artifact: `upstream-candidate-rust-v0.162.0-37960772252`, ID `11630164177`.
- Authoritative original `manifest.json` SHA-256: `59a9e5fbcace88827dc1edfd38abb0b6b45c8d41c7ff956d7af0ed94e9988ee3`.
- Authoritative original `transplant-conflicts.txt` SHA-256: `c3456c3e93130126f8f6f6bc1366bdafcc0c11835244ba70c31b91a564a450a4`.
- Manifest contract v1, `state=blocked_transplant_conflict`, `transplant_dry_run=conflict`. The original downloaded artifact SHA-256 values and contents were inspected again during 3A.1.
- Current-base inventory: **257 customization paths; 2,507 upstream changed paths; 136 overlaps; 124 classified sensitive overlaps; 22 textual conflict paths**.
- Upstream graph `diverged` with merge base `06971ec9aad037d7c32b7466031fbb8b3b407103`. The dry run is *not* safe to promote or to relabel as clean.

All 22 unresolved conflict paths retain the original discovery report's ordering. Their broad ownership is fixed by 1A.3 and will be mapped **path-by-path** in 3A.3. The 136 semantically overlapping paths, including 12 currently unclassified, also require a complete ownership audit; the 22 textual conflicts are only a subset of that task.

## 4. Hard isolation and implementation boundaries

1. **No ordinary upstream preparation on a known conflict.** The existing `.github/workflows/upstream-prepare.yml` must fail closed; do not dispatch it as a workaround.
2. **No canonical candidate branch yet.** The single deterministic candidate ref must be published only after independently accepted source resolution and the applicable validated bridge contract; it must never be force-replaced or premised on a falsified clean transplant.
3. **Original failure stays immutable.** Any manual reconciliation requires a separate, typed conflict-resolution attestation that links this blocked evidence, all conflict paths, resolution owners, source tree, production parent, and semantic review. Never rewrite the v1 `blocked_transplant_conflict` result into `preparation_ready`.
4. **Tooling prerequisite separate from source packets.** If a reconciled candidate cannot satisfy the current v1 validator/promoter, author a narrowly scoped additive tooling-only compatibility PR, validate it independently and request a consequential production merge decision. Do not smuggle promotion-bypass changes into product integration.
5. **Security-sensitive behavior unchanged by assertion alone.** Preserve privileged Windows F2 embedded fallback, hard rejection of explicit elevated-daemon launches, process/ACL guards, adaptive model/routing policy and LVO. Validate tests and semantic effects in their later owned packets.
6. **Production movement invalidates identities.** Any tooling or other production merge requires a fresh identity/manifest/reconciliation checkpoint before final publication; old receipt hashes cannot be adapted by hand.
7. **Windows native authority remains required.** No CI success, GitHub-only test, or SSH elevation waiver substitutes for applicable Daniel-CL release validation.

## 5. Packet dependency queue

- **3A.1 (this packet):** isolated branch from production + pinned machine-readable source/discovery identity. Complete.
- **3A.2:** define versioned, additive reconciliation-attestation contract, evidence checks, exactly-once candidate publication boundary and default-branch tooling prerequisite. Do not publish a candidate. An implementation PR for the prerequisite is separately owner-gated before merge.
- **3A.3:** full 22-path exact conflict ownership table and 136-path overlap ownership/sensitivity inventory, including 12 unclassified paths. No opportunistic source coding.
- **3B.1–3B.5:** core config/persistence, Worker/session, app-server protocol and generated exports, narrowly tested and separately checkpointed.
- **3C.1–3C.4:** Windows daemon/elevation/F2 and TUI routing/slash/copy-paste behavior.
- **3D.1–3D.2:** GPT-6/6.1 catalog, status and adaptive compatibility without feature expansion.
- **3E.1–3E.4:** updater/LVO, Cargo/Bazel compatibility, product `0.4.3` plus tracked-upstream version only at accepted integration stage, validated candidate contract.
- **3F.1–3F.2 then Phase 4:** overlap/security audit, Windows-native release-profile receipts, explicit semantic review/PR, separately approved merge/install/soak.

**3A.1 validation:** official upstream annotated tag-object refs and peeled commit/tree pairs verified from `openai/codex`; production product/upstream files re-read; current production tree verified; absent pre-existing integration/canonical branch verified before creating the new isolated integration branch. The observation JSON is readable from the integration branch. No Rust validation is claimed for a documentation-only scaffold.

**Deferred:** transplant/resolution code, 3A.2 contract, 3A.3 ownership map, all native tests and release gates.
