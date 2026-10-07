# CodexDD 0.4.1 packet 1A.1 — GPT-6.1 Sol model catalog/capability reconnaissance

## Scope

Packet 1A.1 is reconnaissance only. It changes no adaptive runtime routing.

Branch: `dd/codexdd-v0.4.1-gpt-6.1-sol`

Production baseline: `013ab39780bd0adfb503f75742bb90f23e04d330`

Frozen upstream workspace: OpenAI Codex `rust-v0.159.2`

## Result

GPT-6.1 Sol is already present in the frozen `rust-v0.159.2` model catalog inherited by CodexDD. No upstream refresh is required merely to represent the model.

Primary identity:

- slug: `gpt-6.1-sol`
- display name: `GPT-6.1-Sol`
- bundled description: `Latest workhorse model for coding and everyday work.`
- priority: `1`
- visibility: `list`
- API-supported: yes

## Model-catalog source of truth

Codex model metadata starts in `codex-rs/models-manager/models.json` and flows through `codex-rs/models-manager/src/manager.rs`.

The models manager:

1. seeds the in-memory catalog from the bundled model file;
2. can refresh from an authenticated provider `/models` endpoint;
3. caches accepted remote metadata in `models_cache.json` with client-version and provider/auth identity checks;
4. accepts visible ChatGPT/OpenAI API-key remote catalogs as authoritative, otherwise merges remote entries over the bundled catalog;
5. resolves runtime `ModelInfo` from the active catalog.

CodexDD should therefore select stable model identities and rely on active Codex metadata for model capabilities rather than duplicate full capability records.

There is no separate authoritative `model_family` field in `ModelInfo`. Model identity is slug-based with longest-prefix lookup. Narrow provider namespaces are already supported by namespaced-suffix lookup, so provider spellings such as Bedrock's `openai.gpt-6.1-sol` do not require a new CodexDD family abstraction.

## Frozen GPT-6.1 Sol capabilities

The inherited `rust-v0.159.2` bundled entry advertises:

| Field                                | Value                                            |
| ------------------------------------ | ------------------------------------------------ |
| default Codex reasoning selection    | `low`                                            |
| supported Codex selections           | `low`, `medium`, `high`, `xhigh`, `max`, `ultra` |
| normal Codex context window          | `272000`                                         |
| max context override                 | `872000`                                         |
| input modalities                     | text + image                                     |
| verbosity                            | supported; default `low`                         |
| parallel tool calls                  | supported                                        |
| tool mode                            | `code_mode_only`                                 |
| multi-agent backend                  | `v2`                                             |
| multi-agent reasoning effort         | `xhigh`                                          |
| mid-session reasoning-effort updates | supported                                        |
| reasoning summaries                  | supported                                        |
| search                               | supported                                        |
| original image detail                | supported                                        |
| minimum client version               | `0.153.0`                                        |
| Responses Lite                       | enabled                                          |
| Fast service tier                    | supported                                        |
| truncation policy                    | tokens, limit `10000`                            |

These capability fields are unchanged in the current upstream Codex catalog checked on 2026-10-07.

## Reasoning-effort semantics

OpenAI's public GPT-6.1 Sol model reference supports `low`, `medium` (default), `high`, `xhigh`, and `max`. It does not support `none` or `minimal`.

Codex additionally surfaces `ultra`. In Codex, `ultra` is an orchestration/UI selection rather than an additional Responses API reasoning value. `ModelInfo::resolve_reasoning_effort` resolves `ultra` to the model's configured `multi_agent_reasoning_effort`; GPT-6.1 Sol configures that as `xhigh`.

For later CodexDD routing design, ordinary reasoning escalation should use concrete efforts. `ultra` should be selected only where proactive Codex multi-agent behavior is intentionally desired.

The public API default (`medium`) differs from the bundled Codex picker default (`low`). CodexDD adaptive policy should continue setting an explicit effort when policy requires a specific tier.

## Context-window note

Public OpenAI documentation currently lists a 1,050,000-token context window and 128,000 max output for GPT-6.1 Sol. The frozen Codex catalog exposes `272000` as its normal context and `872000` as its maximum override.

CodexDD runtime policy must use the active Codex model metadata rather than substitute public API headline limits.

## Acceptance

Packet 1A.1 is accepted for continuation into routing reconnaissance because all packet criteria are satisfied:

- exact slug/display identity established;
- frozen-upstream availability proven;
- active catalog source-of-truth traced;
- supported Codex selections distinguished from API wire efforts;
- Codex `ultra` semantics established;
- relevant capability metadata recorded;
- provider namespacing understood;
- runtime routing remains unchanged.

## Sources inspected

Repository:

- `codex-rs/models-manager/models.json`
- `codex-rs/models-manager/src/manager.rs`
- `codex-rs/protocol/src/openai_models.rs`
- `codex-rs/protocol/src/openai_models/reasoning_effort.rs`
- OpenAI Codex `rust-v0.159.2`
- current OpenAI Codex mainline catalog for capability comparison

Public OpenAI references checked 2026-10-07:

- `https://developers.openai.com/api/docs/models/gpt-6.1-sol`
- `https://developers.openai.com/api/docs/guides/reasoning`
- `https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6.1-sol`
