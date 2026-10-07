# CodexDD 0.4.1 packet 1A.3 — persistence, status, and regression impact map

## Status

Reconnaissance complete. The 0.4.1 family addition is backward-compatible with persisted 0.4.0 adaptive state and does not require a workflow-state schema bump.

## Durable persistence path

Adaptive family state is persisted as strings, not as a wire enum.

The durable chain is:

1. `AdaptiveEffortState::durable_workflow_snapshot` emits family labels.
2. app-server `ThreadAdaptiveWorkflowState` carries `starting_family: Option<String>` and `current_family: Option<String>`.
3. history `AdaptiveWorkflowStateSnapshot` stores the same fields as `Option<String>`.
4. rollout workflow-state records retain the snapshot.
5. `session_state::restore_family` delegates to `AdaptiveFamily::parse` during resume/fork restoration.

The current workflow-state schema remains valid for `sol61` because the serialized field type is already a string.

## Compatibility contract

0.4.0 persisted values remain valid without translation:

- `luna` -> Luna
- `sol` -> GPT-6 Sol
- legacy `terra` -> GPT-6 Sol
- `astra` -> Astra

  0.4.1 adds:

- `sol61` -> GPT-6.1 Sol

Existing `sol` snapshots must continue to restore to `gpt-6-sol`. They must never be silently upgraded to GPT-6.1 Sol.

This is the primary persistence invariant for 0.4.1.

## Resume and fork behavior

Resume/fork already restores the durable adaptive route separately from ephemeral authority. The new family only requires `AdaptiveFamily::parse` to understand `sol61`.

Ephemeral state remains intentionally excluded after restart:

- pending signals;
- pending attempts;
- successor permits;
- unfinished-turn pressure;
- evidence registries.

No change to those rules is required.

A targeted regression must prove that a persisted `sol61` current family restores to `AdaptiveFamily::Sol61` and that effective routing resolves to `gpt-6.1-sol`.

A separate compatibility regression must preserve the existing `sol` -> `gpt-6-sol` restore behavior.

## /status

`adaptive_effort_status_text` currently maps adaptive families to human labels.

0.4.1 adds:

- `Sol61` -> `Sol 6.1`

Expected examples:

- `Preference: Sol 6.1`
- `Current: Sol 6.1 Medium`
- `Implementation floor: Sol 6.1 Low`

The existing LVO status evidence remains unchanged.

## App-server protocol / generated schema

No structural protocol change is required.

`ThreadAdaptiveWorkflowState.starting_family` and `current_family` are already optional strings. Because no new enum/tag/field is introduced:

- app-server Rust wire structs do not need a shape change;
- JSON schema does not need a family-enum regeneration;
- generated TypeScript/Python protocol types do not need a family-specific update;
- workflow-state schema version remains unchanged.

Tests should still exercise the new string value through persistence to prevent accidental parser drift.

## Regression surfaces

At minimum, 0.4.1 must cover:

1. family identity:
   - parse `sol61`;
   - model mapping -> `gpt-6.1-sol`;
   - legacy `terra` still maps to existing Sol;
2. automatic ladder:
   - Sol High -> Sol 6.1 Low;
   - Sol 6.1 Low -> Medium -> High;
   - Sol 6.1 High -> Astra Low;
   - Astra Max still blocks;
3. complexity:
   - Architectural -> Sol 6.1 Low;
   - lower classes unchanged;
4. quality/budget:
   - Surplus Validation -> Sol 6.1 Medium;
   - Conserve/Balanced unchanged;
   - all mechanical-validation routes remain Luna;
5. capability gate:
   - unfinished/native failure pressure cannot silently cross either new family boundary;
6. status:
   - Sol 6.1 family renders distinctly from Sol;
7. persistence:
   - new `sol61` round trip;
   - old `sol` remains old Sol;
8. admission:
   - effective `gpt-6.1-sol` + matching effort authorizes the corresponding pending route;
9. Worker role behavior:
   - Implementation/Validation/Repair semantics remain unchanged apart from the accepted floors;
10. LVO:

- targeted/work_packet/release profile behavior remains unchanged.

## Daniel-CL gate

The meaningful operator gate should come after the GitHub-side implementation and targeted source audit are complete.

Daniel-CL validation should then prove compilation plus the focused adaptive test surface before CI. There is no value in asking Daniel to run commands during reconnaissance because 1A.1-1A.3 contain no executable behavior.
