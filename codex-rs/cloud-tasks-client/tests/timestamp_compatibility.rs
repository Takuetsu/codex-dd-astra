use codex_cloud_tasks_client::DiffSummary;
use codex_cloud_tasks_client::TaskId;
use codex_cloud_tasks_client::TaskStatus;
use codex_cloud_tasks_client::TaskSummary;
use codex_cloud_tasks_client::format_log_timestamp;

#[test]
fn task_timestamp_keeps_existing_wire_and_log_spelling() {
    for (stored, expected) in [
        ("2026-11-25T17:00:00Z", "2026-11-25T17:00:00Z"),
        ("2026-11-25T15:00:00.5-02:00", "2026-11-25T17:00:00.500Z"),
        ("2026-11-25T17:00:00.12345Z", "2026-11-25T17:00:00.123450Z"),
        (
            "1969-12-31T23:59:59.999999999Z",
            "1969-12-31T23:59:59.999999999Z",
        ),
    ] {
        let mut raw = serde_json::to_value(TaskSummary {
            id: TaskId("task_1".to_string()),
            title: "Test task".to_string(),
            status: TaskStatus::Ready,
            updated_at: jiff::Timestamp::UNIX_EPOCH,
            environment_id: None,
            environment_label: None,
            summary: DiffSummary::default(),
            is_review: false,
            attempt_total: None,
        })
        .unwrap();
        raw["updated_at"] = stored.into();

        let task: TaskSummary = serde_json::from_value(raw).unwrap();
        assert_eq!(
            format_log_timestamp(task.updated_at),
            expected.replace('Z', "+00:00")
        );
        assert_eq!(serde_json::to_value(task).unwrap()["updated_at"], expected);
    }
}
