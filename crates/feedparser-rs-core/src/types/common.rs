/// Link in feed or entry
#[derive(Debug, Clone, Default)]
pub struct Link {
    /// Link URL
    pub href: String,
    /// Link relationship type (e.g., "alternate", "enclosure", "self")
    pub rel: Option<String>,
    /// MIME type of the linked resource
    pub link_type: Option<String>,
    /// Human-readable link title
    pub title: Option<String>,
    /// Length of the linked resource in bytes
    pub length: Option<u64>,
    /// Language of the linked resource
    pub hreflang: Option<String>,
}

/// Person (author, contributor, etc.)
#[derive(Debug, Clone, Default)]
pub struct Person {
    /// Person's name
    pub name: Option<String>,
    /// Person's email address
    pub email: Option<String>,
    /// Person's URI/website
    pub uri: Option<String>,
}

/// Tag/category
#[derive(Debug, Clone)]
pub struct Tag {
    /// Tag term/label
    pub term: String,
    /// Tag scheme/domain
    pub scheme: Option<String>,
    /// Human-readable tag label
    pub label: Option<String>,
}

/// Image metadata
#[derive(Debug, Clone)]
pub struct Image {
    /// Image URL
    pub url: String,
    /// Image title
    pub title: Option<String>,
    /// Link associated with the image
    pub link: Option<String>,
    /// Image width in pixels
    pub width: Option<u32>,
    /// Image height in pixels
    pub height: Option<u32>,
    /// Image description
    pub description: Option<String>,
}

/// Enclosure (attached media file)
#[derive(Debug, Clone)]
pub struct Enclosure {
    /// Enclosure URL
    pub url: String,
    /// File size in bytes
    pub length: Option<u64>,
    /// MIME type
    pub enclosure_type: Option<String>,
}

/// Content block
#[derive(Debug, Clone)]
pub struct Content {
    /// Content body
    pub value: String,
    /// Content MIME type
    pub content_type: Option<String>,
    /// Content language
    pub language: Option<String>,
    /// Base URL for relative links
    pub base: Option<String>,
}

/// Text construct type (Atom-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextType {
    /// Plain text
    Text,
    /// HTML content
    Html,
    /// XHTML content
    Xhtml,
}

/// Text construct with metadata
#[derive(Debug, Clone)]
pub struct TextConstruct {
    /// Text content
    pub value: String,
    /// Content type
    pub content_type: TextType,
    /// Content language
    pub language: Option<String>,
    /// Base URL for relative links
    pub base: Option<String>,
}

/// Generator metadata
#[derive(Debug, Clone)]
pub struct Generator {
    /// Generator name
    pub value: String,
    /// Generator URI
    pub uri: Option<String>,
    /// Generator version
    pub version: Option<String>,
}

/// Source reference (for entries)
#[derive(Debug, Clone)]
pub struct Source {
    /// Source title
    pub title: Option<String>,
    /// Source link
    pub link: Option<String>,
    /// Source ID
    pub id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_default() {
        let link = Link::default();
        assert!(link.href.is_empty());
        assert!(link.rel.is_none());
    }

    #[test]
    fn test_person_default() {
        let person = Person::default();
        assert!(person.name.is_none());
        assert!(person.email.is_none());
        assert!(person.uri.is_none());
    }

    #[test]
    fn test_text_type_equality() {
        assert_eq!(TextType::Text, TextType::Text);
        assert_ne!(TextType::Text, TextType::Html);
    }
}
