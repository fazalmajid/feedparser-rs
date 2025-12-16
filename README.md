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

🚧 **Work in Progress** - Phase 4 (Node.js bindings + CI/CD) complete

[![CI](https://github.com/bug-ops/feedparser-rs/workflows/CI/badge.svg)](https://github.com/bug-ops/feedparser-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/feedparser-rs-core.svg)](https://crates.io/crates/feedparser-rs-core)
[![npm](https://img.shields.io/npm/v/feedparser-rs.svg)](https://www.npmjs.com/package/feedparser-rs)
[![Documentation](https://docs.rs/feedparser-rs-core/badge.svg)](https://docs.rs/feedparser-rs-core)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

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

### Node.js

```bash
npm install feedparser-rs
# or
yarn add feedparser-rs
# or
pnpm add feedparser-rs
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

### Node.js

```javascript
import { parse } from 'feedparser-rs';

const feed = parse('<rss version="2.0">...</rss>');
console.log(feed.version);  // 'rss20'
console.log(feed.feed.title);
console.log(feed.entries.length);
```

See [crates/feedparser-rs-node/README.md](crates/feedparser-rs-node/README.md) for full Node.js API documentation.

## Development

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for task automation. All development tasks are defined in `Makefile.toml`.

### Setup

Install cargo-make:

```bash
cargo install cargo-make
```

### Available Tasks

View all available tasks:

```bash
cargo make --list-all-steps
```

### Common Development Tasks

#### Formatting

```bash
# Format code with nightly rustfmt
cargo make fmt

# Check formatting without modifying files
cargo make fmt-check
```

#### Linting

```bash
# Run clippy
cargo make clippy

# Run all linting checks (format + clippy + doc)
cargo make lint
```

#### Testing

```bash
# Run Rust tests
cargo make test-rust

# Run all tests (Rust + Python + Node.js)
cargo make test

# Run doctests
cargo make doctest
```

#### Security

```bash
# Run all security checks
cargo make deny

# Run specific security checks
cargo make deny-advisories
cargo make deny-licenses
```

#### Coverage

```bash
# Generate Rust coverage
cargo make coverage-rust

# Generate all coverage reports
cargo make coverage
```

#### Benchmarks

```bash
# Run Rust benchmarks
cargo make bench

# Compare Rust vs Python performance
cargo make bench-compare
```

#### Utilities

```bash
# Check for outdated dependencies
cargo make check-versions
```

#### Pre-commit/Pre-push

```bash
# Run checks before committing
cargo make pre-commit

# Run comprehensive checks before pushing
cargo make pre-push
```

#### CI Simulation

```bash
# Run all CI checks locally
cargo make ci-all
```

### Build

```bash
cargo build --workspace
# or
cargo make build
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
