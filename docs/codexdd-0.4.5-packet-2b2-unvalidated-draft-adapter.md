# CodexDD 0.4.5 — Packet 2B.2: UNVALIDATED draft-PR / owner handoff

**Date:** 2026-10-18
**Status:** Daniel-CL targeted validation **PASS: 18/18 tests in 5.603s** at `02809dd07e58ef0088a71364f8601e7c1cd7098d` (operator screenshot). No PR was created.
**Owner-controlled feature branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-automation`.

## Operational status

An isolated upstream candidate must **not** imply a validated release. The existing `upstream-promote.yml` remains the **native-receipt-bound** path for owner-accepted, validated promotion, and v2 reconciliation is separate. GitHub Actions previously denied automated PR creation even when `pull-requests: write` was requested. No workflow permissions, GitHub settings, repository PRs or production branch were changed by this packet.

The new `.github/scripts/upstream_auto_pr.py` is uninvoked and has a literal **`DRAFT_PR_WRITES_ENABLED = False`** gate. The 2B.1 branch-publisher guard is still `AUTO_PUBLISH_ENABLED = False`, while the low-risk path allowlist remains EMPTY. No scheduled or dispatch workflow imports the draft adapter, and no GitHub Actions token is authorized to use it.

## Candidate, identity and denial contract

1. Produce an explicitly `UNVALIDATED / NEEDS_WINDOWS_AND_OWNER_REVIEW` *draft* template with exact original manifest SHA-256, expected candidate Git HEAD/tree, production SHA and upstream tag/commit, source provenance and an incomplete Daniel-CL receipt checklist. The body categorically prohibits merge, tags, installation, CI waivers and auto-merge.
2. Before any future explicitly authorized draft attempt, recheck canonical v1 manifest bytes, zero-overlap and approved-source policy, exact current production remote SHA, candidate branch SHA, official upstream tag object IDs and peeled commits/trees, candidate commit parent, full overlay inventory, immutable manifest blob, release marker, unchanged CodexDD product version and lockfile.
3. Query historical PRs **across all states**, not only open PRs, and block duplicate or ambiguous queries. Recheck immediately before creation (GitHub has no create-PR CAS and write-time races still require conservative handling). Missing GH token, unavailable GitHub API and stale identity must not be interpreted as a green candidate.
4. Future owner-approved create action uses only `gh pr create --draft --repo Takuetsu/codex-dd-astra --base dd/astra-policy-v2 --head <exact-candidate> --title ... --body-file ...`. Return success only after read-back proves **draft**, exact branch/HEAD, base and canonical repository URL.
5. For GitHub's explicit `GitHub Actions is not permitted to create or approve pull requests` denial, return `owner_action_required` with exact candidate SHA/manifest and instructions for a separately owner-authorized route; **never retry with alternate credentials, silently change repository security settings, mark PASS, approve, merge or publish**. Unrecognized errors return `blocked_incomplete` without a success claim.

## Tests and next boundary

`test_upstream_auto_pr.py` adds 18 regression tests using the already validated temporary real-Git 2A.3/2B.1 fixtures, with mocked GitHub CLI calls only. Coverage includes disabled paths, template warnings, forged receipt/manifest/branch, moved production/tag/candidate, exact remote tree/manifest, duplicate closed PRs and truncated queries, missing tokens, Actions-policy failure, unexpected failure, positive draft-only simulated response and draft-ref readback mismatch, pre-create race, safe handoff, workflow permissions and no automated invocation.

2B.2 test: `py -3 "$wt\.github\scripts\test_upstream_auto_pr.py"` from the detached test worktree inside `E:\codexdd`. Prefer running this together with packet **2B.3 receipt/permissions audit** at one cheap validation checkpoint. Do not run the 21-stage Windows suite, create a real PR or enable a writer.

**Actual 0.163.0:** 109 upstream/customization overlap paths and 15 textual conflicts remain blocked for later exact owner-reviewed v2 reconciliation. Neither a draft template nor an Actions status substitutes for that acceptance.
