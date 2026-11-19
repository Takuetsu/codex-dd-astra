# CodexDD 0.4.6 — 3B.2g trusted Core Worker snapshot and cold-resume intake

**Parent:** `9b83f0273e18bd3a5c0fe931d5fc751a6ac11e8a` (3B.2f GitHub Actions #42310439125 green).
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.
**State:** exact-head Windows validation pending; no release, merge or install.

## Changes
The Core-first accepted user Worker assignment can now be serialized as a typed `CodexddFirstWorkerSnapshot` inside Core-written `ThreadSettingsApplied` rollout snapshots. New accepted first-user bindings request an immediate native settings checkpoint and flush, rather than relying on model-provided final text or TUI-local workflow state. The same native snapshot is carried by later thread-settings updates. A normal first message persists no role. Startup-configured Worker bindings are not silently converted into first-user credentials.

For cold resume, Core reads the newest native `ThreadSettingsApplied` snapshot **for the exact resumed thread ID**, only when prior user-turn history exists, validates the role/scope/source shape and re-locks the assignment. A fork never imports the parent's binding; legacy histories or invalid snapshots stay locked and unprivileged. New tests cover serialization roundtrip, malformed and partial-source rejection, and launcher binding precedence.

## Explicit hold
The restored binding remains an admission/governance latch in Core session state. It is **not yet installed into `TurnContext.config` for protected LVO**, and no candidate-SHA transition is authorized. A native protected profile must continue issuing `SETUP_BLOCKED` unless an independently verified Core turn source credential exists. The source of the first user assignment cannot bypass the exact selected-worktree and SHA checks.

## Remaining
- Verify native Core snapshot flush before granting source execution, including failure and interruption cases.
- Connect trusted persisted Core binding into a new turn context without granting authority from TUI/model/tool arguments, and implement explicit owner-approved candidate-SHA transition.
- Run real Node/Rust Windows profile and cold-resume acceptance with an independent Validator.
- 0.4.5 remains frozen; GitHub development branch only.
