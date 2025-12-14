//! Multi-format date parsing for RSS and Atom feeds

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

/// Date format strings to try, in priority order
///
/// Order matters: more specific formats first, then fallbacks
const DATE_FORMATS: &[&str] = &[
    // ISO 8601 / RFC 3339 variants (Atom)
    "%Y-%m-%dT%H:%M:%S%.f%:z", // 2024-12-14T10:30:45.123+00:00
    "%Y-%m-%dT%H:%M:%S%:z",    // 2024-12-14T10:30:45+00:00
    "%Y-%m-%dT%H:%M:%S%.fZ",   // 2024-12-14T10:30:45.123Z
    "%Y-%m-%dT%H:%M:%SZ",      // 2024-12-14T10:30:45Z
    "%Y-%m-%dT%H:%M:%S",       // 2024-12-14T10:30:45 (no timezone)
    "%Y-%m-%d %H:%M:%S",       // 2024-12-14 10:30:45
    "%Y-%m-%d",                // 2024-12-14
    // US date formats
    "%B %d, %Y %H:%M:%S", // December 14, 2024 10:30:45
    "%B %d, %Y",          // December 14, 2024
    "%b %d, %Y",          // Dec 14, 2024
    "%m/%d/%Y %H:%M:%S",  // 12/14/2024 10:30:45
    "%m/%d/%Y",           // 12/14/2024
    // EU date formats
    "%d.%m.%Y %H:%M:%S", // 14.12.2024 10:30:45
    "%d.%m.%Y",          // 14.12.2024
    "%d/%m/%Y",          // 14/12/2024
];

/// Parse date from string, trying multiple formats
///
/// This function attempts to parse dates in the following order:
/// 1. RFC 3339 (Atom standard: 2024-12-14T10:30:00Z)
/// 2. RFC 2822 (RSS standard: Sat, 14 Dec 2024 10:30:00 +0000)
/// 3. Common format strings (ISO 8601 variants, US/EU formats)
///
/// # Arguments
///
/// * `input` - Date string to parse
///
/// # Returns
///
/// * `Some(DateTime<Utc>)` - Successfully parsed date
/// * `None` - Could not parse date
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::date::parse_date;
///
/// // RFC 3339 (Atom)
/// assert!(parse_date("2024-12-14T10:30:00Z").is_some());
///
/// // RFC 2822 (RSS)
/// assert!(parse_date("Sat, 14 Dec 2024 10:30:00 +0000").is_some());
///
/// // ISO 8601 date-only
/// assert!(parse_date("2024-12-14").is_some());
///
/// // Invalid date
/// assert!(parse_date("not a date").is_none());
/// ```
#[must_use]
pub fn parse_date(input: &str) -> Option<DateTime<Utc>> {
    let input = input.trim();

    if input.is_empty() {
        return None;
    }

    // Try RFC 3339 first (most common in Atom)
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try RFC 2822 (RSS pubDate format)
    if let Ok(dt) = DateTime::parse_from_rfc2822(input) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try all format strings
    for fmt in DATE_FORMATS {
        // Try parsing with time component
        if let Ok(dt) = NaiveDateTime::parse_from_str(input, fmt) {
            return Some(dt.and_utc());
        }

        // Try parsing date-only, assume midnight UTC
        if let Ok(date) = NaiveDate::parse_from_str(input, fmt) {
            return date.and_hms_opt(0, 0, 0).map(|dt| dt.and_utc());
        }
    }

    // Could not parse
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_rfc3339_with_timezone() {
        let dt = parse_date("2024-12-14T10:30:00+00:00");
        assert!(dt.is_some());
        let dt = dt.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 14);
        assert_eq!(dt.hour(), 10);
        assert_eq!(dt.minute(), 30);
    }

    #[test]
    fn test_rfc3339_z_suffix() {
        let dt = parse_date("2024-12-14T10:30:00Z");
        assert!(dt.is_some());
        assert_eq!(dt.unwrap().year(), 2024);
    }

    #[test]
    fn test_rfc3339_with_milliseconds() {
        let dt = parse_date("2024-12-14T10:30:00.123Z");
        assert!(dt.is_some());
    }

    #[test]
    fn test_rfc2822_format() {
        let dt = parse_date("Sat, 14 Dec 2024 10:30:00 +0000");
        assert!(dt.is_some());
        let dt = dt.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 12);
    }

    #[test]
    fn test_rfc2822_gmt() {
        let dt = parse_date("Sat, 14 Dec 2024 10:30:00 GMT");
        assert!(dt.is_some());
    }

    #[test]
    fn test_iso8601_date_only() {
        let dt = parse_date("2024-12-14");
        assert!(dt.is_some());
        let dt = dt.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 14);
        assert_eq!(dt.hour(), 0); // Midnight
    }

    #[test]
    fn test_us_format_long_month() {
        let dt = parse_date("December 14, 2024");
        assert!(dt.is_some());
    }

    #[test]
    fn test_us_format_short_month() {
        let dt = parse_date("Dec 14, 2024");
        assert!(dt.is_some());
    }

    #[test]
    fn test_invalid_date() {
        let dt = parse_date("not a date");
        assert!(dt.is_none());
    }

    #[test]
    fn test_empty_string() {
        let dt = parse_date("");
        assert!(dt.is_none());
    }

    #[test]
    fn test_whitespace_only() {
        let dt = parse_date("   ");
        assert!(dt.is_none());
    }

    #[test]
    fn test_partial_date() {
        // Should fail gracefully
        let dt = parse_date("2024-12");
        assert!(dt.is_none());
    }

    #[test]
    fn test_us_date_slash_format() {
        let dt = parse_date("12/14/2024");
        assert!(dt.is_some());
    }

    #[test]
    fn test_eu_date_dot_format() {
        let dt = parse_date("14.12.2024");
        assert!(dt.is_some());
    }
}
