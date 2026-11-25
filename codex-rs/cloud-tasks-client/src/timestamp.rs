use jiff::Timestamp;
use jiff::tz::Offset;
use serde::Serializer;

// Keep the existing RFC 3339 spelling with 0, 3, 6, or 9 fractional digits.
fn fractional_precision(timestamp: Timestamp) -> usize {
    let nanos = timestamp.subsec_nanosecond();
    if nanos == 0 {
        0
    } else if nanos % 1_000_000 == 0 {
        3
    } else if nanos % 1_000 == 0 {
        6
    } else {
        9
    }
}

pub(crate) fn serialize_utc<S>(timestamp: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let precision = fractional_precision(*timestamp);
    serializer.collect_str(&format_args!("{timestamp:.precision$}"))
}

pub fn format_log_timestamp(timestamp: Timestamp) -> String {
    let precision = fractional_precision(timestamp);
    let utc = timestamp.display_with_offset(Offset::UTC);
    format!("{utc:.precision$}")
}
