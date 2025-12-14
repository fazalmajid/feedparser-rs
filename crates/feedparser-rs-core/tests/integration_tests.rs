use feedparser_rs_core::{FeedVersion, detect_format, parse};

/// Helper function to load test fixtures
fn load_fixture(path: &str) -> Vec<u8> {
    // Fixtures are in the workspace root tests/fixtures/ directory
    let fixture_path = format!("../../tests/fixtures/{}", path);
    std::fs::read(&fixture_path)
        .unwrap_or_else(|e| panic!("Failed to load fixture '{}': {}", fixture_path, e))
}

/// Helper to assert basic feed validity
fn assert_feed_valid(result: &feedparser_rs_core::ParsedFeed) {
    // Currently stubs return empty feeds, so we just check it doesn't panic
    // Phase 2: Add real assertions here
    assert!(result.version == FeedVersion::Unknown || !result.bozo);
}

#[test]
fn test_parse_rss_basic_fixture() {
    let xml = load_fixture("rss/basic.xml");
    let result = parse(&xml);

    assert!(result.is_ok(), "Failed to parse RSS fixture");
    let feed = result.unwrap();

    // TODO Phase 2: Add real assertions once parser is implemented
    // assert_eq!(feed.version, FeedVersion::Rss20);
    // assert!(!feed.bozo);
    // assert_eq!(feed.feed.title.as_deref(), Some("Example RSS Feed"));
    // assert_eq!(feed.entries.len(), 2);

    assert_feed_valid(&feed);
}

#[test]
fn test_parse_atom_basic_fixture() {
    let xml = load_fixture("atom/basic.xml");
    let result = parse(&xml);

    assert!(result.is_ok(), "Failed to parse Atom fixture");
    let feed = result.unwrap();

    // TODO Phase 2: Add real assertions once parser is implemented
    // assert_eq!(feed.version, FeedVersion::Atom10);
    // assert!(!feed.bozo);
    // assert_eq!(feed.feed.title.as_deref(), Some("Example Atom Feed"));

    assert_feed_valid(&feed);
}

#[test]
fn test_detect_format_rss() {
    let xml = load_fixture("rss/basic.xml");
    let version = detect_format(&xml);

    // TODO Phase 2: Once detect_format is implemented
    // assert_eq!(version, FeedVersion::Rss20);

    // For now, just ensure it doesn't panic
    let _ = version;
}

#[test]
fn test_detect_format_atom() {
    let xml = load_fixture("atom/basic.xml");
    let version = detect_format(&xml);

    // TODO Phase 2: Once detect_format is implemented
    // assert_eq!(version, FeedVersion::Atom10);

    // For now, just ensure it doesn't panic
    let _ = version;
}

#[test]
fn test_parse_empty_input() {
    let result = parse(b"");

    // Should not panic, might return error or empty feed
    match result {
        Ok(feed) => {
            // Empty input might set bozo flag
            let _ = feed;
        }
        Err(_) => {
            // Or might return error - both are acceptable
        }
    }
}

#[test]
fn test_parse_invalid_xml() {
    let result = parse(b"<invalid><xml>");

    // Should handle gracefully (either error or bozo flag)
    match result {
        Ok(feed) => {
            // Malformed input should set bozo flag (when implemented)
            // TODO Phase 2: assert!(feed.bozo);
            let _ = feed;
        }
        Err(_) => {
            // Or return error - both acceptable
        }
    }
}

#[test]
fn test_capacity_constructors() {
    use feedparser_rs_core::{Entry, FeedMeta, ParsedFeed};

    // Test ParsedFeed::with_capacity
    let feed = ParsedFeed::with_capacity(100);
    assert_eq!(feed.encoding, "utf-8");
    assert_eq!(feed.entries.capacity(), 100);
    assert!(feed.namespaces.capacity() >= 8);

    // Test FeedMeta::with_rss_capacity
    let rss_meta = FeedMeta::with_rss_capacity();
    assert!(rss_meta.links.capacity() >= 2);
    assert!(rss_meta.authors.capacity() >= 1);
    assert!(rss_meta.tags.capacity() >= 3);

    // Test FeedMeta::with_atom_capacity
    let atom_meta = FeedMeta::with_atom_capacity();
    assert!(atom_meta.links.capacity() >= 4);
    assert!(atom_meta.authors.capacity() >= 2);
    assert!(atom_meta.tags.capacity() >= 5);

    // Test Entry::with_capacity
    let entry = Entry::with_capacity();
    assert!(entry.links.capacity() >= 2);
    assert!(entry.content.capacity() >= 1);
    assert!(entry.authors.capacity() >= 1);
    assert!(entry.tags.capacity() >= 3);
}

#[test]
fn test_parse_json_feed_basic() {
    let json = load_fixture("json/basic-1.1.json");
    let result = parse(&json);

    assert!(result.is_ok(), "Failed to parse JSON Feed fixture");
    let feed = result.unwrap();

    assert_eq!(feed.version, FeedVersion::JsonFeed11);
    assert!(!feed.bozo);
    assert_eq!(feed.feed.title.as_deref(), Some("Example JSON Feed"));
    assert_eq!(feed.entries.len(), 1);
    assert_eq!(feed.entries[0].id.as_deref(), Some("1"));
    assert_eq!(feed.entries[0].title.as_deref(), Some("First Post"));
}

#[test]
fn test_parse_json_feed_10() {
    let json = load_fixture("json/basic-1.0.json");
    let result = parse(&json);

    assert!(result.is_ok());
    let feed = result.unwrap();

    assert_eq!(feed.version, FeedVersion::JsonFeed10);
    assert!(!feed.bozo);
}

#[test]
fn test_parse_json_feed_minimal() {
    let json = load_fixture("json/minimal.json");
    let result = parse(&json);

    assert!(result.is_ok());
    let feed = result.unwrap();

    assert_eq!(feed.version, FeedVersion::JsonFeed11);
    assert!(!feed.bozo);
    assert_eq!(feed.feed.title.as_deref(), Some("Minimal Feed"));
    assert_eq!(feed.entries.len(), 0);
}
