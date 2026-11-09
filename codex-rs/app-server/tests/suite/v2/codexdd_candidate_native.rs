//! CodexDD 0.4.6: isolated actual app-server candidate and cold-resume acceptance.
//! This test never runs git in the repository checkout, the user's worktree, or production.

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use app_test_support::MockResponsesConfig;
use app_test_support::TestAppServer;
use app_test_support::create_mock_responses_server_repeating_assistant;
use codex_app_server_protocol::ThreadHistoryMode;
use codex_app_server_protocol::ThreadReadParams;
use codex_app_server_protocol::ThreadReadResponse;
use codex_app_server_protocol::ThreadResumeParams;
use codex_app_server_protocol::ThreadResumeResponse;
use codex_app_server_protocol::ThreadSettingsUpdatedNotification;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::TurnStartParams;
use codex_app_server_protocol::TurnStartResponse;
use codex_app_server_protocol::UserInput;
use codex_protocol::ThreadId;
use codex_protocol::protocol::EventMsg;
use codex_rollout::RolloutItem;
use codex_rollout::RolloutRecorder;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

const CANDIDATE_TIMEOUT: Duration = Duration::from_secs(45);

fn git(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git").arg("-C").arg(root).args(args).output()?;
    ensure!(
        output.status.success(),
        "isolated git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn commit_fixture(root: &Path, body: &str) -> Result<String> {
    std::fs::write(root.join("candidate.txt"), body)?;
    git(root, &["add", "candidate.txt"])?;
    git(root, &["commit", "-m", body])?;
    git(root, &["rev-parse", "HEAD"])
}

async fn authorize_candidate(
    server: &mut TestAppServer,
    thread_id: &str,
    old_sha: &str,
    successor_sha: &str,
) -> Result<()> {
    let text = format!(
        "[codexdd_candidate]\nprevious_sha = \"{old_sha}\"\ncandidate_sha = \"{successor_sha}\"\n\nAuthorize this exact clean descendant for the existing source packet."
    );
    let request_id = server
        .send_turn_start_request(TurnStartParams {
            thread_id: thread_id.to_string(),
            input: vec![UserInput::Text {
                text,
                text_elements: Vec::new(),
            }],
            ..Default::default()
        })
        .await?;
    let _: TurnStartResponse =
        timeout(CANDIDATE_TIMEOUT, server.read_response(request_id)).await??;
    let notice = timeout(
        CANDIDATE_TIMEOUT,
        server.read_stream_until_matching_notification(
            "Core-owned successor source notification",
            |n| {
                n.method == "thread/settings/updated"
                    && n.params
                        .as_ref()
                        .and_then(|p| {
                            serde_json::from_value::<ThreadSettingsUpdatedNotification>(p.clone())
                                .ok()
                        })
                        .is_some_and(|updated| {
                            updated.thread_id == thread_id
                                && updated
                                    .codexdd_first_worker
                                    .and_then(|worker| worker.source_sha)
                                    .as_deref()
                                    == Some(successor_sha)
                        })
            },
        ),
    )
    .await??;
    let updated: ThreadSettingsUpdatedNotification =
        serde_json::from_value(notice.params.context("missing notification body")?)?;
    let worker = updated
        .codexdd_first_worker
        .context("Core source notification is missing its Worker snapshot")?;
    ensure!(worker.source_sha.as_deref() == Some(successor_sha));
    ensure!(worker.role == "implementation");
    ensure!(worker.authorized_scope == "isolated acceptance packet");
    timeout(
        CANDIDATE_TIMEOUT,
        server.read_stream_until_notification_message("turn/completed"),
    )
    .await??;
    Ok(())
}

#[tokio::test]
async fn codexdd_046_3b2n_real_app_server_candidate_and_cold_resume() -> Result<()> {
    let backend = create_mock_responses_server_repeating_assistant("complete").await;
    let home = TempDir::new()?;
    MockResponsesConfig::new(&backend.uri()).write(home.path())?;
    let checkout = TempDir::new()?;
    let root = checkout.path();
    git(root, &["init"])?;
    git(root, &["config", "user.name", "CodexDD isolated test"])?;
    git(root, &["config", "user.email", "codexdd@example.invalid"])?;
    let first_sha = commit_fixture(root, "initial source")?;

    let mut server = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;
    let started = server
        .start_thread(ThreadStartParams {
            model: Some("mock-model".to_string()),
            cwd: Some(root.display().to_string()),
            history_mode: Some(ThreadHistoryMode::Legacy),
            ..Default::default()
        })
        .await?;
    let thread_id = started.thread.id;
    let first_message = format!(
        "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"isolated acceptance packet\"\ntask_kind = \"source_change\"\nsource_root = '{}'\nsource_sha = \"{}\"\n\nWork only in this isolated source packet.",
        root.display(),
        first_sha
    );
    timeout(
        CANDIDATE_TIMEOUT,
        server.start_turn_and_wait_for_completion(TurnStartParams {
            thread_id: thread_id.clone(),
            input: vec![UserInput::Text {
                text: first_message,
                text_elements: Vec::new(),
            }],
            ..Default::default()
        }),
    )
    .await??;

    let second_sha = commit_fixture(root, "approved successor")?;
    authorize_candidate(&mut server, &thread_id, &first_sha, &second_sha).await?;

    let read_id = server
        .send_thread_read_request(ThreadReadParams {
            thread_id: thread_id.clone(),
            include_turns: false,
        })
        .await?;
    let read: ThreadReadResponse =
        timeout(CANDIDATE_TIMEOUT, server.read_response(read_id)).await??;
    let path = read.thread.path.context("native thread rollout must exist")?;
    let thread_uuid = ThreadId::from_string(&thread_id)?;
    let (items, _, _) = RolloutRecorder::load_rollout_items(&path).await?;
    let durable_sha = items.iter().rev().find_map(|item| match item {
        RolloutItem::EventMsg(EventMsg::ThreadSettingsApplied(event))
            if event.thread_id == Some(thread_uuid) =>
        {
            event
                .thread_settings
                .codexdd_first_worker
                .as_ref()
                .and_then(|worker| worker.source_sha.clone())
        }
        _ => None,
    });
    ensure!(
        durable_sha.as_deref() == Some(second_sha.as_str()),
        "the successor source was not flushed to this thread's native rollout"
    );
    timeout(CANDIDATE_TIMEOUT, server.shutdown_gracefully()).await??;

    // A new process must restore the last exact source SHA rather than
    // silently adopting HEAD, replaying the original first message or
    // treating the candidate as a fresh Worker assignment.
    let mut restarted = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;
    let resume_id = restarted
        .send_thread_resume_request(ThreadResumeParams {
            thread_id: thread_id.clone(),
            ..Default::default()
        })
        .await?;
    let resumed: ThreadResumeResponse =
        timeout(CANDIDATE_TIMEOUT, restarted.read_response(resume_id)).await??;
    ensure!(resumed.thread.id == thread_id);
    let third_sha = commit_fixture(root, "authorized after cold resume")?;
    authorize_candidate(&mut restarted, &thread_id, &second_sha, &third_sha).await?;
    let (items, _, _) = RolloutRecorder::load_rollout_items(&path).await?;
    let persisted = items.iter().rev().find_map(|item| match item {
        RolloutItem::EventMsg(EventMsg::ThreadSettingsApplied(event))
            if event.thread_id == Some(thread_uuid) =>
        {
            event
                .thread_settings
                .codexdd_first_worker
                .as_ref()
                .and_then(|worker| worker.source_sha.clone())
        }
        _ => None,
    });
    ensure!(persisted.as_deref() == Some(third_sha.as_str()));
    timeout(CANDIDATE_TIMEOUT, restarted.shutdown_gracefully()).await??;
    Ok(())
}
