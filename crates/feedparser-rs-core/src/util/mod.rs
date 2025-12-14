//! Utility functions for feed parsing
//!
//! This module provides helper functions for common feed parsing tasks.

pub mod date;
pub mod encoding;
pub mod sanitize;
/// Text processing utilities
pub mod text;

// Re-export commonly used functions
pub use date::parse_date;
