# CodexDD Incident Report — Bracketed Paste Markers Can Break Worker Binding on Windows

**Status:** Reported for future release scoping  
**Observed CodexDD version:** 0.3.2 (`2ec8acc393a7`)  
**Observed Codex TUI:** OpenAI Codex v0.155.1  
**Platform:** Windows / PowerShell / Windows terminal-console sessions

## Summary

Literal bracketed-paste markers such as `[200~` and `[201~` have intermittently leaked into Codex prompt text on Windows.

This becomes a CodexDD governance problem when it happens on the **first assignment** of a Worker because CodexDD intentionally requires the first nonblank line to be exactly:

```text
[adaptive_worker]
```

If a leaked paste marker precedes that header, the effective input becomes something like:

```text
[200~[adaptive_worker]
role = "implementation"
authorized_scope = "..."
...
[201~
```

The language model may still understand and act on the task prose, while CodexDD fails closed and never binds Worker authority.

Observed `/status` state in failed sessions:

```text
Worker role: Unspecified
Worker scope: None
Worker binding: Locked unbound
Workflow terminal: None
```

Manually typing the authority header before pasting the body reliably avoids the failure and produces a correctly bound Worker.

## Why this matters

This is not only a display/paste annoyance.

CodexDD's first-assignment parser is an authority boundary. A pasted task can appear to run successfully at the model layer while the CodexDD runtime has no bound role, scope, or valid structured terminal state.

Observed consequences include:

- implementation work proceeding in an apparently normal session while the Worker remains `Locked unbound`;
- validation sessions that produce prose verdicts but never have authoritative Worker state;
- repeated Worker restarts;
- manual recovery and duplicated validation effort;
- users needing to inspect `/status` after every fresh Worker creation to detect the failure.

A leaked trailing `[201~` after a Worker is already bound is mostly cosmetic. The critical case is a leading `[200~` before the first `[adaptive_worker]` line.

## Reproduction pattern

1. Start a new Worker using `/new`.
2. Paste a complete first assignment beginning with:
   ```text
   [adaptive_worker]
   role = "implementation"
   authorized_scope = "..."
   ```
3. Intermittently observe literal `[200~` and/or `[201~` in the submitted prompt/transcript.
4. The model may acknowledge and start the requested task.
5. Run `/status`.
6. Observe:
   ```text
   Worker role: Unspecified
   Worker scope: None
   Worker binding: Locked unbound
   Workflow terminal: None
   ```
7. Start another fresh Worker.
8. Manually type the authority-header lines and the required blank line.
9. Paste only the task body.
10. Run `/status`.
11. Observe correct binding:
   ```text
   Worker role: Implementation
   Worker binding: Bound
   ```

The same pattern has been observed for `role = "validation"`.

## Current reliable workaround

On affected Windows sessions:

1. Run `/new`.
2. Manually type:
   ```text
   [adaptive_worker]
   role = "implementation"
   authorized_scope = "..."
   ```
3. Manually enter the blank line after the header.
4. Paste only the task body.
5. Immediately run `/status`.
6. Continue only if the Worker role/scope are correct and `Worker binding: Bound`.

For Validation Workers, use the same sequence with `role = "validation"`.

## Suspected boundary

The observations are consistent with a bracketed-paste sequence crossing the terminal/TUI/input boundary before CodexDD performs first-assignment parsing.

Potential sequence:

1. terminal bracketed-paste mode wraps a paste with `ESC [ 200 ~` and `ESC [ 201 ~`;
2. Codex TUI/input handling intermittently fails to consume one or both markers;
3. the marker reaches the message payload, possibly textualized as `[200~` / `[201~`;
4. CodexDD sees a non-matching first nonblank line;
5. the authority parser correctly fails closed into `Locked unbound`.

This report does **not** establish whether the originating defect is in Windows terminal handling, upstream Codex TUI, or CodexDD's input handoff. That needs tracing.

## Recommended investigation

Instrument the first-assignment input immediately before CodexDD authority parsing and determine whether affected sessions deliver:

- raw `\x1b[200~` / `\x1b[201~` sequences;
- already textualized `[200~` / `[201~`;
- or some other normalized representation.

Also compare behavior across:

- Windows Terminal;
- classic console host / PowerShell;
- typed first assignment;
- full-message paste;
- multiline paste after a manually typed header.

## Recommended defensive fix

Keep the existing **fail-closed authority model**.

Do not make `[adaptive_worker]` generally fuzzy.

Instead, normalize only known bracketed-paste framing at the input boundary before first-line authority parsing.

Suggested implementation:

1. Detect recognized bracketed-paste start/end framing surrounding the submitted message.
2. Strip only that framing before authority-header evaluation.
3. Preserve strict rejection for unrelated prefixes or malformed headers.
4. Consider emitting a specific diagnostic if paste framing is detected but cannot be normalized safely.

Example diagnostic:

```text
Worker binding rejected: bracketed-paste marker detected before [adaptive_worker].
Retry with a clean first assignment.
```

That would be substantially easier to diagnose than generic `Locked unbound`.

## Regression coverage to add

Add parser/integration tests for:

1. normal typed header;
2. raw leading `\x1b[200~`;
3. raw trailing `\x1b[201~`;
4. raw start + end markers;
5. textualized leading `[200~`;
6. textualized trailing `[201~`;
7. textualized start + end markers;
8. unrelated malformed prefix still fails closed;
9. successfully normalized input binds the correct role and scope;
10. post-binding continuation prompts containing a trailing marker do not alter existing authority state.

## Release-scoping recommendation

Treat this as a CodexDD robustness/governance fix rather than only terminal polish because it can create a split-brain state:

- model task execution appears active;
- CodexDD Worker authority is absent.

A future release should ideally address both:

- the actual paste-marker leak at the lowest practical layer; and
- defensive first-assignment normalization/diagnostics in CodexDD.

## Tracking note

GitHub Issues are disabled in this fork, so this documentation PR is being used as the project-visible tracking artifact for future release mapping.
