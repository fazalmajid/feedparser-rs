use crate::error::{FeedError, Result};
use std::net::{Ipv4Addr, Ipv6Addr};
use url::Url;

// Localhost variations that should be blocked
const LOCALHOST_VARIANTS: &[&str] = &[
    "localhost",
    "localhost.localdomain",
    "127.0.0.1",
    "::1",
    "[::1]",
];

// Internal TLDs that should be blocked
const INTERNAL_TLDS: &[&str] = &[
    ".local",
    ".localhost",
    ".internal",
    ".intranet",
    ".corp",
    ".home",
    ".lan",
];

// Cloud metadata endpoints that should be blocked
const METADATA_DOMAINS: &[&str] = &[
    "metadata.google.internal",
    "169.254.169.254",
    "metadata",
    "metadata.azure.com",
];

/// Validates a URL to prevent Server-Side Request Forgery (SSRF) attacks
///
/// This function ensures that URLs only point to public, safe destinations.
///
/// # Security Checks
///
/// 1. Only HTTP and HTTPS schemes are allowed
/// 2. Private IP ranges are blocked (RFC 1918, RFC 4193)
/// 3. Localhost and loopback addresses are blocked
/// 4. Link-local addresses are blocked (169.254.0.0/16)
/// 5. Cloud metadata endpoints are blocked
/// 6. Internal domain names are blocked (.local, .internal)
///
/// # Errors
///
/// Returns `FeedError::Http` if:
/// - The URL is malformed or invalid
/// - The URL scheme is not HTTP or HTTPS
/// - The URL points to a private IP address, localhost, or internal domain
/// - The URL points to a cloud metadata endpoint
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::http::validation::validate_url;
///
/// // These are allowed
/// assert!(validate_url("https://example.com/feed.xml").is_ok());
/// assert!(validate_url("http://blog.example.org/rss").is_ok());
///
/// // These are blocked
/// assert!(validate_url("http://localhost/").is_err());
/// assert!(validate_url("http://192.168.1.1/").is_err());
/// assert!(validate_url("http://169.254.169.254/").is_err());
/// assert!(validate_url("file:///etc/passwd").is_err());
/// ```
pub fn validate_url(url_str: &str) -> Result<Url> {
    // Parse URL
    let url = Url::parse(url_str).map_err(|e| FeedError::Http {
        message: format!("Invalid URL: {e}"),
    })?;

    // Check 1: Only allow HTTP/HTTPS schemes
    match url.scheme() {
        "http" | "https" => {}
        scheme => {
            return Err(FeedError::Http {
                message: format!(
                    "Unsupported URL scheme '{scheme}': only 'http' and 'https' are allowed"
                ),
            })
        }
    }

    // Check 2: URL must have a host
    let host = url.host().ok_or_else(|| FeedError::Http {
        message: "URL must have a host".to_string(),
    })?;

    // Check 3: Validate host based on type
    match host {
        url::Host::Ipv4(ip) => {
            validate_ipv4(ip)?;
        }
        url::Host::Ipv6(ip) => {
            validate_ipv6(ip)?;
        }
        url::Host::Domain(domain) => {
            validate_domain(domain)?;
        }
    }

    Ok(url)
}

/// Validates an IPv4 address to prevent SSRF
fn validate_ipv4(ip: Ipv4Addr) -> Result<()> {
    if ip.is_private() {
        return Err(FeedError::Http {
            message: format!("Private IP address not allowed: {ip} (RFC 1918)"),
        });
    }

    if ip.is_loopback() {
        return Err(FeedError::Http {
            message: format!("Loopback address not allowed: {ip}"),
        });
    }

    if ip.is_link_local() {
        return Err(FeedError::Http {
            message: format!("Link-local address not allowed: {ip} (169.254.0.0/16)"),
        });
    }

    if ip.is_broadcast() {
        return Err(FeedError::Http {
            message: format!("Broadcast address not allowed: {ip}"),
        });
    }

    if ip.is_documentation() {
        return Err(FeedError::Http {
            message: format!("Documentation IP not allowed: {ip} (RFC 5737)"),
        });
    }

    // Block cloud metadata endpoints specifically
    let octets = ip.octets();
    if octets[0] == 169 && octets[1] == 254 && octets[2] == 169 && octets[3] == 254 {
        return Err(FeedError::Http {
            message: "AWS metadata endpoint blocked: 169.254.169.254".to_string(),
        });
    }

    // Block carrier-grade NAT (100.64.0.0/10)
    if octets[0] == 100 && (octets[1] & 0xC0) == 64 {
        return Err(FeedError::Http {
            message: format!("Carrier-grade NAT address not allowed: {ip} (100.64.0.0/10)"),
        });
    }

    // Block 0.0.0.0/8
    if octets[0] == 0 {
        return Err(FeedError::Http {
            message: format!("0.0.0.0/8 range not allowed: {ip}"),
        });
    }

    Ok(())
}

/// Validates an IPv6 address to prevent SSRF
fn validate_ipv6(ip: Ipv6Addr) -> Result<()> {
    if ip.is_loopback() {
        return Err(FeedError::Http {
            message: format!("IPv6 loopback address not allowed: {ip}"),
        });
    }

    if ip.is_unicast_link_local() {
        return Err(FeedError::Http {
            message: format!("IPv6 link-local address not allowed: {ip} (fe80::/10)"),
        });
    }

    // Check for Unique Local Addresses (ULA) - fc00::/7
    let segments = ip.segments();
    if (segments[0] & 0xFE00) == 0xFC00 {
        return Err(FeedError::Http {
            message: format!("IPv6 unique local address not allowed: {ip} (fc00::/7)"),
        });
    }

    // Block multicast addresses
    if ip.is_multicast() {
        return Err(FeedError::Http {
            message: format!("IPv6 multicast address not allowed: {ip} (ff00::/8)"),
        });
    }

    Ok(())
}

/// Validates a domain name to prevent SSRF
fn validate_domain(domain: &str) -> Result<()> {
    let domain_lower = domain.to_lowercase();

    // Block localhost variations
    if LOCALHOST_VARIANTS.contains(&domain_lower.as_str()) {
        return Err(FeedError::Http {
            message: format!("Localhost domain not allowed: {domain}"),
        });
    }

    // Block internal TLDs
    for tld in INTERNAL_TLDS {
        if domain_lower.ends_with(tld) {
            return Err(FeedError::Http {
                message: format!("Internal domain TLD not allowed: {domain}"),
            });
        }
    }

    // Block cloud metadata endpoints
    if METADATA_DOMAINS.contains(&domain_lower.as_str()) {
        return Err(FeedError::Http {
            message: format!("Cloud metadata domain not allowed: {domain}"),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Positive tests - these should pass
    #[test]
    fn test_valid_http_url() {
        assert!(validate_url("http://example.com/feed.xml").is_ok());
    }

    #[test]
    fn test_valid_https_url() {
        assert!(validate_url("https://blog.example.org/rss").is_ok());
    }

    #[test]
    fn test_valid_with_port() {
        assert!(validate_url("https://example.com:8443/feed").is_ok());
    }

    #[test]
    fn test_valid_with_path() {
        assert!(validate_url("https://example.com/path/to/feed.xml").is_ok());
    }

    // Negative tests - scheme validation
    #[test]
    fn test_reject_file_scheme() {
        assert!(validate_url("file:///etc/passwd").is_err());
    }

    #[test]
    fn test_reject_ftp_scheme() {
        assert!(validate_url("ftp://example.com/file").is_err());
    }

    #[test]
    fn test_reject_javascript_scheme() {
        assert!(validate_url("javascript:alert(1)").is_err());
    }

    #[test]
    fn test_reject_data_scheme() {
        assert!(validate_url("data:text/html,<script>alert(1)</script>").is_err());
    }

    // Negative tests - IPv4 private ranges
    #[test]
    fn test_reject_ipv4_private_10() {
        assert!(validate_url("http://10.0.0.1/").is_err());
        assert!(validate_url("http://10.255.255.255/").is_err());
    }

    #[test]
    fn test_reject_ipv4_private_172() {
        assert!(validate_url("http://172.16.0.1/").is_err());
        assert!(validate_url("http://172.31.255.255/").is_err());
    }

    #[test]
    fn test_reject_ipv4_private_192() {
        assert!(validate_url("http://192.168.0.1/").is_err());
        assert!(validate_url("http://192.168.255.255/").is_err());
    }

    #[test]
    fn test_reject_ipv4_localhost() {
        assert!(validate_url("http://127.0.0.1/").is_err());
        assert!(validate_url("http://127.0.0.2/").is_err());
    }

    #[test]
    fn test_reject_ipv4_link_local() {
        assert!(validate_url("http://169.254.169.254/").is_err());
        assert!(validate_url("http://169.254.0.1/").is_err());
    }

    #[test]
    fn test_reject_ipv4_zero() {
        assert!(validate_url("http://0.0.0.0/").is_err());
    }

    #[test]
    fn test_reject_ipv4_broadcast() {
        assert!(validate_url("http://255.255.255.255/").is_err());
    }

    // Negative tests - IPv6
    #[test]
    fn test_reject_ipv6_loopback() {
        assert!(validate_url("http://[::1]/").is_err());
    }

    #[test]
    fn test_reject_ipv6_link_local() {
        assert!(validate_url("http://[fe80::1]/").is_err());
    }

    #[test]
    fn test_reject_ipv6_unique_local() {
        assert!(validate_url("http://[fc00::1]/").is_err());
        assert!(validate_url("http://[fd00::1]/").is_err());
    }

    // Negative tests - domain names
    #[test]
    fn test_reject_localhost_domain() {
        assert!(validate_url("http://localhost/").is_err());
    }

    #[test]
    fn test_reject_local_tld() {
        assert!(validate_url("http://myserver.local/").is_err());
    }

    #[test]
    fn test_reject_internal_tld() {
        assert!(validate_url("http://server.internal/").is_err());
    }

    #[test]
    fn test_reject_cloud_metadata() {
        assert!(validate_url("http://metadata.google.internal/").is_err());
        assert!(validate_url("http://metadata.azure.com/").is_err());
    }

    // Edge cases
    #[test]
    fn test_reject_no_host() {
        assert!(validate_url("http://").is_err());
    }

    #[test]
    fn test_reject_invalid_url() {
        assert!(validate_url("not a url").is_err());
    }

    #[test]
    fn test_public_ip_allowed() {
        // Public IPs should be allowed
        assert!(validate_url("http://8.8.8.8/").is_ok());
        assert!(validate_url("http://1.1.1.1/").is_ok());
    }

    #[test]
    fn test_carrier_grade_nat_blocked() {
        assert!(validate_url("http://100.64.0.1/").is_err());
        assert!(validate_url("http://100.127.255.255/").is_err());
    }

    #[test]
    fn test_ipv6_multicast_blocked() {
        assert!(validate_url("http://[ff00::1]/").is_err());
        assert!(validate_url("http://[ff02::1]/").is_err());
    }
}
