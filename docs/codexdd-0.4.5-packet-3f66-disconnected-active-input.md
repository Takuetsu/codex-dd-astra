# CodexDD 0.4.5 — 3F.66 disconnected view input ownership restoration (NONRELEASE)

## Source-review finding
The official upstream 0.163.0 TUI event-loop review retained `active_view_accepts_input_when_disconnected()` checks for Ctrl+C/Ctrl+D, permitted interactive view keys and offline paste. The 0.4.5 candidate removed all three call sites, even though the `ChatWidget`, `BottomPane` and `BottomPaneView` permission methods remained implemented.

The removal breaks existing behavior when a remote app-server disconnects with an explicitly permitted local configuration/picker view open: its keyboard can be incorrectly restricted, Ctrl+C can exit instead of cancelling the view, and paste uses composer-specific recovery rather than view ownership.

## Exact constrained restoration
- Preserve the global Ctrl+C/Ctrl+D escape only when the active view is *not* explicitly permitted to handle input during disconnection.
- Route disconnected input to an explicitly permitted bottom-pane view or the agents overview. Do not treat unrelated active views as trusted.
- Restore the overview-to-conversation transition only when the overview is actually active.
- Preserve local offline composer editing, but do not route paste through the disconnected composer recovery handler when an explicitly permitted active view owns it.

This is a routing-only TUI source change. The existing native test `disconnected_config_panel_keeps_input` exercises config search/paste and Ctrl+C cancellation; `lost_mutation_reply_preserves_work_without_resubmitting` protects offline draft behavior and no duplicate submissions. Both require Windows native execution on the new source before owner acceptance.

## Status and nonrelease conditions
3F.66 is stacked on #75 / #74 / #73 / #72 and is **not** covered by their exact-SHA native receipts. Source-only Rustfmt and guard assertions are required first. The formal ledger remains `109 SOURCE_EVIDENCED / 0 OWNER_ACCEPTED`; the 13 explicit high-priority holds are not automatically waived. Unbounded event queue/backpressure, API-key model popup and realtime-stop negative coverage remain. No release, signing, merge, publish, install or version promotion. Installed product stays 0.4.4.
