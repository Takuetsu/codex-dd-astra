# CodexDD 0.4.5 — 3F.67 API-key model-popup capability boundary (NONRELEASE)

## Source-review finding

At startup the TUI already calls `AppServerSession::constrain_api_key_models` to enforce `ApiKeyCyberAccessPrograms` forwarding requirements. On `/model` refresh, however, the 0.4.5 TUI accepted a raw `ModelListResponse` through `AppEvent::ModelsLoaded` and replaced its filtered catalog immediately. The earlier upstream `ApiKeyCompatModelsLoaded` deferred event was removed from both the dispatcher and `AppEvent` type, leaving a credential/feature boundary inconsistent between bootstrap and interactive refresh.

## Scoped restoration

- Restore a dedicated `ApiKeyCompatModelsLoaded` event and handle it in the TUI app dispatcher.
- When the popup request ID is current and the account is an API key, launch the existing bounded `constrain_api_key_models` async lookup and **do not** apply the raw catalog.
- Carry the original request ID through completion; `ChatWidget::on_models_loaded` rejects obsolete requests.
- Reject API-key-filtered callbacks when the account is no longer API-key, preventing stale cross-account promotion.
- Keep non-API-key and error replies on the previous path. No new model privileges are granted.
- Add a native TUI regression that supplies a raw model advertising `DaybreakBlue`, checks the catalog is not updated prematurely, verifies the later stripped model applies, and verifies a forged stale callback after account switch cannot reinstate access programs.

The test may expose previously uncaught runtime interactions and must be validated on Windows **before** any owner acceptance. Do not weaken feature filtering or change the test to green.

## Remaining holds and release discipline

Formal review stays **0/109 OWNER_ACCEPTED**, with all **13 high-priority paths still held**. The bounded live thread-event queue/backpressure issue is unresolved. The 3F.62 snapshot-gate failure, 3F.65 native acceptance and 3F.66 offline-view source gate are tracked separately with immutable source identities. No full 21-stage validation, merge, release, signing, publication, version promotion or installation. Production stays 0.4.4.
