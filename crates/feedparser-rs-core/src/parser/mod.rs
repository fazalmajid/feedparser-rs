mod detect;

use crate::{error::Result, types::ParsedFeed};

pub use detect::detect_format;

/// Parse feed from raw bytes
///
/// This is the main entry point for parsing feeds. It automatically detects
/// the feed format (RSS, Atom, JSON) and parses accordingly.
///
/// # Errors
///
/// Returns a `FeedError` if the feed cannot be parsed. However, in most cases,
/// the parser will set the `bozo` flag and return partial results rather than
/// returning an error.
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::parse;
///
/// let xml = r#"
///     <?xml version="1.0"?>
///     <rss version="2.0">
///         <channel>
///             <title>Example Feed</title>
///         </channel>
///     </rss>
/// "#;
///
/// // Parsing will be fully implemented in Phase 2
/// let feed = parse(xml.as_bytes()).unwrap();
/// assert!(feed.bozo == false);
/// ```
pub fn parse(_data: &[u8]) -> Result<ParsedFeed> {
    // TODO: Implement in Phase 2
    // For now, return a basic ParsedFeed
    Ok(ParsedFeed::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_returns_ok() {
        let result = parse(b"test");
        assert!(result.is_ok());
    }
}
