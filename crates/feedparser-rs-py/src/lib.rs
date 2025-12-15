use pyo3::prelude::*;
use pyo3::types::PyModule;

use feedparser_rs_core as core;

mod error;
mod limits;
mod types;

use error::convert_feed_error;
use limits::PyParserLimits;
use types::PyParsedFeed;

#[pymodule]
fn _feedparser_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_with_limits, m)?)?;
    m.add_function(wrap_pyfunction!(detect_format, m)?)?;
    m.add_class::<PyParsedFeed>()?;
    m.add_class::<PyParserLimits>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

/// Parse an RSS/Atom/JSON Feed from bytes or string
#[pyfunction]
#[pyo3(signature = (source, /))]
fn parse(py: Python<'_>, source: &Bound<'_, PyAny>) -> PyResult<PyParsedFeed> {
    parse_with_limits(py, source, None)
}

/// Parse with custom resource limits for DoS protection
#[pyfunction]
#[pyo3(signature = (source, limits=None))]
fn parse_with_limits(
    py: Python<'_>,
    source: &Bound<'_, PyAny>,
    limits: Option<&PyParserLimits>,
) -> PyResult<PyParsedFeed> {
    let bytes: Vec<u8> = if let Ok(s) = source.extract::<String>() {
        if s.starts_with("http://") || s.starts_with("https://") {
            return Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "URL fetching not implemented. Use requests.get(url).content",
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

    let parser_limits = limits.map(|l| l.to_core_limits()).unwrap_or_default();
    let parsed = core::parse_with_limits(&bytes, parser_limits).map_err(convert_feed_error)?;
    PyParsedFeed::from_core(py, parsed)
}

/// Detect feed format without full parsing
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
    Ok(core::detect_format(&bytes).to_string())
}
