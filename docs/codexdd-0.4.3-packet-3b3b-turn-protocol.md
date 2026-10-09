# CodexDD 0.4.3 — packet 3B.3b: turn protocol adaptive signal

**Status:** SOURCE RECONCILED; native build, generated schemas and security acceptance **PENDING**.

## Source and result

- Production fork: `4828e3b4232781963594cfaaebdc49c5531de8b1`.
- Official target: `rust-v0.162.0` at `c1382380de69521303b416720a52f42d51af6248`.
- Owned overlap: `codex-rs/app-server-protocol/src/protocol/v2/turn.rs`.
- Merge commit: `71fc6527face047cbba451fe676200bdede1fcdf`.
- Used the **complete official upstream 0.162.0 source**, adding the fork's `AdaptiveRuntimeSignalEnvelope` and `AdaptiveRuntimeSignalNotification` DTO definitions directly before the upstream turn diff notification. Removing precisely this 631-character addition recreates the official upstream file byte-for-byte.
- The envelope carries `source_turn_id`, `signal_kind`, `evidence_refs`, and optional `diagnostic_note`; the notification also binds `thread_id`. Existing serde, JSON Schema and TypeScript derives and `camelCase` conventions remain intact.
- The merged source was re-read from GitHub and matched the intended bytes. This is a **source-level** union only.

## Semantic and security obligations

- The signal must remain an **experimental** notification, not a stable arbitrary remote mutation route. The server must validate the initiating thread, signal origin, and authorization; the presence of a typed DTO grants no trust by itself.
- Complete 3B.3 server-side request handlers and event handling reconciliation; confirm trusted adaptive signals cannot be spoofed.
- Generated schema, SDK and compressed protocol exports are owned by 3B.4; regenerate from reconciled source, never hand-edit generated exports.
- Native Rust and Windows regression tests and semantic review are required before Gate 3; nothing in this packet constitutes a native PASS receipt.
- The original v1 discovery artifact remains `blocked_transplant_conflict` and the deterministic canonical candidate is **not** published.

**Next:** continue 3B.3 app-server handwritten overlaps, followed by 3B.4 generators and 3C Windows/TUI conflicts. All source work remains isolated from production.
