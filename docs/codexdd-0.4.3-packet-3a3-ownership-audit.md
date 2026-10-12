# CodexDD 0.4.3 — packet 3A.3: complete overlap and conflict ownership

**Status:** OWNERSHIP MATRIX COMPLETE — no path has been silently omitted or marked semantically accepted. Source conflicts remain unresolved. This is a recovery planning checkpoint, not prepared-candidate evidence.

## Source and reproducibility

- Current production base: `4828e3b4232781963594cfaaebdc49c5531de8b1`; official tracked: `rust-v0.159.2` / `ff6aec96948b70d94983af2641a6b67c94faeff5`; selected target: `rust-v0.162.0` / `c1382380de69521303b416720a52f42d51af6248`.
- Executed GitHub discovery [37960772252](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252) produced **257** customization paths, **2,507** changed upstream paths, **136** overlapping paths, **124** flagged sensitive paths, **12** old-classifier unmarked paths, and **22** textual conflicts.
- Source-of-truth blocked manifest digest: `59a9e5fbcace88827dc1edfd38abb0b6b45c8d41c7ff956d7af0ed94e9988ee3`; conflict file digest: `c3456c3e93130126f8f6f6bc1366bdafcc0c11835244ba70c31b91a564a450a4`.
- The exact 136-path list from prior Phase 2 planning was cross-checked against the new executed artifact's list (same 136 paths, FNV-1a32 `ad3f06bc`); the executed list's newline-joined SHA-256 is `27e3145698e3b5c486fd9660d61dbffb87ebf755c058bd29fb20958b01872bcd`.
- Complete [machine-readable map](codexdd-0.4.3-packet-3a3-overlap-ownership.json) has one named owner for every path and a `not_started` resolution/review state on **every** entry; 22 textual paths explicitly link to the exact original conflict report. All 136 path strings are distinct, and the 22 conflicts form an exact subset.

## Ownership counts (136 exact overlaps)

| Owner packet | Paths | Textual conflicts | Subsystem |
| --- | ---: | ---: | --- |
| 3B.1 | 8 | 3 | Config and rollout persistence |
| 3B.2 | 21 | 0 | Core lifecycle, session and thread metadata |
| 3B.3 | 14 | 0 | App-server and protocol handwritten source |
| 3B.4 | 13 | 5 | Generated app-server schemas and SDK |
| 3B.5 | 8 | 0 | Core targeted tests |
| 3C.1 | 4 | 4 | Windows daemon, startup and elevation F2 |
| 3C.2 | 19 | 7 | TUI/session routing, history and working directory |
| 3C.3 | 15 | 3 | Interactive controls, slash commands and copy/paste |
| 3C.4 | 26 | 0 | TUI/Windows regression tests |
| 3D.1 | 5 | 0 | Model catalog and status |
| 3D.2 | 1 | 0 | CLI entrypoint compatibility |
| 3E.2 | 2 | 0 | Cargo.lock and Bazel |
| **Total** | **136** | **22** | |

These owners are **implementation/review responsibilities**, not proof that any path is compatible. Owners may split further when a packet spans more than 5–10 meaningful source files; the canonical map must be updated with new checkpointed evidence if ownership changes.

## 12 policy-unclassified overlaps — mandatory security/semantic review

| Paths | Owner | Provisional priority | Reason |
| --- | --- | --- | --- |
| `app-server-daemon/src/backend/windows.rs`, `backend/windows_tests.rs`, `src/lib.rs` | 3C.1 / 3C.4 | **Critical** | Elevated SSH, automatic embedded fallback and explicit privileged-daemon guards |
| `cli/src/main.rs`, `cli/tests/daemon_startup.rs` | 3D.2 / 3C.4 | High | CLI startup/privilege behavior and daemon assertions |
| `core/tests/suite/agent_websocket.rs`, `scenarios.rs`, `unified_exec.rs` | 3B.5 | High | Real agent network and execution behavior |
| `external-agent-migration/src/sessions/append.rs` | 3B.2 | High | Session migration and append semantics |
| `rollout-trace/src/protocol_event.rs` | 3B.3 | High | Protocol event persistence/fidelity |
| `sdk/python/src/openai_codex/generated/notification_registry.py`, `v2_all.py` | 3B.4 | High | Source-derived generated SDK compatibility |

No unclassified path is excluded from semantic review because it escaped the original classifier. In a future classifier-hardening tooling packet, explicitly cover the missing Windows daemon and Python generated roots without losing existing v1 behavior.

## Resolution and review exit requirements

1. For all 22 textual conflicts, record the original conflict path, packet owner, incoming upstream change, retained CodexDD behavior, final source resolution, focused test and semantic acceptance evidence. Never use a blanket "prefer ours/theirs" rule without a file-specific review.
2. For the other 114 overlapping paths, record semantic comparison and resolution/disposition by owner even if Git transplant does not report a textual conflict.
3. For the 12 formerly unclassified paths, document manual security/generator risk classification and test evidence explicitly; no inference from a numeric sensitive-overlap counter is sufficient.
4. Compare the finalized integrated source against the **full then-current** upstream and fork deltas, not just these original 136 overlap paths; account for new upstream non-overlaps and generated changes.
5. Preserve the blocked original manifest, use only the separately validated v2 recovery contract for a resolved candidate, and re-baseline on any production movement.

**3A.3 exit:** full source overlap ownership recorded and cross-checked, with a durable matrix that future packets can update. **Next implementation gate:** add the isolated, tested reconciliation tooling bridge on a separate tooling-only PR, independently reviewed before any merge. Do not start candidate publication or promotion until that bridge is accepted.
