use super::ContextualUserFragment;
use codex_protocol::models::ContentItemKind;
use codex_protocol::timestamp::Timestamp;

pub(crate) struct CurrentTimeReminder {
    current_time: Timestamp,
}

impl CurrentTimeReminder {
    pub(crate) fn new(current_time: Timestamp) -> Self {
        Self { current_time }
    }

    pub(crate) fn formatted_time(&self) -> String {
        let utc = self
            .current_time
            .to_rfc3339_opts(codex_protocol::timestamp::SecondsFormat::Secs, true);
        format!("{} UTC", utc.trim_end_matches('Z').replace('T', " "))
    }
}

impl ContextualUserFragment for CurrentTimeReminder {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("current_time.reminder".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<current_time_reminder>", "</current_time_reminder>")
    }

    fn body(&self) -> String {
        format!("It is {}.", self.formatted_time())
    }
}

pub(crate) struct CurrentTimeUnavailable;

impl CurrentTimeUnavailable {
    pub(crate) const MESSAGE: &str = "failed to read current time";
}

impl ContextualUserFragment for CurrentTimeUnavailable {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("current_time.unavailable".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<current_time_unavailable>", "</current_time_unavailable>")
    }

    fn body(&self) -> String {
        Self::MESSAGE.to_string()
    }
}
