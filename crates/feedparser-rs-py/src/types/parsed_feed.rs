use feedparser_rs_core::ParsedFeed as CoreParsedFeed;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::entry::PyEntry;
use super::feed_meta::PyFeedMeta;

/// Parsed feed result (analogous to feedparser.FeedParserDict)
///
/// This class provides access to feed metadata, entries, and parsing status.
/// It mimics the behavior of Python's feedparser library for compatibility.
///
/// Examples:
///     >>> import feedparser_rs
///     >>> d = feedparser_rs.parse('<rss>...</rss>')
///     >>> print(d.feed.title)
///     >>> print(d.version)
///     >>> print(d.bozo)
///     >>> for entry in d.entries:
///     ...     print(entry.title)
#[pyclass(name = "FeedParserDict", module = "feedparser_rs")]
pub struct PyParsedFeed {
    feed: Py<PyFeedMeta>,
    entries: Vec<Py<PyEntry>>,
    bozo: bool,
    bozo_exception: Option<String>,
    encoding: String,
    version: String,
    namespaces: Py<PyDict>,
}

impl PyParsedFeed {
    /// Convert from core ParsedFeed with Python context
    pub fn from_core(py: Python<'_>, core: CoreParsedFeed) -> PyResult<Self> {
        let feed = Py::new(py, PyFeedMeta::from_core(core.feed))?;

        let entries: PyResult<Vec<_>> = core
            .entries
            .into_iter()
            .map(|e| Py::new(py, PyEntry::from_core(e)))
            .collect();

        let namespaces = PyDict::new(py);
        for (prefix, uri) in core.namespaces {
            namespaces.set_item(prefix, uri)?;
        }

        Ok(Self {
            feed,
            entries: entries?,
            bozo: core.bozo,
            bozo_exception: core.bozo_exception,
            encoding: core.encoding,
            version: core.version.to_string(),
            namespaces: namespaces.unbind(),
        })
    }
}

#[pymethods]
impl PyParsedFeed {
    /// Feed metadata
    ///
    /// Returns:
    ///     FeedMeta: Feed-level metadata (title, link, etc.)
    #[getter]
    fn feed(&self, py: Python<'_>) -> Py<PyFeedMeta> {
        self.feed.clone_ref(py)
    }

    /// List of feed entries/items
    ///
    /// Returns:
    ///     list[Entry]: All entries in the feed
    #[getter]
    fn entries(&self, py: Python<'_>) -> Vec<Py<PyEntry>> {
        self.entries.iter().map(|e| e.clone_ref(py)).collect()
    }

    /// True if parsing encountered errors
    ///
    /// The bozo flag indicates that the feed is "bozo" - malformed but still
    /// parseable. This matches feedparser's tolerant parsing behavior.
    ///
    /// Returns:
    ///     bool: True if errors were encountered
    #[getter]
    fn bozo(&self) -> bool {
        self.bozo
    }

    /// Description of parsing error (if bozo is True)
    ///
    /// Returns:
    ///     str | None: Error message if bozo is True, None otherwise
    #[getter]
    fn bozo_exception(&self) -> Option<&str> {
        self.bozo_exception.as_deref()
    }

    /// Detected or declared encoding (e.g., "utf-8")
    ///
    /// Returns:
    ///     str: Character encoding used
    #[getter]
    fn encoding(&self) -> &str {
        &self.encoding
    }

    /// Detected feed format version (e.g., "rss20", "atom10")
    ///
    /// Returns:
    ///     str: Feed version string
    #[getter]
    fn version(&self) -> &str {
        &self.version
    }

    /// XML namespaces (prefix -> URI mapping)
    ///
    /// Returns:
    ///     dict[str, str]: Namespace prefix to URI mapping
    #[getter]
    fn namespaces(&self, py: Python<'_>) -> Py<PyDict> {
        self.namespaces.clone_ref(py)
    }

    fn __repr__(&self) -> String {
        format!(
            "FeedParserDict(version='{}', bozo={}, entries={})",
            self.version,
            self.bozo,
            self.entries.len()
        )
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}
