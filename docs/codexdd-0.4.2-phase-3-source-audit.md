# CodexDD 0.4.2 Phase 3 source and workflow audit

**Status: SOURCE AUDIT COMPLETE — UPDATED-HEAD LOCAL CHECKS PENDING**

**Release:** CodexDD 0.4.2

**Branch:** `dd/codexdd-v0.4.2-automatic-upstream-update`

**Production base:** `f45c5a119296d6e53c6429f9b101b3f52d91047e`

## Native evidence already received

Daniel-CL completed the first 0.4.2 implementation validation gate before the Phase 3 audit:

- `codex-rs/cli/tests/version_reporting.rs`: PASS, 1/1;
- Python upstream-sync regression suite: PASS, reported by the operator as part of the chained gate;
- repository-owned `work-packet` validation: PASS, 6/6 stages;
- adaptive TUI nextest subset: 189 passed;
- native `profile_end`: `status=pass`, `exit_code=0`, `completed_stages=6`.

The native work-packet receipt was for commit `da6535c390e915d11265caa60257358ac76e26ac`, **before** the additional Phase 3 workflow hardening. These source changes require updated-head revalidation. Do not claim the earlier PASS applies to the newer branch HEAD.

## Phase 3 audit findings and repairs

### 3A.1 — official upstream tag provenance

**Finding:** The initial discovery implementation selected from the combined local tag namespace after fetching upstream. A fork-local `rust-v*` tag not present in official `openai/codex` could enter discovery.

**Repair:** Select candidate release names strictly from `git ls-remote --tags --refs upstream`, pointing at the official repository. Require the tracked release tag to exist there as well. Preparation re-verifies both tracked and target tags against official upstream before transplant and before remote writes.

### 3A.2 — stale tag and duplicate-branch race

**Finding:** Preparation initially rechecked the production SHA and target tag just before publishing, but it did not revalidate the tracked tag and could race with another actor creating the same candidate branch after `ls-remote`.

**Repair:** Recheck both official tag identities and the exact production baseline. Candidate publication uses a Git `--force-with-lease=refs/heads/<candidate>:` expectation that the remote candidate branch is **absent**; this is an absence guard, not permission to overwrite an existing candidate.

### 3A.3 — semantic-overlap classification

**Finding:** The original classifier flagged named adaptive files and status but could miss significant upstream overlap in `tui/src/app.rs`, `chatwidget.rs`, core thread/session call sites, configuration, and app-server code.

**Repair:** Conservatively classify TUI, core/runtime, app-server/runtime, and configuration paths as sensitive in addition to the original specific categories. All overlapping files remain recorded in the full manifest, including overlaps not categorized as sensitive.

### 3A.4 — premature promotion of sensitive candidates

**Finding:** The initial promotion workflow would accept a candidate in `manual_semantic_review_required` state with only a `work-packet` receipt.

**Repair:** If the candidate manifest requires semantic review, promotion now additionally requires all of:

1. the operator explicitly acknowledges accepted semantic review in a manual workflow-dispatch boolean;
2. a repository GitHub link to the accepted review evidence;
3. a native candidate-bound `release` validation PASS receipt.

For non-sensitive candidates, the original candidate-bound `work-packet` receipt remains an eligible promotion gate.

The accepted review evidence link is included in the PR description for auditability. CI and owner merge remain separate consequential gates.

## Safety contract retained

- scheduled discovery: repository read-only; no remote mutation;
- manual preparation: candidate branch only; no automatic product version bump or PR;
- manual promotion: requires current branch, manifest, receipt, upstream provenance, and production identity; may open PR only;
- production merge, install, deployment, and release publication: explicit operator action only;
- OpenAI Codex `rust-v0.160.1` remains a future candidate and is **not integrated** into 0.4.2.

## Residual limitations

- A JSON receipt supplied manually to GitHub is an operator-provided assertion tied to candidate identities and native validation fields. Its contents are **not independently cryptographically attested** to the Daniel-CL machine. Explicit operator dispatch, native local test output, GitHub review evidence (where required), and ordinary CI remain necessary controls.
- The workflows have not yet been exercised end-to-end on the default branch; they must not be declared functionally proven based on Rust work-packet PASS alone.
- The read-only discovery workflow continues to fetch official upstream tags and perform local Git work, which may use CI runner minutes. Its read-only permission constrains repository side effects.
- No GitHub PR or CI has been started for 0.4.2 yet.

## Next local gate

On Daniel-CL, fetch the updated feature HEAD and run:

1. `git diff --check` against production;
2. `python .github\scripts\test_upstream_sync.py`;
3. Windows PowerShell parse check for the new candidate-receipt script;
4. YAML parse/format validation for the three upstream workflows (using the repository's installed formatting tools if present).

Do not start the expensive release profile or PR/CI until the cheap updated-head checks are green.

Once they pass, the broad `release` profile is the next authoritative Windows gate. 
