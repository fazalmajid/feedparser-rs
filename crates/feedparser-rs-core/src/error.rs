use thiserror::Error;

/// Feed parsing errors
#[derive(Error, Debug, Clone)]
pub enum FeedError {
    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlError(String),

    /// I/O error
    #[error("IO error: {0}")]
    IoError(String),

    /// Invalid feed format
    #[error("Invalid feed format: {0}")]
    InvalidFormat(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for feed parsing operations
pub type Result<T> = std::result::Result<T, FeedError>;

impl From<quick_xml::Error> for FeedError {
    fn from(err: quick_xml::Error) -> Self {
        Self::XmlError(err.to_string())
    }
}

impl From<serde_json::Error> for FeedError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err.to_string())
    }
}

impl From<std::io::Error> for FeedError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = FeedError::XmlError("test".to_string());
        assert_eq!(err.to_string(), "XML parsing error: test");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let feed_err = FeedError::from(io_err);
        assert!(matches!(feed_err, FeedError::IoError(_)));
    }

    #[test]
    #[allow(clippy::unnecessary_wraps)]
    fn test_result_type() {
        fn get_result() -> Result<i32> {
            Ok(42)
        }
        let result = get_result();
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), 42);

        let error: Result<i32> = Err(FeedError::Unknown("test".to_string()));
        assert!(error.is_err());
    }
}
