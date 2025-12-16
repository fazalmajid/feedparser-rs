# feedparser-rs Copilot Instructions

## Project Overview
High-performance RSS/Atom/JSON Feed parser in Rust with Python (PyO3) and Node.js (napi-rs) bindings. Drop-in replacement for Python's `feedparser` with 10-100x performance improvement.

**MSRV:** Rust 1.88.0 | **Edition:** 2024 | **License:** MIT/Apache-2.0

## Architecture

### Workspace Structure
- **`crates/feedparser-rs-core`** — Pure Rust parser. All parsing logic lives here.
- **`crates/feedparser-rs-py`** — Python bindings via PyO3/maturin.
- **`crates/feedparser-rs-node`** — Node.js bindings via napi-rs.

### Parser Flow
1. `parse()` → `detect_format()` identifies RSS/Atom/JSON
2. Routes to `parser/rss.rs`, `parser/atom.rs`, or `parser/json.rs`
3. Namespace handlers in `namespace/` extract iTunes, Dublin Core, Media RSS, Podcast 2.0
4. Returns `ParsedFeed` with `bozo` flag for error tolerance

## Idiomatic Rust & Performance

### Type Safety First
- Prefer strong types over primitives: `FeedVersion` enum, not `&str`
- Use `Option<T>` and `Result<T, E>` — never sentinel values
- Leverage generics and trait bounds for reusable code:
```rust
fn collect_limited<T, I: Iterator<Item = T>>(iter: I, limit: usize) -> Vec<T> {
    iter.take(limit).collect()
}
```

### Zero-Cost Abstractions
- Use `&str` over `String` in function parameters
- Prefer iterators over index-based loops: `.iter().filter().map()`
- Use `Cow<'_, str>` when ownership is conditionally needed
- Avoid allocations in hot paths — reuse buffers where possible

### Edition 2024 Features
- Use `gen` blocks for custom iterators where applicable
- Leverage improved async patterns for HTTP module
- Apply new lifetime elision rules for cleaner signatures

### Safety Guidelines
- `#![warn(unsafe_code)]` is enabled — avoid `unsafe` unless absolutely necessary
- All public APIs must have doc comments (`#![warn(missing_docs)]`)
- Use `thiserror` for error types with proper `#[error]` attributes

## Critical Conventions

### Error Handling: Bozo Pattern (MANDATORY)
**Never panic or return errors for malformed input.** Set `bozo = true` and continue:
```rust
match parse_date(&text) {
    Some(dt) => entry.published = Some(dt),
    None => {
        feed.bozo = true;
        feed.bozo_exception = Some(format!("Invalid date: {text}"));
        // Continue parsing!
    }
}
```

### API Compatibility with Python feedparser
Field names must match `feedparser` exactly: `feed.title`, `entries[0].summary`, `version` returns `"rss20"`, `"atom10"`

### XML Parsing with quick-xml
Use tolerant mode — no strict validation:
```rust
let mut reader = Reader::from_reader(data);
reader.config_mut().trim_text(true);
// Do NOT enable check_end_names — tolerance over strictness
```

## Development Commands

All automation via `cargo-make`:

| Command | Purpose |
|---------|---------|
| `cargo make fmt` | Format with nightly rustfmt |
| `cargo make clippy` | Lint (excludes py bindings) |
| `cargo make test-rust` | Rust tests (nextest) |
| `cargo make pre-commit` | fmt + clippy + test-rust |
| `cargo make bench` | Criterion benchmarks |
| `cargo make msrv-check` | Verify MSRV 1.88.0 compatibility |

### Bindings
```bash
# Python
cd crates/feedparser-rs-py && maturin develop && pytest tests/ -v

# Node.js
cd crates/feedparser-rs-node && pnpm install && pnpm build && pnpm test
```

## Testing Patterns

Use `include_str!()` for fixtures in `tests/fixtures/`:
```rust
#[test]
fn test_rss20_basic() {
    let xml = include_str!("../../tests/fixtures/rss/example.xml");
    let feed = parse(xml.as_bytes()).unwrap();
    assert!(!feed.bozo);
}
```

Always verify malformed feeds set bozo but still parse:
```rust
#[test]
fn test_malformed_sets_bozo() {
    let xml = b"<rss><channel><title>Broken</title></rss>";
    let feed = parse(xml).unwrap();
    assert!(feed.bozo);
    assert_eq!(feed.feed.title.as_deref(), Some("Broken")); // Still parsed!
}
```

## Commit & Branch Conventions
- Branch: `feat/`, `fix/`, `docs/`, `refactor/`, `test/`
- Commits: [Conventional Commits](https://conventionalcommits.org/)

## What NOT to Do
- Don't use `.unwrap()` or `.expect()` in parser code — use bozo pattern
- Don't add dependencies without workspace-level declaration in root `Cargo.toml`
- Don't skip `--exclude feedparser-rs-py` in workspace-wide Rust commands (PyO3 needs special handling)
- Don't break API compatibility with Python feedparser field names
