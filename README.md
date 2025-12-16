# feedparser-rs

[![Crates.io](https://img.shields.io/crates/v/feedparser-rs-core)](https://crates.io/crates/feedparser-rs-core)
[![docs.rs](https://img.shields.io/docsrs/feedparser-rs-core)](https://docs.rs/feedparser-rs-core)
[![CI](https://img.shields.io/github/actions/workflow/status/bug-ops/feedparser-rs/ci.yml?branch=main)](https://github.com/bug-ops/feedparser-rs/actions)
[![npm](https://img.shields.io/npm/v/feedparser-rs)](https://www.npmjs.com/package/feedparser-rs)
[![License](https://img.shields.io/crates/l/feedparser-rs-core)](LICENSE-MIT)

High-performance RSS/Atom/JSON Feed parser for Rust, with Python and Node.js bindings. A drop-in replacement for Python's `feedparser` library with 10-100x performance improvement.

## Features

- **Multi-format support** — RSS 0.9x, 1.0, 2.0 / Atom 0.3, 1.0 / JSON Feed 1.0, 1.1
- **Tolerant parsing** — Handles malformed feeds with `bozo` flag pattern (like Python feedparser)
- **HTTP fetching** — Built-in support for fetching feeds from URLs with compression
- **Multi-language bindings** — Native Python (PyO3) and Node.js (napi-rs) bindings
- **feedparser-compatible API** — 100% API compatibility with Python feedparser

## Installation

### Rust

```bash
cargo add feedparser-rs-core
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
feedparser-rs-core = "0.1"
```

> [!IMPORTANT]
> Requires Rust 1.88.0 or later (edition 2024).

### Node.js

```bash
npm install feedparser-rs
# or
yarn add feedparser-rs
# or
pnpm add feedparser-rs
```

### Python

```bash
pip install feedparser-rs
```

## Usage

### Rust Usage

```rust
use feedparser_rs_core::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <title>Example Feed</title>
                <link>https://example.com</link>
                <item>
                    <title>First Post</title>
                    <link>https://example.com/post/1</link>
                </item>
            </channel>
        </rss>
    "#;

    let feed = parse(xml.as_bytes())?;

    println!("Version: {}", feed.version.as_str());  // "rss20"
    println!("Title: {:?}", feed.feed.title);
    println!("Entries: {}", feed.entries.len());

    for entry in &feed.entries {
        println!("  - {:?}", entry.title);
    }

    Ok(())
}
```

#### Fetching from URL

```rust
use feedparser_rs_core::fetch_and_parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed = fetch_and_parse("https://example.com/feed.xml")?;
    println!("Fetched {} entries", feed.entries.len());
    Ok(())
}
```

> [!TIP]
> Use `fetch_and_parse` for URL fetching with automatic compression handling (gzip, deflate, brotli).

### Node.js Usage

```javascript
import { parse, fetchAndParse } from 'feedparser-rs';

// Parse from string
const feed = parse('<rss version="2.0">...</rss>');
console.log(feed.version);  // 'rss20'
console.log(feed.feed.title);
console.log(feed.entries.length);

// Fetch from URL
const remoteFeed = await fetchAndParse('https://example.com/feed.xml');
```

See [Node.js API documentation](crates/feedparser-rs-node/README.md) for complete reference.

### Python Usage

```python
import feedparser_rs

# Parse from bytes or string
d = feedparser_rs.parse(b'<rss>...</rss>')
print(d.version)       # 'rss20'
print(d.feed.title)
print(d.bozo)          # True if parsing had issues
print(d.entries[0].published_parsed)  # time.struct_time
```

> [!NOTE]
> Python bindings provide `time.struct_time` for date fields, matching the original feedparser API.

## Cargo Features

| Feature | Description | Default |
|---------|-------------|---------|
| `http` | Enable URL fetching with reqwest (gzip/deflate/brotli support) | Yes |

To disable HTTP support:

```toml
[dependencies]
feedparser-rs-core = { version = "0.1", default-features = false }
```

## Workspace Structure

This repository contains multiple crates:

| Crate | Description | Package |
|-------|-------------|---------|
| [`feedparser-rs-core`](crates/feedparser-rs-core) | Core Rust parser | [crates.io](https://crates.io/crates/feedparser-rs-core) |
| [`feedparser-rs-node`](crates/feedparser-rs-node) | Node.js bindings | [npm](https://www.npmjs.com/package/feedparser-rs) |
| [`feedparser-rs-py`](crates/feedparser-rs-py) | Python bindings | [PyPI](https://pypi.org/project/feedparser-rs) |

## Development

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for task automation.

```bash
# Install cargo-make
cargo install cargo-make

# Run all checks (format, lint, test)
cargo make ci-all

# Run tests
cargo make test

# Run benchmarks
cargo make bench
```

See all available tasks:

```bash
cargo make --list-all-steps
```

## Benchmarks

Run benchmark comparison against Python feedparser:

```bash
cargo make bench-compare
```

## MSRV Policy

Minimum Supported Rust Version: **1.88.0** (edition 2024).

MSRV increases are considered breaking changes and will result in a minor version bump.

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) before submitting a pull request.

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
