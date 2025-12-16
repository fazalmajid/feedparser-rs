# Parser Module Instructions

**Applies to:** `crates/feedparser-rs-core/src/parser/**`

## Core Principles

### Tolerant Parsing is MANDATORY

**NEVER panic or return errors for malformed feeds.** The bozo pattern is the foundation of this project:

```rust
// ✅ CORRECT - Set bozo flag and continue
match reader.read_event_into(&mut buf) {
    Ok(Event::Start(e)) => { /* process */ }
    Err(e) => {
        feed.bozo = true;
        feed.bozo_exception = Some(e.to_string());
        // CONTINUE PARSING - don't return error
    }
    _ => {}
}

// ❌ WRONG - Never panic or abort parsing
match reader.read_event_into(&mut buf) {
    Ok(Event::Start(e)) => { /* process */ }
    Err(e) => return Err(e.into()), // NO! This breaks tolerance
    _ => {}
}
```

**Why?** Real-world feeds are often broken:
- Missing closing tags
- Invalid dates
- Malformed XML
- Wrong encoding declarations
- Mixed namespaces

Python feedparser handles all these gracefully. We must too.

## Function Length Rules

### CRITICAL: No function >100 lines

Current technical debt in `parser/rss.rs`:
- `parse_channel` - 280 lines (needs refactoring)
- `parse_item` - 328 lines (needs refactoring)

**When writing new parser code:**
1. Keep functions <50 lines (target)
2. Never exceed 100 lines (hard limit)
3. Extract inline parsing to separate functions

### Refactoring Pattern

```rust
// ✅ GOOD - Delegate to specialized functions
fn parse_channel(reader: &mut Reader<&[u8]>, feed: &mut ParsedFeed, limits: &ParserLimits, depth: &mut usize) -> Result<()> {
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e) | Event::Empty(e)) => {
                match e.name().as_ref() {
                    tag if is_standard_rss_tag(tag) =>
                        parse_channel_standard(tag, reader, &mut buf, feed, limits)?,
                    tag if is_itunes_tag_any(tag) =>
                        parse_channel_itunes(tag, &e, reader, &mut buf, feed, limits, depth)?,
                    tag if is_podcast_tag(tag) =>
                        parse_channel_podcast(tag, &e, reader, &mut buf, feed, limits)?,
                    _ => skip_element(reader, &mut buf, limits, *depth)?
                }
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"channel" => break,
            Err(e) => {
                feed.bozo = true;
                feed.bozo_exception = Some(e.to_string());
            }
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}

// Helper functions (<50 lines each)
fn parse_channel_standard(...) -> Result<()> { ... }
fn parse_channel_itunes(...) -> Result<bool> { ... }
fn parse_channel_podcast(...) -> Result<bool> { ... }
```

## quick-xml Usage Patterns

### Reader Configuration

```rust
let mut reader = Reader::from_reader(data);
reader.config_mut().trim_text(true);
// DO NOT enable check_end_names - we need tolerance for mismatched tags
```

### Event Loop Pattern

```rust
let mut buf = Vec::with_capacity(EVENT_BUFFER_CAPACITY); // Reuse buffer
let mut depth: usize = 1;

loop {
    match reader.read_event_into(&mut buf) {
        Ok(Event::Start(e)) => {
            depth += 1;
            check_depth(depth, limits.max_nesting_depth)?;
            // Process start tag
        }
        Ok(Event::End(e)) => {
            depth = depth.saturating_sub(1);
            // Check for terminating tag
            if e.local_name().as_ref() == b"channel" {
                break;
            }
        }
        Ok(Event::Text(e)) => {
            // Extract text content
            let text = e.unescape().unwrap_or_default();
        }
        Ok(Event::Empty(e)) => {
            // Self-closing tag (e.g., <link href="..." />)
        }
        Ok(Event::Eof) => break,
        Err(e) => {
            feed.bozo = true;
            feed.bozo_exception = Some(format!("XML error: {e}"));
            // Continue parsing if possible
        }
        _ => {} // Ignore other events (comments, PI, etc.)
    }
    buf.clear(); // Reuse buffer allocation
}
```

## Format-Specific Rules

### RSS Parsers (`rss.rs`, `rss10.rs`)

1. **Version Detection**: RSS 0.9x, 1.0, 2.0 have different structures
   - RSS 2.0: `<rss version="2.0"><channel>...</channel></rss>`
   - RSS 1.0: `<rdf:RDF><channel>...</channel><item>...</item></rdf:RDF>` (items outside channel)
   - RSS 0.9x: No version attribute

2. **Date Formats**: Use RFC 2822 parser first, fallback to others
   ```rust
   let dt = DateTime::parse_from_rfc2822(date_str)
       .ok()
       .or_else(|| DateTime::parse_from_rfc3339(date_str).ok())
       .map(|d| d.with_timezone(&Utc));
   ```

3. **Namespace Handling**: Extract iTunes, Dublin Core, Media RSS attributes
   - Use `is_itunes_tag()`, `is_dc_tag()`, etc. helpers
   - Delegate to namespace-specific parsers

### Atom Parser (`atom.rs`)

1. **Text Constructs**: Atom has three content types
   ```rust
   fn parse_text_construct(element: &BytesStart, reader: &mut Reader, buf: &mut Vec<u8>) -> TextConstruct {
       let content_type = element.attributes()
           .find(|a| a.key.as_ref() == b"type")
           .and_then(|a| a.unescape_value().ok())
           .map(|v| match v.as_ref() {
               "html" => TextType::Html,
               "xhtml" => TextType::Xhtml,
               _ => TextType::Text,
           })
           .unwrap_or(TextType::Text);
       // ...
   }
   ```

2. **Links**: Atom supports multiple link relations
   - `rel="alternate"` → main link
   - `rel="enclosure"` → media attachments
   - `rel="self"` → feed URL

3. **Dates**: Use ISO 8601/RFC 3339 parser first

### JSON Feed Parser (`json.rs`)

1. **Use serde_json**: Already structured, no XML complexity
2. **Version Detection**: Check `version` field ("https://jsonfeed.org/version/1", "https://jsonfeed.org/version/1.1")
3. **Bozo Pattern**: Set bozo for missing required fields (title, items)

## Depth Checking (DoS Protection)

Always check nesting depth to prevent stack overflow:

```rust
fn check_depth(current: usize, max: usize) -> Result<()> {
    if current > max {
        return Err(FeedError::InvalidFormat(format!(
            "XML nesting depth {current} exceeds maximum {max}"
        )));
    }
    Ok(())
}
```

## Namespace Detection

Use helpers from `parser/namespace_detection.rs`:

```rust
if let Some(itunes_element) = is_itunes_tag(tag) {
    // tag is b"itunes:author" or similar
    let itunes = feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
    // Process iTunes-specific field
}

if let Some(dc_element) = is_dc_tag(tag) {
    // Dublin Core namespace
    dublin_core::handle_feed_element(&dc_element, &text, &mut feed.feed);
}
```

## Text Extraction Pattern

```rust
fn read_text(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>, limits: &ParserLimits) -> Result<String> {
    let mut text = String::with_capacity(TEXT_BUFFER_CAPACITY);

    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Text(e)) => {
                append_bytes(&mut text, e.as_ref(), limits.max_text_length)?;
            }
            Ok(Event::CData(e)) => {
                append_bytes(&mut text, e.as_ref(), limits.max_text_length)?;
            }
            Ok(Event::End(_) | Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(text)
}
```

## Date Parsing Delegation

Never inline date parsing. Use `util/date.rs`:

```rust
use crate::util::date::parse_date;

// ✅ CORRECT
match parse_date(&text) {
    Some(dt) => entry.published = Some(dt),
    None if !text.is_empty() => {
        feed.bozo = true;
        feed.bozo_exception = Some("Invalid date format".to_string());
    }
    None => {} // Empty text, no error
}

// ❌ WRONG - Inline date parsing duplicates logic
let dt = DateTime::parse_from_rfc3339(&text).ok(); // Misses other formats
```

## Testing Requirements

Every parser function must have:
1. **Basic test**: Well-formed feed
2. **Malformed test**: Broken feed sets bozo but still parses
3. **Edge case tests**: Empty fields, missing required fields, excessive nesting

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rss20_valid() {
        let xml = include_str!("../../../tests/fixtures/rss/basic.xml");
        let feed = parse_rss20(xml.as_bytes()).unwrap();
        assert!(!feed.bozo);
        assert_eq!(feed.version, FeedVersion::Rss20);
    }

    #[test]
    fn test_rss20_malformed_sets_bozo() {
        let xml = b"<rss><channel><title>Test</channel></rss>"; // Missing </title>
        let feed = parse_rss20(xml).unwrap();
        assert!(feed.bozo);
        assert!(feed.bozo_exception.is_some());
        assert_eq!(feed.feed.title.as_deref(), Some("Test")); // Still extracted!
    }

    #[test]
    fn test_rss20_excessive_nesting() {
        let xml = b"<rss><channel><item><nested1><nested2>..."; // 100+ levels
        let result = parse_rss20(xml);
        assert!(result.is_err() || result.unwrap().bozo);
    }
}
```

## Performance Considerations

1. **Reuse buffers**: `Vec::with_capacity()` + `clear()`, not repeated allocations
2. **Avoid clones in hot paths**: Use references where possible
3. **Bounded collections**: Apply limits via `try_push_limited()` helper
4. **Early termination**: Stop parsing after `max_entries` reached (set bozo flag)

```rust
// ✅ GOOD - Reuse buffer
let mut buf = Vec::with_capacity(EVENT_BUFFER_CAPACITY);
loop {
    reader.read_event_into(&mut buf)?;
    // Process
    buf.clear(); // Reuse allocation
}

// ❌ BAD - Allocate every iteration
loop {
    let mut buf = Vec::new(); // New heap allocation each time
    reader.read_event_into(&mut buf)?;
}
```

## Common Pitfalls

### Don't Skip Elements Without Checking Depth

```rust
// ✅ CORRECT
skip_element(reader, buf, limits, depth)?;

// ❌ WRONG - Depth not checked, could overflow stack
loop {
    match reader.read_event_into(buf) {
        Ok(Event::End(_)) => break,
        _ => {}
    }
}
```

### Don't Use Panic-Happy Methods

```rust
// ❌ WRONG
let value = attributes.find(|a| a.key.as_ref() == b"href").unwrap();

// ✅ CORRECT
if let Some(attr) = attributes.find(|a| a.key.as_ref() == b"href") {
    if let Ok(value) = attr.unescape_value() {
        // Use value
    }
}
```

### Don't Ignore Limits

```rust
// ❌ WRONG - Unbounded growth
feed.entries.push(entry);

// ✅ CORRECT - Bounded with error handling
if feed.entries.is_at_limit(limits.max_entries) {
    feed.bozo = true;
    feed.bozo_exception = Some(format!("Entry limit exceeded: {}", limits.max_entries));
    skip_element(reader, buf, limits, depth)?;
} else {
    feed.entries.push(entry);
}
```

## References

- RSS 2.0 Spec: https://www.rssboard.org/rss-specification
- RSS 1.0 Spec: https://web.resource.org/rss/1.0/spec
- Atom 1.0 (RFC 4287): https://www.rfc-editor.org/rfc/rfc4287
- JSON Feed: https://www.jsonfeed.org/version/1.1/
- quick-xml docs: https://docs.rs/quick-xml/latest/quick_xml/
