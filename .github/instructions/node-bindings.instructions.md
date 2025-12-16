---
applyTo: "crates/feedparser-rs-node/**"
---

# Node.js Bindings Code Review Instructions

This file contains specific code review rules for the Node.js bindings in `crates/feedparser-rs-node/`.

## Overview

The Node.js bindings use **napi-rs** to expose the Rust core parser to JavaScript/TypeScript. The bindings must provide an ergonomic JavaScript API while maintaining security and performance.

## Critical Rules

### 1. Input Validation (CWE-770)

**ALWAYS validate input size BEFORE processing to prevent DoS attacks.**

```rust
// CORRECT: Validate size before processing
#[napi]
pub fn parse(source: Either<Buffer, String>) -> Result<ParsedFeed> {
    let input_len = match &source {
        Either::A(buf) => buf.len(),
        Either::B(s) => s.len(),
    };

    if input_len > MAX_FEED_SIZE {
        return Err(Error::from_reason(format!(
            "Feed size ({} bytes) exceeds maximum allowed ({} bytes)",
            input_len, MAX_FEED_SIZE
        )));
    }

    // Now safe to process
    let bytes: &[u8] = match &source {
        Either::A(buf) => buf.as_ref(),
        Either::B(s) => s.as_bytes(),
    };
    // ...
}
```

```rust
// WRONG: No size validation
#[napi]
pub fn parse(source: Either<Buffer, String>) -> Result<ParsedFeed> {
    let bytes: &[u8] = match &source {
        Either::A(buf) => buf.as_ref(),
        Either::B(s) => s.as_bytes(),
    };
    // ... immediate processing without size check
}
```

### 2. Error Handling

**Use `Error::from_reason()` for user-facing errors with clear messages.**

```rust
// CORRECT: Clear error message with context
.map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;

// WRONG: Generic error
.map_err(|e| Error::from_reason(e.to_string()))?;
```

**Never expose internal error details that could aid attackers:**

```rust
// CORRECT: Safe error message
return Err(Error::from_reason("Feed size exceeds maximum allowed"));

// WRONG: Exposes internal details
return Err(Error::from_reason(format!("Internal buffer at {:p}", ptr)));
```

### 3. NAPI Struct Definitions

**Use `#[napi(object)]` for plain data objects:**

```rust
// CORRECT: Plain data object
#[napi(object)]
pub struct ParsedFeed {
    pub feed: FeedMeta,
    pub entries: Vec<Entry>,
    pub bozo: bool,
    // ...
}
```

**Use `#[napi(js_name = "...")]` for JavaScript naming conventions:**

```rust
// CORRECT: Use js_name for JavaScript conventions
#[napi(object)]
pub struct Link {
    pub href: String,
    #[napi(js_name = "type")]  // 'type' is reserved in JS, use js_name
    pub link_type: Option<String>,
}
```

**Reserved JavaScript keywords that need `js_name`:**
- `type` -> use field name like `link_type`, `content_type`, `enclosure_type`
- `class`, `function`, `var`, `let`, `const` (if ever needed)

### 4. Date/Time Handling

**Convert DateTime to milliseconds since epoch for JavaScript compatibility:**

```rust
// CORRECT: Milliseconds for JavaScript Date compatibility
pub updated: Option<i64>,

impl From<CoreFeedMeta> for FeedMeta {
    fn from(core: CoreFeedMeta) -> Self {
        Self {
            updated: core.updated.map(|dt| dt.timestamp_millis()),
            // ...
        }
    }
}
```

```rust
// WRONG: Seconds (JavaScript Date uses milliseconds)
pub updated: Option<i64>,
updated: core.updated.map(|dt| dt.timestamp()),
```

### 5. Type Conversions

**Handle potential overflow in u64 to i64 conversions:**

```rust
// CORRECT: Safe conversion with fallback
pub length: Option<i64>,
length: core.length.map(|l| i64::try_from(l).unwrap_or(i64::MAX)),

// WRONG: Unchecked cast
length: core.length.map(|l| l as i64),
```

### 6. Input Type Handling

**Accept both Buffer and String using `Either`:**

```rust
// CORRECT: Accept both types
#[napi]
pub fn parse(source: Either<Buffer, String>) -> Result<ParsedFeed> {
    let bytes: &[u8] = match &source {
        Either::A(buf) => buf.as_ref(),
        Either::B(s) => s.as_bytes(),
    };
    // ...
}
```

### 7. Feature Flags

**Use conditional compilation for optional features:**

```rust
// CORRECT: Feature-gated HTTP functionality
#[cfg(feature = "http")]
#[napi]
pub fn parse_url(url: String, ...) -> Result<ParsedFeed> {
    // ...
}

// CORRECT: Feature-gated fields
#[napi(object)]
pub struct ParsedFeed {
    #[cfg(feature = "http")]
    pub headers: Option<HashMap<String, String>>,
}
```

### 8. Vector Pre-allocation

**Pre-allocate vectors when converting collections:**

```rust
// CORRECT: Pre-allocate for better performance
impl From<CoreEntry> for Entry {
    fn from(core: CoreEntry) -> Self {
        let links_cap = core.links.len();
        let content_cap = core.content.len();

        Self {
            links: {
                let mut v = Vec::with_capacity(links_cap);
                v.extend(core.links.into_iter().map(Link::from));
                v
            },
            content: {
                let mut v = Vec::with_capacity(content_cap);
                v.extend(core.content.into_iter().map(Content::from));
                v
            },
            // ...
        }
    }
}
```

```rust
// ACCEPTABLE for small collections: Direct collect
links: core.links.into_iter().map(Link::from).collect(),
```

### 9. Documentation

**All public functions must have JSDoc-compatible documentation:**

```rust
// CORRECT: Comprehensive documentation
/// Parse an RSS/Atom/JSON Feed from bytes or string
///
/// # Arguments
///
/// * `source` - Feed content as Buffer, string, or Uint8Array
///
/// # Returns
///
/// Parsed feed result with metadata and entries
///
/// # Errors
///
/// Returns error if input exceeds size limit or parsing fails catastrophically
#[napi]
pub fn parse(source: Either<Buffer, String>) -> Result<ParsedFeed> {
```

**Include JavaScript examples in documentation:**

```rust
/// # Examples
///
/// ```javascript
/// const feedparser = require('feedparser-rs');
///
/// const feed = await feedparser.parseUrl("https://example.com/feed.xml");
/// console.log(feed.feed.title);
/// ```
```

### 10. Struct Field Documentation

**Document all fields for TypeScript type generation:**

```rust
#[napi(object)]
pub struct Entry {
    /// Unique entry identifier
    pub id: Option<String>,
    /// Entry title
    pub title: Option<String>,
    /// Publication date (milliseconds since epoch)
    pub published: Option<i64>,
    // ...
}
```

## API Conventions

### Function Naming

Use camelCase for JavaScript function names (napi-rs does this automatically):
- Rust: `parse_with_options` -> JS: `parseWithOptions`
- Rust: `detect_format` -> JS: `detectFormat`
- Rust: `parse_url` -> JS: `parseUrl`

### Optional Parameters

Use `Option<T>` for optional parameters:

```rust
#[napi]
pub fn parse_url(
    url: String,
    etag: Option<String>,
    modified: Option<String>,
    user_agent: Option<String>,
) -> Result<ParsedFeed>
```

### Return Types

- Return `Result<T>` for operations that can fail
- Return `T` directly for infallible operations
- Never panic in public functions

## Security Requirements

### 1. URL Validation

For HTTP functions, validate URL schemes:

```rust
// The core crate handles SSRF protection, but document it
/// # Security
///
/// - Only HTTP and HTTPS URLs are accepted
/// - Private IP addresses and localhost are blocked
/// - Redirects follow the same security rules
```

### 2. Size Limits

Always enforce size limits:

```rust
const DEFAULT_MAX_FEED_SIZE: usize = 100 * 1024 * 1024; // 100 MB

#[napi]
pub fn parse_with_options(
    source: Either<Buffer, String>,
    max_size: Option<u32>,  // Allow user to customize
) -> Result<ParsedFeed>
```

### 3. No Unsafe Code

The Node.js bindings should NOT contain any `unsafe` code. All unsafe operations should be in the core crate if necessary.

## Testing Requirements

### 1. Test Both Input Types

```javascript
// Test with Buffer
const buf = Buffer.from('<rss>...</rss>');
const feed1 = feedparser.parse(buf);

// Test with String
const str = '<rss>...</rss>';
const feed2 = feedparser.parse(str);
```

### 2. Test Size Limits

```javascript
const largeFeed = 'x'.repeat(200 * 1024 * 1024);
expect(() => feedparser.parse(largeFeed)).toThrow(/exceeds maximum/);
```

### 3. Test Feature Flags

Verify HTTP functions are only available when the `http` feature is enabled.

## Common Review Issues

### Issue: Missing size validation
**Fix:** Add input length check before processing

### Issue: Using `unwrap()` in public functions
**Fix:** Use `map_err()` with `Error::from_reason()`

### Issue: Date as seconds instead of milliseconds
**Fix:** Use `timestamp_millis()` not `timestamp()`

### Issue: Unsafe i64 cast
**Fix:** Use `i64::try_from(l).unwrap_or(i64::MAX)`

### Issue: Missing documentation
**Fix:** Add `///` documentation with examples

### Issue: Missing `js_name` for reserved keywords
**Fix:** Add `#[napi(js_name = "type")]` for fields named `type`

## Integration with Core Crate

The Node.js bindings are a thin wrapper around `feedparser-rs-core`. Rules:

1. **No parsing logic** - All parsing is in the core crate
2. **Type conversions only** - Convert core types to napi-compatible types
3. **Error mapping** - Map `FeedError` to napi `Error`
4. **Feature parity** - Keep in sync with Python bindings features

## Checklist for PRs

- [ ] Input size validated before processing
- [ ] All public functions have documentation
- [ ] No `unwrap()` or `expect()` in public functions
- [ ] Dates converted to milliseconds
- [ ] Large number conversions are safe (u64 -> i64)
- [ ] Reserved keywords use `js_name`
- [ ] Feature flags properly applied
- [ ] No unsafe code
- [ ] Tests cover both Buffer and String inputs
