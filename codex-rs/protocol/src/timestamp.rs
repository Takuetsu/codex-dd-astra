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
use time::Date;
use time::Duration;
use time::Month;
use time::OffsetDateTime;
use time::PrimitiveDateTime;
use time::Time;
use time::UtcDateTime;
use time::UtcOffset;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    second: UtcDateTime,
    // The second field has no fractional part. During a leap second, this field
    // holds 1_000_000_000 plus the fractional nanoseconds, as in the old codec.
    nanos: u32,
}

const NANOS_PER_SECOND: u32 = 1_000_000_000;
const MIN_YEAR: i32 = -262_143;
const MAX_YEAR: i32 = 262_142;

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
    pub const UNIX_EPOCH: Self = Self {
        second: UtcDateTime::UNIX_EPOCH,
        nanos: 0,
    };

    pub fn now() -> Self {
        Self::from_utc(UtcDateTime::now())
    }

    pub fn from_timestamp(seconds: i64, nanos: u32) -> Option<Self> {
        let seconds = UtcDateTime::from_unix_timestamp(seconds).ok()?;
        if !(MIN_YEAR..=MAX_YEAR).contains(&seconds.year())
            || nanos >= 2 * NANOS_PER_SECOND
            || (nanos >= NANOS_PER_SECOND && seconds.second() != 59)
        {
            return None;
        }
        Some(Self {
            second: seconds,
            nanos,
        })
    }

    pub fn from_timestamp_millis(millis: i64) -> Option<Self> {
        UtcDateTime::from_unix_timestamp_nanos(i128::from(millis) * 1_000_000)
            .ok()
            .and_then(Self::checked_from_utc)
    }

    pub fn timestamp(&self) -> i64 {
        self.second.unix_timestamp()
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.timestamp() * 1_000 + i64::from(self.nanos / 1_000_000)
    }

    pub fn timestamp_micros(&self) -> i64 {
        self.timestamp() * 1_000_000 + i64::from(self.nanos / 1_000)
    }

    pub fn timestamp_nanos_opt(&self) -> Option<i64> {
        i64::try_from(self.unix_timestamp_nanos()).ok()
    }

    pub fn nanosecond(self) -> u32 {
        self.nanos
    }

    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        if !self.is_leap_second() {
            return self
                .regular_time()
                .checked_add(duration)
                .and_then(Self::checked_from_utc);
        }
        // A fractional adjustment can stay within this second or the leap
        // second that follows it. Whole seconds cross that two-second interval.
        let nanos = i64::from(self.nanos) + i64::from(duration.subsec_nanoseconds());
        if duration.whole_seconds() == 0 && nanos < i64::from(2 * NANOS_PER_SECOND) {
            return Some(Self {
                nanos: nanos as u32,
                ..self
            });
        }
        let delta = if duration.whole_seconds() < 0 {
            duration.checked_add(Duration::SECOND)?
        } else {
            duration
        };
        self.regular_time()
            .checked_add(delta)
            .and_then(Self::checked_from_utc)
    }

    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        if self.is_leap_second() {
            self.checked_add(Duration::ZERO.checked_sub(duration)?)
        } else {
            self.regular_time()
                .checked_sub(duration)
                .and_then(Self::checked_from_utc)
        }
    }

    pub fn parse_from_rfc3339(value: &str) -> Result<Self, InvalidTimestamp> {
        if !matches!(value.as_bytes().get(10), Some(b'T' | b't' | b' ')) {
            return Err(InvalidTimestamp);
        }
        let normalized;
        let value = if let Some((prefix, offset)) = value.rsplit_once('−')
            && offset.len() == 5
        {
            normalized = format!("{prefix}-{offset}");
            &normalized
        } else {
            value
        };
        Self::parse_second(value, |value| {
            OffsetDateTime::parse(value, &Rfc3339)
                .ok()
                .and_then(OffsetDateTime::checked_to_utc)
        })
        .ok_or(InvalidTimestamp)
    }

    pub fn parse_rollout_filename_timestamp(value: &str) -> Option<Self> {
        const FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
            format_description!("[year]-[month]-[day]T[hour]-[minute]-[second]");
        Self::parse_second(value, |value| {
            PrimitiveDateTime::parse(value, FORMAT)
                .ok()
                .map(|date| date.assume_utc().to_utc())
        })
    }

    pub fn to_rollout_filename_timestamp(self) -> String {
        let date = self.second;
        format!(
            "{}T{:02}-{:02}-{:02}",
            date.date(),
            date.hour(),
            date.minute(),
            self.display_second()
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
        let nanos = self.fractional_nanos();
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

    fn from_utc(date: UtcDateTime) -> Self {
        let Some(timestamp) = Self::checked_from_utc(date) else {
            panic!("timestamp outside the supported calendar range");
        };
        timestamp
    }

    fn checked_from_utc(date: UtcDateTime) -> Option<Self> {
        Self::from_timestamp(date.unix_timestamp(), date.nanosecond())
    }

    fn parse_second(value: &str, parse: impl FnOnce(&str) -> Option<UtcDateTime>) -> Option<Self> {
        if value.as_bytes().get(17..19) != Some(b"60") {
            return parse(value).and_then(Self::checked_from_utc);
        }
        // Parse the same civil second and offset through time, retaining the
        // leap marker before its RFC 3339 parser can round it away.
        let mut preceding = value.to_owned();
        preceding.replace_range(17..19, "59");
        let date = parse(&preceding)?;
        Self::from_timestamp(date.unix_timestamp(), date.nanosecond() + NANOS_PER_SECOND)
    }

    fn is_leap_second(self) -> bool {
        self.nanos >= NANOS_PER_SECOND
    }

    fn display_second(self) -> u8 {
        self.second.second() + u8::from(self.is_leap_second())
    }

    fn fractional_nanos(self) -> u32 {
        self.nanos % NANOS_PER_SECOND
    }

    fn regular_time(self) -> UtcDateTime {
        self.second + Duration::nanoseconds(i64::from(self.fractional_nanos()))
    }

    fn unix_timestamp_nanos(self) -> i128 {
        self.second.unix_timestamp_nanos() + i128::from(self.nanos)
    }

    fn rfc3339(self, digits: usize, use_z: bool) -> impl fmt::Display {
        struct DisplayTimestamp(Timestamp, usize, bool);

        impl fmt::Display for DisplayTimestamp {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let Self(timestamp, digits, use_z) = *self;
                let date = timestamp.second;
                write!(
                    f,
                    "{}T{:02}:{:02}:{:02}",
                    date.date(),
                    date.hour(),
                    date.minute(),
                    timestamp.display_second()
                )?;
                if digits != 0 {
                    let fraction = timestamp.fractional_nanos() / 10u32.pow(9 - digits as u32);
                    write!(f, ".{fraction:0digits$}")?;
                }
                f.write_str(if use_z { "Z" } else { "+00:00" })
            }
        }

        DisplayTimestamp(self, digits, use_z)
    }

    pub fn local(self) -> Option<jiff::Zoned> {
        self.in_timezone(jiff::tz::TimeZone::system())
    }

    pub fn utc(self) -> Option<jiff::Zoned> {
        self.in_timezone(jiff::tz::TimeZone::UTC)
    }

    fn in_timezone(self, timezone: jiff::tz::TimeZone) -> Option<jiff::Zoned> {
        jiff::Timestamp::from_nanosecond(self.regular_time().unix_timestamp_nanos())
            .ok()
            .map(|timestamp| timestamp.to_zoned(timezone))
    }
}

impl std::str::FromStr for Timestamp {
    type Err = InvalidTimestamp;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_legacy_timestamp(value).ok_or(InvalidTimestamp)
    }
}

// The previous Serde codec used Chrono's relaxed FromStr grammar. Keep this
// separate from the explicitly RFC 3339 parsing used for protocol fields.
fn parse_legacy_timestamp(value: &str) -> Option<Timestamp> {
    let value = value.trim_start();
    let (year, rest) = match value.as_bytes().first()? {
        b'+' | b'-' => {
            let (magnitude, rest) = take_digits(&value[1..], usize::MAX)?;
            let magnitude: i32 = magnitude.parse().ok()?;
            let year = if value.starts_with('-') {
                -magnitude
            } else {
                magnitude
            };
            (year, rest)
        }
        _ => {
            let (year, rest) = take_digits(value, 4)?;
            (year.parse().ok()?, rest)
        }
    };
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return None;
    }
    let (month, rest) = take_component(rest.trim_start().strip_prefix('-')?)?;
    let (day, rest) = take_component(rest.trim_start().strip_prefix('-')?)?;
    let (hour, rest) = take_component(rest.strip_prefix(['T', 't', ' '])?)?;
    let (minute, rest) = take_component(rest.trim_start().strip_prefix(':')?)?;
    let (second, rest) = take_component(rest.trim_start().strip_prefix(':')?)?;
    let (nanos, rest) = match rest.strip_prefix('.') {
        Some(fraction) => {
            let (digits, rest) = take_digits(fraction, usize::MAX)?;
            let precision = digits.len().min(9);
            let nanos = digits[..precision].parse::<u32>().ok()? * 10u32.pow(9 - precision as u32);
            (nanos, rest)
        }
        None => (0, rest),
    };
    let offset = parse_legacy_offset(rest.trim())?;
    let date = Date::from_calendar_date(year, Month::try_from(month).ok()?, day).ok()?;
    let leap = second == 60;
    let time = Time::from_hms(hour, minute, if leap { 59 } else { second }).ok()?;
    let utc = date
        .with_time(time)
        .assume_offset(offset)
        .checked_to_utc()?;
    Timestamp::from_timestamp(
        utc.unix_timestamp(),
        nanos + u32::from(leap) * NANOS_PER_SECOND,
    )
}

fn take_digits(value: &str, limit: usize) -> Option<(&str, &str)> {
    let len = value
        .bytes()
        .take(limit)
        .take_while(u8::is_ascii_digit)
        .count();
    (len != 0).then(|| value.split_at(len))
}

fn take_component(value: &str) -> Option<(u8, &str)> {
    let (digits, rest) = take_digits(value.trim_start(), 2)?;
    Some((digits.parse().ok()?, rest))
}

fn parse_legacy_offset(value: &str) -> Option<UtcOffset> {
    if value.eq_ignore_ascii_case("z") || value.eq_ignore_ascii_case("utc") {
        return Some(UtcOffset::UTC);
    }
    let (sign, rest) = if let Some(rest) = value.strip_prefix('+') {
        (1, rest)
    } else {
        (-1, value.strip_prefix(['-', '−'])?)
    };
    let (hours, rest) = take_digits(rest, 2)?;
    let (minutes, rest) = take_digits(
        rest.trim_start_matches(|c: char| c == ':' || c.is_whitespace()),
        2,
    )?;
    if hours.len() != 2 || minutes.len() != 2 || !rest.is_empty() {
        return None;
    }
    let hours: i8 = hours.parse().ok()?;
    let minutes: i8 = minutes.parse().ok()?;
    if hours >= 24 {
        return None;
    }
    UtcOffset::from_hms(sign * hours, sign * minutes, 0).ok()
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        Self::from_utc(time.into())
    }
}

impl From<Timestamp> for SystemTime {
    fn from(timestamp: Timestamp) -> Self {
        let base: Self = timestamp
            .regular_time()
            .to_offset(time::UtcOffset::UTC)
            .into();
        base + std::time::Duration::from_secs(u64::from(timestamp.is_leap_second()))
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rfc3339(self.fractional_digits(), true).fmt(formatter)
    }
}

impl From<OffsetDateTime> for Timestamp {
    fn from(time: OffsetDateTime) -> Self {
        Self::from_utc(time.to_utc())
    }
}

impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, duration: Duration) -> Self {
        let Some(timestamp) = self.checked_add(duration) else {
            panic!("timestamp overflow");
        };
        timestamp
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
        let Some(timestamp) = self.checked_sub(duration) else {
            panic!("timestamp overflow");
        };
        timestamp
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, other: Self) -> Duration {
        let wall_seconds = self.timestamp().rem_euclid(86_400);
        let other_wall_seconds = other.timestamp().rem_euclid(86_400);
        let extra_seconds = i64::from(other.is_leap_second() && wall_seconds > other_wall_seconds)
            - i64::from(self.is_leap_second() && wall_seconds < other_wall_seconds);
        (self.second - other.second)
            + Duration::seconds(extra_seconds)
            + Duration::nanoseconds(i64::from(self.nanos) - i64::from(other.nanos))
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
        value.parse().map_err(serde::de::Error::custom)
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
        assert_eq!(timestamp.timestamp_micros(), -1);
        let restored = Timestamp::from_timestamp_millis(-1).unwrap();
        assert_eq!(restored.to_rfc3339(), "1969-12-31T23:59:59.999+00:00");
    }

    #[test]
    fn legacy_calendar_bounds_keep_epoch_microseconds_representable() {
        let min = time::macros::utc_datetime!(-262143-01-01 00:00:00);
        let max = time::macros::utc_datetime!(+262142-12-31 23:59:59);
        let first = Timestamp::from_timestamp(min.unix_timestamp(), 0).unwrap();
        let last = Timestamp::from_timestamp(max.unix_timestamp(), 999_999_999).unwrap();

        assert_eq!(first.timestamp_micros(), min.unix_timestamp() * 1_000_000);
        assert_eq!(
            last.timestamp_micros(),
            max.unix_timestamp() * 1_000_000 + 999_999
        );
        assert!(first.checked_sub(Duration::NANOSECOND).is_none());
        assert!(last.checked_add(Duration::NANOSECOND).is_none());
        assert!(Timestamp::from_timestamp(min.unix_timestamp() - 1, 0).is_none());
        assert!(Timestamp::from_timestamp(max.unix_timestamp() + 1, 0).is_none());
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

    #[test]
    fn timestamp_deserialization_preserves_legacy_spellings() {
        // Expected UTC encodings captured from the pinned Chrono 0.4.43 codec.
        for (input, expected) in [
            ("2026-11-25T17:00:00+0000", "2026-11-25T17:00:00Z"),
            ("2026-11-25T17:00:00 UTC", "2026-11-25T17:00:00Z"),
            ("2026-11-25T17:00:00utc", "2026-11-25T17:00:00Z"),
            (
                " 2026- 1-2t3: 4:5.1234567899 −02 : 30 ",
                "2026-01-02T05:34:05.123456789Z",
            ),
            ("2017-1-1 5: 29:60.5+0530", "2016-12-31T23:59:60.500Z"),
            ("+10000-1-1T0:0:0Z", "+10000-01-01T00:00:00Z"),
            ("-00001-01-01T00:00:00Z", "-0001-01-01T00:00:00Z"),
            ("-262143-01-01T00:00:00Z", "-262143-01-01T00:00:00Z"),
            (
                "+262142-12-31T23:59:59.999999999Z",
                "+262142-12-31T23:59:59.999999999Z",
            ),
        ] {
            let timestamp: Timestamp = serde_json::from_value(serde_json::json!(input)).unwrap();
            let encoded = serde_json::to_value(timestamp).unwrap();
            assert_eq!(encoded, expected, "{input}");
            assert_eq!(
                serde_json::from_value::<Timestamp>(encoded).unwrap(),
                timestamp
            );
        }
    }

    #[test]
    fn legacy_timestamp_parser_rejects_invalid_spellings() {
        for input in [
            "10000-01-01T00:00:00Z",
            "+262143-01-01T00:00:00Z",
            "2026-02-29T17:00:00Z",
            "2026-11-25T17:00:61Z",
            "2026-11-25T17:00:00.Z",
            "2026-11-25T17:00:00+01",
            "2026-11-25T17:00:00+24:00",
            "2026-11-25T17:00:00+01:60",
            "2026-11-25T17:00:00UTCextra",
        ] {
            assert!(input.parse::<Timestamp>().is_err(), "{input}");
        }
        assert!(Timestamp::parse_from_rfc3339("2026-11-25T17:00:00UTC").is_err());
    }

    #[test]
    fn rfc3339_preserves_the_legacy_unicode_offset_minus() {
        let timestamp = Timestamp::parse_from_rfc3339("2026-11-25T14:30:00−02:30").unwrap();
        assert_eq!(timestamp.to_string(), "2026-11-25T17:00:00Z");
        assert!(Timestamp::parse_from_rfc3339("2026−11-25T17:00:00Z").is_err());
    }

    #[test]
    fn leap_second_fixtures_match_the_chrono_codec() {
        // Expected values captured from the pinned Chrono 0.4.43 codec.
        for (input, utc, seconds, millis, nanos) in [
            (
                "2016-12-31T23:59:60.500Z",
                "2016-12-31T23:59:60.500Z",
                1_483_228_799,
                1_483_228_800_500,
                1_500_000_000,
            ),
            (
                "2017-01-01T05:29:60.123456789+05:30",
                "2016-12-31T23:59:60.123456789Z",
                1_483_228_799,
                1_483_228_800_123,
                1_123_456_789,
            ),
            (
                "1969-12-31T23:59:60.250Z",
                "1969-12-31T23:59:60.250Z",
                -1,
                250,
                1_250_000_000,
            ),
        ] {
            let timestamp: Timestamp = serde_json::from_value(serde_json::json!(input)).unwrap();

            assert_eq!(serde_json::to_value(timestamp).unwrap(), utc);
            assert_eq!(timestamp.timestamp(), seconds);
            assert_eq!(timestamp.timestamp_millis(), millis);
            assert_eq!(timestamp.nanosecond(), nanos);
            assert_eq!(Timestamp::from_timestamp(seconds, nanos), Some(timestamp));
            assert_eq!(
                timestamp.timestamp_nanos_opt(),
                Some(seconds * i64::from(NANOS_PER_SECOND) + i64::from(nanos))
            );
        }
    }

    #[test]
    fn leap_second_retains_rollout_spelling_and_timestamp_order() {
        let filename = "2016-12-31T23-59-60";
        let start = Timestamp::parse_rollout_filename_timestamp(filename).unwrap();
        let previous = Timestamp::parse_from_rfc3339("2016-12-31T23:59:59.999Z").unwrap();
        let next = Timestamp::parse_from_rfc3339("2017-01-01T00:00:00Z").unwrap();
        let halfway = start + Duration::milliseconds(500);

        assert_eq!(start.to_rollout_filename_timestamp(), filename);
        assert_eq!(start.to_rfc3339(), "2016-12-31T23:59:60+00:00");
        assert_eq!(halfway.to_string(), "2016-12-31T23:59:60.500Z");
        assert!(previous < start && start < halfway && halfway < next);
        assert_eq!(halfway + Duration::milliseconds(500), next);
        assert_eq!(halfway - Duration::milliseconds(501), previous);
        assert_eq!(
            SystemTime::from(halfway),
            SystemTime::from(next) + std::time::Duration::from_millis(500)
        );
        assert!(Timestamp::from_timestamp(0, NANOS_PER_SECOND).is_none());
    }
}
