/// iTunes podcast metadata for feeds
///
/// Contains podcast-level iTunes namespace metadata from the `itunes:` prefix.
/// Namespace URI: `http://www.itunes.com/dtds/podcast-1.0.dtd`
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::ItunesFeedMeta;
///
/// let mut itunes = ItunesFeedMeta::default();
/// itunes.author = Some("John Doe".to_string());
/// itunes.explicit = Some(false);
/// itunes.podcast_type = Some("episodic".to_string());
///
/// assert_eq!(itunes.author.as_deref(), Some("John Doe"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct ItunesFeedMeta {
    /// Podcast author (itunes:author)
    pub author: Option<String>,
    /// Podcast owner contact information (itunes:owner)
    pub owner: Option<ItunesOwner>,
    /// Podcast categories with optional subcategories
    pub categories: Vec<ItunesCategory>,
    /// Explicit content flag (itunes:explicit)
    pub explicit: Option<bool>,
    /// Podcast artwork URL (itunes:image href attribute)
    pub image: Option<String>,
    /// Search keywords (itunes:keywords)
    pub keywords: Vec<String>,
    /// Podcast type: "episodic" or "serial"
    pub podcast_type: Option<String>,
}

/// iTunes podcast metadata for episodes
///
/// Contains episode-level iTunes namespace metadata from the `itunes:` prefix.
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::ItunesEntryMeta;
///
/// let mut episode = ItunesEntryMeta::default();
/// episode.duration = Some(3600); // 1 hour
/// episode.episode = Some(42);
/// episode.season = Some(3);
/// episode.episode_type = Some("full".to_string());
///
/// assert_eq!(episode.duration, Some(3600));
/// ```
#[derive(Debug, Clone, Default)]
pub struct ItunesEntryMeta {
    /// Episode title override (itunes:title)
    pub title: Option<String>,
    /// Episode author (itunes:author)
    pub author: Option<String>,
    /// Episode duration in seconds
    ///
    /// Parsed from various formats: "3600", "60:00", "1:00:00"
    pub duration: Option<u32>,
    /// Explicit content flag for this episode
    pub explicit: Option<bool>,
    /// Episode-specific artwork URL (itunes:image href)
    pub image: Option<String>,
    /// Episode number (itunes:episode)
    pub episode: Option<u32>,
    /// Season number (itunes:season)
    pub season: Option<u32>,
    /// Episode type: "full", "trailer", or "bonus"
    pub episode_type: Option<String>,
}

/// iTunes podcast owner information
///
/// Contact information for the podcast owner (itunes:owner).
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::ItunesOwner;
///
/// let owner = ItunesOwner {
///     name: Some("Jane Doe".to_string()),
///     email: Some("jane@example.com".to_string()),
/// };
///
/// assert_eq!(owner.name.as_deref(), Some("Jane Doe"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct ItunesOwner {
    /// Owner's name (itunes:name)
    pub name: Option<String>,
    /// Owner's email address (itunes:email)
    pub email: Option<String>,
}

/// iTunes category with optional subcategory
///
/// Categories follow Apple's podcast category taxonomy.
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::ItunesCategory;
///
/// let category = ItunesCategory {
///     text: "Technology".to_string(),
///     subcategory: Some("Software How-To".to_string()),
/// };
///
/// assert_eq!(category.text, "Technology");
/// ```
#[derive(Debug, Clone)]
pub struct ItunesCategory {
    /// Category name (text attribute)
    pub text: String,
    /// Optional subcategory (nested itunes:category text attribute)
    pub subcategory: Option<String>,
}

/// Podcast 2.0 metadata
///
/// Modern podcast namespace extensions from `https://podcastindex.org/namespace/1.0`
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::PodcastMeta;
///
/// let mut podcast = PodcastMeta::default();
/// podcast.guid = Some("9b024349-ccf0-5f69-a609-6b82873eab3c".to_string());
///
/// assert!(podcast.guid.is_some());
/// ```
#[derive(Debug, Clone, Default)]
pub struct PodcastMeta {
    /// Transcript URLs (podcast:transcript)
    pub transcripts: Vec<PodcastTranscript>,
    /// Funding/donation links (podcast:funding)
    pub funding: Vec<PodcastFunding>,
    /// People associated with podcast (podcast:person)
    pub persons: Vec<PodcastPerson>,
    /// Permanent podcast GUID (podcast:guid)
    pub guid: Option<String>,
}

/// Podcast 2.0 transcript
///
/// Links to transcript files in various formats.
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::PodcastTranscript;
///
/// let transcript = PodcastTranscript {
///     url: "https://example.com/transcript.txt".to_string(),
///     transcript_type: Some("text/plain".to_string()),
///     language: Some("en".to_string()),
///     rel: None,
/// };
///
/// assert_eq!(transcript.url, "https://example.com/transcript.txt");
/// ```
#[derive(Debug, Clone)]
pub struct PodcastTranscript {
    /// Transcript URL (url attribute)
    pub url: String,
    /// MIME type (type attribute): "text/plain", "text/html", "application/json", etc.
    pub transcript_type: Option<String>,
    /// Language code (language attribute): "en", "es", etc.
    pub language: Option<String>,
    /// Relationship (rel attribute): "captions" or empty
    pub rel: Option<String>,
}

/// Podcast 2.0 funding information
///
/// Links for supporting the podcast financially.
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::PodcastFunding;
///
/// let funding = PodcastFunding {
///     url: "https://example.com/donate".to_string(),
///     message: Some("Support our show!".to_string()),
/// };
///
/// assert_eq!(funding.url, "https://example.com/donate");
/// ```
#[derive(Debug, Clone)]
pub struct PodcastFunding {
    /// Funding URL (url attribute)
    pub url: String,
    /// Optional message/call-to-action (text content)
    pub message: Option<String>,
}

/// Podcast 2.0 person
///
/// Information about hosts, guests, or other people associated with the podcast.
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::PodcastPerson;
///
/// let host = PodcastPerson {
///     name: "John Doe".to_string(),
///     role: Some("host".to_string()),
///     group: None,
///     img: Some("https://example.com/john.jpg".to_string()),
///     href: Some("https://example.com/john".to_string()),
/// };
///
/// assert_eq!(host.name, "John Doe");
/// assert_eq!(host.role.as_deref(), Some("host"));
/// ```
#[derive(Debug, Clone)]
pub struct PodcastPerson {
    /// Person's name (text content)
    pub name: String,
    /// Role: "host", "guest", "editor", etc. (role attribute)
    pub role: Option<String>,
    /// Group name (group attribute)
    pub group: Option<String>,
    /// Image URL (img attribute)
    pub img: Option<String>,
    /// Personal URL/homepage (href attribute)
    pub href: Option<String>,
}

/// Parse duration from various iTunes duration formats
///
/// Supports multiple duration formats:
/// - Seconds only: "3600" → 3600 seconds
/// - MM:SS format: "60:30" → 3630 seconds
/// - HH:MM:SS format: "1:00:30" → 3630 seconds
///
/// # Arguments
///
/// * `s` - Duration string in any supported format
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::parse_duration;
///
/// assert_eq!(parse_duration("3600"), Some(3600));
/// assert_eq!(parse_duration("60:30"), Some(3630));
/// assert_eq!(parse_duration("1:00:30"), Some(3630));
/// assert_eq!(parse_duration("1:30"), Some(90));
/// assert_eq!(parse_duration("invalid"), None);
/// ```
pub fn parse_duration(s: &str) -> Option<u32> {
    let s = s.trim();

    // Try parsing as plain seconds first
    if let Ok(secs) = s.parse::<u32>() {
        return Some(secs);
    }

    // Parse HH:MM:SS or MM:SS format
    let parts: Vec<&str> = s.split(':').collect();
    match parts.len() {
        1 => s.parse().ok(),
        2 => {
            // MM:SS
            let min = parts[0].parse::<u32>().ok()?;
            let sec = parts[1].parse::<u32>().ok()?;
            Some(min * 60 + sec)
        }
        3 => {
            // HH:MM:SS
            let hr = parts[0].parse::<u32>().ok()?;
            let min = parts[1].parse::<u32>().ok()?;
            let sec = parts[2].parse::<u32>().ok()?;
            Some(hr * 3600 + min * 60 + sec)
        }
        _ => None,
    }
}

/// Parse iTunes explicit flag from various string representations
///
/// Accepts multiple boolean representations:
/// - True values: "yes", "true", "explicit"
/// - False values: "no", "false", "clean"
/// - Unknown values return None
///
/// Case-insensitive matching.
///
/// # Arguments
///
/// * `s` - Explicit flag string
///
/// # Examples
///
/// ```
/// use feedparser_rs_core::parse_explicit;
///
/// assert_eq!(parse_explicit("yes"), Some(true));
/// assert_eq!(parse_explicit("YES"), Some(true));
/// assert_eq!(parse_explicit("true"), Some(true));
/// assert_eq!(parse_explicit("explicit"), Some(true));
///
/// assert_eq!(parse_explicit("no"), Some(false));
/// assert_eq!(parse_explicit("false"), Some(false));
/// assert_eq!(parse_explicit("clean"), Some(false));
///
/// assert_eq!(parse_explicit("unknown"), None);
/// ```
pub fn parse_explicit(s: &str) -> Option<bool> {
    match s.trim().to_lowercase().as_str() {
        "yes" | "true" | "explicit" => Some(true),
        "no" | "false" | "clean" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_seconds() {
        assert_eq!(parse_duration("3600"), Some(3600));
        assert_eq!(parse_duration("0"), Some(0));
        assert_eq!(parse_duration("7200"), Some(7200));
    }

    #[test]
    fn test_parse_duration_mmss() {
        assert_eq!(parse_duration("60:30"), Some(3630));
        assert_eq!(parse_duration("1:30"), Some(90));
        assert_eq!(parse_duration("0:45"), Some(45));
        assert_eq!(parse_duration("120:00"), Some(7200));
    }

    #[test]
    fn test_parse_duration_hhmmss() {
        assert_eq!(parse_duration("1:00:30"), Some(3630));
        assert_eq!(parse_duration("2:30:45"), Some(9045));
        assert_eq!(parse_duration("0:01:30"), Some(90));
        assert_eq!(parse_duration("10:00:00"), Some(36000));
    }

    #[test]
    fn test_parse_duration_whitespace() {
        assert_eq!(parse_duration("  3600  "), Some(3600));
        assert_eq!(parse_duration("  1:30:00  "), Some(5400));
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert_eq!(parse_duration("invalid"), None);
        assert_eq!(parse_duration("1:2:3:4"), None);
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("abc:def"), None);
    }

    #[test]
    fn test_parse_explicit_true_variants() {
        assert_eq!(parse_explicit("yes"), Some(true));
        assert_eq!(parse_explicit("YES"), Some(true));
        assert_eq!(parse_explicit("Yes"), Some(true));
        assert_eq!(parse_explicit("true"), Some(true));
        assert_eq!(parse_explicit("TRUE"), Some(true));
        assert_eq!(parse_explicit("explicit"), Some(true));
        assert_eq!(parse_explicit("EXPLICIT"), Some(true));
    }

    #[test]
    fn test_parse_explicit_false_variants() {
        assert_eq!(parse_explicit("no"), Some(false));
        assert_eq!(parse_explicit("NO"), Some(false));
        assert_eq!(parse_explicit("No"), Some(false));
        assert_eq!(parse_explicit("false"), Some(false));
        assert_eq!(parse_explicit("FALSE"), Some(false));
        assert_eq!(parse_explicit("clean"), Some(false));
        assert_eq!(parse_explicit("CLEAN"), Some(false));
    }

    #[test]
    fn test_parse_explicit_whitespace() {
        assert_eq!(parse_explicit("  yes  "), Some(true));
        assert_eq!(parse_explicit("  no  "), Some(false));
    }

    #[test]
    fn test_parse_explicit_unknown() {
        assert_eq!(parse_explicit("unknown"), None);
        assert_eq!(parse_explicit("maybe"), None);
        assert_eq!(parse_explicit(""), None);
        assert_eq!(parse_explicit("1"), None);
    }

    #[test]
    fn test_itunes_feed_meta_default() {
        let meta = ItunesFeedMeta::default();
        assert!(meta.author.is_none());
        assert!(meta.owner.is_none());
        assert!(meta.categories.is_empty());
        assert!(meta.explicit.is_none());
        assert!(meta.image.is_none());
        assert!(meta.keywords.is_empty());
        assert!(meta.podcast_type.is_none());
    }

    #[test]
    fn test_itunes_entry_meta_default() {
        let meta = ItunesEntryMeta::default();
        assert!(meta.title.is_none());
        assert!(meta.author.is_none());
        assert!(meta.duration.is_none());
        assert!(meta.explicit.is_none());
        assert!(meta.image.is_none());
        assert!(meta.episode.is_none());
        assert!(meta.season.is_none());
        assert!(meta.episode_type.is_none());
    }

    #[test]
    fn test_itunes_owner_default() {
        let owner = ItunesOwner::default();
        assert!(owner.name.is_none());
        assert!(owner.email.is_none());
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_itunes_category_clone() {
        let category = ItunesCategory {
            text: "Technology".to_string(),
            subcategory: Some("Software".to_string()),
        };
        let cloned = category.clone();
        assert_eq!(cloned.text, "Technology");
        assert_eq!(cloned.subcategory.as_deref(), Some("Software"));
    }

    #[test]
    fn test_podcast_meta_default() {
        let meta = PodcastMeta::default();
        assert!(meta.transcripts.is_empty());
        assert!(meta.funding.is_empty());
        assert!(meta.persons.is_empty());
        assert!(meta.guid.is_none());
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_podcast_transcript_clone() {
        let transcript = PodcastTranscript {
            url: "https://example.com/transcript.txt".to_string(),
            transcript_type: Some("text/plain".to_string()),
            language: Some("en".to_string()),
            rel: None,
        };
        let cloned = transcript.clone();
        assert_eq!(cloned.url, "https://example.com/transcript.txt");
        assert_eq!(cloned.transcript_type.as_deref(), Some("text/plain"));
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_podcast_funding_clone() {
        let funding = PodcastFunding {
            url: "https://example.com/donate".to_string(),
            message: Some("Support us!".to_string()),
        };
        let cloned = funding.clone();
        assert_eq!(cloned.url, "https://example.com/donate");
        assert_eq!(cloned.message.as_deref(), Some("Support us!"));
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_podcast_person_clone() {
        let person = PodcastPerson {
            name: "John Doe".to_string(),
            role: Some("host".to_string()),
            group: None,
            img: Some("https://example.com/john.jpg".to_string()),
            href: Some("https://example.com".to_string()),
        };
        let cloned = person.clone();
        assert_eq!(cloned.name, "John Doe");
        assert_eq!(cloned.role.as_deref(), Some("host"));
    }
}
