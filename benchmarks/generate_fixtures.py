#!/usr/bin/env python3
"""Generate benchmark fixture feeds"""

from pathlib import Path


def generate_rss_feed(num_entries: int) -> str:
    """Generate RSS 2.0 feed with specified number of entries"""

    entries = []
    for i in range(num_entries):
        entries.append(f"""
    <item>
      <title>Entry {i+1} - Lorem ipsum dolor sit amet</title>
      <link>https://example.com/entry/{i+1}</link>
      <description>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
        Ut enim ad minim veniam, quis nostrud exercitation ullamco.
      </description>
      <pubDate>Mon, 01 Jan 2025 {i%24:02d}:{i%60:02d}:00 GMT</pubDate>
      <guid>https://example.com/entry/{i+1}</guid>
    </item>
""")

    return f"""<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Benchmark Feed ({num_entries} entries)</title>
    <link>https://example.com</link>
    <description>Test feed for benchmarking</description>
    <language>en-us</language>
    <lastBuildDate>Mon, 01 Jan 2025 00:00:00 GMT</lastBuildDate>
{''.join(entries)}
  </channel>
</rss>
"""


def main():
    fixtures_dir = Path(__file__).parent / "fixtures"
    fixtures_dir.mkdir(exist_ok=True)

    configs = [
        ("small.xml", 5),
        ("medium.xml", 50),
        ("large.xml", 500),
    ]

    for filename, num_entries in configs:
        xml = generate_rss_feed(num_entries)
        filepath = fixtures_dir / filename
        filepath.write_text(xml)

        size_kb = len(xml) / 1024
        print(f"Generated {filename}: {num_entries} entries, {size_kb:.1f} KB")


if __name__ == "__main__":
    main()
