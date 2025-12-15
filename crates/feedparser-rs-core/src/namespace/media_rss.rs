/// Media RSS Specification
///
/// Namespace: <http://search.yahoo.com/mrss/>
/// Prefix: media
///
/// This module provides parsing support for Media RSS elements commonly
/// used in video/audio feeds and podcasts.
///
/// Common elements:
/// - `media:content` → enclosures
/// - `media:thumbnail` → (could add thumbnails field)
/// - `media:title` → title (fallback)
/// - `media:description` → summary (fallback)
/// - `media:keywords` → tags (comma-separated)
/// - `media:category` → tags
/// - `media:credit` → contributors
use crate::types::{Enclosure, Entry, Tag};

/// Media RSS namespace URI
pub const MEDIA_NAMESPACE: &str = "http://search.yahoo.com/mrss/";

/// Handle Media RSS element at entry level
///
/// Note: This is a simplified implementation. Full Media RSS support
/// would require parsing element attributes (url, type, width, height, etc.)
///
/// # Arguments
///
/// * `element` - Local name of the element (without namespace prefix)
/// * `text` - Text content of the element
/// * `entry` - Entry to update
pub fn handle_entry_element(element: &str, text: &str, entry: &mut Entry) {
    match element {
        "title" => {
            if entry.title.is_none() {
                entry.title = Some(text.to_string());
            }
        }
        "description" => {
            if entry.summary.is_none() {
                entry.summary = Some(text.to_string());
            }
        }
        "keywords" => {
            // Comma-separated keywords
            for keyword in text.split(',') {
                let keyword = keyword.trim();
                if !keyword.is_empty() {
                    entry.tags.push(Tag {
                        term: keyword.to_string(),
                        scheme: None,
                        label: None,
                    });
                }
            }
        }
        "category" => {
            if !text.is_empty() {
                entry.tags.push(Tag {
                    term: text.to_string(),
                    scheme: None,
                    label: None,
                });
            }
        }
        _ => {
            // Other elements like media:content, media:thumbnail, media:credit
            // would require attribute parsing which needs integration with
            // the XML parser. For now, we skip these.
        }
    }
}

/// Handle Media RSS content element with attributes
///
/// This function would be called by the parser when it encounters
/// a media:content element with attributes.
///
/// TODO: Integrate with parser to extract attributes
#[allow(dead_code)]
pub fn handle_media_content(url: &str, mime_type: Option<&str>, length: Option<u64>) -> Enclosure {
    Enclosure {
        url: url.to_string(),
        enclosure_type: mime_type.map(String::from),
        length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_title() {
        let mut entry = Entry::default();
        handle_entry_element("title", "Video Title", &mut entry);

        assert_eq!(entry.title.as_deref(), Some("Video Title"));
    }

    #[test]
    fn test_media_description() {
        let mut entry = Entry::default();
        handle_entry_element("description", "Video description", &mut entry);

        assert_eq!(entry.summary.as_deref(), Some("Video description"));
    }

    #[test]
    fn test_media_keywords() {
        let mut entry = Entry::default();
        handle_entry_element("keywords", "tech, programming, rust", &mut entry);

        assert_eq!(entry.tags.len(), 3);
        assert_eq!(entry.tags[0].term, "tech");
        assert_eq!(entry.tags[1].term, "programming");
        assert_eq!(entry.tags[2].term, "rust");
    }

    #[test]
    fn test_media_keywords_with_spaces() {
        let mut entry = Entry::default();
        handle_entry_element("keywords", "  tech  ,  programming  ", &mut entry);

        assert_eq!(entry.tags.len(), 2);
        assert_eq!(entry.tags[0].term, "tech");
        assert_eq!(entry.tags[1].term, "programming");
    }

    #[test]
    fn test_media_category() {
        let mut entry = Entry::default();
        handle_entry_element("category", "Technology", &mut entry);

        assert_eq!(entry.tags.len(), 1);
        assert_eq!(entry.tags[0].term, "Technology");
    }

    #[test]
    fn test_media_content_helper() {
        let enclosure = handle_media_content(
            "http://example.com/video.mp4",
            Some("video/mp4"),
            Some(1_024_000),
        );

        assert_eq!(enclosure.url, "http://example.com/video.mp4");
        assert_eq!(enclosure.enclosure_type.as_deref(), Some("video/mp4"));
        assert_eq!(enclosure.length, Some(1_024_000));
    }

    #[test]
    fn test_empty_keywords() {
        let mut entry = Entry::default();
        handle_entry_element("keywords", "", &mut entry);

        assert!(entry.tags.is_empty());
    }

    #[test]
    fn test_keywords_with_empty_values() {
        let mut entry = Entry::default();
        handle_entry_element("keywords", "tech, , programming", &mut entry);

        assert_eq!(entry.tags.len(), 2);
        assert_eq!(entry.tags[0].term, "tech");
        assert_eq!(entry.tags[1].term, "programming");
    }
}
