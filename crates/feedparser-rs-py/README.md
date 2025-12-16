# feedparser-rs

High-performance RSS/Atom/JSON Feed parser for Python with feedparser-compatible API.

## Features

- **Fast**: Native Rust implementation via PyO3
- **Tolerant parsing**: Bozo flag for graceful handling of malformed feeds
- **Multi-format**: RSS 0.9x/1.0/2.0, Atom 0.3/1.0, JSON Feed 1.0/1.1
- **Podcast support**: iTunes and Podcast 2.0 namespace extensions
- **Familiar API**: Inspired by feedparser, easy migration path
- **DoS protection**: Built-in resource limits

## Installation

```bash
pip install feedparser-rs
```

## Usage

```python
import feedparser_rs

# Parse from string or bytes
d = feedparser_rs.parse('<rss>...</rss>')
d = feedparser_rs.parse(b'<rss>...</rss>')

# Access data
print(d.feed.title)
print(d.version)  # "rss20", "atom10", etc.
print(d.bozo)     # True if parsing errors occurred

for entry in d.entries:
    print(entry.title)
    print(entry.published_parsed)  # time.struct_time
```

## Migration from feedparser

```python
# Option 1: alias import
import feedparser_rs as feedparser
d = feedparser.parse(feed_content)

# Option 2: direct import
import feedparser_rs
d = feedparser_rs.parse(feed_content)
```

> **Note**: URL fetching is not yet implemented. Use `requests.get(url).content` to fetch feeds.

## Advanced Usage

### Custom Resource Limits

```python
import feedparser_rs

limits = feedparser_rs.ParserLimits(
    max_feed_size_bytes=50_000_000,  # 50 MB
    max_entries=5_000,
    max_authors=20,
    max_links_per_entry=50,
)

d = feedparser_rs.parse_with_limits(feed_data, limits)
```

### Format Detection

```python
import feedparser_rs

version = feedparser_rs.detect_format(feed_data)
print(version)  # "rss20", "atom10", "json11", etc.
```

### Podcast Support

```python
import feedparser_rs

d = feedparser_rs.parse(podcast_feed)

# iTunes metadata
if d.feed.itunes:
    print(d.feed.itunes.author)
    print(d.feed.itunes.categories)

# Episode metadata
for entry in d.entries:
    if entry.itunes:
        print(f"Duration: {entry.itunes.duration}s")
```

## API Reference

### Functions

- `parse(source)` — Parse feed from bytes or str
- `parse_with_limits(source, limits)` — Parse with custom resource limits
- `detect_format(source)` — Detect feed format without full parsing

### Classes

- `FeedParserDict` — Parsed feed result
  - `.feed` — Feed metadata
  - `.entries` — List of entries
  - `.bozo` — True if parsing errors occurred
  - `.version` — Feed version string
  - `.encoding` — Character encoding

- `ParserLimits` — Resource limits configuration

## Requirements

- Python >= 3.9

## Development

```bash
git clone https://github.com/bug-ops/feedparser-rs
cd feedparser-rs/crates/feedparser-rs-py
pip install maturin
maturin develop
```

## License

MIT OR Apache-2.0

## Links

- [GitHub](https://github.com/bug-ops/feedparser-rs)
- [PyPI](https://pypi.org/project/feedparser-rs/)
- [Issues](https://github.com/bug-ops/feedparser-rs/issues)
