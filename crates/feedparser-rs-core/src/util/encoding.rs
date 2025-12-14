//! Encoding detection and conversion utilities
//!
//! This module provides functions for detecting character encoding
//! and converting to UTF-8.

use encoding_rs::{Encoding, UTF_8};

/// Detect character encoding from byte data
///
/// Detection order:
/// 1. BOM (Byte Order Mark)
/// 2. XML declaration (<?xml encoding="..."?>)
/// 3. Default to UTF-8
///
/// # Arguments
///
/// * `data` - Raw byte data
///
/// # Returns
///
/// Detected encoding name (e.g., "utf-8", "iso-8859-1")
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::encoding::detect_encoding;
///
/// // UTF-8 with BOM
/// let data = b"\xEF\xBB\xBF<?xml version=\"1.0\"?>";
/// assert_eq!(detect_encoding(data), "UTF-8");
///
/// // XML declaration
/// let data = b"<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>";
/// assert_eq!(detect_encoding(data), "windows-1252");
/// ```
pub fn detect_encoding(data: &[u8]) -> &'static str {
    if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return "UTF-8";
    }
    // UTF-32 BOMs must be checked BEFORE UTF-16 BOMs
    // because UTF-32LE BOM (FF FE 00 00) starts with UTF-16LE BOM (FF FE)
    if data.starts_with(&[0x00, 0x00, 0xFE, 0xFF]) {
        return "UTF-32BE";
    }
    if data.starts_with(&[0xFF, 0xFE, 0x00, 0x00]) {
        return "UTF-32LE";
    }
    if data.starts_with(&[0xFF, 0xFE]) {
        return "UTF-16LE";
    }
    if data.starts_with(&[0xFE, 0xFF]) {
        return "UTF-16BE";
    }

    if let Some(encoding) = extract_xml_encoding(data) {
        return encoding;
    }

    "UTF-8"
}

/// Extract encoding from XML declaration
///
/// Parses <?xml version="1.0" encoding="..."?> declaration
fn extract_xml_encoding(data: &[u8]) -> Option<&'static str> {
    let search_len = data.len().min(512);
    let search_data = &data[..search_len];

    if let Ok(header) = std::str::from_utf8(search_data) {
        if let Some(enc_start) = header.find("encoding=") {
            let after_eq = &header[enc_start + 9..];
            let quote = after_eq.chars().next()?;
            if quote == '"' || quote == '\'' {
                let quote_end = after_eq[1..].find(quote)?;
                let encoding_name = &after_eq[1..=quote_end];
                return normalize_encoding_name(encoding_name);
            }
        }
    }

    None
}

/// Normalize encoding name to encoding_rs canonical form
fn normalize_encoding_name(name: &str) -> Option<&'static str> {
    let normalized = name.trim().to_lowercase();
    Encoding::for_label(normalized.as_bytes()).map(|enc| enc.name())
}

/// Convert data to UTF-8 from detected encoding
///
/// # Arguments
///
/// * `data` - Raw byte data in unknown encoding
/// * `encoding_name` - Encoding name (e.g., "iso-8859-1")
///
/// # Returns
///
/// * `Ok(String)` - UTF-8 string
/// * `Err(String)` - Error message if conversion failed
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::encoding::convert_to_utf8;
///
/// let latin1 = b"\xE9"; // é in ISO-8859-1
/// let utf8 = convert_to_utf8(latin1, "iso-8859-1").unwrap();
/// assert_eq!(utf8, "é");
/// ```
pub fn convert_to_utf8(data: &[u8], encoding_name: &str) -> Result<String, String> {
    let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(UTF_8);

    let (cow, _encoding_used, had_errors) = encoding.decode(data);

    if had_errors {
        Err(format!(
            "Encoding conversion from {} had errors",
            encoding_name
        ))
    } else {
        Ok(cow.into_owned())
    }
}

/// Detect encoding and convert to UTF-8 in one step
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::encoding::detect_and_convert;
///
/// let data = b"<?xml version=\"1.0\"?><root>Test</root>";
/// let (utf8, detected_encoding) = detect_and_convert(data).unwrap();
/// assert_eq!(detected_encoding, "UTF-8");
/// assert!(utf8.contains("Test"));
/// ```
pub fn detect_and_convert(data: &[u8]) -> Result<(String, &'static str), String> {
    let encoding_name = detect_encoding(data);
    let utf8_string = convert_to_utf8(data, encoding_name)?;
    Ok((utf8_string, encoding_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_utf8_bom() {
        let data = b"\xEF\xBB\xBF<?xml version=\"1.0\"?>";
        assert_eq!(detect_encoding(data), "UTF-8");
    }

    #[test]
    fn test_detect_utf16le_bom() {
        let data = b"\xFF\xFE<\x00?\x00x\x00m\x00l\x00";
        assert_eq!(detect_encoding(data), "UTF-16LE");
    }

    #[test]
    fn test_detect_utf16be_bom() {
        let data = b"\xFE\xFF\x00<\x00?\x00x\x00m\x00l";
        assert_eq!(detect_encoding(data), "UTF-16BE");
    }

    #[test]
    fn test_detect_from_xml_declaration() {
        let data = b"<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>";
        assert_eq!(detect_encoding(data).to_lowercase(), "windows-1252");
    }

    #[test]
    fn test_detect_from_xml_declaration_single_quotes() {
        let data = b"<?xml version='1.0' encoding='UTF-8'?>";
        assert_eq!(detect_encoding(data), "UTF-8");
    }

    #[test]
    fn test_detect_default_utf8() {
        let data = b"<?xml version=\"1.0\"?>";
        assert_eq!(detect_encoding(data), "UTF-8");
    }

    #[test]
    fn test_convert_iso8859_1() {
        let data = b"\xE9";
        let utf8 = convert_to_utf8(data, "iso-8859-1").unwrap();
        assert_eq!(utf8, "é");
    }

    #[test]
    fn test_convert_windows1252() {
        let data = b"\x93Hello\x94";
        let utf8 = convert_to_utf8(data, "windows-1252").unwrap();
        assert!(utf8.contains("Hello"));
    }

    #[test]
    fn test_detect_and_convert() {
        let data = b"<?xml version=\"1.0\"?><root>Test</root>";
        let (utf8, encoding) = detect_and_convert(data).unwrap();
        assert_eq!(encoding, "UTF-8");
        assert!(utf8.contains("Test"));
    }

    #[test]
    fn test_extract_xml_encoding_double_quotes() {
        let data = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>";
        assert!(extract_xml_encoding(data).is_some());
    }

    #[test]
    fn test_extract_xml_encoding_single_quotes() {
        let data = b"<?xml version='1.0' encoding='UTF-8'?>";
        assert!(extract_xml_encoding(data).is_some());
    }

    #[test]
    fn test_extract_xml_encoding_none() {
        let data = b"<?xml version=\"1.0\"?>";
        assert!(extract_xml_encoding(data).is_none());
    }

    #[test]
    fn test_normalize_encoding_name() {
        assert_eq!(normalize_encoding_name("UTF-8"), Some("UTF-8"));
        assert_eq!(normalize_encoding_name("utf-8"), Some("UTF-8"));
        assert_eq!(normalize_encoding_name("  UTF-8  "), Some("UTF-8"));
        assert_eq!(normalize_encoding_name("ISO-8859-1"), Some("windows-1252"));
    }

    #[test]
    fn test_convert_utf8_to_utf8() {
        let data = "Hello".as_bytes();
        let result = convert_to_utf8(data, "utf-8").unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_detect_no_encoding_declaration() {
        let data = b"<rss><channel></channel></rss>";
        assert_eq!(detect_encoding(data), "UTF-8");
    }

    #[test]
    fn test_empty_data() {
        let data = b"";
        assert_eq!(detect_encoding(data), "UTF-8");
    }
}
