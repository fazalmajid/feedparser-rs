# feedparser-rs

High-performance RSS/Atom/JSON Feed parser written in Rust with Node.js bindings.

Drop-in replacement for Python's `feedparser` library, offering 10-100x performance improvement.

## Features

- **Fast**: Written in Rust, 10-100x faster than Python feedparser
- **Tolerant**: Handles malformed feeds with bozo flag (like feedparser)
- **Multi-format**: Supports RSS 0.9x/1.0/2.0, Atom 0.3/1.0, JSON Feed 1.0/1.1
- **Zero-copy**: Efficient parsing with minimal allocations
- **TypeScript**: Full TypeScript definitions included

## Installation

```bash
npm install feedparser-rs
# or
yarn add feedparser-rs
# or
pnpm add feedparser-rs
```

## Quick Start

```javascript
import { parse } from 'feedparser-rs';

// Parse from string
const feed = parse(`
  <?xml version="1.0"?>
  <rss version="2.0">
    <channel>
      <title>My Blog</title>
      <item>
        <title>Hello World</title>
        <link>https://example.com/1</link>
      </item>
    </channel>
  </rss>
`);

console.log(feed.feed.title);  // "My Blog"
console.log(feed.entries[0].title);  // "Hello World"
console.log(feed.version);  // "rss20"

// Parse from Buffer (e.g., HTTP response)
const response = await fetch('https://example.com/feed.xml');
const buffer = Buffer.from(await response.arrayBuffer());
const feed = parse(buffer);
```

## API

### `parse(source: Buffer | string | Uint8Array): ParsedFeed`

Parse a feed from bytes or string.

**Parameters:**
- `source` - Feed content as Buffer, string, or Uint8Array

**Returns:**
- `ParsedFeed` object with feed metadata and entries

**Throws:**
- `Error` if parsing fails catastrophically

**Example:**
```javascript
const feed = parse('<rss version="2.0">...</rss>');
console.log(feed.feed.title);
console.log(feed.entries.length);
```

### `detectFormat(source: Buffer | string | Uint8Array): string`

Detect feed format without full parsing.

**Parameters:**
- `source` - Feed content as Buffer, string, or Uint8Array

**Returns:**
- Format string: `"rss20"`, `"atom10"`, `"json11"`, etc.

**Example:**
```javascript
const format = detectFormat('<feed xmlns="http://www.w3.org/2005/Atom">...</feed>');
console.log(format);  // "atom10"
```

## Types

### ParsedFeed

```typescript
interface ParsedFeed {
  feed: FeedMeta;
  entries: Entry[];
  bozo: boolean;
  bozo_exception?: string;
  encoding: string;
  version: string;
  namespaces: Record<string, string>;
}
```

### FeedMeta

```typescript
interface FeedMeta {
  title?: string;
  title_detail?: TextConstruct;
  link?: string;
  links: Link[];
  subtitle?: string;
  subtitle_detail?: TextConstruct;
  updated?: number;  // Milliseconds since epoch
  author?: string;
  author_detail?: Person;
  authors: Person[];
  contributors: Person[];
  publisher?: string;
  publisher_detail?: Person;
  language?: string;
  rights?: string;
  rights_detail?: TextConstruct;
  generator?: string;
  generator_detail?: Generator;
  image?: Image;
  icon?: string;
  logo?: string;
  tags: Tag[];
  id?: string;
  ttl?: number;
}
```

### Entry

```typescript
interface Entry {
  id?: string;
  title?: string;
  title_detail?: TextConstruct;
  link?: string;
  links: Link[];
  summary?: string;
  summary_detail?: TextConstruct;
  content: Content[];
  published?: number;  // Milliseconds since epoch
  updated?: number;
  created?: number;
  expired?: number;
  author?: string;
  author_detail?: Person;
  authors: Person[];
  contributors: Person[];
  publisher?: string;
  publisher_detail?: Person;
  tags: Tag[];
  enclosures: Enclosure[];
  comments?: string;
  source?: Source;
}
```

### Other Types

```typescript
interface TextConstruct {
  value: string;
  type: "text" | "html" | "xhtml";
  language?: string;
  base?: string;
}

interface Link {
  href: string;
  rel?: string;
  type?: string;
  title?: string;
  length?: number;
  hreflang?: string;
}

interface Person {
  name?: string;
  email?: string;
  uri?: string;
}

interface Tag {
  term: string;
  scheme?: string;
  label?: string;
}

interface Image {
  url: string;
  title?: string;
  link?: string;
  width?: number;
  height?: number;
  description?: string;
}

interface Enclosure {
  url: string;
  length?: number;
  type?: string;
}

interface Content {
  value: string;
  type?: string;
  language?: string;
  base?: string;
}

interface Generator {
  value: string;
  uri?: string;
  version?: string;
}

interface Source {
  title?: string;
  link?: string;
  id?: string;
}
```

## Error Handling

The library uses a "bozo" flag (like feedparser) to indicate parsing errors while still returning partial results:

```javascript
const feed = parse('<rss><channel><title>Broken</title></rss>');

if (feed.bozo) {
  console.warn('Feed has errors:', feed.bozo_exception);
}

// Still can access parsed data
console.log(feed.feed.title);  // "Broken"
```

## Dates

All date fields are returned as milliseconds since Unix epoch (number type). Convert to JavaScript Date:

```javascript
const feed = parse(xmlWithDates);

const entry = feed.entries[0];
if (entry.published) {
  const date = new Date(entry.published);
  console.log(date.toISOString());
}
```

## Performance

Benchmarks vs Python feedparser (parsing 100KB RSS feed):

| Library | Time | Speedup |
|---------|------|---------|
| feedparser-rs | 0.5ms | 100x |
| feedparser (Python) | 50ms | 1x |

See [benchmarks/](../../benchmarks/) for detailed results and methodology.

## Platform Support

Pre-built binaries available for:
- macOS (Intel & Apple Silicon)
- Linux (x64, ARM64)
- Windows (x64)

Supported Node.js versions: 18, 20, 22

## Development

```bash
# Install dependencies
npm install

# Build native module
npm run build

# Run tests
npm test
```

## License

MIT OR Apache-2.0

## Links

- [GitHub](https://github.com/bug-ops/feedparser-rs)
- [npm](https://www.npmjs.com/package/feedparser-rs)
- [Documentation](https://docs.rs/feedparser-rs-core)
- [Changelog](../../CHANGELOG.md)
