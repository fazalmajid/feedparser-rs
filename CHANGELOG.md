# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-12-16

### Added
- HTTP fetching with `http` feature (enabled by default)
- `parse_url` and `parse_url_with_limits` functions for URL fetching
- Conditional GET support (ETag, Last-Modified) for bandwidth-efficient caching
- Automatic compression handling (gzip, deflate, brotli)
- Node.js `fetchAndParse` function for URL fetching
- Podcast namespace support (iTunes and Podcast 2.0)
- CONTRIBUTING.md guide
- GitHub issue and PR templates

### Changed
- Renamed crate from `feedparser-rs-core` to `feedparser-rs`
- Default features now include `http` for URL fetching support
- Migrated to cargo-make for task automation
- Updated documentation with more accurate claims

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
- Rust core library
- Parser limits for security (max nesting depth, entry count, etc.)
- Comprehensive test coverage
- Documentation with examples

[Unreleased]: https://github.com/bug-ops/feedparser-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/bug-ops/feedparser-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/bug-ops/feedparser-rs/releases/tag/v0.1.0
