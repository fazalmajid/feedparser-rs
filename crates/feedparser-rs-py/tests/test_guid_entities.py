"""Test XML entity decoding in guid elements.

Regression test for a bug where quick-xml 0.39's Event::GeneralRef events
(emitted for XML entity references like &#038;) were silently dropped,
causing decoded characters to vanish from entry.id / entry.guid.
"""

import feedparser_rs


def test_guid_with_numeric_character_reference():
    """Test that &#038; in guid decodes to & (matching Python feedparser)."""
    xml = b"""<?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <item>
                <guid isPermaLink="false">https://sidequested.com/?post_type=webcomic1&#038;p=3172</guid>
            </item>
        </channel>
    </rss>"""

    d = feedparser_rs.parse(xml)

    assert d.entries[0].id == "https://sidequested.com/?post_type=webcomic1&p=3172"
    assert d.entries[0].guid == d.entries[0].id


def test_guid_with_amp_entity():
    """Test that &amp; in guid decodes to &."""
    xml = b"""<?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <item>
                <guid>https://example.com/?a=1&amp;b=2</guid>
            </item>
        </channel>
    </rss>"""

    d = feedparser_rs.parse(xml)

    assert d.entries[0].id == "https://example.com/?a=1&b=2"


def test_guid_with_hex_character_reference():
    """Test that &#x26; (hex for &) in guid decodes correctly."""
    xml = b"""<?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <item>
                <guid>https://example.com/?a=1&#x26;b=2</guid>
            </item>
        </channel>
    </rss>"""

    d = feedparser_rs.parse(xml)

    assert d.entries[0].id == "https://example.com/?a=1&b=2"


def test_multiple_entities_in_guid():
    """Test guid with multiple entity references."""
    xml = b"""<?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <item>
                <guid>https://example.com/?a=1&amp;b=2&amp;c=3</guid>
            </item>
        </channel>
    </rss>"""

    d = feedparser_rs.parse(xml)

    assert d.entries[0].id == "https://example.com/?a=1&b=2&c=3"
