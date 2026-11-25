//! UTC instants shared by the protocol and the thread persistence layers.

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::time::SystemTime;
use time::Duration;
use time::OffsetDateTime;
use time::PrimitiveDateTime;
use time::UtcDateTime;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(UtcDateTime);

#[derive(Debug, thiserror::Error)]
#[error("invalid RFC 3339 timestamp")]
pub struct InvalidTimestamp;

#[derive(Clone, Copy)]
pub enum SecondsFormat {
    Secs,
    Millis,
    Micros,
}

impl Timestamp {
    pub const UNIX_EPOCH: Self = Self(UtcDateTime::UNIX_EPOCH);

    pub fn now() -> Self {
        Self(UtcDateTime::now())
    }

    pub fn from_timestamp(seconds: i64, nanos: u32) -> Option<Self> {
        let seconds = UtcDateTime::from_unix_timestamp(seconds).ok()?;
        seconds.replace_nanosecond(nanos).ok().map(Self)
    }

    pub fn from_timestamp_millis(millis: i64) -> Option<Self> {
        UtcDateTime::from_unix_timestamp_nanos(i128::from(millis) * 1_000_000)
            .ok()
            .map(Self)
    }

    pub fn timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.timestamp() * 1_000 + i64::from(self.0.millisecond())
    }

    pub fn timestamp_nanos_opt(&self) -> Option<i64> {
        i64::try_from(self.0.unix_timestamp_nanos()).ok()
    }

    pub fn nanosecond(self) -> u32 {
        self.0.nanosecond()
    }

    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        self.0.checked_add(duration).map(Self)
    }

    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        self.0.checked_sub(duration).map(Self)
    }

    pub fn parse_from_rfc3339(value: &str) -> Result<Self, InvalidTimestamp> {
        if !matches!(value.as_bytes().get(10), Some(b'T' | b't' | b' ')) {
            return Err(InvalidTimestamp);
        }
        OffsetDateTime::parse(value, &Rfc3339)
            .ok()
            .and_then(OffsetDateTime::checked_to_utc)
            .map(Self)
            .ok_or(InvalidTimestamp)
    }

    pub fn parse_rollout_filename_timestamp(value: &str) -> Option<Self> {
        const FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
            format_description!("[year]-[month]-[day]T[hour]-[minute]-[second]");
        PrimitiveDateTime::parse(value, FORMAT)
            .ok()
            .map(|date| Self(date.assume_utc().to_utc()))
    }

    pub fn to_rollout_filename_timestamp(self) -> String {
        let date = self.0;
        format!(
            "{}T{:02}-{:02}-{:02}",
            date.date(),
            date.hour(),
            date.minute(),
            date.second()
        )
    }

    pub fn to_rfc3339(self) -> String {
        self.rfc3339(self.fractional_digits(), false).to_string()
    }

    pub fn to_rfc3339_opts(self, format: SecondsFormat, use_z: bool) -> String {
        let digits = match format {
            SecondsFormat::Secs => 0,
            SecondsFormat::Millis => 3,
            SecondsFormat::Micros => 6,
        };
        self.rfc3339(digits, use_z).to_string()
    }

    fn fractional_digits(self) -> usize {
        let nanos = self.nanosecond();
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

    fn rfc3339(self, digits: usize, use_z: bool) -> impl fmt::Display {
        struct DisplayTimestamp(Timestamp, usize, bool);

        impl fmt::Display for DisplayTimestamp {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let Self(timestamp, digits, use_z) = *self;
                let date = timestamp.0;
                write!(
                    f,
                    "{}T{:02}:{:02}:{:02}",
                    date.date(),
                    date.hour(),
                    date.minute(),
                    date.second()
                )?;
                if digits != 0 {
                    let fraction = timestamp.nanosecond() / 10u32.pow(9 - digits as u32);
                    write!(f, ".{fraction:0digits$}")?;
                }
                f.write_str(if use_z { "Z" } else { "+00:00" })
            }
        }

        DisplayTimestamp(self, digits, use_z)
    }

    pub(crate) fn local(self) -> Option<jiff::Zoned> {
        jiff::Timestamp::from_nanosecond(self.0.unix_timestamp_nanos())
            .ok()
            .map(|timestamp| timestamp.to_zoned(jiff::tz::TimeZone::system()))
    }
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        Self(time.into())
    }
}

impl From<Timestamp> for SystemTime {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0.to_offset(time::UtcOffset::UTC).into()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rfc3339(self.fractional_digits(), true).fmt(formatter)
    }
}

impl From<OffsetDateTime> for Timestamp {
    fn from(time: OffsetDateTime) -> Self {
        Self(time.to_utc())
    }
}

impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, duration: Duration) -> Self {
        Self(self.0 + duration)
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self {
        Self(self.0 - duration)
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, other: Self) -> Duration {
        self.0 - other.0
    }
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.rfc3339(self.fractional_digits(), true))
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = std::borrow::Cow::<str>::deserialize(deserializer)?;
        Self::parse_from_rfc3339(&value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_epoch_fraction_rounds_down_at_both_sqlite_precisions() {
        let timestamp =
            Timestamp::parse_from_rfc3339("1969-12-31T21:59:59.999999999-02:00").unwrap();

        assert_eq!(timestamp.timestamp(), -1);
        assert_eq!(timestamp.timestamp_millis(), -1);
        let restored = Timestamp::from_timestamp_millis(-1).unwrap();
        assert_eq!(restored.to_rfc3339(), "1969-12-31T23:59:59.999+00:00");
    }

    #[test]
    fn serialized_timestamps_preserve_utc_and_fraction_spelling() {
        let timestamp: Timestamp =
            serde_json::from_str(r#""2026-11-25T15:00:00.5-02:00""#).unwrap();
        assert_eq!(
            serde_json::to_string(&timestamp).unwrap(),
            r#""2026-11-25T17:00:00.500Z""#
        );
        assert_eq!(
            timestamp.to_rfc3339_opts(SecondsFormat::Micros, true),
            "2026-11-25T17:00:00.500000Z"
        );
        let legacy = Timestamp::parse_rollout_filename_timestamp("2026-11-25T17-00-00").unwrap();
        assert_eq!(legacy.to_rfc3339(), "2026-11-25T17:00:00+00:00");
    }
}
