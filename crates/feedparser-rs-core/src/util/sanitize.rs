//! HTML sanitization utilities
//!
//! This module provides functions for sanitizing HTML content to prevent XSS attacks
//! while preserving safe formatting.

use ammonia::Builder;
use std::collections::HashSet;

/// Sanitize HTML content, removing dangerous tags and attributes
///
/// This function uses ammonia to clean HTML content, allowing only safe tags
/// and attributes. It's designed to match feedparser's sanitization behavior.
///
/// # Arguments
///
/// * `input` - HTML string to sanitize
///
/// # Returns
///
/// Sanitized HTML string with dangerous content removed
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::sanitize::sanitize_html;
///
/// let unsafe_html = r#"<p>Hello</p><script>alert('XSS')</script>"#;
/// let safe_html = sanitize_html(unsafe_html);
/// assert_eq!(safe_html, "<p>Hello</p>");
/// ```
pub fn sanitize_html(input: &str) -> String {
    let safe_tags: HashSet<&str> = [
        // Text formatting
        "a", "abbr", "acronym", "b", "cite", "code", "em", "i", "kbd", "mark", "s", "samp",
        "small", "strike", "strong", "sub", "sup", "u", "var",
        // Structural
        "br", "div", "hr", "p", "span",
        // Headings
        "h1", "h2", "h3", "h4", "h5", "h6",
        // Lists
        "dd", "dl", "dt", "li", "ol", "ul",
        // Tables
        "caption", "table", "tbody", "td", "tfoot", "th", "thead", "tr",
        // Quotes
        "blockquote", "q",
        // Pre-formatted
        "pre",
        // Media
        "img",
    ]
    .iter()
    .copied()
    .collect();

    let safe_attrs: HashSet<&str> = ["alt", "cite", "class", "href", "id", "src", "title"]
        .iter()
        .copied()
        .collect();

    Builder::default()
        .tags(safe_tags)
        .generic_attributes(safe_attrs)
        .link_rel(Some("nofollow noopener noreferrer"))
        .url_schemes(["http", "https", "mailto"].iter().copied().collect())
        .clean(input)
        .to_string()
}

/// Decode HTML entities to Unicode characters
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::sanitize::decode_entities;
///
/// assert_eq!(decode_entities("&lt;p&gt;Hello&lt;/p&gt;"), "<p>Hello</p>");
/// assert_eq!(decode_entities("&amp;amp;"), "&amp;");
/// ```
pub fn decode_entities(input: &str) -> String {
    html_escape::decode_html_entities(input).to_string()
}

/// Sanitize and decode HTML content
///
/// This combines sanitization and entity decoding in the correct order:
/// 1. Decode entities first
/// 2. Then sanitize HTML
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::sanitize::sanitize_and_decode;
///
/// let input = "&lt;p&gt;Safe&lt;/p&gt;&lt;script&gt;alert('XSS')&lt;/script&gt;";
/// let output = sanitize_and_decode(input);
/// assert_eq!(output, "<p>Safe</p>");
/// ```
pub fn sanitize_and_decode(input: &str) -> String {
    let decoded = decode_entities(input);
    sanitize_html(&decoded)
}

/// Strip all HTML tags, leaving only text content
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::sanitize::strip_tags;
///
/// assert_eq!(strip_tags("<p>Hello <b>world</b></p>"), "Hello world");
/// ```
pub fn strip_tags(input: &str) -> String {
    Builder::default()
        .tags(HashSet::new())
        .clean(input)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_removes_script() {
        let html = r#"<p>Hello</p><script>alert('XSS')</script>"#;
        let clean = sanitize_html(html);
        assert!(!clean.contains("script"));
        assert!(clean.contains("Hello"));
    }

    #[test]
    fn test_sanitize_allows_safe_tags() {
        let html = r#"<p>Hello <b>world</b> <a href="http://example.com">link</a></p>"#;
        let clean = sanitize_html(html);
        assert!(clean.contains("<p>"));
        assert!(clean.contains("<b>"));
        assert!(clean.contains("<a"));
    }

    #[test]
    fn test_sanitize_removes_onclick() {
        let html = r#"<a href="/" onclick="alert('XSS')">Click</a>"#;
        let clean = sanitize_html(html);
        assert!(!clean.contains("onclick"));
        assert!(clean.contains("href"));
    }

    #[test]
    fn test_decode_entities() {
        assert_eq!(decode_entities("&lt;p&gt;"), "<p>");
        assert_eq!(decode_entities("&amp;"), "&");
        assert_eq!(decode_entities("&quot;"), "\"");
        assert_eq!(decode_entities("&#39;"), "'");
    }

    #[test]
    fn test_decode_numeric_entities() {
        assert_eq!(decode_entities("&#60;"), "<");
        assert_eq!(decode_entities("&#x3C;"), "<");
    }

    #[test]
    fn test_sanitize_and_decode() {
        let input = "&lt;p&gt;Safe&lt;/p&gt;&lt;script&gt;Bad&lt;/script&gt;";
        let output = sanitize_and_decode(input);
        assert!(output.contains("<p>Safe</p>"));
        assert!(!output.contains("script"));
    }

    #[test]
    fn test_strip_tags() {
        let html = "<p>Hello <b>world</b></p>";
        assert_eq!(strip_tags(html), "Hello world");
    }

    #[test]
    fn test_xss_img_onerror() {
        let html = r#"<img src="x" onerror="alert('XSS')">"#;
        let clean = sanitize_html(html);
        assert!(!clean.contains("onerror"));
    }

    #[test]
    fn test_xss_javascript_url() {
        let html = r#"<a href="javascript:alert('XSS')">Click</a>"#;
        let clean = sanitize_html(html);
        assert!(!clean.contains("javascript:"));
    }

    #[test]
    fn test_xss_iframe() {
        let html = r#"<iframe src="http://evil.com"></iframe>"#;
        let clean = sanitize_html(html);
        assert!(!clean.contains("iframe"));
    }

    #[test]
    fn test_xss_data_url() {
        let html = r#"<a href="data:text/html,<script>alert('XSS')</script>">Click</a>"#;
        let clean = sanitize_html(html);
        assert!(!clean.contains("data:"));
    }

    #[test]
    fn test_sanitize_empty_string() {
        assert_eq!(sanitize_html(""), "");
    }

    #[test]
    fn test_sanitize_plain_text() {
        let text = "Plain text with no tags";
        assert_eq!(sanitize_html(text), text);
    }

    #[test]
    fn test_decode_entities_no_entities() {
        let text = "No entities here";
        assert_eq!(decode_entities(text), text);
    }

    #[test]
    fn test_strip_tags_nested() {
        let html = "<div><p>Hello <span><b>world</b></span></p></div>";
        assert_eq!(strip_tags(html), "Hello world");
    }

    #[test]
    fn test_sanitize_link_rel_attribute() {
        let html = r#"<a href="http://example.com">Link</a>"#;
        let clean = sanitize_html(html);
        assert!(clean.contains("nofollow"));
        assert!(clean.contains("noopener"));
        assert!(clean.contains("noreferrer"));
    }
}
