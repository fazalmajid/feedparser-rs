use std::collections::HashMap;

/// HTTP response from feed fetch
#[derive(Debug, Clone)]
pub struct FeedHttpResponse {
    /// HTTP status code
    pub status: u16,
    /// Final URL after redirects
    pub url: String,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
    /// `ETag` header value
    pub etag: Option<String>,
    /// Last-Modified header value
    pub last_modified: Option<String>,
    /// Content-Type header value
    pub content_type: Option<String>,
    /// Encoding extracted from Content-Type
    pub encoding: Option<String>,
}

impl FeedHttpResponse {
    /// Extract charset from Content-Type header
    ///
    /// Parses header like "text/xml; charset=utf-8" and returns "utf-8"
    pub fn extract_charset_from_content_type(content_type: &str) -> Option<String> {
        content_type.split(';').find_map(|part| {
            part.trim()
                .strip_prefix("charset=")
                .map(|s| s.trim_matches('"').to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_charset_simple() {
        let ct = "text/xml; charset=utf-8";
        assert_eq!(
            FeedHttpResponse::extract_charset_from_content_type(ct),
            Some("utf-8".to_string())
        );
    }

    #[test]
    fn test_extract_charset_quoted() {
        let ct = "application/xml; charset=\"ISO-8859-1\"";
        assert_eq!(
            FeedHttpResponse::extract_charset_from_content_type(ct),
            Some("ISO-8859-1".to_string())
        );
    }

    #[test]
    fn test_extract_charset_no_charset() {
        let ct = "application/xml";
        assert_eq!(
            FeedHttpResponse::extract_charset_from_content_type(ct),
            None
        );
    }

    #[test]
    fn test_extract_charset_multiple_params() {
        let ct = "text/html; boundary=something; charset=utf-8";
        assert_eq!(
            FeedHttpResponse::extract_charset_from_content_type(ct),
            Some("utf-8".to_string())
        );
    }
}
