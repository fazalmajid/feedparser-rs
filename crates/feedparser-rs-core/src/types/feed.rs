use super::{
    common::{Generator, Image, Link, Person, Tag, TextConstruct},
    entry::Entry,
    version::FeedVersion,
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Feed metadata
#[derive(Debug, Clone, Default)]
pub struct FeedMeta {
    /// Feed title
    pub title: Option<String>,
    /// Detailed title with metadata
    pub title_detail: Option<TextConstruct>,
    /// Primary feed link
    pub link: Option<String>,
    /// All links associated with this feed
    pub links: Vec<Link>,
    /// Feed subtitle/description
    pub subtitle: Option<String>,
    /// Detailed subtitle with metadata
    pub subtitle_detail: Option<TextConstruct>,
    /// Last update date
    pub updated: Option<DateTime<Utc>>,
    /// Primary author name
    pub author: Option<String>,
    /// Detailed author information
    pub author_detail: Option<Person>,
    /// All authors
    pub authors: Vec<Person>,
    /// Contributors
    pub contributors: Vec<Person>,
    /// Publisher name
    pub publisher: Option<String>,
    /// Detailed publisher information
    pub publisher_detail: Option<Person>,
    /// Feed language (e.g., "en-us")
    pub language: Option<String>,
    /// Copyright/rights statement
    pub rights: Option<String>,
    /// Detailed rights with metadata
    pub rights_detail: Option<TextConstruct>,
    /// Generator name
    pub generator: Option<String>,
    /// Detailed generator information
    pub generator_detail: Option<Generator>,
    /// Feed image
    pub image: Option<Image>,
    /// Icon URL (small image)
    pub icon: Option<String>,
    /// Logo URL (larger image)
    pub logo: Option<String>,
    /// Feed-level tags/categories
    pub tags: Vec<Tag>,
    /// Unique feed identifier
    pub id: Option<String>,
    /// Time-to-live (update frequency hint) in minutes
    pub ttl: Option<u32>,
}

/// Parsed feed result
///
/// This is the main result type returned by the parser, analogous to
/// Python feedparser's `FeedParserDict`.
#[derive(Debug, Clone, Default)]
pub struct ParsedFeed {
    /// Feed metadata
    pub feed: FeedMeta,
    /// Feed entries/items
    pub entries: Vec<Entry>,
    /// True if parsing encountered errors
    pub bozo: bool,
    /// Description of parsing error (if bozo is true)
    pub bozo_exception: Option<String>,
    /// Detected or declared encoding
    pub encoding: String,
    /// Detected feed format version
    pub version: FeedVersion,
    /// XML namespaces (prefix -> URI)
    pub namespaces: HashMap<String, String>,
}

impl ParsedFeed {
    /// Creates a new `ParsedFeed` with default UTF-8 encoding
    #[must_use]
    pub fn new() -> Self {
        Self {
            encoding: String::from("utf-8"),
            ..Default::default()
        }
    }

    /// Creates a `ParsedFeed` with pre-allocated capacity for entries
    ///
    /// This method pre-allocates space for the expected number of entries,
    /// reducing memory allocations during parsing.
    ///
    /// # Arguments
    ///
    /// * `entry_count` - Expected number of entries in the feed
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs_core::ParsedFeed;
    ///
    /// let feed = ParsedFeed::with_capacity(50);
    /// assert_eq!(feed.encoding, "utf-8");
    /// ```
    #[must_use]
    pub fn with_capacity(entry_count: usize) -> Self {
        Self {
            entries: Vec::with_capacity(entry_count),
            namespaces: HashMap::with_capacity(8), // Typical feeds have 3-8 namespaces
            encoding: String::from("utf-8"),
            ..Default::default()
        }
    }
}

impl FeedMeta {
    /// Creates `FeedMeta` with capacity hints for typical RSS 2.0 feeds
    ///
    /// Pre-allocates collections based on common RSS 2.0 field usage:
    /// - 1-2 links (channel link, self link)
    /// - 1 author (managingEditor)
    /// - 0-3 tags (categories)
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs_core::FeedMeta;
    ///
    /// let meta = FeedMeta::with_rss_capacity();
    /// ```
    #[must_use]
    pub fn with_rss_capacity() -> Self {
        Self {
            links: Vec::with_capacity(2),
            authors: Vec::with_capacity(1),
            contributors: Vec::with_capacity(0),
            tags: Vec::with_capacity(3),
            ..Default::default()
        }
    }

    /// Creates `FeedMeta` with capacity hints for typical Atom 1.0 feeds
    ///
    /// Pre-allocates collections based on common Atom 1.0 field usage:
    /// - 3-5 links (alternate, self, related, etc.)
    /// - 1-2 authors
    /// - 1 contributor
    /// - 3-5 tags (categories)
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs_core::FeedMeta;
    ///
    /// let meta = FeedMeta::with_atom_capacity();
    /// ```
    #[must_use]
    pub fn with_atom_capacity() -> Self {
        Self {
            links: Vec::with_capacity(4),
            authors: Vec::with_capacity(2),
            contributors: Vec::with_capacity(1),
            tags: Vec::with_capacity(5),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_meta_default() {
        let meta = FeedMeta::default();
        assert!(meta.title.is_none());
        assert!(meta.links.is_empty());
        assert!(meta.authors.is_empty());
    }

    #[test]
    fn test_parsed_feed_default() {
        let feed = ParsedFeed::default();
        assert!(!feed.bozo);
        assert!(feed.bozo_exception.is_none());
        assert_eq!(feed.version, FeedVersion::Unknown);
        assert!(feed.entries.is_empty());
    }

    #[test]
    fn test_parsed_feed_new() {
        let feed = ParsedFeed::new();
        assert_eq!(feed.encoding, "utf-8");
        assert!(!feed.bozo);
    }

    #[test]
    fn test_parsed_feed_clone() {
        let feed = ParsedFeed {
            version: FeedVersion::Rss20,
            bozo: true,
            ..ParsedFeed::new()
        };

        assert_eq!(feed.version, FeedVersion::Rss20);
        assert!(feed.bozo);
    }
}
