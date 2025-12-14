# feedparser-rs

High-performance RSS/Atom/JSON Feed parser for Rust, with Python and Node.js bindings.

## Overview

**feedparser-rs** is a drop-in replacement for Python's `feedparser` library, written in Rust for 10-100x performance improvement.

### Features

- Parse RSS 0.9x, 1.0, 2.0
- Parse Atom 0.3, 1.0
- Parse JSON Feed 1.0, 1.1
- Tolerant parsing with bozo flag pattern
- 100% API compatibility with Python feedparser
- Python bindings via PyO3
- Node.js bindings via napi-rs

## Status

🚧 **Work in Progress** - Phase 1 (Foundation) complete

## Installation

### Rust

```toml
[dependencies]
feedparser-rs-core = "0.1"
```

### Python (Coming in Phase 4)

```bash
pip install feedparser-rs
```

### Node.js (Coming in Phase 6)

```bash
npm install feedparser-rs
```

## Usage

### Rust

```rust
use feedparser_rs_core::parse;

let xml = r#"
    <?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <title>Example Feed</title>
        </channel>
    </rss>
"#;

let feed = parse(xml.as_bytes())?;
println!("Version: {}", feed.version.as_str());
println!("Title: {}", feed.feed.title.unwrap());
```

### Python

```python
import feedparser_rs

d = feedparser_rs.parse(b'<rss>...</rss>')
print(d.version)  # 'rss20'
print(d.feed.title)
```

## Development

### Build

```bash
cargo build --workspace
```

### Test

```bash
cargo nextest run
```

### Lint

```bash
cargo clippy --workspace -- -D warnings
```

### Format

```bash
cargo +nightly fmt
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please read our contributing guidelines.

### Code of Conduct

This project follows the Rust Code of Conduct.
