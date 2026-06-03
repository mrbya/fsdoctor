use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::error::{Error, Result};

/// Formats a timestamp for storage.
pub fn format_timestamp(timestamp: OffsetDateTime) -> Result<String> {
    timestamp
        .format(&Rfc3339)
        .map_err(|_error| Error::InvalidProjectDatabase)
}

/// Parses a timestamp from storage.
pub fn parse_timestamp(timestamp: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(timestamp, &Rfc3339).map_err(|_error| Error::InvalidProjectDatabase)
}

/// Converts u64 to i64.
pub fn u64_to_i64(value: u64) -> Result<i64> {
    i64::try_from(value).map_err(|_error| Error::NumericOverflow)
}

/// Optional u64 to i64 conversion.
pub fn optional_u64_to_i64(value: Option<u64>) -> Result<Option<i64>> {
    value
        .map(|inner| i64::try_from(inner).map_err(|_error| Error::NumericOverflow))
        .transpose()
}

/// Optional i128 to i64 conversion.
pub fn optional_i128_to_i64(value: Option<i128>) -> Result<Option<i64>> {
    value
        .map(|inner| i64::try_from(inner).map_err(|_error| Error::NumericOverflow))
        .transpose()
}
