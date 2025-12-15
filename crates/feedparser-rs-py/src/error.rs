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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_xml_error() {
        let err = FeedError::XmlError("malformed".to_string());
        let py_err = convert_feed_error(err);
        assert!(py_err.to_string().contains("XML parse error: malformed"));
    }

    #[test]
    fn test_convert_io_error() {
        let err = FeedError::IoError("file not found".to_string());
        let py_err = convert_feed_error(err);
        assert!(py_err.to_string().contains("I/O error: file not found"));
    }

    #[test]
    fn test_convert_invalid_format_error() {
        let err = FeedError::InvalidFormat("unknown format".to_string());
        let py_err = convert_feed_error(err);
        assert!(
            py_err
                .to_string()
                .contains("Invalid feed format: unknown format")
        );
    }

    #[test]
    fn test_convert_encoding_error() {
        let err = FeedError::EncodingError("invalid encoding".to_string());
        let py_err = convert_feed_error(err);
        assert!(
            py_err
                .to_string()
                .contains("Encoding error: invalid encoding")
        );
    }

    #[test]
    fn test_convert_json_error() {
        let err = FeedError::JsonError("invalid json".to_string());
        let py_err = convert_feed_error(err);
        assert!(
            py_err
                .to_string()
                .contains("JSON parse error: invalid json")
        );
    }

    #[test]
    fn test_convert_unknown_error() {
        let err = FeedError::Unknown("unexpected".to_string());
        let py_err = convert_feed_error(err);
        assert!(py_err.to_string().contains("Unknown error: unexpected"));
    }
}
