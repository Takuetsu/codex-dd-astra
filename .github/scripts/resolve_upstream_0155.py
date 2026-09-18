from __future__ import annotations

from pathlib import Path


UNION_FILES = {
    "codex-rs/build-info/src/build_info_tests.rs",
    "codex-rs/tui/src/app.rs",
    "codex-rs/tui/src/app/startup.rs",
    "codex-rs/tui/src/app/test_support.rs",
    "codex-rs/tui/src/app/tests.rs",
    "codex-rs/tui/src/app/tests/startup.rs",
}


def snippet(text: str) -> list[str]:
    return [line + "\n" for line in text.strip("\n").split("\n")]


def resolve_block(
    path: str,
    index: int,
    ours: list[str],
    base: list[str],
    theirs: list[str],
) -> list[str]:
    del base

    if path in UNION_FILES:
        return ours + theirs

    if path == "codex-rs/core/src/thread_manager.rs":
        if index != 0:
            raise RuntimeError(f"unexpected conflict {index} in {path}")
        return snippet(
            """
        let interrupted_marker = InterruptedTurnHistoryMarker::from_config_and_version(
            &options.config,
            multi_agent_version,
        );
        let workflow_state =
            workflow_state.unwrap_or_else(|| workflow_state_from_history(&history));
        options.initial_history = append_child_workflow_state_snapshot(
            fork_history_from_snapshot(snapshot, history, interrupted_marker),
            workflow_state,
        );
        let agent_control = self.agent_control_for_config(&options.config);
"""
        )

    if path == "codex-rs/core/src/tools/context.rs":
        if index != 0:
            raise RuntimeError(f"unexpected conflict {index} in {path}")
        return snippet(
            """
        let evidence_metadata_len = evidence_id
            .map(evidence_metadata_text)
            .map_or(0, |metadata| {
                metadata.len().saturating_add(/*newline*/ 1)
            });
        let output_budget = with_serialization_allowance(self.truncation_policy)
            .byte_budget()
            .saturating_sub(
                header
                    .len()
                    .saturating_add(/*rhs*/ 1)
                    .saturating_add(evidence_metadata_len),
            );
"""
        )

    if path == "codex-rs/tui/src/chatwidget/input_flow.rs":
        if index != 0:
            raise RuntimeError(f"unexpected conflict {index} in {path}")
        return snippet(
            """
    ) -> bool {
        self.queue_user_message_with_options_and_source(
            user_message,
            action,
            pending_pastes,
            UserMessageSource::Prompt,
        )
    }

    pub(super) fn queue_user_message_with_options_and_source(
        &mut self,
        user_message: UserMessage,
        action: QueuedInputAction,
        pending_pastes: Vec<(String, String)>,
        source: UserMessageSource,
    ) -> bool {
        if source == UserMessageSource::Prompt {
            self.invalidate_adaptive_successor_for_manual_input();
        }
"""
        )

    if path == "codex-rs/tui/src/slash_command.rs":
        if index != 0:
            raise RuntimeError(f"unexpected conflict {index} in {path}")
        # Upstream removed SandboxReadRoot and added Voice. Voice is already
        # outside this conflict block; preserve codexdd's Adaptive inline args.
        return snippet(
            """
                | SlashCommand::Adaptive
"""
        )

    raise RuntimeError(f"no resolver for conflict {index} in {path}")


def resolve_file(path: str) -> None:
    file_path = Path(path)
    lines = file_path.read_text(encoding="utf-8").splitlines(keepends=True)
    out: list[str] = []
    i = 0
    conflict_index = 0

    while i < len(lines):
        if not lines[i].startswith("<<<<<<< "):
            out.append(lines[i])
            i += 1
            continue

        ours_start = i + 1
        base_marker = next(
            j for j in range(ours_start, len(lines)) if lines[j].startswith("||||||| ")
        )
        separator = next(
            j for j in range(base_marker + 1, len(lines)) if lines[j].startswith("=======")
        )
        end_marker = next(
            j for j in range(separator + 1, len(lines)) if lines[j].startswith(">>>>>>> ")
        )

        ours = lines[ours_start:base_marker]
        base = lines[base_marker + 1 : separator]
        theirs = lines[separator + 1 : end_marker]
        out.extend(resolve_block(path, conflict_index, ours, base, theirs))
        conflict_index += 1
        i = end_marker + 1

    if conflict_index == 0:
        raise RuntimeError(f"expected at least one conflict marker in {path}")

    resolved = "".join(out)
    if any(marker in resolved for marker in ("<<<<<<< ", "||||||| ", "=======", ">>>>>>> ")):
        raise RuntimeError(f"unresolved conflict marker remains in {path}")

    if path == "codex-rs/build-info/src/build_info_tests.rs":
        resolved = resolved.replace(
            "use crate::BuildInfo;\nuse crate::codexdd_compact_identity_for_commit;\nuse crate::codexdd_version;\nuse crate::build_id;\n",
            "use crate::BuildInfo;\nuse crate::build_id;\nuse crate::codexdd_compact_identity_for_commit;\nuse crate::codexdd_version;\n",
        )
        resolved = resolved.replace(
            'assert_eq!(codexdd_version(), "0.1.0");',
            'assert_eq!(codexdd_version(), "0.2.0");',
        )
        resolved = resolved.replace('"0.1.0 (0123456789ab)"', '"0.2.0 (0123456789ab)"')
        resolved = resolved.replace(
            '"0.1.0 (0123456789ab-dirty)"',
            '"0.2.0 (0123456789ab-dirty)"',
        )
        resolved = resolved.replace('"0.1.0 (dev)"', '"0.2.0 (dev)"')

    file_path.write_text(resolved, encoding="utf-8")


def main() -> None:
    paths = [
        "codex-rs/build-info/src/build_info_tests.rs",
        "codex-rs/core/src/thread_manager.rs",
        "codex-rs/core/src/tools/context.rs",
        "codex-rs/tui/src/app.rs",
        "codex-rs/tui/src/app/startup.rs",
        "codex-rs/tui/src/app/test_support.rs",
        "codex-rs/tui/src/app/tests.rs",
        "codex-rs/tui/src/app/tests/startup.rs",
        "codex-rs/tui/src/chatwidget/input_flow.rs",
        "codex-rs/tui/src/slash_command.rs",
    ]
    for path in paths:
        resolve_file(path)


if __name__ == "__main__":
    main()
