# CodexDD 0.4.5 — 3F.55: formal 109-path semantic owner reviews opened

**NONRELEASE.** All 109 original upstream-0.163 overlapping source paths have individual, exact-Git-blob review entries at `docs/upstream-candidates/codexdd-045-3f55-formal-owner-approvals.json`. This is a new, independent per-path signoff process; it does not rewrite the historical overlap baseline or pretend source-provenance tests imply semantic acceptance.

## Current decision counts

- **109 of 109** owner-specific review records created, with current candidate Git blobs and exact official-0.163 source IDs.
- **37** have supporting source/behavior evidence attached and may proceed to accountable owner review. This covers all 12 generated schemas and SDK outputs, all 11 app-server/protocol paths, five Bazel/dependency paths, and nine original core/TUI direct-conflict paths.
- **72** have verified source identities but remain queued for per-path upstream intent, fork behavior and native/security review.
- **0 of 109** formally signed off. The original matrix remains `PENDING_OWNER_REVIEW / NOT_ACCEPTED`. A review is not an approval, and an ancestor green run cannot be recharacterized as exact-final-source native acceptance.

## Evidence anchors and source boundaries

`3F.53` [native schema generator #41062921661](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41062921661) proved stable/experimental schema, configuration and generated Python SDK zero byte drift on unchanged source inputs. `3F.54` [source/format check #41065052850](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41065052850) was green on `72cd09d3f52c076d445078758af20489ccdb4e74`; the user supplied a matching Daniel-CL 16-stage/9-selector native run receipt (954 seconds; 2026-10-30T14:09:19Z). `3F.48` [optimized five-program build #40922368730](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40922368730) proved five EXEs/PDBs on exact earlier source `57152222a1ef523fe6e2f5079871672423c6cb56`.

These receipts are recorded per owner path *with their actual source revision*, not claimed to be complete proof for every API route, CLI flag, memory replay, authority check, daemon startup or release archive.

## Owner acceptance protocol

For each path, an accountable reviewer must:
1. Compare exact official upstream 0.163 semantics against CodexDD's fork differences, including deletions, trust boundaries, API behavior and backward compatibility.
2. Confirm the path's specific `required_evidence` and native/negative test outcomes at the recorded candidate source; no source drift after acceptance.
3. Set the explicit per-path `formal_owner_decision=OWNER_ACCEPTED` only with a real reviewer identity and UTC timestamp; review fixture tests reject missing fields or non-unique path IDs.
4. Keep per-path product `release_status=NOT_ACCEPTED` until the *separate* 109-path release transaction is approved. A generated-schema proof and scoped native tests cannot automatically set this flag.

The final 21-stage release profile is intentionally still blocked: the main owner matrix retains 109 pending source decisions. The seven-program Windows *unsigned developer* packaging is a separate 3F.56 task; full signing, voice DLL closure, installer, release authorization and distribution testing remain outside this formal source-review initiation.

**Do not merge, sign, publish or install 0.4.5 from this ledger.** Production remains 0.4.4 / rust-v0.162.0.
