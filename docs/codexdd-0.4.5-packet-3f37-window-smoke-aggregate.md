# CodexDD 0.4.5 — Packet 3F.37: rerun all eight focused Windows selectors after deterministic reconnect fixture

**NONRELEASE, 60-minute Windows validation; not 21-stage release acceptance.** [3F.35B run #40743751119](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40743751119) compiled `codex-tui` and validated the locked Windows dependency resolver/security source successfully. The first three selectors passed. Selector four failed an `insta` snapshot because the test expected a server-version warning but its fixture did not explicitly enable `show_server_version_notice`; the remainder were never run.

The [3F.36 source-only fixture proof #40753891316](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40753891316) is green and pins exactly one added test-fixture setting `app.local_settings.tui.show_server_version_notice = true` in the failed selector, preserving the original snapshot and production reconnect logic. Its one-line change is verified by inverse blob reconstruction, not an arbitrary snapshot rewrite.

**3F.37 Windows native plan:** exact current branch source and protected Cargo/deny pins, verify unchanged startup `e51254ea...`, session `1a99fcac...`, reconnect test `3faafe11...`, unchanged expected snapshot `335071a...`. Re-run all current 3F.32, 3F.20, 3F.34, 3F.25, 3F.36 and owner guards before Rust. Execute the same eight startup/adaptive, reconnect/exhaustion, daemon, API-key Daybreak, dynamic authorization and older-WS-session selectors as 3F.35B.

**Improvement:** continue through all eight selectors even if one fails, saving a separate full log for each and reporting the complete list of failed selectors at the end. This avoids another ~20-minute native compilation per first failure. A missing log or failed selector is never counted as green. No product runtime code, dependencies, packaging, installer, merge, version markers or 21-stage suite is changed.

A green 3F.37 would validate only these focused native behaviors, not direct WSL clipboard/daemon exit end-to-end testing, full 109-path semantic signoff or shipping optimized CLI/PDB/combined archive. Draft #58 remains nonrelease; CodexDD 0.4.4 stays installed.
