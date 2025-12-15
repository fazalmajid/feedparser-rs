use super::response::FeedHttpResponse;
use crate::error::{FeedError, Result};
use reqwest::blocking::{Client, Response};
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, HeaderMap, HeaderValue, IF_MODIFIED_SINCE, IF_NONE_MATCH, USER_AGENT,
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
    #[must_use]
    pub fn with_user_agent(mut self, agent: String) -> Self {
        self.user_agent = agent;
        self
    }

    /// Sets request timeout
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
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
        let mut headers = HeaderMap::new();

        // Standard headers
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&self.user_agent).map_err(|e| FeedError::Http {
                message: format!("Invalid User-Agent: {e}"),
            })?,
        );

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

        // Conditional GET headers
        if let Some(etag_val) = etag {
            headers.insert(
                IF_NONE_MATCH,
                HeaderValue::from_str(etag_val).map_err(|e| FeedError::Http {
                    message: format!("Invalid ETag: {e}"),
                })?,
            );
        }

        if let Some(modified_val) = modified {
            headers.insert(
                IF_MODIFIED_SINCE,
                HeaderValue::from_str(modified_val).map_err(|e| FeedError::Http {
                    message: format!("Invalid Last-Modified: {e}"),
                })?,
            );
        }

        // Merge extra headers
        if let Some(extra) = extra_headers {
            headers.extend(extra.clone());
        }

        let response =
            self.client
                .get(url)
                .headers(headers)
                .send()
                .map_err(|e| FeedError::Http {
                    message: format!("HTTP request failed: {e}"),
                })?;

        Self::build_response(response, url)
    }

    /// Converts `reqwest` Response to `FeedHttpResponse`
    fn build_response(response: Response, _original_url: &str) -> Result<FeedHttpResponse> {
        let status = response.status().as_u16();
        let url = response.url().to_string();

        // Convert headers to HashMap
        let mut headers_map = HashMap::new();
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
}
