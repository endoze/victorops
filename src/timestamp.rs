//! Serde support for the `Option<OffsetDateTime>` timestamp fields in [`crate::types`].
//!
//! The wire format matches what these fields used with chrono's `DateTime<Utc>`, so requests
//! and re-serialized responses stay byte-identical:
//!
//! - Serializing converts to UTC and writes RFC 3339 with a `Z` suffix. Fractional seconds are
//!   omitted when zero, otherwise written as 3, 6, or 9 digits, whichever is the shortest exact
//!   form (`2024-01-15T10:30:00Z`, `2024-01-15T10:30:00.120Z`).
//! - Deserializing accepts RFC 3339 plus the relaxed forms chrono accepted: a space instead of
//!   `T`, an offset without a colon (`+0530`), and a `UTC` suffix. Any offset is converted to
//!   UTC. `null` becomes `None`.

use serde::{Deserialize, Deserializer, Serializer, de};
use std::borrow::Cow;
use std::fmt::{self, Write};
use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime, UtcOffset};

/// Serializes an optional timestamp; use with `skip_serializing_if = "Option::is_none"`.
pub(crate) fn serialize<S>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match value {
    Some(dt) => serializer.collect_str(&Utc(*dt)),
    None => serializer.serialize_none(),
  }
}

/// Deserializes an optional timestamp; use with `default` so a missing field is `None`.
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
  D: Deserializer<'de>,
{
  let Some(raw) = Option::<Cow<'de, str>>::deserialize(deserializer)? else {
    return Ok(None);
  };
  parse(&raw).map(Some).map_err(de::Error::custom)
}

struct Utc(OffsetDateTime);

impl fmt::Display for Utc {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let dt = self.0.to_offset(UtcOffset::UTC);
    let year = dt.year();
    if (0..=9999).contains(&year) {
      write!(f, "{year:04}")?;
    } else {
      write!(f, "{year:+05}")?;
    }
    write!(
      f,
      "-{:02}-{:02}T{:02}:{:02}:{:02}",
      u8::from(dt.month()),
      dt.day(),
      dt.hour(),
      dt.minute(),
      dt.second()
    )?;
    let nano = dt.nanosecond();
    if nano == 0 {
    } else if nano.is_multiple_of(1_000_000) {
      write!(f, ".{:03}", nano / 1_000_000)?;
    } else if nano.is_multiple_of(1_000) {
      write!(f, ".{:06}", nano / 1_000)?;
    } else {
      write!(f, ".{nano:09}")?;
    }
    f.write_char('Z')
  }
}

fn parse(raw: &str) -> Result<OffsetDateTime, time::error::Parse> {
  OffsetDateTime::parse(&normalize(raw.trim_end()), &Rfc3339).map(|dt| dt.to_offset(UtcOffset::UTC))
}

fn normalize(raw: &str) -> Cow<'_, str> {
  let bytes = raw.as_bytes();
  let space_separator = bytes.get(10) == Some(&b' ');
  let utc_suffix = bytes.len() >= 3 && bytes[bytes.len() - 3..].eq_ignore_ascii_case(b"UTC");
  let colonless_offset = raw.len() >= 5 && {
    let tail = &bytes[bytes.len() - 5..];
    matches!(tail[0], b'+' | b'-') && tail[1..].iter().all(u8::is_ascii_digit)
  };

  if !space_separator && !utc_suffix && !colonless_offset {
    return Cow::Borrowed(raw);
  }

  let mut out = raw.to_owned();
  if space_separator {
    out.replace_range(10..11, "T");
  }
  if utc_suffix {
    out.truncate(out.len() - 3);
    out.truncate(out.trim_end().len());
    out.push('Z');
  } else if colonless_offset {
    out.insert(out.len() - 2, ':');
  }
  Cow::Owned(out)
}

#[cfg(test)]
mod tests {
  use super::*;
  use time::macros::datetime;

  fn parse_ok(raw: &str) -> OffsetDateTime {
    parse(raw).unwrap_or_else(|e| panic!("{raw}: {e}"))
  }

  #[test]
  fn formats_like_chrono_utc() {
    let cases = [
      (datetime!(2024-01-15 10:30:00 UTC), "2024-01-15T10:30:00Z"),
      (
        datetime!(2024-01-15 10:30:00.12 UTC),
        "2024-01-15T10:30:00.120Z",
      ),
      (
        datetime!(2024-01-15 10:30:00.000_5 UTC),
        "2024-01-15T10:30:00.000500Z",
      ),
      (
        datetime!(2024-01-15 10:30:00.000_000_001 UTC),
        "2024-01-15T10:30:00.000000001Z",
      ),
      (datetime!(999-03-04 05:06:07 UTC), "0999-03-04T05:06:07Z"),
      (
        datetime!(2024-01-15 12:30:00 +02:00),
        "2024-01-15T10:30:00Z",
      ),
    ];
    for (dt, expected) in cases {
      assert_eq!(Utc(dt).to_string(), expected);
    }
  }

  #[test]
  fn parses_rfc3339_and_converts_to_utc() {
    assert_eq!(
      parse_ok("2024-01-15T10:30:00Z"),
      datetime!(2024-01-15 10:30:00 UTC)
    );
    assert_eq!(
      parse_ok("2024-01-15T10:30:00.454Z"),
      datetime!(2024-01-15 10:30:00.454 UTC)
    );
    assert_eq!(
      parse_ok("2024-01-15T05:30:00-05:00"),
      datetime!(2024-01-15 10:30:00 UTC)
    );
    assert_eq!(
      parse_ok("2024-01-15T05:30:00-05:00").offset(),
      UtcOffset::UTC
    );
    assert_eq!(
      parse_ok("2024-01-15t10:30:00z"),
      datetime!(2024-01-15 10:30:00 UTC)
    );
  }

  #[test]
  fn parses_relaxed_forms_chrono_accepted() {
    let expected = datetime!(2024-01-15 10:30:00 UTC);
    for raw in [
      "2024-01-15 10:30:00Z",
      "2024-01-15T16:00:00+0530",
      "2024-01-15T10:30:00UTC",
      "2024-01-15 10:30:00 UTC",
      "2024-01-15T10:30:00Z ",
    ] {
      assert_eq!(parse_ok(raw), expected, "{raw}");
    }
  }

  #[test]
  fn rejects_invalid_input() {
    for raw in [
      "",
      "2024-01-15",
      "2024-01-15T10:30:00",
      "not a date",
      "éab",
      "2024-01-15éUTC",
    ] {
      assert!(parse(raw).is_err(), "{raw}");
    }
  }
}
