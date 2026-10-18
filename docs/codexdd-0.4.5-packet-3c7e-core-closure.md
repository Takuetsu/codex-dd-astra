# CodexDD 0.4.5 — Packet 3C.7e active core Rust closure

**Status: INCOMPLETE / NOT A RELEASE CANDIDATE.** Three active Rust sources staged as exact upstream 0.163.0 Git blobs. Seven static Python checks added but unrun. This is not native Rust validation.

`exec_policy/executable_identity.rs`, its tests, and `mcp_tool_call/conversation_history.rs` were byte-identical between pinned official 0.162.0 and production CodexDD 0.4.4. They are staged from official 0.163.0 without replacing CodexDD-specific policy. The associated static test pins original and target Git blobs without requiring the upstream commit in Daniel-CL's Git object database.

Three retired Unix zsh-fork modules remain on disk but are not registered by 0.163.0. Their deletion is intentionally deferred. This closes the active Rust path inventory under core/src, not the entire cross-crate source graph or API compatibility.

Prior 3C.4–3C.6 static checks passed 106/106 on Daniel-CL. 3C.7 checks, **Cargo.lock** reconciliation, native Rust compilation, focused runtime/security tests, generated schemas, full Windows release suite and CI are pending. Publisher remains disabled; product marker stays 0.4.4. Do not merge or publish this checkpoint.
