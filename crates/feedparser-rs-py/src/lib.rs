use pyo3::prelude::*;
use pyo3::types::PyModule;

use feedparser_rs_core as core;

mod error;
mod limits;
mod types;

use error::convert_feed_error;
use limits::PyParserLimits;
use types::PyParsedFeed;

/// feedparser_rs: High-performance RSS/Atom/JSON Feed parser
///
/// Drop-in replacement for Python's feedparser library with 10-100x speedup.
/// Written in Rust with PyO3 bindings.
#[pymodule]
fn _feedparser_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_with_limits, m)?)?;
    m.add_function(wrap_pyfunction!(detect_format, m)?)?;

    // Classes
    m.add_class::<PyParsedFeed>()?;
    m.add_class::<PyParserLimits>()?;

    // Version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}

/// Parse an RSS/Atom/JSON Feed from bytes, string, or file path
///
/// This function provides the same API as Python's feedparser.parse() for
/// drop-in compatibility. It accepts feed content as bytes or string and
/// returns a parsed feed result.
///
/// Args:
///     source: Feed content as bytes, str, or file path (str starting with http:// loads URL)
///
/// Returns:
///     FeedParserDict: Parsed feed with .feed, .entries, .bozo, .version, etc.
///
/// Raises:
///     TypeError: If source is not str or bytes
///     NotImplementedError: If source is an HTTP URL (use requests.get(url).content)
///
/// Examples:
///     >>> import feedparser_rs
///     >>> # From string
///     >>> d = feedparser_rs.parse('<rss version="2.0">...</rss>')
///     >>> print(d.version)
///     'rss20'
///     >>> # From bytes
///     >>> d = feedparser_rs.parse(b'<rss>...</rss>')
///     >>> print(d.feed.title)
///     'Example Feed'
///     >>> # From file
///     >>> with open('feed.xml', 'rb') as f:
///     ...     d = feedparser_rs.parse(f.read())
///     >>> # Access entries
///     >>> for entry in d.entries:
///     ...     print(entry.title)
///     ...     print(entry.published_parsed)  # time.struct_time
#[pyfunction]
#[pyo3(signature = (source, /))]
fn parse(py: Python<'_>, source: &Bound<'_, PyAny>) -> PyResult<PyParsedFeed> {
    parse_with_limits(py, source, None)
}

/// Parse with custom resource limits
///
/// Allows specifying custom limits to protect against DoS attacks from
/// malicious feeds that attempt to exhaust memory or CPU resources.
///
/// Args:
///     source: Feed content as bytes or str
///     limits: Optional ParserLimits object with custom thresholds
///
/// Returns:
///     FeedParserDict: Parsed feed result
///
/// Raises:
///     TypeError: If source is not str or bytes
///     ValueError: If feed exceeds specified limits
///
/// Examples:
///     >>> import feedparser_rs
///     >>> limits = feedparser_rs.ParserLimits(
///     ...     max_feed_size_bytes=50_000_000,  # 50 MB
///     ...     max_entries=5_000
///     ... )
///     >>> d = feedparser_rs.parse_with_limits(feed_data, limits)
#[pyfunction]
#[pyo3(signature = (source, limits=None))]
fn parse_with_limits(
    py: Python<'_>,
    source: &Bound<'_, PyAny>,
    limits: Option<&PyParserLimits>,
) -> PyResult<PyParsedFeed> {
    // Extract bytes from source (str or bytes)
    let bytes: Vec<u8> = if let Ok(s) = source.extract::<String>() {
        // Check if it's a URL (not implemented yet - Phase 6)
        if s.starts_with("http://") || s.starts_with("https://") {
            return Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "URL fetching not implemented yet. Use requests.get(url).content for now.",
            ));
        }
        s.into_bytes()
    } else if let Ok(b) = source.extract::<Vec<u8>>() {
        b
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "source must be str or bytes",
        ));
    };

    // Use provided limits or default
    let parser_limits = limits.map(|l| l.to_core_limits()).unwrap_or_default();

    // Parse
    let parsed = core::parse_with_limits(&bytes, parser_limits).map_err(convert_feed_error)?;

    PyParsedFeed::from_core(py, parsed)
}

/// Detect feed format without full parsing
///
/// Quickly determines the feed format by examining the root element and
/// attributes without parsing the entire feed structure.
///
/// Args:
///     source: Feed content as bytes or str
///
/// Returns:
///     str: Feed version string (e.g., "rss20", "atom10", "json11", "")
///
/// Raises:
///     TypeError: If source is not str or bytes
///
/// Examples:
///     >>> import feedparser_rs
///     >>> version = feedparser_rs.detect_format('<rss version="2.0">...</rss>')
///     >>> print(version)
///     'rss20'
///     >>> version = feedparser_rs.detect_format(b'<feed xmlns="http://www.w3.org/2005/Atom">...')
///     >>> print(version)
///     'atom10'
#[pyfunction]
#[pyo3(signature = (source, /))]
fn detect_format(source: &Bound<'_, PyAny>) -> PyResult<String> {
    let bytes: Vec<u8> = if let Ok(s) = source.extract::<String>() {
        s.into_bytes()
    } else if let Ok(b) = source.extract::<Vec<u8>>() {
        b
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "source must be str or bytes",
        ));
    };

    let version = core::detect_format(&bytes);
    Ok(version.to_string())
}
