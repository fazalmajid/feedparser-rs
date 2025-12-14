use super::common::{Content, Enclosure, Link, Person, Source, Tag, TextConstruct};
use chrono::{DateTime, Utc};

/// Feed entry/item
#[derive(Debug, Clone, Default)]
pub struct Entry {
    /// Unique entry identifier
    pub id: Option<String>,
    /// Entry title
    pub title: Option<String>,
    /// Detailed title with metadata
    pub title_detail: Option<TextConstruct>,
    /// Primary link
    pub link: Option<String>,
    /// All links associated with this entry
    pub links: Vec<Link>,
    /// Short description/summary
    pub summary: Option<String>,
    /// Detailed summary with metadata
    pub summary_detail: Option<TextConstruct>,
    /// Full content blocks
    pub content: Vec<Content>,
    /// Publication date
    pub published: Option<DateTime<Utc>>,
    /// Last update date
    pub updated: Option<DateTime<Utc>>,
    /// Creation date
    pub created: Option<DateTime<Utc>>,
    /// Expiration date
    pub expired: Option<DateTime<Utc>>,
    /// Primary author name
    pub author: Option<String>,
    /// Detailed author information
    pub author_detail: Option<Person>,
    /// All authors
    pub authors: Vec<Person>,
    /// Contributors
    pub contributors: Vec<Person>,
    /// Publisher name
    pub publisher: Option<String>,
    /// Detailed publisher information
    pub publisher_detail: Option<Person>,
    /// Tags/categories
    pub tags: Vec<Tag>,
    /// Media enclosures (audio, video, etc.)
    pub enclosures: Vec<Enclosure>,
    /// Comments URL or text
    pub comments: Option<String>,
    /// Source feed reference
    pub source: Option<Source>,
}

impl Entry {
    /// Creates `Entry` with pre-allocated capacity for collections
    ///
    /// Pre-allocates space for typical entry fields:
    /// - 1-2 links (alternate, related)
    /// - 1 content block
    /// - 1 author
    /// - 2-3 tags
    /// - 0-1 enclosures
    ///
    /// # Examples
    ///
    /// ```
    /// use feedparser_rs_core::Entry;
    ///
    /// let entry = Entry::with_capacity();
    /// ```
    #[must_use]
    pub fn with_capacity() -> Self {
        Self {
            links: Vec::with_capacity(2),
            content: Vec::with_capacity(1),
            authors: Vec::with_capacity(1),
            contributors: Vec::with_capacity(0),
            tags: Vec::with_capacity(3),
            enclosures: Vec::with_capacity(1),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_default() {
        let entry = Entry::default();
        assert!(entry.id.is_none());
        assert!(entry.title.is_none());
        assert!(entry.links.is_empty());
        assert!(entry.content.is_empty());
        assert!(entry.authors.is_empty());
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_entry_clone() {
        fn create_entry() -> Entry {
            Entry {
                title: Some("Test".to_string()),
                links: vec![Link::default()],
                ..Default::default()
            }
        }
        let entry = create_entry();
        let cloned = entry.clone();
        assert_eq!(cloned.title.as_deref(), Some("Test"));
        assert_eq!(cloned.links.len(), 1);
    }
}
