use super::response::FeedHttpResponse;
use super::validation::validate_url;
use crate::error::{FeedError, Result};
use reqwest::blocking::{Client, Response};
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, HeaderMap, HeaderName, HeaderValue, IF_MODIFIED_SINCE, IF_NONE_MATCH,
    USER_AGENT,
};
use std::collections::HashMap;
use std::time::Duration;

/// HTTP client for fetching feeds
pub struct FeedHttpClient {
    client: Client,
    user_agent: String,
    timeout: Duration,
}

impl FeedHttpClient {
    /// Creates a new HTTP client with default settings
    ///
    /// Default settings:
    /// - 30 second timeout
    /// - Gzip, deflate, and brotli compression enabled
    /// - Maximum 10 redirects
    /// - Custom User-Agent
    ///
    /// # Errors
    ///
    /// Returns `FeedError::Http` if the underlying HTTP client cannot be created.
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .map_err(|e| FeedError::Http {
                message: format!("Failed to create HTTP client: {e}"),
            })?;

        Ok(Self {
            client,
            user_agent: format!(
                "feedparser-rs/{} (+https://github.com/bug-ops/feedparser-rs)",
                env!("CARGO_PKG_VERSION")
            ),
            timeout: Duration::from_secs(30),
        })
    }

    /// Sets a custom User-Agent header
    ///
    /// # Security
    ///
    /// User-Agent is truncated to 512 bytes to prevent header injection attacks.
    #[must_use]
    pub fn with_user_agent(mut self, agent: String) -> Self {
        // Truncate to 512 bytes to prevent header injection
        const MAX_USER_AGENT_LEN: usize = 512;
        self.user_agent = if agent.len() > MAX_USER_AGENT_LEN {
            agent.chars().take(MAX_USER_AGENT_LEN).collect()
        } else {
            agent
        };
        self
    }

    /// Sets request timeout
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Insert header with consistent error handling
    ///
    /// Helper method to reduce boilerplate in header insertion.
    #[inline]
    fn insert_header(
        headers: &mut HeaderMap,
        name: HeaderName,
        value: &str,
        field_name: &str,
    ) -> Result<()> {
        headers.insert(
            name,
            HeaderValue::from_str(value).map_err(|e| FeedError::Http {
                message: format!("Invalid {field_name}: {e}"),
            })?,
        );
        Ok(())
    }

    /// Fetches a feed from the given URL
    ///
    /// Supports conditional GET with `ETag` and `Last-Modified` headers.
    ///
    /// # Arguments
    ///
    /// * `url` - HTTP/HTTPS URL to fetch
    /// * `etag` - Optional `ETag` from previous fetch
    /// * `modified` - Optional `Last-Modified` from previous fetch
    /// * `extra_headers` - Additional custom headers
    ///
    /// # Errors
    ///
    /// Returns `FeedError::Http` if the request fails or headers are invalid.
    pub fn get(
        &self,
        url: &str,
        etag: Option<&str>,
        modified: Option<&str>,
        extra_headers: Option<&HeaderMap>,
    ) -> Result<FeedHttpResponse> {
        // Validate URL to prevent SSRF attacks
        let validated_url = validate_url(url)?;
        let url_str = validated_url.as_str();

        let mut headers = HeaderMap::new();

        // Standard headers
        Self::insert_header(&mut headers, USER_AGENT, &self.user_agent, "User-Agent")?;

        headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "application/rss+xml, application/atom+xml, application/xml, text/xml, */*",
            ),
        );

        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );

        // Conditional GET headers with length validation
        if let Some(etag_val) = etag {
            // Truncate ETag to 1KB to prevent oversized headers
            const MAX_ETAG_LEN: usize = 1024;
            let sanitized_etag = if etag_val.len() > MAX_ETAG_LEN {
                &etag_val[..MAX_ETAG_LEN]
            } else {
                etag_val
            };
            Self::insert_header(&mut headers, IF_NONE_MATCH, sanitized_etag, "ETag")?;
        }

        if let Some(modified_val) = modified {
            // Truncate Last-Modified to 64 bytes (RFC 822 dates are ~30 bytes)
            const MAX_MODIFIED_LEN: usize = 64;
            let sanitized_modified = if modified_val.len() > MAX_MODIFIED_LEN {
                &modified_val[..MAX_MODIFIED_LEN]
            } else {
                modified_val
            };
            Self::insert_header(
                &mut headers,
                IF_MODIFIED_SINCE,
                sanitized_modified,
                "Last-Modified",
            )?;
        }

        // Merge extra headers
        if let Some(extra) = extra_headers {
            headers.extend(extra.clone());
        }

        let response = self
            .client
            .get(url_str)
            .headers(headers)
            .send()
            .map_err(|e| FeedError::Http {
                message: format!("HTTP request failed: {e}"),
            })?;

        Self::build_response(response, url_str)
    }

    /// Converts `reqwest` Response to `FeedHttpResponse`
    fn build_response(response: Response, _original_url: &str) -> Result<FeedHttpResponse> {
        let status = response.status().as_u16();
        let url = response.url().to_string();

        // Convert headers to HashMap with pre-allocated capacity
        let mut headers_map = HashMap::with_capacity(response.headers().len());
        for (name, value) in response.headers() {
            if let Ok(val_str) = value.to_str() {
                headers_map.insert(name.to_string(), val_str.to_string());
            }
        }

        // Extract caching headers
        let etag = headers_map.get("etag").cloned();
        let last_modified = headers_map.get("last-modified").cloned();
        let content_type = headers_map.get("content-type").cloned();

        // Extract encoding from Content-Type
        let encoding = content_type
            .as_ref()
            .and_then(|ct| FeedHttpResponse::extract_charset_from_content_type(ct));

        // Read body (handles gzip/deflate automatically)
        let body = if status == 304 {
            // Not Modified - no body
            Vec::new()
        } else {
            response
                .bytes()
                .map_err(|e| FeedError::Http {
                    message: format!("Failed to read response body: {e}"),
                })?
                .to_vec()
        };

        Ok(FeedHttpResponse {
            status,
            url,
            headers: headers_map,
            body,
            etag,
            last_modified,
            content_type,
            encoding,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = FeedHttpClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_custom_user_agent() {
        let client = FeedHttpClient::new()
            .unwrap()
            .with_user_agent("CustomBot/1.0".to_string());
        assert_eq!(client.user_agent, "CustomBot/1.0");
    }

    #[test]
    fn test_custom_timeout() {
        let timeout = Duration::from_secs(60);
        let client = FeedHttpClient::new().unwrap().with_timeout(timeout);
        assert_eq!(client.timeout, timeout);
    }

    // SSRF protection tests
    #[test]
    fn test_reject_localhost_url() {
        let client = FeedHttpClient::new().unwrap();
        let result = client.get("http://localhost/feed.xml", None, None, None);
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        assert!(err_msg.contains("Localhost domain not allowed"));
    }

    #[test]
    fn test_reject_private_ip() {
        let client = FeedHttpClient::new().unwrap();
        let result = client.get("http://192.168.1.1/feed.xml", None, None, None);
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        assert!(err_msg.contains("Private IP address not allowed"));
    }

    #[test]
    fn test_reject_metadata_endpoint() {
        let client = FeedHttpClient::new().unwrap();
        let result = client.get("http://169.254.169.254/latest/meta-data/", None, None, None);
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        // Should be rejected as AWS metadata endpoint or link-local
        assert!(err_msg.contains("metadata") || err_msg.contains("Link-local"));
    }

    #[test]
    fn test_reject_file_scheme() {
        let client = FeedHttpClient::new().unwrap();
        let result = client.get("file:///etc/passwd", None, None, None);
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        assert!(err_msg.contains("Unsupported URL scheme"));
    }

    #[test]
    fn test_reject_internal_domain() {
        let client = FeedHttpClient::new().unwrap();
        let result = client.get("http://server.local/feed.xml", None, None, None);
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        assert!(err_msg.contains("Internal domain TLD not allowed"));
    }

    #[test]
    fn test_insert_header_valid() {
        let mut headers = HeaderMap::new();
        let result =
            FeedHttpClient::insert_header(&mut headers, USER_AGENT, "TestBot/1.0", "User-Agent");
        assert!(result.is_ok());
        assert_eq!(headers.get(USER_AGENT).unwrap(), "TestBot/1.0");
    }

    #[test]
    fn test_insert_header_invalid_value() {
        let mut headers = HeaderMap::new();
        // Invalid header value with control characters
        let result = FeedHttpClient::insert_header(
            &mut headers,
            USER_AGENT,
            "Invalid\nHeader",
            "User-Agent",
        );
        assert!(result.is_err());
        match result {
            Err(FeedError::Http { message }) => {
                assert!(message.contains("Invalid User-Agent"));
            }
            _ => panic!("Expected Http error"),
        }
    }

    #[test]
    fn test_insert_header_multiple_headers() {
        let mut headers = HeaderMap::new();

        FeedHttpClient::insert_header(&mut headers, USER_AGENT, "TestBot/1.0", "User-Agent")
            .unwrap();

        FeedHttpClient::insert_header(&mut headers, ACCEPT, "application/xml", "Accept").unwrap();

        assert_eq!(headers.len(), 2);
        assert_eq!(headers.get(USER_AGENT).unwrap(), "TestBot/1.0");
        assert_eq!(headers.get(ACCEPT).unwrap(), "application/xml");
    }
}
