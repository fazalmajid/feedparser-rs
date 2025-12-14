//! feedparser-rs-core: High-performance RSS/Atom/JSON Feed parser
//!
//! This crate provides a pure Rust implementation of feed parsing with
//! compatibility for Python's feedparser library.
//!
//! # Examples
//!
//! ```
//! use feedparser_rs_core::parse;
//!
//! let xml = r#"
//!     <?xml version="1.0"?>
//!     <rss version="2.0">
//!         <channel>
//!             <title>Example Feed</title>
//!         </channel>
//!     </rss>
//! "#;
//!
//! // Parsing will be fully implemented in Phase 2
//! let feed = parse(xml.as_bytes()).unwrap();
//! assert!(feed.bozo == false);
//! ```
//!
//! # Features
//!
//! - Parse RSS 0.9x, 1.0, 2.0
//! - Parse Atom 0.3, 1.0
//! - Parse JSON Feed 1.0, 1.1
//! - Tolerant parsing with bozo flag
//! - Multi-format date parsing
//! - HTML sanitization
//! - Encoding detection
//!
//! # Architecture
//!
//! The library provides core data structures like [`ParsedFeed`], [`Entry`], and [`FeedMeta`]
//! for representing parsed feed data. The main entry point is the [`parse`] function which
//! automatically detects feed format and returns parsed results.

mod compat;
mod error;
mod limits;
mod parser;
mod types;
mod util;

pub use error::{FeedError, Result};
pub use limits::{LimitError, ParserLimits};
pub use parser::{detect_format, parse};
pub use types::{
    Content, Enclosure, Entry, FeedMeta, FeedVersion, Generator, Image, Link, ParsedFeed, Person,
    Source, Tag, TextConstruct, TextType,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let xml = r#"
            <?xml version="1.0"?>
            <rss version="2.0">
                <channel>
                    <title>Test</title>
                </channel>
            </rss>
        "#;

        let result = parse(xml.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parsed_feed_new() {
        let feed = ParsedFeed::new();
        assert_eq!(feed.encoding, "utf-8");
        assert!(!feed.bozo);
        assert_eq!(feed.version, FeedVersion::Unknown);
    }

    #[test]
    fn test_feed_version_display() {
        assert_eq!(FeedVersion::Rss20.to_string(), "rss20");
        assert_eq!(FeedVersion::Atom10.to_string(), "atom10");
    }
}
