//! Parser configuration options
//!
//! This module provides configuration options for customizing feed parsing behavior.
//! Options control features like URL resolution, HTML sanitization, and resource limits.

use crate::limits::ParserLimits;

/// Parser configuration options
///
/// Controls various aspects of feed parsing behavior including URL resolution,
/// HTML sanitization, and resource limits for `DoS` protection.
///
/// # Examples
///
/// ```
/// use feedparser_rs::ParseOptions;
///
/// // Default options (recommended for most use cases)
/// let options = ParseOptions::default();
/// assert!(options.resolve_relative_uris);
/// assert!(options.sanitize_html);
///
/// // Custom options for restricted environment
/// let custom = ParseOptions {
///     resolve_relative_uris: true,
///     sanitize_html: false, // Trust feed content
///     limits: feedparser_rs::ParserLimits::strict(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Whether to resolve relative URLs to absolute URLs
    ///
    /// When `true`, relative URLs in links, images, and other resources
    /// are converted to absolute URLs using the feed's base URL.
    ///
    /// Default: `true`
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs::ParseOptions;
    ///
    /// let mut options = ParseOptions::default();
    /// options.resolve_relative_uris = false; // Keep relative URLs
    /// ```
    pub resolve_relative_uris: bool,

    /// Whether to sanitize HTML content in feed entries
    ///
    /// When `true`, HTML content in titles, summaries, and content blocks
    /// is sanitized to remove potentially dangerous elements and attributes
    /// (scripts, iframes, etc.) while preserving safe formatting.
    ///
    /// Default: `true`
    ///
    /// # Security
    ///
    /// Disabling HTML sanitization is **not recommended** unless you fully
    /// trust the feed source and have other security measures in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs::ParseOptions;
    ///
    /// let mut options = ParseOptions::default();
    /// options.sanitize_html = false; // Disable for trusted feeds
    /// ```
    pub sanitize_html: bool,

    /// Parser limits for `DoS` protection
    ///
    /// Controls maximum allowed sizes for collections, text fields,
    /// and overall feed size to prevent resource exhaustion attacks.
    ///
    /// Default: `ParserLimits::default()`
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs::{ParseOptions, ParserLimits};
    ///
    /// let options = ParseOptions {
    ///     limits: ParserLimits::strict(), // Use stricter limits
    ///     ..Default::default()
    /// };
    /// ```
    pub limits: ParserLimits,
}

impl Default for ParseOptions {
    /// Creates default parse options
    ///
    /// Default configuration:
    /// - `resolve_relative_uris`: `true`
    /// - `sanitize_html`: `true`
    /// - `limits`: `ParserLimits::default()`
    ///
    /// These defaults are suitable for most use cases and provide
    /// good security and compatibility.
    fn default() -> Self {
        Self {
            resolve_relative_uris: true,
            sanitize_html: true,
            limits: ParserLimits::default(),
        }
    }
}

impl ParseOptions {
    /// Creates permissive parse options
    ///
    /// Suitable for trusted feeds where you want maximum compatibility
    /// and performance:
    /// - `resolve_relative_uris`: `true`
    /// - `sanitize_html`: `false`
    /// - `limits`: `ParserLimits::permissive()`
    ///
    /// # Security Warning
    ///
    /// Use only with trusted feed sources!
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs::ParseOptions;
    ///
    /// let options = ParseOptions::permissive();
    /// assert!(!options.sanitize_html);
    /// ```
    #[must_use]
    pub const fn permissive() -> Self {
        Self {
            resolve_relative_uris: true,
            sanitize_html: false,
            limits: ParserLimits::permissive(),
        }
    }

    /// Creates strict parse options
    ///
    /// Suitable for untrusted feeds in resource-constrained environments:
    /// - `resolve_relative_uris`: `false` (preserve original URLs)
    /// - `sanitize_html`: `true` (remove dangerous content)
    /// - `limits`: `ParserLimits::strict()` (tight resource limits)
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs::ParseOptions;
    ///
    /// let options = ParseOptions::strict();
    /// assert!(options.sanitize_html);
    /// assert!(!options.resolve_relative_uris);
    /// ```
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            resolve_relative_uris: false,
            sanitize_html: true,
            limits: ParserLimits::strict(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = ParseOptions::default();
        assert!(options.resolve_relative_uris);
        assert!(options.sanitize_html);
        assert_eq!(options.limits.max_entries, 10_000);
    }

    #[test]
    fn test_permissive_options() {
        let options = ParseOptions::permissive();
        assert!(options.resolve_relative_uris);
        assert!(!options.sanitize_html);
        assert_eq!(options.limits.max_entries, 100_000);
    }

    #[test]
    fn test_strict_options() {
        let options = ParseOptions::strict();
        assert!(!options.resolve_relative_uris);
        assert!(options.sanitize_html);
        assert_eq!(options.limits.max_entries, 1_000);
    }

    #[test]
    fn test_custom_options() {
        let options = ParseOptions {
            resolve_relative_uris: false,
            sanitize_html: false,
            limits: ParserLimits::permissive(),
        };
        assert!(!options.resolve_relative_uris);
        assert!(!options.sanitize_html);
        assert_eq!(options.limits.max_entries, 100_000);
    }

    #[test]
    fn test_options_clone() {
        let options1 = ParseOptions::default();
        let options2 = options1.clone();
        assert_eq!(
            options1.resolve_relative_uris,
            options2.resolve_relative_uris
        );
        assert_eq!(options1.sanitize_html, options2.sanitize_html);
    }

    #[test]
    fn test_options_debug() {
        let options = ParseOptions::default();
        let debug_str = format!("{options:?}");
        assert!(debug_str.contains("ParseOptions"));
        assert!(debug_str.contains("resolve_relative_uris"));
        assert!(debug_str.contains("sanitize_html"));
    }
}
