//! Text processing utilities
//!
//! This module provides functions for text manipulation,
//! such as trimming, normalizing whitespace, and encoding conversion.

/// Efficient bytes to string conversion - zero-copy for valid UTF-8
///
/// Uses `std::str::from_utf8()` for zero-copy conversion when the input
/// is valid UTF-8, falling back to lossy conversion otherwise.
///
/// # Examples
///
/// ```
/// use feedparser_rs::util::text::bytes_to_string;
///
/// let valid_utf8 = b"Hello, world!";
/// assert_eq!(bytes_to_string(valid_utf8), "Hello, world!");
///
/// let invalid_utf8 = &[0xFF, 0xFE, 0xFD];
/// let result = bytes_to_string(invalid_utf8);
/// assert!(!result.is_empty()); // Lossy conversion succeeded
/// ```
#[inline]
pub fn bytes_to_string(value: &[u8]) -> String {
    std::str::from_utf8(value).map_or_else(
        |_| String::from_utf8_lossy(value).into_owned(),
        std::string::ToString::to_string,
    )
}
