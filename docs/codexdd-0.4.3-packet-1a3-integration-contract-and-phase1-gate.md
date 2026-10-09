# CodexDD 0.4.3 — Phase 1 packet 1A.3: integration contract, ownership and acceptance gate

**Status:** 1A.3 COMPLETE — **Phase 1 Gate 1: AWAITING EXPLICIT OWNER ACCEPTANCE.**  
**Prepared:** 2026-10-09.  
**Documentation-only branch:** `dd/codexdd-v0.4.3-phase1-recon`.  
**Scope:** source integration ownership, necessary blocked-conflict recovery design, pinned upstream identities, safety invariants, numbered packet decomposition, evidence and Windows validation plan. No runtime files, updater tools, CI workflows, source/provenance/version files or production branches changed by this packet.

> **Important:** This is a proposed release plan, not approval to prepare/transplant an upstream candidate, edit product code, create a PR, merge, install, waive a test, or change the operator's environment.

## 1. Accepted planning inputs, not executable write authorization

| Anchor | Frozen Phase 1 value |
| --- | --- |
| Fork / production | `Takuetsu/codex-dd-astra`, `dd/astra-policy-v2` |
| Current production HEAD | `55139187f31d044d9f413245c38ba2da058cab2e` |
| Current production tree | `614e1e9cfb1b9749f3a57fa21f48848d7d7d7153` |
| Deployed runtime evidence | `codexdd 0.4.2+gdc10de2b0240`, hotfix PR #48; **not** the current production Git HEAD |
| Current product source identity | `codexdd 0.4.2` |
| Tracked official upstream | `rust-v0.159.2` / `ff6aec96948b70d94983af2641a6b67c94faeff5` / tree `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` |
| Owner-selected target upstream | `rust-v0.162.0` / `c1382380de69521303b416720a52f42d51af6248` / tree `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |
| Target candidate key | `rust-v0.162.0@c1382380de69521303b416720a52f42d51af6248` |
| Reserved canonical candidate ref | `automation/upstream-candidate-rust-v0.162.0-c1382380de69` |
| Authoritative target OS / native gate | Windows; Daniel-CL through SSH only |

The target is **exactly** 0.162.0 even if an unrelated scheduled job discovers a newer stable tag. Git commit/tree and tag-object provenance must be rechecked immediately before any remote branch write. The source-base SHA above is a **reconnaissance checkpoint**, not a perpetual preparation base: any production movement invalidates new work based on it until identities/delta/manifest are regenerated.

- Packet [1A.1](codexdd-0.4.3-packet-1a1-baseline-reconnaissance.md) verified current product/upstream/production identity and discovered an obsolete manifest.
- Packet [1A.2 report](codexdd-0.4.3-packet-1a2-full-overlap-reconnaissance.md) and [complete path inventory](codexdd-0.4.3-packet-1a2-tree-inventory.json) recorded **256** fork customization paths, **2,507** net upstream changes, **136** exact overlaps, **124** policy-sensitive overlaps and **12** policy-unclassified overlaps. These are pinned, exact, nontruncated tree comparisons.
- The earlier read-only discovery run **37936059803** reported **22** textual conflicts, but its manifest was tied to obsolete production SHA `dc10de2b...`. Its paths are conflict **forecasts**, not a fresh preparation receipt. Do not reuse that manifest.

## 2. Key 0.4.2 tooling constraint discovered during 1A.3

Read and checked live production versions of:
- `.github/workflows/upstream-sync.yml` (six-hour/manual **read-only** scheduled discovery, chooses the **latest** official stable release; no fixed-target workflow input);
- `.github/workflows/upstream-prepare.yml` (explicit identity-bound dispatch; checks exact production/tracked/target; synthetic customization-delta transplant; **fails with no published candidate if a conflict is detected**);
- `.github/workflows/upstream-promote.yml` (manual PR promotion; reads candidate HEAD, production base, manifest SHA-256, receipt, branch identity, and owner semantic acceptance; **only accepts manifest contract v1 states `preparation_ready` / `manual_semantic_review_required`**);
- `.github/scripts/upstream_sync.py` (v1 `candidate_state()` correctly reports `blocked_transplant_conflict` for *any* conflicting synthetic transplant);
- `scripts/codexdd-validate-upstream-candidate.ps1` (requires **exact** deterministic candidate branch name, committed manifest under `docs/upstream-candidates/`, clean worktree, production ancestry, native profile PASS, SHA-bound receipt).

**Actual blocker:** Under the 0.4.2 contract, a conflicting transplant cannot produce a candidate branch and its v1 `blocked_transplant_conflict` manifest is **not promotable**. This is correct fail-closed behavior, **not** an error to suppress. It is forbidden to copy a blocked manifest and manually relabel it `manual_semantic_review_required`, change `transplant_dry_run` to `clean`, or manually open a PR claiming native 0.4.2 promotion was satisfied.

### Chosen solution: gated, additive conflict-reconciliation lane

The recommended 0.4.3 route is **not** to weaken normal `upstream-prepare.yml`. Preserve its clean-only publish policy. Introduce a narrowly scoped, **operator-approved conflict-reconciliation bridge** as a prerequisite, only if a fresh exact-target dry run confirms conflicts:

1. **After Gate 1:** refresh the actual live production identity, exact official selected tag, fixed-target manifest, nontruncated tree inventory and read-only synthetic dry run. The scheduled latest-tag job cannot substitute for the **owner-pinned 0.162.0** if latest changes. Use/extend read-only tooling in a bounded separate packet rather than silently changing target. Keep immutable blocked discovery evidence.
2. **Clean path:** only if the *current-base* dry run is clean, explicitly dispatch current `upstream-prepare.yml` with the accepted four identities; it may create **only** the canonical candidate branch, bound manifest and provenance. No PR or version bump. Gate 2 then audits that prepared candidate.
3. **Conflicted path (expected, not yet proved against current base):** the normal preparation workflow must **remain blocked** and must not publish `automation/upstream-candidate-...`. Present the actual conflict set and a **Gate 2 conflict-recovery authorization** before entering source reconciliation on a separate integration branch. This Gate 2 variant approves the **recovery approach**, **not** a prepared candidate; the ordinary candidate-publication acceptance is deferred until reconciliation is complete.
4. **Recovery prerequisite:** before relying on a new contract in GitHub Actions, explicitly review, validate and (only with a separate owner approval) merge a **small tooling-only compatibility PR** adding the minimum required support for pinned-target discovery, manual resolved-conflict evidence, and matching validation/promotion checks. This is a distinct **owner-gated precondition**, not an unattended side effect of Gate 1. It must not change CodexDD product/runtime/routing or tracked upstream, and its CI/Windows tests are separately assessed. This is necessary because `workflow_dispatch` for promotion uses workflow/helper files from production checkout, **not** code merely drafted on the future candidate branch.
5. **Critical re-anchor after prerequisite tooling PR:** if that PR is explicitly merged, **production HEAD and tree change**, so all old manifests/candidate SHA bindings become stale. Re-run the selected-target discovery and reconcile changes on a newly accepted production base before staging source. Do not transfer an older manifest forward by editing SHA fields.
6. **Manual resolution:** on the approved isolated integration branch only, reconstruct the target-upstream tree plus CodexDD customization delta with exact provenance, resolve textual conflicts in small audited work packets, then verify all 136 current-overlap paths and any new-baseline changes. No automatically created/force-replaced canonical candidate branch is allowed during this process.
7. **Post-resolution attestation:** preserve the initial **blocked** conflict manifest and original 22/current conflict evidence without alteration. Produce an **additional, versioned, clearly typed conflict-resolution attestation** linking the accepted production/tracked/target/tree identities, full conflict paths, each owner/resolution, final resolved source tree and semantic-review evidence. The optional narrow contract extension must reject mismatched, absent, stale or fabricated resolution evidence and require the native **release** PASS. Do not falsely reclassify the original failed dry run as clean; do not treat a clean resolved tree as semantic acceptance.
8. **Final canonical candidate:** only after source resolution and audit, explicitly publish **once** to the deterministic `automation/upstream-candidate-rust-v0.162.0-c1382380de69` branch using an absence-guarded ref update, with the allowed versioned candidate manifest and verification evidence. The candidate must descend from its recorded production base. Any existing candidate ref is a stop condition, not permission to force-push. Accept candidate publication at a separate **Gate 2B** before final Windows promotion.
9. **Native promotion:** update/verify the default-branch local candidate validation and `upstream-promote.yml` path to recognize the additive conflict-resolution evidence **without** weakening original v1 clean candidate support, human semantic acknowledgement, accepted GitHub review-evidence link or full release-profile requirement. If a new contract cannot be proven safe and present on production, **stop** instead of using a manual PR that bypasses those checks.

The above is a **proposed** necessary bridge for this specific conflicting update, not an implemented capability. It deliberately adds an explicit precondition and gate to the existing 0.4.2 workflow rather than silently continuing after conflict. If the operator rejects the prerequisite tooling PR or conflict-recovery gate, 0.4.3 remains **blocked at preparation**, with no integration source edits.

## 3. Branch, manifest and checkpoint ownership

| Ref / artifact | Owner | Lifecycle and allowed writes |
| --- | --- | --- |
| `dd/astra-policy-v2` | Production | **No direct work-packet commits**; only owner-approved and validated PR merges |
| `dd/codexdd-v0.4.3-phase1-recon` | Assistant — planning | 1A.1–1A.3 read-only evidence and docs only; never a runtime candidate or promotion branch |
| Narrow `dd/codexdd-v0.4.3-upstream-recovery-tooling` prerequisite | Assistant — if Gate 1 accepts need | Workflow/helper/test-only conditional compatibility PR; no code/runtime version update; **owner must explicitly authorize its PR and merge** |
| `dd/codexdd-v0.4.3-upstream-0.162.0-integration` | Assistant — only after Gate 2 conflict recovery | Conflict resolution and small 3A–3F source checkpoints; born from the **freshly accepted production base**; no production writes, no claim of promotion authority |
| `automation/upstream-candidate-rust-v0.162.0-c1382380de69` | 0.4.2/approved extension candidate workflow | Reserved for **one** identity-bound, fully prepared/reconciled and accepted source candidate; local PowerShell validator and promotion workflow require this exact name |
| `docs/upstream-candidates/codexdd-upstream-rust-v0.162.0-c1382380de69.json` | Candidate artifact | Frozen identity-bound manifest, with validated resolution sidecar **if and only if** explicit extension authorizes conflicted path; never rewrite a blocked v1 as clean |
| Windows validation receipt | Daniel-CL profile + verifier | Must bind final candidate HEAD, manifest path/hash, current production SHA and exact upstream target; any source commit after receipt requires new validation and new receipt |

**Commit discipline:** each packet must checkpoint exact SHA, name owned files, verify the new HEAD, identify deferred work, and stop. Use normal **fast-forward-only** pushes for reviewed packet changes to staging branch; no destructive ref moves. A candidate branch becoming stale, moving after receipt, or conflicting with an existing ref is an operator decision requiring new evidence rather than an automatic rebase.

## 4. Historical 22-conflict ownership and explicit 3A–3F packet queue

The following ownership covers the **historical** conflict list from run 37936059803; 2A.1 must verify actual current-base conflicts before changes. File ownership also includes affected nonconflicting dependencies revealed in the exact 136 overlap inventory.

| Packet | Owner / bounded scope | Historical conflicted paths explicitly owned | Exit evidence |
| --- | --- | --- | --- |
| **3A.1** | Canonical scaffold, baseline & transplant provenance | None; fresh production/tracked/target anchors, owned source change baseline | Identity/tree and no-duplicate checkpoint |
| **3A.2** | Conflict-recovery branch & attestation contract integration | None; work only after Gate 2 and prerequisite bridge approval | Immutable original blocked evidence, typed resolution evidence |
| **3A.3** | Per-path ownership map, conflict matrix | Full set across packets, no opportunistic coding | Owner matrix accepted / unresolved list |
| **3B.1** | Adaptive config and rollout persistence | `codex-rs/config/src/config_toml.rs`; `codex-rs/core/src/config/mod.rs`; `codex-rs/rollout/src/policy.rs` | Config layering, `WorkflowState`, `AdaptiveRuntimeSignal` persistence |
| **3B.2** | Core thread/session + Worker/resume/fork | No direct historical textual conflict needed; core/runtime direct semantic overlaps | Worker/Designer, trusted signals, successor admission, fork/resume coverage |
| **3B.3** | App-server/protocol source merge | None of the historical generator conflicts; source definitions and consumers | `ThreadAdaptiveWorkflowState`, runtime signals and new upstream protocol APIs |
| **3B.4** | Generated schema/SDK reconstruction | Both app-server `.json.zst` exports; `schema/typescript/{ClientRequest,ServerNotification,ServerNotificationEnvelope}.ts` | Real generator output and drift-check; **no handwritten compressed artifacts** |
| **3B.5** | Narrow core/app-server/serialization tests | Test sources owned by 3B, not new features | Adaptive state lifecycle tests PASS or explicit blocked evidence |
| **3C.1** | Windows daemon/elevation/F2 source | `app-server-daemon/src/backend/windows.rs`; `app-server-daemon/src/lib.rs`; `tui/src/daemon_startup.rs`; `tui/src/startup_orchestration.rs` | Elevated SSH embedded **without** expected F2 warning; real errors warn, explicit daemon starts blocked |
| **3C.2** | TUI thread/session routing | `tui/src/app/{thread_routing,thread_session_state,working_directory}.rs`; `tui/src/app_server_session.rs`; `tui/src/app_server_session/{reasoning_defaults_tests,rollout_history}.rs`; `tui/src/session_start.rs` | Permissions/worktree/profile, Worker binding, history replay, detached fork |
| **3C.3** | Interactive controls and slash commands | `tui/src/app/tests/permission_selection_tests.rs`; `tui/src/chatwidget/slash_dispatch.rs`; `tui/src/slash_command.rs` | Slash/copy/paste/alternate-screen and keyboard behavior |
| **3C.4** | TUI/daemon targeted regression evidence | Follow-on narrow tests only | F2, startup, interaction regressions and no inherited PR #48 waiver |
| **3D.1** | Model/catalog/status compatibility | `tui/src/app/tests/model_catalog.rs`, status and catalog caller overlaps | Luna → Sol → Sol 6.1 → Astra policy unchanged; `/status` |
| **3D.2** | Catalog, capability trust & CLI tests | No new routing ladder or model migration | Existing model families/efforts supported, no unsupported auto escalation |
| **3E.1** | LVO and updater compatibility | Repo-native LVO and upstream prepare/promote regression set | Preserved mechanical source-edit floor; read-only discovery; blocked vs resolved candidate state |
| **3E.2** | Cargo/CI/toolchain/build/protocol drift | `Cargo.lock`, Bazel, lock/generated and Windows packaging changes | Rust toolchain **1.95.0** unchanged; locked build and Windows packaging evidence |
| **3E.3** | Product version, upstream release, provenance | `codex-rs/codexdd-version.txt`, `codex-rs/upstream-codex-release.txt`, `cli/tests/version_reporting.rs` | **Explicit** CodexDD **0.4.3** bump only on owner-accepted integration branch; upstream **0.162.0** in provenance; Cargo workspace upstream version reconciled |
| **3E.4** | Final candidate/receipt contract | The canonical manifest and optional approved conflict-resolution sidecar | Frozen candidate HEAD/tree, full manual semantic review, verified promo readiness |
| **3F.1** | Full diff/overlap and security audit | All then-current exact overlap paths, including 12 unclassified | Every overlap accounted for, preserve Windows privilege boundaries |
| **3F.2** | Source audit checkpoint and Gate 3 | Version/build/generated/native narrow-test evidence | Unresolved errors enumerated; owner accepts before release validation |

Packet ownership can be split **smaller** when a single row exceeds roughly 5–10 meaningful files or mixes source and generator/testing. **No listed packet has been implemented or tested at this time.**

## 5. Mandatory semantic invariants / acceptance tests

1. **Core persistence and recovery:** persisted `WorkflowState` and `AdaptiveRuntimeSignal`, bounded telemetry filtering, Worker context through session reload/resume/fork/detached child, trusted signal transfer, terminalization and LVO repair-budget persistence.
2. **Model policy:** GPT-6 Luna, Sol, Sol 6.1, Astra, accurate catalog capability data, `/status`, Worker/Designer bindings and explicit implementation/validation complexity floors remain unchanged. Do not import unsupported policy/model changes from upstream by default.
3. **Windows F2/security:** expected elevated SSH implicit startup uses embedded path **with no spurious F2 warning**; failed token probe logs diagnostic; unrelated genuine daemon errors warn; elevated explicit start/restart/bootstrap remain rejected. Never disable ACL, privilege, job, detached-process or breakaway protections for passing tests.
4. **App-server/protocol:** preserve CodexDD workflow-state RPC and runtime signals while incorporating new upstream fields; regenerate source-derived stable/experimental app-server schema exports, TypeScript and Python bindings; verify source/artifact parity.
5. **Windows non-overlap changes:** review upstream sandbox/service, signed installer, Windows CLI startup, Bazel 9.2 / Rust 1.95 dependencies. Zero direct fork overlap is not a safe-to-ignore classification.
6. **Read-only updater safety:** scheduled discovery has only read permission; explicit prepare must fail closed on stale identity/conflict/duplicate; no unattended candidate PR/merge/deployment; receipt/promotion remains exact-HEAD and manifest-hash bound.
7. **No cross-project work:** no Breakwater/Cycle Vengeance changes or unrelated adaptive enhancements.

## 6. Windows test and operator responsibility gates

**Operator only supplies short, meaningful SSH-to-Daniel-CL commands; assistant owns GitHub source changes.** No RDP or ordinary non-elevated interactive desktop session is presumed.

- **During Phases 2–3:** run static workflow/manifest/recovery Python regression checks first, targeted Rust/Windows tests per packet; use repository-owned `targeted` / `work-packet` LVO where applicable. Known Windows Rust stack-heavy tests may need `RUST_MIN_STACK=16777216` **on the unchanged test**, without treating an actual product failure as a stack workaround.
- **Before upstream candidate PR:** `scripts/codexdd-validate-upstream-candidate.ps1 -Profile release` on the clean **canonical** candidate branch, using the native **21-stage** `release` profile and `CODEXDD_UPSTREAM_VALIDATION_RECEIPT`. Verify current production ancestor, exact candidate HEAD, manifest hash, target identity, and any required additional conflict-resolution attestation.
- **Native smoke:** elevated SSH launch without `--no-daemon`, F2 **No warnings**, `/status` Adaptive Effort on/Astra; explicit elevated daemon start/restart/bootstrap fail; genuine warnings still surface; resume/fork and protected Worker flow. Check copy/paste and alternate-screen interaction behavior.
- **Non-elevated shared daemon:** PR #48's one-off waiver is **not inherited**. If no eligible normal desktop remains available, report the test as **UNVERIFIED** and obtain a *new, 0.4.3-specific* owner decision before any release approval. No Task Scheduler job-breakaway/ACL compromise.
- **Promote PR:** only after accepted sensitive semantic review evidence is linked and the native release-profile receipt passes, use the approved `upstream-promote.yml` contract. Review CI and branch protection; **separate explicit owner authorization** for squash merge, installation with previous executable backup, post-install F2/status and soak.
- Native release evidence is tied to **the exact final candidate HEAD**; any late source change means fresh targeted/work-packet checks as appropriate and a new release receipt. GitHub CI alone is not release-authoritative.

## 7. Stop, recovery and rollback rules

| Condition | Required action |
| --- | --- |
| Production moves before workflow preparation or prerequisite tooling PR merges | **Stop**, regenerate target-pinned baseline manifest and upstream conflict comparison; do not re-label old SHA fields |
| Tracked official tag, target official tag, tree or commit cannot be verified | **Stop**; no fallback to local tags, moving branch names or latest target |
| New 0.162.0 candidate ref already exists or is created during publish | **Stop**; no overwrite/force; owner-directed recovery with new independent evidence |
| Synthetic transplant conflicts | `blocked_transplant_conflict` remains true; no candidate branch or PR; need explicit conflict-recovery authorization |
| Unresolved overlap, untrusted generated resources, schema drift, unsafe security change | **Stop** before candidate/publication; isolate and checkpoint only coherent diagnostic work |
| Branch moves after native receipt, stale manifest or shifted production base | Receipt invalid; stop and revalidate from current authoritative state, with new native receipt |
| Non-elevated daemon test unavailable | Mark UNVERIFIED, no inherited waiver; owner decision required |
| CI fails / test warning is security relevant | No merge or install; diagnose within owned packet and revalidate |
| Post-install behavioral regression | Owner-directed rollback to **backed-up 0.4.2 executable**, preserve user `CODEX_HOME` and credentials, record actual runtime/provenance |

Destructive branch deletion, force pushes, direct production edits, skipped receipt checks, remote install/restart and release publication remain forbidden without the applicable explicit owner approval.

## 8. Gate 1 owner decision now required

**Evidence provided:** 1A.1 identity baseline, 1A.2 full exact tree inventory and semantic review, this concrete 1A.3 branch/recovery/ownership/validation design. **Source work is not authorized by having a plan alone.**

**Recommendation to owner:** accept **Phase 1 Gate 1** for the owner-selected **0.162.0** update and authorize a refreshed target-pinned, read-only current-base manifest plus the **planning and validation** of the narrowly scoped conflict-reconciliation prerequisite. This approval **does not** authorize direct production writes, tooling PR merge, an upstream candidate publication, a test waiver, regular source changes on a conflicted integration branch, CI promotion, install or deployment. A subsequent dedicated owner decision remains required if the refreshed dry run conflicts and the prerequisite/tooling approach is still needed.

**Next executable packet if Gate 1 is accepted:** **2A.1 — refreshed pinned-target manifest and safe conflict-preparation assessment**, with explicit checkpoint. If confirmed conflicts persist, stop before preparation and present the narrow reconciliation tooling/change gate. If no conflicts remain, use the normal safe `upstream-prepare.yml` path under its own explicit approval.

**Gate 1 = NOT YET ACCEPTED. Gate 2 = NOT YET ACCEPTED. No actual source integration performed.**
