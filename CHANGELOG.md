# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Node.js bindings via napi-rs
- npm package `feedparser-rs`
- Criterion benchmarks for Rust
- CI/CD pipeline with GitHub Actions
- Cross-platform builds (Linux, macOS, Windows)
- TypeScript definitions
- Comprehensive Node.js test suite
- Benchmark comparison infrastructure
- Python feedparser benchmark baseline

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2025-12-14

### Added
- Initial release
- RSS 2.0, 1.0, 0.9x parsing
- Atom 1.0, 0.3 parsing
- JSON Feed 1.0, 1.1 parsing
- Multi-format date parsing
- HTML sanitization
- Encoding detection
- Tolerant parsing with bozo flag
- Rust core library `feedparser-rs-core`
- Parser limits for security (max nesting depth, entry count, etc.)
- Comprehensive test coverage
- Documentation with examples

[Unreleased]: https://github.com/bug-ops/feedparser-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/bug-ops/feedparser-rs/releases/tag/v0.1.0
