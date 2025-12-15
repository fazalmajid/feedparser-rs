use feedparser_rs_core::ParserLimits as CoreParserLimits;
use pyo3::prelude::*;

/// Resource limits for feed parsing
///
/// Protects against DoS attacks from malicious feeds that attempt to
/// exhaust memory or CPU resources.
///
/// Examples:
///     >>> import feedparser_rs
///     >>> limits = feedparser_rs.ParserLimits(
///     ...     max_feed_size_bytes=50_000_000,  # 50 MB
///     ...     max_entries=5_000
///     ... )
///     >>> d = feedparser_rs.parse_with_limits(feed_data, limits)
#[pyclass(name = "ParserLimits", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyParserLimits {
    max_feed_size_bytes: usize,
    max_entries: usize,
    max_links_per_feed: usize,
    max_links_per_entry: usize,
    max_authors: usize,
    max_contributors: usize,
    max_tags: usize,
    max_content_blocks: usize,
    max_enclosures: usize,
}

#[pymethods]
impl PyParserLimits {
    /// Create parser limits with custom values
    ///
    /// Args:
    ///     max_feed_size_bytes: Maximum feed size in bytes (default: 100 MB)
    ///     max_entries: Maximum number of entries (default: 10,000)
    ///     max_links_per_feed: Maximum links at feed level (default: 100)
    ///     max_links_per_entry: Maximum links per entry (default: 50)
    ///     max_authors: Maximum authors per feed/entry (default: 20)
    ///     max_contributors: Maximum contributors per feed/entry (default: 20)
    ///     max_tags: Maximum tags per feed/entry (default: 100)
    ///     max_content_blocks: Maximum content blocks per entry (default: 10)
    ///     max_enclosures: Maximum enclosures per entry (default: 20)
    ///
    /// Returns:
    ///     ParserLimits: New limits object
    #[new]
    #[pyo3(signature = (
        max_feed_size_bytes=100_000_000,
        max_entries=10_000,
        max_links_per_feed=100,
        max_links_per_entry=50,
        max_authors=20,
        max_contributors=20,
        max_tags=100,
        max_content_blocks=10,
        max_enclosures=20
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        max_feed_size_bytes: usize,
        max_entries: usize,
        max_links_per_feed: usize,
        max_links_per_entry: usize,
        max_authors: usize,
        max_contributors: usize,
        max_tags: usize,
        max_content_blocks: usize,
        max_enclosures: usize,
    ) -> Self {
        Self {
            max_feed_size_bytes,
            max_entries,
            max_links_per_feed,
            max_links_per_entry,
            max_authors,
            max_contributors,
            max_tags,
            max_content_blocks,
            max_enclosures,
        }
    }

    #[getter]
    fn max_feed_size_bytes(&self) -> usize {
        self.max_feed_size_bytes
    }

    #[getter]
    fn max_entries(&self) -> usize {
        self.max_entries
    }

    #[getter]
    fn max_links_per_feed(&self) -> usize {
        self.max_links_per_feed
    }

    #[getter]
    fn max_links_per_entry(&self) -> usize {
        self.max_links_per_entry
    }

    #[getter]
    fn max_authors(&self) -> usize {
        self.max_authors
    }

    #[getter]
    fn max_contributors(&self) -> usize {
        self.max_contributors
    }

    #[getter]
    fn max_tags(&self) -> usize {
        self.max_tags
    }

    #[getter]
    fn max_content_blocks(&self) -> usize {
        self.max_content_blocks
    }

    #[getter]
    fn max_enclosures(&self) -> usize {
        self.max_enclosures
    }

    fn __repr__(&self) -> String {
        format!(
            "ParserLimits(max_feed_size_bytes={}, max_entries={})",
            self.max_feed_size_bytes, self.max_entries
        )
    }
}

impl PyParserLimits {
    /// Convert to core ParserLimits
    pub(crate) fn to_core_limits(&self) -> CoreParserLimits {
        CoreParserLimits {
            max_feed_size_bytes: self.max_feed_size_bytes,
            max_entries: self.max_entries,
            max_links_per_feed: self.max_links_per_feed,
            max_links_per_entry: self.max_links_per_entry,
            max_authors: self.max_authors,
            max_contributors: self.max_contributors,
            max_tags: self.max_tags,
            max_content_blocks: self.max_content_blocks,
            max_enclosures: self.max_enclosures,
            max_namespaces: 100,     // Use default
            max_nesting_depth: 100,  // Use default
            max_text_length: 10 * 1024 * 1024, // 10 MB
            max_attribute_length: 64 * 1024,   // 64 KB
        }
    }
}
