//! RSS 2.0 parser implementation

use crate::{
    ParserLimits,
    error::{FeedError, Result},
    namespace::{content, dublin_core, media_rss},
    types::{
        Enclosure, Entry, FeedVersion, Image, ItunesCategory, ItunesEntryMeta, ItunesFeedMeta,
        ItunesOwner, Link, MediaContent, MediaThumbnail, ParsedFeed, PodcastFunding, PodcastMeta,
        Source, Tag, TextConstruct, TextType, parse_duration, parse_explicit,
    },
    util::parse_date,
};
use quick_xml::{Reader, events::Event};

use super::common::{
    EVENT_BUFFER_CAPACITY, FromAttributes, LimitedCollectionExt, init_feed, read_text, skip_element,
};

/// Parse RSS 2.0 feed from raw bytes
///
/// Parses an RSS 2.0 feed in tolerant mode, setting the bozo flag
/// on errors but continuing to extract as much data as possible.
///
/// # Arguments
///
/// * `data` - Raw RSS XML data
///
/// # Returns
///
/// * `Ok(ParsedFeed)` - Successfully parsed feed (may have bozo flag set)
/// * `Err(FeedError)` - Fatal error that prevented any parsing
///
/// # Examples
///
/// ```ignore
/// let xml = br#"
///     <rss version="2.0">
///         <channel>
///             <title>Example</title>
///         </channel>
///     </rss>
/// "#;
///
/// let feed = parse_rss20(xml).unwrap();
/// assert_eq!(feed.feed.title.as_deref(), Some("Example"));
/// ```
#[allow(dead_code)]
pub fn parse_rss20(data: &[u8]) -> Result<ParsedFeed> {
    parse_rss20_with_limits(data, ParserLimits::default())
}

/// Parse RSS 2.0 with custom parser limits
pub fn parse_rss20_with_limits(data: &[u8], limits: ParserLimits) -> Result<ParsedFeed> {
    limits
        .check_feed_size(data.len())
        .map_err(|e| FeedError::InvalidFormat(e.to_string()))?;

    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);

    let mut feed = init_feed(FeedVersion::Rss20, limits.max_entries);
    let mut buf = Vec::with_capacity(EVENT_BUFFER_CAPACITY);
    let mut depth: usize = 1;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"channel" => {
                depth += 1;
                if let Err(e) = parse_channel(&mut reader, &mut feed, &limits, &mut depth) {
                    feed.bozo = true;
                    feed.bozo_exception = Some(e.to_string());
                }
                depth = depth.saturating_sub(1);
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                feed.bozo = true;
                feed.bozo_exception = Some(format!("XML parsing error: {e}"));
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(feed)
}

/// Parse <channel> element (feed metadata and items)
fn parse_channel(
    reader: &mut Reader<&[u8]>,
    feed: &mut ParsedFeed,
    limits: &ParserLimits,
    depth: &mut usize,
) -> Result<()> {
    let mut buf = Vec::with_capacity(EVENT_BUFFER_CAPACITY);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e) | Event::Empty(e)) => {
                *depth += 1;
                if *depth > limits.max_nesting_depth {
                    return Err(FeedError::InvalidFormat(format!(
                        "XML nesting depth {} exceeds maximum {}",
                        depth, limits.max_nesting_depth
                    )));
                }

                // Use full qualified name to distinguish standard RSS tags from namespaced tags
                // (e.g., <image> vs <itunes:image>, <category> vs <itunes:category>)
                match e.name().as_ref() {
                    b"title" => {
                        feed.feed.title = Some(read_text(reader, &mut buf, limits)?);
                    }
                    b"link" => {
                        let link_text = read_text(reader, &mut buf, limits)?;
                        feed.feed.link = Some(link_text.clone());
                        feed.feed.links.try_push_limited(
                            Link {
                                href: link_text,
                                rel: Some("alternate".to_string()),
                                ..Default::default()
                            },
                            limits.max_links_per_feed,
                        );
                    }
                    b"description" => {
                        feed.feed.subtitle = Some(read_text(reader, &mut buf, limits)?);
                    }
                    b"language" => {
                        feed.feed.language = Some(read_text(reader, &mut buf, limits)?);
                    }
                    b"pubDate" => {
                        let text = read_text(reader, &mut buf, limits)?;
                        match parse_date(&text) {
                            Some(dt) => feed.feed.updated = Some(dt),
                            None if !text.is_empty() => {
                                feed.bozo = true;
                                feed.bozo_exception = Some("Invalid pubDate format".to_string());
                            }
                            None => {}
                        }
                    }
                    b"managingEditor" => {
                        feed.feed.author = Some(read_text(reader, &mut buf, limits)?);
                    }
                    b"webMaster" => {
                        feed.feed.publisher = Some(read_text(reader, &mut buf, limits)?);
                    }
                    b"generator" => {
                        feed.feed.generator = Some(read_text(reader, &mut buf, limits)?);
                    }
                    b"ttl" => {
                        let text = read_text(reader, &mut buf, limits)?;
                        feed.feed.ttl = text.parse().ok();
                    }
                    b"category" => {
                        let term = read_text(reader, &mut buf, limits)?;
                        feed.feed.tags.try_push_limited(
                            Tag {
                                term,
                                scheme: None,
                                label: None,
                            },
                            limits.max_tags,
                        );
                    }
                    b"image" => {
                        if let Ok(image) = parse_image(reader, &mut buf, limits, depth) {
                            feed.feed.image = Some(image);
                        }
                    }
                    b"item" => {
                        if feed.entries.is_at_limit(limits.max_entries) {
                            feed.bozo = true;
                            feed.bozo_exception =
                                Some(format!("Entry limit exceeded: {}", limits.max_entries));
                            skip_element(reader, &mut buf, limits, *depth)?;
                            *depth = depth.saturating_sub(1);
                            continue;
                        }

                        match parse_item(reader, &mut buf, limits, depth) {
                            Ok(entry) => feed.entries.push(entry),
                            Err(e) => {
                                feed.bozo = true;
                                feed.bozo_exception = Some(e.to_string());
                            }
                        }
                    }
                    tag => {
                        // Check for iTunes and Podcast 2.0 namespace tags
                        let handled = if is_itunes_tag(tag, b"author") {
                            let text = read_text(reader, &mut buf, limits)?;
                            let itunes =
                                feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
                            itunes.author = Some(text);
                            true
                        } else if is_itunes_tag(tag, b"owner") {
                            let itunes =
                                feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
                            if let Ok(owner) = parse_itunes_owner(reader, &mut buf, limits, depth) {
                                itunes.owner = Some(owner);
                            }
                            true
                        } else if is_itunes_tag(tag, b"category") {
                            // Parse category inline to avoid borrow conflicts
                            let mut category_text = String::new();
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"text"
                                    && let Ok(value) = attr.unescape_value()
                                {
                                    category_text =
                                        value.chars().take(limits.max_attribute_length).collect();
                                }
                            }
                            let itunes =
                                feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
                            itunes.categories.push(ItunesCategory {
                                text: category_text,
                                subcategory: None,
                            });
                            skip_element(reader, &mut buf, limits, *depth)?;
                            true
                        } else if is_itunes_tag(tag, b"explicit") {
                            let text = read_text(reader, &mut buf, limits)?;
                            let itunes =
                                feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
                            itunes.explicit = parse_explicit(&text);
                            true
                        } else if is_itunes_tag(tag, b"image") {
                            let itunes =
                                feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"href"
                                    && let Ok(value) = attr.unescape_value()
                                {
                                    itunes.image = Some(
                                        value.chars().take(limits.max_attribute_length).collect(),
                                    );
                                }
                            }
                            // NOTE: Don't call skip_element - itunes:image is typically self-closing
                            //       and calling skip_element would consume the next tag's end event
                            true
                        } else if is_itunes_tag(tag, b"keywords") {
                            let text = read_text(reader, &mut buf, limits)?;
                            let itunes =
                                feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
                            itunes.keywords = text
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            true
                        } else if is_itunes_tag(tag, b"type") {
                            let text = read_text(reader, &mut buf, limits)?;
                            let itunes =
                                feed.feed.itunes.get_or_insert_with(ItunesFeedMeta::default);
                            itunes.podcast_type = Some(text);
                            true
                        } else if tag.starts_with(b"podcast:guid") {
                            let text = read_text(reader, &mut buf, limits)?;
                            let podcast =
                                feed.feed.podcast.get_or_insert_with(PodcastMeta::default);
                            podcast.guid = Some(text);
                            true
                        } else if tag.starts_with(b"podcast:funding") {
                            // Parse funding inline to avoid borrow conflicts
                            let mut url = String::new();
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"url"
                                    && let Ok(value) = attr.unescape_value()
                                {
                                    url = value.chars().take(limits.max_attribute_length).collect();
                                }
                            }
                            let message_text = read_text(reader, &mut buf, limits)?;
                            let message = if message_text.is_empty() {
                                None
                            } else {
                                Some(message_text)
                            };
                            let podcast =
                                feed.feed.podcast.get_or_insert_with(PodcastMeta::default);
                            podcast.funding.push(PodcastFunding { url, message });
                            true
                        } else if let Some(dc_element) = is_dc_tag(tag) {
                            // Dublin Core namespace
                            let dc_elem = dc_element.to_string();
                            let text = read_text(reader, &mut buf, limits)?;
                            dublin_core::handle_feed_element(&dc_elem, &text, &mut feed.feed);
                            true
                        } else if let Some(_content_element) = is_content_tag(tag) {
                            // Content namespace - typically only used at entry level
                            skip_element(reader, &mut buf, limits, *depth)?;
                            true
                        } else if let Some(_media_element) = is_media_tag(tag) {
                            // Media RSS namespace - typically only used at entry level
                            skip_element(reader, &mut buf, limits, *depth)?;
                            true
                        } else {
                            false
                        };

                        if !handled {
                            skip_element(reader, &mut buf, limits, *depth)?;
                        }
                    }
                }
                *depth = depth.saturating_sub(1);
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"channel" => {
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

/// Parse <item> element (entry)
fn parse_item(
    reader: &mut Reader<&[u8]>,
    buf: &mut Vec<u8>,
    limits: &ParserLimits,
    depth: &mut usize,
) -> Result<Entry> {
    let mut entry = Entry::with_capacity();

    loop {
        match reader.read_event_into(buf) {
            Ok(event @ (Event::Start(_) | Event::Empty(_))) => {
                let is_empty = matches!(event, Event::Empty(_));
                let (Event::Start(e) | Event::Empty(e)) = &event else {
                    unreachable!()
                };

                *depth += 1;
                if *depth > limits.max_nesting_depth {
                    return Err(FeedError::InvalidFormat(format!(
                        "XML nesting depth {} exceeds maximum {}",
                        depth, limits.max_nesting_depth
                    )));
                }

                // Use full qualified name to distinguish standard RSS tags from namespaced tags
                match e.name().as_ref() {
                    b"title" => {
                        entry.title = Some(read_text(reader, buf, limits)?);
                    }
                    b"link" => {
                        let link_text = read_text(reader, buf, limits)?;
                        entry.link = Some(link_text.clone());
                        entry.links.try_push_limited(
                            Link {
                                href: link_text,
                                rel: Some("alternate".to_string()),
                                ..Default::default()
                            },
                            limits.max_links_per_entry,
                        );
                    }
                    b"description" => {
                        let desc = read_text(reader, buf, limits)?;
                        entry.summary = Some(desc.clone());
                        entry.summary_detail = Some(TextConstruct {
                            value: desc,
                            content_type: TextType::Html,
                            language: None,
                            base: None,
                        });
                    }
                    b"guid" => {
                        entry.id = Some(read_text(reader, buf, limits)?);
                    }
                    b"pubDate" => {
                        let text = read_text(reader, buf, limits)?;
                        entry.published = parse_date(&text);
                    }
                    b"author" => {
                        entry.author = Some(read_text(reader, buf, limits)?);
                    }
                    b"category" => {
                        let term = read_text(reader, buf, limits)?;
                        entry.tags.try_push_limited(
                            Tag {
                                term,
                                scheme: None,
                                label: None,
                            },
                            limits.max_tags,
                        );
                    }
                    b"enclosure" => {
                        if let Some(enclosure) = parse_enclosure(e, limits) {
                            entry
                                .enclosures
                                .try_push_limited(enclosure, limits.max_enclosures);
                        }
                        skip_element(reader, buf, limits, *depth)?;
                    }
                    b"comments" => {
                        entry.comments = Some(read_text(reader, buf, limits)?);
                    }
                    b"source" => {
                        if let Ok(source) = parse_source(reader, buf, limits, depth) {
                            entry.source = Some(source);
                        }
                    }
                    tag => {
                        // Check for iTunes and Podcast 2.0 namespace tags
                        let handled = if is_itunes_tag(tag, b"title") {
                            let text = read_text(reader, buf, limits)?;
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            itunes.title = Some(text);
                            true
                        } else if is_itunes_tag(tag, b"author") {
                            let text = read_text(reader, buf, limits)?;
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            itunes.author = Some(text);
                            true
                        } else if is_itunes_tag(tag, b"duration") {
                            let text = read_text(reader, buf, limits)?;
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            itunes.duration = parse_duration(&text);
                            true
                        } else if is_itunes_tag(tag, b"explicit") {
                            let text = read_text(reader, buf, limits)?;
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            itunes.explicit = parse_explicit(&text);
                            true
                        } else if is_itunes_tag(tag, b"image") {
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"href"
                                    && let Ok(value) = attr.unescape_value()
                                {
                                    itunes.image = Some(
                                        value.chars().take(limits.max_attribute_length).collect(),
                                    );
                                }
                            }
                            // NOTE: Don't call skip_element - itunes:image is typically self-closing
                            true
                        } else if is_itunes_tag(tag, b"episode") {
                            let text = read_text(reader, buf, limits)?;
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            itunes.episode = text.parse().ok();
                            true
                        } else if is_itunes_tag(tag, b"season") {
                            let text = read_text(reader, buf, limits)?;
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            itunes.season = text.parse().ok();
                            true
                        } else if is_itunes_tag(tag, b"episodeType") {
                            let text = read_text(reader, buf, limits)?;
                            let itunes = entry.itunes.get_or_insert_with(ItunesEntryMeta::default);
                            itunes.episode_type = Some(text);
                            true
                        } else if tag.starts_with(b"podcast:transcript") {
                            // Podcast 2.0 transcript not stored in Entry for now
                            skip_element(reader, buf, limits, *depth)?;
                            true
                        } else if tag.starts_with(b"podcast:person") {
                            // Parse person inline to avoid borrow conflicts
                            // Podcast 2.0 person not stored in Entry for now (no podcast field)
                            skip_element(reader, buf, limits, *depth)?;
                            true
                        } else if let Some(dc_element) = is_dc_tag(tag) {
                            // Dublin Core namespace
                            let dc_elem = dc_element.to_string();
                            let text = read_text(reader, buf, limits)?;
                            dublin_core::handle_entry_element(&dc_elem, &text, &mut entry);
                            true
                        } else if let Some(content_element) = is_content_tag(tag) {
                            // Content namespace
                            let content_elem = content_element.to_string();
                            let text = read_text(reader, buf, limits)?;
                            content::handle_entry_element(&content_elem, &text, &mut entry);
                            true
                        } else if let Some(media_element) = is_media_tag(tag) {
                            // Media RSS namespace - handle both text elements and attribute-based elements
                            if media_element == "thumbnail" {
                                // media:thumbnail has attributes
                                if let Some(thumbnail) = MediaThumbnail::from_attributes(
                                    e.attributes().flatten(),
                                    limits.max_attribute_length,
                                ) {
                                    entry
                                        .media_thumbnails
                                        .try_push_limited(thumbnail, limits.max_enclosures);
                                }
                                // Don't call skip_element if it's self-closing
                                if !is_empty {
                                    skip_element(reader, buf, limits, *depth)?;
                                }
                            } else if media_element == "content" {
                                // media:content has attributes
                                if let Some(media) = MediaContent::from_attributes(
                                    e.attributes().flatten(),
                                    limits.max_attribute_length,
                                ) {
                                    entry
                                        .media_content
                                        .try_push_limited(media, limits.max_enclosures);
                                }
                                // Don't call skip_element if it's self-closing
                                if !is_empty {
                                    skip_element(reader, buf, limits, *depth)?;
                                }
                            } else {
                                // Other media elements (title, description, keywords, category)
                                let media_elem = media_element.to_string();
                                let text = read_text(reader, buf, limits)?;
                                media_rss::handle_entry_element(&media_elem, &text, &mut entry);
                            }
                            true
                        } else {
                            false
                        };

                        if !handled {
                            skip_element(reader, buf, limits, *depth)?;
                        }
                    }
                }
                *depth = depth.saturating_sub(1);
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"item" => {
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(entry)
}

/// Parse <image> element
fn parse_image(
    reader: &mut Reader<&[u8]>,
    buf: &mut Vec<u8>,
    limits: &ParserLimits,
    depth: &mut usize,
) -> Result<Image> {
    let mut url = String::new();
    let mut title = None;
    let mut link = None;
    let mut width = None;
    let mut height = None;
    let mut description = None;

    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                *depth += 1;
                if *depth > limits.max_nesting_depth {
                    return Err(FeedError::InvalidFormat(format!(
                        "XML nesting depth {} exceeds maximum {}",
                        depth, limits.max_nesting_depth
                    )));
                }

                match e.local_name().as_ref() {
                    b"url" => url = read_text(reader, buf, limits)?,
                    b"title" => title = Some(read_text(reader, buf, limits)?),
                    b"link" => link = Some(read_text(reader, buf, limits)?),
                    b"width" => {
                        if let Ok(w) = read_text(reader, buf, limits)?.parse() {
                            width = Some(w);
                        }
                    }
                    b"height" => {
                        if let Ok(h) = read_text(reader, buf, limits)?.parse() {
                            height = Some(h);
                        }
                    }
                    b"description" => description = Some(read_text(reader, buf, limits)?),
                    _ => skip_element(reader, buf, limits, *depth)?,
                }
                *depth = depth.saturating_sub(1);
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"image" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    if url.is_empty() {
        return Err(FeedError::InvalidFormat("Image missing url".to_string()));
    }

    Ok(Image {
        url,
        title,
        link,
        width,
        height,
        description,
    })
}

#[inline]
fn parse_enclosure(e: &quick_xml::events::BytesStart, limits: &ParserLimits) -> Option<Enclosure> {
    Enclosure::from_attributes(e.attributes().flatten(), limits.max_attribute_length)
}

/// Parse <source> element
fn parse_source(
    reader: &mut Reader<&[u8]>,
    buf: &mut Vec<u8>,
    limits: &ParserLimits,
    depth: &mut usize,
) -> Result<Source> {
    let mut title = None;
    let mut link = None;
    let id = None;

    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                *depth += 1;
                if *depth > limits.max_nesting_depth {
                    return Err(FeedError::InvalidFormat(format!(
                        "XML nesting depth {} exceeds maximum {}",
                        depth, limits.max_nesting_depth
                    )));
                }

                match e.local_name().as_ref() {
                    b"title" => title = Some(read_text(reader, buf, limits)?),
                    b"url" => link = Some(read_text(reader, buf, limits)?),
                    _ => skip_element(reader, buf, limits, *depth)?,
                }
                *depth = depth.saturating_sub(1);
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"source" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(Source { title, link, id })
}

/// Check if element name matches an iTunes namespace tag
///
/// iTunes tags can appear as either:
/// - `itunes:tag` (with namespace prefix)
/// - Just `tag` in the iTunes namespace URI
///
/// The fallback `name == tag` is intentional and safe because:
/// 1. iTunes namespace elements SHOULD have a prefix (e.g., `itunes:author`)
/// 2. Fallback exists for feeds that don't use the prefix but declare iTunes namespace
/// 3. Match order in calling code ensures standard RSS elements (title, link, etc.) are
///    handled first in the outer match statement, preventing incorrect matches
#[inline]
fn is_itunes_tag(name: &[u8], tag: &[u8]) -> bool {
    // Check for "itunes:tag" pattern
    if name.starts_with(b"itunes:") && &name[7..] == tag {
        return true;
    }
    // Also check for just the tag name (some feeds don't use prefix)
    name == tag
}

/// Check if element name matches a Dublin Core namespace tag
#[inline]
fn is_dc_tag(name: &[u8]) -> Option<&str> {
    if name.starts_with(b"dc:") {
        std::str::from_utf8(&name[3..]).ok()
    } else {
        None
    }
}

/// Check if element name matches a Content namespace tag
#[inline]
fn is_content_tag(name: &[u8]) -> Option<&str> {
    if name.starts_with(b"content:") {
        std::str::from_utf8(&name[8..]).ok()
    } else {
        None
    }
}

/// Check if element name matches a Media RSS namespace tag
#[inline]
fn is_media_tag(name: &[u8]) -> Option<&str> {
    if name.starts_with(b"media:") {
        std::str::from_utf8(&name[6..]).ok()
    } else {
        None
    }
}

/// Parse iTunes owner from <itunes:owner> element
fn parse_itunes_owner(
    reader: &mut Reader<&[u8]>,
    buf: &mut Vec<u8>,
    limits: &ParserLimits,
    depth: &mut usize,
) -> Result<ItunesOwner> {
    let mut owner = ItunesOwner::default();

    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) => {
                *depth += 1;
                if *depth > limits.max_nesting_depth {
                    return Err(FeedError::InvalidFormat(format!(
                        "XML nesting depth {} exceeds maximum {}",
                        depth, limits.max_nesting_depth
                    )));
                }

                let tag_name = e.local_name();
                if is_itunes_tag(tag_name.as_ref(), b"name") {
                    owner.name = Some(read_text(reader, buf, limits)?);
                } else if is_itunes_tag(tag_name.as_ref(), b"email") {
                    owner.email = Some(read_text(reader, buf, limits)?);
                } else {
                    skip_element(reader, buf, limits, *depth)?;
                }
                *depth = depth.saturating_sub(1);
            }
            Ok(Event::End(_) | Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    Ok(owner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_basic_rss() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <title>Test Feed</title>
                <link>http://example.com</link>
                <description>Test description</description>
            </channel>
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        assert_eq!(feed.version, FeedVersion::Rss20);
        assert!(!feed.bozo);
        assert_eq!(feed.feed.title.as_deref(), Some("Test Feed"));
        assert_eq!(feed.feed.link.as_deref(), Some("http://example.com"));
        assert_eq!(feed.feed.subtitle.as_deref(), Some("Test description"));
    }

    #[test]
    fn test_parse_rss_with_items() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <title>Test</title>
                <item>
                    <title>Item 1</title>
                    <link>http://example.com/1</link>
                    <description>Description 1</description>
                    <guid>item-1</guid>
                </item>
                <item>
                    <title>Item 2</title>
                    <link>http://example.com/2</link>
                </item>
            </channel>
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        assert_eq!(feed.entries.len(), 2);
        assert_eq!(feed.entries[0].title.as_deref(), Some("Item 1"));
        assert_eq!(feed.entries[0].id.as_deref(), Some("item-1"));
        assert_eq!(feed.entries[1].title.as_deref(), Some("Item 2"));
    }

    #[test]
    fn test_parse_rss_with_dates() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <pubDate>Sat, 14 Dec 2024 10:30:00 +0000</pubDate>
                <item>
                    <pubDate>Fri, 13 Dec 2024 09:00:00 +0000</pubDate>
                </item>
            </channel>
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        assert!(feed.feed.updated.is_some());
        assert!(feed.entries[0].published.is_some());

        let dt = feed.feed.updated.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 14);
    }

    #[test]
    fn test_parse_rss_with_invalid_date() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <pubDate>not a date</pubDate>
            </channel>
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        assert!(feed.bozo);
        assert!(feed.bozo_exception.is_some());
        assert!(feed.bozo_exception.unwrap().contains("Invalid pubDate"));
    }

    #[test]
    fn test_parse_rss_with_categories() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <item>
                    <category>Tech</category>
                    <category>News</category>
                </item>
            </channel>
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        assert_eq!(feed.entries[0].tags.len(), 2);
        assert_eq!(feed.entries[0].tags[0].term, "Tech");
        assert_eq!(feed.entries[0].tags[1].term, "News");
    }

    #[test]
    fn test_parse_rss_with_enclosure() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <item>
                    <enclosure url="http://example.com/audio.mp3"
                               length="12345"
                               type="audio/mpeg"/>
                </item>
            </channel>
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        assert_eq!(feed.entries[0].enclosures.len(), 1);
        assert_eq!(
            feed.entries[0].enclosures[0].url,
            "http://example.com/audio.mp3"
        );
        assert_eq!(feed.entries[0].enclosures[0].length, Some(12345));
        assert_eq!(
            feed.entries[0].enclosures[0].enclosure_type.as_deref(),
            Some("audio/mpeg")
        );
    }

    #[test]
    fn test_parse_rss_malformed_continues() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <title>Test</title>
                <item>
                    <title>Item 1</title>
                </item>
                <!-- Missing close tag but continues -->
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        // Should still extract some data
        assert_eq!(feed.feed.title.as_deref(), Some("Test"));
    }

    #[test]
    fn test_parse_rss_with_cdata() {
        let xml = br#"<?xml version="1.0"?>
        <rss version="2.0">
            <channel>
                <item>
                    <description><![CDATA[HTML <b>content</b> here]]></description>
                </item>
            </channel>
        </rss>"#;

        let feed = parse_rss20(xml).unwrap();
        assert_eq!(
            feed.entries[0].summary.as_deref(),
            Some("HTML <b>content</b> here")
        );
    }
}
