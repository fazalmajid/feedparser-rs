use feedparser_rs_core::FeedError;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

pub fn convert_feed_error(err: FeedError) -> PyErr {
    match err {
        FeedError::XmlError(msg) => PyValueError::new_err(format!("XML parse error: {}", msg)),
        FeedError::IoError(msg) => PyRuntimeError::new_err(format!("I/O error: {}", msg)),
        FeedError::InvalidFormat(msg) => {
            PyValueError::new_err(format!("Invalid feed format: {}", msg))
        }
        FeedError::EncodingError(msg) => PyValueError::new_err(format!("Encoding error: {}", msg)),
        FeedError::JsonError(msg) => PyValueError::new_err(format!("JSON parse error: {}", msg)),
        FeedError::Unknown(msg) => PyRuntimeError::new_err(format!("Unknown error: {}", msg)),
    }
}

// Note: Error conversion is tested via Python integration tests (pytest)
// since PyErr.to_string() requires Python GIL to be initialized.
