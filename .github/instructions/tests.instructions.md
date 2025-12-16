# Testing Guidelines

**Applies to:** `tests/**`, `crates/**/tests/**`, `crates/**/benches/**`

## Testing Philosophy

### Test Pyramid

```
        /\
       /  \  E2E (Cross-language integration)
      /____\
     /      \  Integration (Full parser tests)
    /________\
   /          \  Unit (Individual functions)
  /____________\
```

**Distribution:**
- 70% Unit tests (fast, focused)
- 25% Integration tests (real feeds)
- 5% E2E tests (Python/Node.js bindings)

### Tolerant Parsing Validation

**Every parser test must verify bozo flag behavior:**

```rust
#[test]
fn test_malformed_sets_bozo() {
    let xml = b"<rss><channel><title>Test</channel></rss>"; // Missing </title>
    let feed = parse(xml).unwrap();

    assert!(feed.bozo, "Bozo flag should be set for malformed XML");
    assert!(feed.bozo_exception.is_some(), "Should have error description");
    assert_eq!(feed.feed.title.as_deref(), Some("Test"), "Should still parse partial data");
}
```

**Why?** Tolerant parsing is our core value proposition. Tests must verify we continue parsing after errors.

## Test Organization

### Unit Tests (In-Module)

**Located in:** Same file as implementation (`#[cfg(test)] mod tests`)

```rust
// crates/feedparser-rs-core/src/util/date.rs

pub fn parse_date(input: &str) -> Option<DateTime<Utc>> {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_rfc3339() {
        let date = parse_date("2024-01-15T12:00:00Z");
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.year(), 2024);
    }

    #[test]
    fn test_parse_date_rfc2822() {
        let date = parse_date("Mon, 15 Jan 2024 12:00:00 GMT");
        assert!(date.is_some());
    }

    #[test]
    fn test_parse_date_invalid_returns_none() {
        let date = parse_date("not a date");
        assert!(date.is_none(), "Invalid dates should return None, not panic");
    }
}
```

**Rules:**
1. Test happy path AND error cases
2. Verify None/Err returns (no panic)
3. Keep tests focused (one assertion per test when possible)

### Integration Tests

**Located in:** `crates/feedparser-rs-core/tests/*.rs`

```rust
// tests/integration_tests.rs

use feedparser_rs_core::{parse, FeedVersion};

#[test]
fn test_parse_rss20_full() {
    let xml = include_str!("fixtures/rss/full_example.xml");
    let feed = parse(xml.as_bytes()).unwrap();

    assert_eq!(feed.version, FeedVersion::Rss20);
    assert!(!feed.bozo);

    // Validate feed metadata
    assert_eq!(feed.feed.title.as_deref(), Some("Example Feed"));
    assert_eq!(feed.feed.link.as_deref(), Some("http://example.com"));

    // Validate entries
    assert_eq!(feed.entries.len(), 3);
    assert_eq!(feed.entries[0].title.as_deref(), Some("First Entry"));
}
```

**Use fixtures:** Store test feeds in `tests/fixtures/`

## Test Fixtures Organization

```
tests/fixtures/
├── rss/
│   ├── rss20_basic.xml           # Minimal valid RSS 2.0
│   ├── rss20_full.xml            # All optional fields
│   ├── rss20_podcast.xml         # iTunes namespace
│   └── malformed/
│       ├── missing_closing_tag.xml
│       ├── invalid_date.xml
│       └── unknown_namespace.xml
├── atom/
│   ├── atom10_basic.xml
│   ├── atom10_full.xml
│   └── malformed/
├── json/
│   ├── json11_basic.json
│   └── malformed/
└── real_world/                   # Actual feeds from production
    ├── github_commits.atom
    ├── nytimes_rss.xml
    └── podcast_serial.xml
```

### Fixture Loading Pattern

```rust
#[test]
fn test_rss20_basic() {
    let xml = include_str!("fixtures/rss/rss20_basic.xml");
    let feed = parse(xml.as_bytes()).unwrap();
    // Assertions...
}
```

**Why `include_str!`?**
- Embedded at compile time (no runtime file I/O)
- Tests work without filesystem dependencies
- Fast execution

## Test Naming Conventions

```rust
// Pattern: test_<function>_<scenario>_<expected_result>

#[test]
fn test_parse_rss20_valid_succeeds() { }

#[test]
fn test_parse_atom_malformed_sets_bozo() { }

#[test]
fn test_sanitize_html_removes_script_tags() { }

#[test]
fn test_parse_date_rfc3339_returns_datetime() { }

#[test]
fn test_parse_date_invalid_returns_none() { }
```

**Rules:**
1. Start with `test_`
2. Include function/module name
3. Describe scenario
4. State expected outcome

## Bozo Flag Testing Patterns

### Pattern 1: Malformed XML

```rust
#[test]
fn test_missing_closing_tag_sets_bozo() {
    let xml = b"<rss><channel><title>Test</channel></rss>";
    let feed = parse(xml).unwrap();

    assert!(feed.bozo);
    assert!(feed.bozo_exception.as_ref().unwrap().contains("missing"));
    assert_eq!(feed.feed.title.as_deref(), Some("Test"));
}
```

### Pattern 2: Invalid Data Type

```rust
#[test]
fn test_invalid_date_sets_bozo() {
    let xml = b"<rss version=\"2.0\"><channel>
        <pubDate>not a date</pubDate>
    </channel></rss>";
    let feed = parse(xml).unwrap();

    assert!(feed.bozo);
    assert!(feed.bozo_exception.as_ref().unwrap().contains("date"));
}
```

### Pattern 3: Limit Exceeded

```rust
#[test]
fn test_excessive_nesting_sets_bozo() {
    // Generate deeply nested XML (>100 levels)
    let xml = generate_deep_xml(150);
    let feed = parse(xml.as_bytes()).unwrap();

    assert!(feed.bozo);
    assert!(feed.bozo_exception.as_ref().unwrap().contains("nesting"));
}
```

## Namespace Testing

Test namespace handlers independently:

```rust
#[test]
fn test_itunes_category_parsing() {
    let xml = br#"<rss xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
        <channel>
            <itunes:category text="Technology">
                <itunes:category text="Software" />
            </itunes:category>
        </channel>
    </rss>"#;

    let feed = parse(xml).unwrap();
    let itunes = feed.feed.itunes.as_ref().unwrap();

    assert_eq!(itunes.categories.len(), 1);
    assert_eq!(itunes.categories[0].text, "Technology");
    assert_eq!(itunes.categories[0].subcategory.as_deref(), Some("Software"));
}
```

## Python Binding Tests

**Located in:** `crates/feedparser-rs-py/tests/*.py`

### pytest Setup

```python
# tests/test_basic.py

import pytest
import feedparser_rs

def test_parse_bytes():
    xml = b'<rss version="2.0"><channel><title>Test</title></channel></rss>'
    feed = feedparser_rs.parse(xml)
    assert feed.feed.title == "Test"

def test_parse_string():
    xml = '<rss version="2.0"><channel><title>Test</title></channel></rss>'
    feed = feedparser_rs.parse(xml)
    assert feed.feed.title == "Test"

def test_bozo_flag():
    xml = b'<rss><channel><title>Broken</channel></rss>'
    feed = feedparser_rs.parse(xml)
    assert feed.bozo is True
    assert feed.bozo_exception is not None
```

### Compatibility Tests with feedparser

```python
# tests/test_compatibility.py

import feedparser
import feedparser_rs
import pytest

@pytest.mark.parametrize("fixture", [
    "tests/fixtures/rss/rss20_basic.xml",
    "tests/fixtures/atom/atom10_basic.xml",
])
def test_compatibility_with_feedparser(fixture):
    with open(fixture) as f:
        xml = f.read()

    fp = feedparser.parse(xml)
    fprs = feedparser_rs.parse(xml)

    # Compare key fields
    assert fprs.version == fp.version
    assert fprs.feed.title == fp.feed.title
    assert len(fprs.entries) == len(fp.entries)

    if fprs.entries:
        assert fprs.entries[0].title == fp.entries[0].title
```

### time.struct_time Testing

```python
import time

def test_published_parsed_returns_struct_time():
    xml = b'''<rss version="2.0"><channel><item>
        <pubDate>Mon, 01 Jan 2024 12:00:00 GMT</pubDate>
    </item></channel></rss>'''

    feed = feedparser_rs.parse(xml)
    entry = feed.entries[0]

    # Verify it's a time.struct_time
    assert isinstance(entry.published_parsed, time.struct_time)

    # Verify values
    assert entry.published_parsed.tm_year == 2024
    assert entry.published_parsed.tm_mon == 1
    assert entry.published_parsed.tm_mday == 1
    assert entry.published_parsed.tm_hour == 12
```

## Benchmarking

**Located in:** `crates/feedparser-rs-core/benches/parsing.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use feedparser_rs_core::parse;

fn bench_rss20_small(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/rss/rss20_basic.xml");
    c.bench_function("rss20_small", |b| {
        b.iter(|| parse(black_box(xml.as_bytes())))
    });
}

fn bench_rss20_large(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/rss/rss20_1000_entries.xml");
    c.bench_function("rss20_large", |b| {
        b.iter(|| parse(black_box(xml.as_bytes())))
    });
}

criterion_group!(benches, bench_rss20_small, bench_rss20_large);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo make bench

# Run specific benchmark
cargo bench --bench parsing
```

## Edge Case Testing Checklist

### XML Edge Cases
- [ ] Empty elements: `<title></title>` vs `<title/>`
- [ ] CDATA sections: `<![CDATA[content]]>`
- [ ] HTML entities: `&lt;`, `&gt;`, `&amp;`
- [ ] Numeric entities: `&#x27;`, `&#39;`
- [ ] Nested elements (100+ levels)
- [ ] Very long text (>1MB)
- [ ] Mixed encodings
- [ ] BOM markers

### Date Edge Cases
- [ ] RFC 3339: `2024-01-15T12:00:00Z`
- [ ] RFC 2822: `Mon, 15 Jan 2024 12:00:00 GMT`
- [ ] No timezone: `2024-01-15T12:00:00`
- [ ] Invalid dates: `2024-13-45` (month 13, day 45)
- [ ] Empty date strings
- [ ] Whitespace-only dates

### Namespace Edge Cases
- [ ] Multiple namespaces
- [ ] Unknown namespaces
- [ ] Namespace prefix redefinition
- [ ] Default namespace overrides

### DoS Edge Cases
- [ ] Feeds >50MB (should reject)
- [ ] >10,000 entries (should truncate + set bozo)
- [ ] Deeply nested XML (>100 levels)
- [ ] Very long attribute values (>10KB)

## Test Coverage Goals

**Minimum coverage:** 80% line coverage

```bash
# Generate coverage report
cargo make coverage

# View HTML report
open target/llvm-cov/html/index.html
```

**Critical paths requiring 100% coverage:**
- `parser/detect.rs` (format detection)
- `http/validation.rs` (SSRF protection)
- `util/sanitize.rs` (XSS protection)
- `limits.rs` (DoS protection)

## Performance Testing

### cargo-nextest

Use nextest for faster test execution:

```bash
# Run tests with nextest
cargo nextest run

# Parallel execution (faster)
cargo nextest run --jobs 8
```

### Test Execution Time

**Target:** All tests should complete in <30 seconds

If tests are slow:
1. Use smaller fixtures for unit tests
2. Move large feed tests to integration tests
3. Use `#[ignore]` for very slow tests
4. Run slow tests separately: `cargo test -- --ignored`

## Common Pitfalls

### Don't Use .unwrap() in Tests

```rust
// ❌ BAD - Panic message unclear
#[test]
fn test_parse() {
    let feed = parse(xml).unwrap();
    assert_eq!(feed.feed.title.unwrap(), "Test");
}

// ✅ GOOD - Clear assertion messages
#[test]
fn test_parse() {
    let feed = parse(xml).expect("Failed to parse XML");
    assert_eq!(
        feed.feed.title.as_deref(),
        Some("Test"),
        "Feed title should be 'Test'"
    );
}
```

### Don't Forget Negative Tests

```rust
// ✅ GOOD - Test both success and failure
#[test]
fn test_parse_valid_succeeds() {
    let result = parse(valid_xml);
    assert!(result.is_ok());
}

#[test]
fn test_parse_invalid_sets_bozo() {
    let result = parse(invalid_xml);
    assert!(result.is_ok()); // Should still return Ok with bozo flag
    assert!(result.unwrap().bozo);
}
```

### Don't Skip Bozo Validation

```rust
// ❌ BAD - Only tests success case
#[test]
fn test_parse_entry() {
    let feed = parse(xml).unwrap();
    assert_eq!(feed.entries.len(), 1);
}

// ✅ GOOD - Tests tolerant parsing
#[test]
fn test_parse_entry_malformed() {
    let feed = parse(broken_xml).unwrap();
    assert!(feed.bozo, "Should set bozo flag for malformed XML");
    assert_eq!(feed.entries.len(), 1, "Should still parse partial entry");
}
```

## Documentation Tests

Use doc comments with examples:

```rust
/// Parses a date string in various formats.
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::util::date::parse_date;
///
/// let date = parse_date("2024-01-15T12:00:00Z");
/// assert!(date.is_some());
/// ```
///
/// Invalid dates return `None`:
///
/// ```
/// use feedparser_rs_core::util::date::parse_date;
///
/// let date = parse_date("not a date");
/// assert!(date.is_none());
/// ```
pub fn parse_date(input: &str) -> Option<DateTime<Utc>> {
    // Implementation
}
```

**Run doc tests:**
```bash
cargo test --doc
```

## References

- cargo-nextest: https://nexte.st/
- Criterion benchmarking: https://bheisler.github.io/criterion.rs/book/
- pytest documentation: https://docs.pytest.org/
- Python feedparser tests: https://github.com/kurtmckee/feedparser/tree/develop/tests
