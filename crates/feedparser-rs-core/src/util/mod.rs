//! Utility functions for feed parsing
//!
//! This module provides helper functions for common feed parsing tasks.

pub mod base_url;
pub mod date;
pub mod encoding;
pub mod sanitize;
/// Text processing utilities
pub mod text;

// Re-export commonly used functions
pub use base_url::{is_safe_url, BaseUrlContext, combine_bases, resolve_url};
pub use date::parse_date;
