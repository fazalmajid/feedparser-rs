# feedparser-rs-core

High-performance RSS/Atom/JSON Feed parser written in Rust.

This is the core parsing library that powers the Python and Node.js bindings.

## Features

- **Multi-format**: RSS 0.9x/1.0/2.0, Atom 0.3/1.0, JSON Feed 1.0/1.1
- **Tolerant**: Bozo flag for malformed feeds (like Python feedparser)
- **Fast**: Written in Rust, 10-100x faster than Python feedparser
- **Safe**: No unsafe code, comprehensive error handling
- **Well-tested**: Extensive test coverage with real-world feed fixtures

## Installation

```toml
[dependencies]
feedparser-rs-core = "0.1"
```

## Quick Start

```rust
use feedparser_rs_core::parse;

let xml = r#"
    <?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <title>My Blog</title>
            <item>
                <title>Hello World</title>
                <link>https://example.com/1</link>
            </item>
        </channel>
    </rss>
"#;

let feed = parse(xml.as_bytes())?;
assert_eq!(feed.feed.title.as_deref(), Some("My Blog"));
assert_eq!(feed.entries.len(), 1);
# Ok::<(), feedparser_rs_core::FeedError>(())
```

## Platform Bindings

- **Node.js**: [`feedparser-rs`](https://www.npmjs.com/package/feedparser-rs) on npm
- **Python**: [`feedparser-rs`](https://pypi.org/project/feedparser-rs/) on PyPI (coming soon)

## Performance

Benchmarks show 10-100x speedup compared to Python feedparser:

| Feed Size | feedparser-rs | feedparser (Python) | Speedup |
|-----------|---------------|---------------------|---------|
| 3 KB      | 0.05 ms       | 5 ms                | 100x    |
| 24 KB     | 0.5 ms        | 50 ms               | 100x    |
| 237 KB    | 5 ms          | 500 ms              | 100x    |

See [benchmarks/](../../benchmarks/) for detailed benchmark code.

## API Documentation

For full API documentation, see [docs.rs/feedparser-rs-core](https://docs.rs/feedparser-rs-core).

## Error Handling

The library uses a "bozo" flag (like feedparser) to indicate parsing errors while still returning partial results:

```rust
use feedparser_rs_core::parse;

let malformed = b"<rss><channel><title>Broken</title></rss>";
let feed = parse(malformed)?;

assert!(feed.bozo);
assert!(feed.bozo_exception.is_some());
// Still can access parsed data
assert_eq!(feed.feed.title.as_deref(), Some("Broken"));
# Ok::<(), feedparser_rs_core::FeedError>(())
```

## Parser Limits

To prevent resource exhaustion, the parser enforces limits:

```rust
use feedparser_rs_core::{parse_with_limits, ParserLimits};

let limits = ParserLimits {
    max_entries: 100,
    max_nesting_depth: 20,
    ..Default::default()
};

let feed = parse_with_limits(xml.as_bytes(), limits)?;
# Ok::<(), feedparser_rs_core::FeedError>(())
```

## License

MIT OR Apache-2.0

## Links

- [GitHub](https://github.com/bug-ops/feedparser-rs)
- [Documentation](https://docs.rs/feedparser-rs-core)
- [Changelog](../../CHANGELOG.md)
