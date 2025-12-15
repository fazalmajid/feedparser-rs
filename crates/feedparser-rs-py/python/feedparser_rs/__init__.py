"""
feedparser_rs: High-performance RSS/Atom/JSON Feed parser

Drop-in replacement for Python's feedparser library with 10-100x performance.
Written in Rust with PyO3 bindings for maximum speed and safety.

Usage:
    >>> import feedparser_rs
    >>> d = feedparser_rs.parse('<rss>...</rss>')
    >>> print(d.feed.title)
    >>> print(d.entries[0].published_parsed)

For full documentation, see: https://github.com/rabax/feedparser-rs
"""

from ._feedparser_rs import (
    FeedParserDict,
    ParserLimits,
    __version__,
    detect_format,
    parse,
    parse_with_limits,
)

__all__ = [
    "parse",
    "parse_with_limits",
    "detect_format",
    "FeedParserDict",
    "ParserLimits",
    "__version__",
]

# Type alias for better IDE support
ParseResult = FeedParserDict
