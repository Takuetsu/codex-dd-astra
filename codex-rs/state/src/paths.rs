use codex_protocol::timestamp::Timestamp;
use std::path::Path;

pub(crate) async fn file_modified_time_utc(path: &Path) -> Option<Timestamp> {
    let modified = tokio::fs::metadata(path).await.ok()?.modified().ok()?;
    let updated_at: Timestamp = modified.into();
    Some(updated_at)
}
