# Type Definitions Instructions

**Applies to:** `crates/feedparser-rs-core/src/types/**`

## Core Principles

### API Compatibility is CRITICAL

All type definitions MUST match Python feedparser's field names and behavior exactly. This is a drop-in replacement, not a new API.

**Python feedparser field names → Rust field names (MUST BE IDENTICAL)**

```python
# Python feedparser
d.feed.title            # NOT d.feed.name or d.feed.heading
d.feed.link             # NOT d.feed.url
d.entries[0].summary    # NOT d.entries[0].description
d.entries[0].published_parsed  # NOT d.entries[0].publication_date
```

**Always verify against Python feedparser documentation**: https://feedparser.readthedocs.io/

### Design Philosophy

1. **Tolerant by Design**: All fields are `Option<T>` or `Vec<T>` (feeds often have missing data)
2. **Flat Structure**: Avoid deep nesting (matches Python feedparser's dict-like access)
3. **Value Types**: Use `String`, not `&str` (owned data, no lifetime complexity)
4. **Standard Types**: Use `chrono::DateTime<Utc>` for dates, not custom types

## Type Categories

### Core Feed Types

**Located in:** `types/mod.rs`

```rust
/// Main parsing result (equivalent to Python's FeedParserDict)
pub struct ParsedFeed {
    pub feed: FeedMeta,              // Feed-level metadata
    pub entries: Vec<Entry>,          // Items/entries
    pub bozo: bool,                   // Error flag (CRITICAL)
    pub bozo_exception: Option<String>, // Error description
    pub encoding: String,             // Character encoding
    pub version: FeedVersion,         // Format version
    pub namespaces: HashMap<String, String>, // XML namespaces
}
```

**Rules:**
- NEVER remove `bozo` or `bozo_exception` fields (core to tolerant parsing)
- NEVER rename fields without checking Python feedparser compatibility
- ALL fields must be public (accessed from Python/Node.js bindings)

### Feed Metadata Types

**Located in:** `types/mod.rs`

```rust
pub struct FeedMeta {
    // Basic metadata
    pub title: Option<String>,
    pub title_detail: Option<TextConstruct>,  // Type info (text/html/xhtml)
    pub link: Option<String>,                 // Primary link
    pub links: Vec<Link>,                     // All links
    pub subtitle: Option<String>,
    pub subtitle_detail: Option<TextConstruct>,

    // Dates
    pub updated: Option<DateTime<Utc>>,

    // People
    pub author: Option<String>,               // Primary author name
    pub author_detail: Option<Person>,        // Full author info
    pub authors: Vec<Person>,                 // All authors
    pub contributors: Vec<Person>,
    pub publisher: Option<String>,
    pub publisher_detail: Option<Person>,

    // Classification
    pub tags: Vec<Tag>,
    pub language: Option<String>,
    pub rights: Option<String>,
    pub rights_detail: Option<TextConstruct>,

    // Technical
    pub generator: Option<String>,
    pub generator_detail: Option<Generator>,
    pub id: Option<String>,
    pub ttl: Option<u32>,

    // Images
    pub image: Option<Image>,
    pub icon: Option<String>,
    pub logo: Option<String>,

    // Namespace extensions
    pub itunes: Option<ItunesFeedMeta>,      // iTunes podcast metadata
    pub podcast: Option<PodcastMeta>,        // Podcast 2.0 namespace
}
```

**Rules:**
- Use `Option<String>` for optional text fields (NOT `Option<&str>`)
- Use `Vec<T>` for collections (NOT arrays or fixed-size)
- Namespace extensions go in separate fields (`itunes`, `podcast`, etc.)

### Entry Types

**Located in:** `types/mod.rs`

```rust
pub struct Entry {
    pub id: Option<String>,
    pub title: Option<String>,
    pub title_detail: Option<TextConstruct>,
    pub link: Option<String>,
    pub links: Vec<Link>,
    pub summary: Option<String>,
    pub summary_detail: Option<TextConstruct>,
    pub content: Vec<Content>,            // Multiple content blocks

    // Dates
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub created: Option<DateTime<Utc>>,   // Dublin Core
    pub expired: Option<DateTime<Utc>>,   // Rarely used

    // People
    pub author: Option<String>,
    pub author_detail: Option<Person>,
    pub authors: Vec<Person>,
    pub contributors: Vec<Person>,
    pub publisher: Option<String>,
    pub publisher_detail: Option<Person>,

    // Media
    pub enclosures: Vec<Enclosure>,       // Audio/video attachments
    pub tags: Vec<Tag>,
    pub comments: Option<String>,
    pub source: Option<Source>,

    // Namespace extensions
    pub itunes: Option<ItunesEntryMeta>,
    pub podcast: Option<PodcastEntryMeta>,
}
```

### Common Data Types

**Located in:** `types/common.rs`

```rust
/// Text with type information (Atom text constructs)
pub struct TextConstruct {
    pub value: String,
    pub content_type: TextType,  // text, html, xhtml
    pub language: Option<String>,
    pub base: Option<String>,    // xml:base attribute
}

pub enum TextType {
    Text,    // Plain text
    Html,    // HTML (needs sanitization)
    Xhtml,   // XHTML (structured)
}

/// Link with relationship and metadata
pub struct Link {
    pub href: String,
    pub rel: Option<String>,        // "alternate", "enclosure", "self", etc.
    pub link_type: Option<String>,  // MIME type (e.g., "text/html")
    pub title: Option<String>,
    pub length: Option<u64>,        // Size in bytes
    pub hreflang: Option<String>,   // Language
}

/// Person (author, contributor, etc.)
pub struct Person {
    pub name: Option<String>,
    pub email: Option<String>,
    pub uri: Option<String>,
}

/// Tag/category
pub struct Tag {
    pub term: String,               // Tag name (required)
    pub scheme: Option<String>,     // Taxonomy URL
    pub label: Option<String>,      // Display name
}

/// Media enclosure (podcast episode, video, etc.)
pub struct Enclosure {
    pub url: String,
    pub length: Option<u64>,
    pub enclosure_type: Option<String>, // MIME type
}
```

### Version Enum

**Located in:** `types/version.rs`

**CRITICAL**: `as_str()` must return Python feedparser-compatible strings

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeedVersion {
    Rss090,
    Rss091,
    Rss092,
    Rss10,
    Rss20,
    Atom03,
    Atom10,
    JsonFeed10,
    JsonFeed11,
    Unknown,
}

impl FeedVersion {
    /// Returns feedparser-compatible version string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rss090 => "rss090",      // NOT "RSS 0.90"
            Self::Rss091 => "rss091",      // NOT "RSS 0.91"
            Self::Rss092 => "rss092",
            Self::Rss10 => "rss10",        // NOT "rss1.0" or "RSS 1.0"
            Self::Rss20 => "rss20",        // NOT "rss2.0" or "RSS 2.0"
            Self::Atom03 => "atom03",
            Self::Atom10 => "atom10",      // NOT "atom" or "Atom 1.0"
            Self::JsonFeed10 => "json10",
            Self::JsonFeed11 => "json11",
            Self::Unknown => "",           // Empty string, not "unknown"
        }
    }
}
```

## Namespace Extension Types

### iTunes Podcast Metadata

**Located in:** `types/podcast.rs`

```rust
pub struct ItunesFeedMeta {
    pub author: Option<String>,
    pub owner: Option<ItunesOwner>,
    pub image: Option<String>,        // URL
    pub categories: Vec<ItunesCategory>,
    pub explicit: Option<bool>,
    pub podcast_type: Option<String>, // "episodic", "serial"
    pub keywords: Vec<String>,
    pub complete: Option<bool>,
    pub new_feed_url: Option<String>,
}

pub struct ItunesCategory {
    pub text: String,
    pub subcategory: Option<String>,
}

pub struct ItunesEntryMeta {
    pub title: Option<String>,
    pub episode: Option<u32>,
    pub season: Option<u32>,
    pub episode_type: Option<String>, // "full", "trailer", "bonus"
    pub duration: Option<u32>,        // Seconds
    pub image: Option<String>,
    pub explicit: Option<bool>,
}
```

### Podcast 2.0 Namespace

**Located in:** `types/podcast.rs`

```rust
pub struct PodcastMeta {
    pub guid: Option<String>,
    pub locked: Option<bool>,
    pub funding: Vec<PodcastFunding>,
    pub value: Vec<PodcastValue>,
}

pub struct PodcastEntryMeta {
    pub transcript: Vec<PodcastTranscript>,
    pub chapters: Option<PodcastChapters>,
    pub soundbite: Vec<PodcastSoundbite>,
    pub person: Vec<PodcastPerson>,
}

pub struct PodcastTranscript {
    pub url: String,
    pub transcript_type: Option<String>,  // "text/plain", "text/html", etc.
    pub language: Option<String>,
    pub rel: Option<String>,
}
```

## Design Patterns

### Option Unwrapping Helpers

Provide helper methods to avoid verbose unwrapping in parsers:

```rust
impl FeedMeta {
    /// Set title with text construct
    pub fn set_title(&mut self, text: TextConstruct) {
        self.title = Some(text.value.clone());
        self.title_detail = Some(text);
    }

    /// Set alternate link (first alternate link found)
    pub fn set_alternate_link(&mut self, href: String, limit: usize) {
        if self.link.is_none() {
            self.link = Some(href.clone());
        }
        self.links.try_push_limited(Link::alternate(href), limit);
    }
}
```

### Constructor Helpers

Provide constructors for common patterns:

```rust
impl Link {
    pub fn alternate(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            rel: Some("alternate".to_string()),
            link_type: None,
            title: None,
            length: None,
            hreflang: None,
        }
    }

    pub fn enclosure(href: impl Into<String>, mime_type: Option<String>) -> Self {
        Self {
            href: href.into(),
            rel: Some("enclosure".to_string()),
            link_type: mime_type,
            title: None,
            length: None,
            hreflang: None,
        }
    }
}

impl TextConstruct {
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            content_type: TextType::Text,
            language: None,
            base: None,
        }
    }

    pub fn html(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            content_type: TextType::Html,
            language: None,
            base: None,
        }
    }
}
```

## Bounded Collections (DoS Protection)

**Located in:** `types/generics.rs`

Provide extension trait for bounded growth:

```rust
pub trait BoundedVec<T> {
    fn try_push_limited(&mut self, item: T, limit: usize) -> Result<(), ()>;
    fn is_at_limit(&self, limit: usize) -> bool;
}

impl<T> BoundedVec<T> for Vec<T> {
    fn try_push_limited(&mut self, item: T, limit: usize) -> Result<(), ()> {
        if self.len() >= limit {
            Err(())
        } else {
            self.push(item);
            Ok(())
        }
    }

    fn is_at_limit(&self, limit: usize) -> bool {
        self.len() >= limit
    }
}
```

**Usage in parsers:**

```rust
// ✅ CORRECT - Bounded growth
if feed.entries.try_push_limited(entry, limits.max_entries).is_err() {
    feed.bozo = true;
    feed.bozo_exception = Some(format!("Entry limit exceeded: {}", limits.max_entries));
}

// ❌ WRONG - Unbounded growth (DoS risk)
feed.entries.push(entry);
```

## Serialization Support

All types should derive `serde::Serialize` for debugging and export:

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFeed {
    pub feed: FeedMeta,
    pub entries: Vec<Entry>,
    // ...
}
```

**But NOT** `Deserialize` for main types (we don't deserialize from JSON into ParsedFeed).

## Documentation Requirements

### Every Public Type Must Have Doc Comments

```rust
/// Represents a parsed RSS/Atom feed with metadata and entries.
///
/// This is the main result type returned by `parse()`. It corresponds to
/// Python feedparser's `FeedParserDict`.
///
/// # Fields
///
/// * `bozo` - Set to `true` if parsing encountered errors but continued
/// * `bozo_exception` - Description of the parsing error, if any
/// * `version` - Detected feed format (e.g., "rss20", "atom10")
pub struct ParsedFeed {
    // ...
}
```

### Explain Field Semantics

```rust
/// Link relationship type.
///
/// Common values:
/// * `"alternate"` - Main content link (HTML page)
/// * `"enclosure"` - Media attachment (podcast episode, video)
/// * `"self"` - Feed's canonical URL
/// * `"related"` - Related resource
pub rel: Option<String>,
```

## Testing Requirements

### Provide Test Fixtures

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_feed_default() {
        let feed = ParsedFeed {
            feed: FeedMeta::default(),
            entries: vec![],
            bozo: false,
            bozo_exception: None,
            encoding: "utf-8".to_string(),
            version: FeedVersion::Unknown,
            namespaces: HashMap::new(),
        };

        assert!(!feed.bozo);
        assert_eq!(feed.version.as_str(), "");
    }

    #[test]
    fn test_version_strings_match_feedparser() {
        assert_eq!(FeedVersion::Rss20.as_str(), "rss20");
        assert_eq!(FeedVersion::Atom10.as_str(), "atom10");
        assert_eq!(FeedVersion::Unknown.as_str(), "");
    }
}
```

## Common Pitfalls

### Don't Break API Compatibility

```rust
// ❌ WRONG - Field name doesn't match Python feedparser
pub struct Entry {
    pub description: Option<String>,  // Should be "summary"
}

// ✅ CORRECT
pub struct Entry {
    pub summary: Option<String>,  // Matches feedparser
}
```

### Don't Use References in Struct Fields

```rust
// ❌ WRONG - Lifetime complexity, doesn't work with Python bindings
pub struct FeedMeta<'a> {
    pub title: Option<&'a str>,
}

// ✅ CORRECT - Owned data
pub struct FeedMeta {
    pub title: Option<String>,
}
```

### Don't Use Custom Date Types

```rust
// ❌ WRONG - Custom type not compatible with chrono ecosystem
pub struct CustomDateTime {
    year: i32,
    month: u32,
    // ...
}

// ✅ CORRECT - Use chrono
use chrono::{DateTime, Utc};
pub published: Option<DateTime<Utc>>,
```

### Don't Make Fields Private

```rust
// ❌ WRONG - Python bindings need access
pub struct FeedMeta {
    title: Option<String>,  // Private
}

// ✅ CORRECT
pub struct FeedMeta {
    pub title: Option<String>,  // Public
}
```

## References

- Python feedparser field names: https://feedparser.readthedocs.io/en/latest/reference.html
- Atom text constructs (RFC 4287 §3.1): https://www.rfc-editor.org/rfc/rfc4287#section-3.1
- RSS 2.0 element list: https://www.rssboard.org/rss-specification
- iTunes podcast tags: https://help.apple.com/itc/podcasts_connect/#/itcb54353390
- Podcast 2.0 namespace: https://github.com/Podcastindex-org/podcast-namespace
