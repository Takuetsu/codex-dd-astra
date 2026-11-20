# CodexDD 0.4.6 — Packet 3E.1 isolated candidate identity

**Created:** 2026-11-18
**Candidate branch:** `dd/codexdd-v0.4.6-rc1`, isolated, never production and not a release publication.
**Source parent:** development checkpoint `6bee467cc006a30f267efa53e313cc56dd2de739`; last exact native regression-tested _source_ SHA `87e89e416611553c22807c9df6a4b25c7d0589ed`; [3D.1d evidence](codexdd-0.4.6-packet-3d1d-integrated-native-acceptance-2026-11-18.md), [Windows run #44403595032](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44403595032).
**Disposition:** **CANDIDATE METADATA ALLOCATED; CLI/TUI VERSION AND FINAL RELEASE VALIDATION PENDING.**

## Compact packet card

- **Outcome:** Isolate a candidate source that actually reports product version `0.4.6`, while leaving production `dd/astra-policy-v2` and the 0.4.6 source-development branch untouched.
- **Authority:** 0.4.6 is the owner-selected recovery release. Isolated Git objects/branch, source metadata, test expectation and snapshot only. No artifact, tag, merge, installation, game changes or owner release acceptance.
- **Source identity:** Base `6bee467cc006a30f267efa53e313cc56dd2de739` from `dd/codexdd-v0.4.6-agent-reliability`. Branch candidate HEAD is the commit that adds this file and makes the three exact version changes; verify it independently before building.
- **Checks:** Exact source/branch/parent, `codex-rs/codexdd-version.txt`, upstream marker, CLI `--version`, TUI `/copy` status snapshot, V8 checksums, release harness, and on-candidate Windows targeted proofs before any release claim.
- **Stop:** Candidate identity source check is NOT 3E.2 integration, 3F semantic/security review, Phase 4 real-agent validation, or full native release acceptance.

## Candidate allocation (only these source-facing changes)

1. `codex-rs/codexdd-version.txt`: `0.4.4` -> `0.4.6`.
2. `codex-rs/cli/tests/version_reporting.rs`: previously stale `codexdd 0.4.3+g` -> `codexdd 0.4.6+g`. A source assertion, not an executed CLI binary proof.
3. `codex-rs/tui/src/chatwidget/snapshots/codex_tui__chatwidget__tests__slash_copy_whole_status.snap`: previously stale `codexdd: 0.4.3 (dev)` -> `codexdd: 0.4.6 (dev)`.
4. This packet's document records the acceptance boundary.

**Distinct upstream identity:** `codex-rs/upstream-codex-release.txt = rust-v0.162.0`; `codex-rs/Cargo.toml` workspace version remains `0.162.0`. No new upstream merge, dependency/lock manipulation, generated protocol update or 0.4.5 import is authorized.

## Validation and promotion holds

- Because the candidate changes source identity/test expectations, `87e89e41` Windows green is historical _implementation_ evidence, not a candidate-bound release receipt. Obtain source-matched focused Windows `codex-cli --version`, TUI snapshot and relevant Core/TUI integration checks on this exact candidate commit. Do not assume older CI runs on a different SHA are current.
- The development CI workflow `.github/workflows/codexdd-046-3b1-github-regression.yml` is intentionally gated to the source-development branch and its `0.4.4` marker. It will not become valid candidate CI by simply pointing at this branch; a separate narrowly approved 3E.2 candidate check is required.
- The repository's existing `scripts/codexdd-test-release.ps1` defines 21 stages, including Rustfmt, clippy, native Core/TUI/protocol/state/restart checks and `cli-release-build`. Rusty V8 pointer-compression/sandbox asset checks must use repository-pinned checksums, not substituted archives.
- P0 source/workflow and fake READY regression requirements remain binding. Conditional P1 worker-seat/tool_choice hypotheses remain deferred without an exact supported in-capacity repro.
- Later packets: **3E.2** candidate-specific Windows integration and real-agent usability; **3F.1** independent source/security audit; **3F.2** candidate artifact/provenance; **4.1–4.3** Daniel-CL smoke, representative paired work, exact native release profile, separate owner signoff. No installed host runtime-health receipt or release authorization is created by this checkpoint.

## Next action

Verify candidate HEAD and the four-file diff against `6bee467cc006a30f267efa53e313cc56dd2de739`, perform the cheapest possible source identity/format check and then arrange focused Windows candidate-bound CLI/TUI validation. Do not run the 21-stage profile simply to test metadata or install an unvalidated candidate.
