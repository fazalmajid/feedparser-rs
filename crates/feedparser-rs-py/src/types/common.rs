use feedparser_rs_core::{
    Content as CoreContent, Enclosure as CoreEnclosure, Generator as CoreGenerator,
    Image as CoreImage, Link as CoreLink, Person as CorePerson, Source as CoreSource,
    Tag as CoreTag, TextConstruct as CoreTextConstruct, TextType,
};
use pyo3::prelude::*;

/// Text construct with metadata (for title, subtitle, summary, etc.)
#[pyclass(name = "TextConstruct", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyTextConstruct {
    inner: CoreTextConstruct,
}

impl PyTextConstruct {
    pub fn from_core(core: CoreTextConstruct) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyTextConstruct {
    /// Text content
    #[getter]
    fn value(&self) -> &str {
        &self.inner.value
    }

    /// Content type: "text", "html", or "xhtml"
    #[getter]
    #[pyo3(name = "type")]
    fn content_type(&self) -> &str {
        match self.inner.content_type {
            TextType::Text => "text",
            TextType::Html => "html",
            TextType::Xhtml => "xhtml",
        }
    }

    /// Language code (e.g., "en", "fr")
    #[getter]
    fn language(&self) -> Option<&str> {
        self.inner.language.as_deref()
    }

    /// Base URL for relative links
    #[getter]
    fn base(&self) -> Option<&str> {
        self.inner.base.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "TextConstruct(type='{}', value='{}')",
            self.content_type(),
            &self.inner.value.chars().take(50).collect::<String>()
        )
    }
}

/// Link with metadata
#[pyclass(name = "Link", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyLink {
    inner: CoreLink,
}

impl PyLink {
    pub fn from_core(core: CoreLink) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyLink {
    /// Link URL
    #[getter]
    fn href(&self) -> &str {
        &self.inner.href
    }

    /// Link relationship (e.g., "alternate", "enclosure", "self")
    #[getter]
    fn rel(&self) -> Option<&str> {
        self.inner.rel.as_deref()
    }

    /// MIME type (e.g., "text/html", "application/xml")
    #[getter]
    #[pyo3(name = "type")]
    fn link_type(&self) -> Option<&str> {
        self.inner.link_type.as_deref()
    }

    /// Link title
    #[getter]
    fn title(&self) -> Option<&str> {
        self.inner.title.as_deref()
    }

    /// Content length in bytes
    #[getter]
    fn length(&self) -> Option<u64> {
        self.inner.length
    }

    /// Language of the linked resource
    #[getter]
    fn hreflang(&self) -> Option<&str> {
        self.inner.hreflang.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "Link(href='{}', rel='{}')",
            &self.inner.href,
            self.inner.rel.as_deref().unwrap_or("alternate")
        )
    }
}

/// Person (author, contributor, publisher)
#[pyclass(name = "Person", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyPerson {
    inner: CorePerson,
}

impl PyPerson {
    pub fn from_core(core: CorePerson) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyPerson {
    /// Person's name
    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name.as_deref()
    }

    /// Email address
    #[getter]
    fn email(&self) -> Option<&str> {
        self.inner.email.as_deref()
    }

    /// Homepage or profile URL
    #[getter]
    fn uri(&self) -> Option<&str> {
        self.inner.uri.as_deref()
    }

    fn __repr__(&self) -> String {
        if let Some(name) = &self.inner.name {
            format!("Person(name='{}')", name)
        } else if let Some(email) = &self.inner.email {
            format!("Person(email='{}')", email)
        } else {
            "Person()".to_string()
        }
    }
}

/// Tag/Category
#[pyclass(name = "Tag", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyTag {
    inner: CoreTag,
}

impl PyTag {
    pub fn from_core(core: CoreTag) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyTag {
    /// Tag term/name
    #[getter]
    fn term(&self) -> &str {
        &self.inner.term
    }

    /// Categorization scheme
    #[getter]
    fn scheme(&self) -> Option<&str> {
        self.inner.scheme.as_deref()
    }

    /// Human-readable label
    #[getter]
    fn label(&self) -> Option<&str> {
        self.inner.label.as_deref()
    }

    fn __repr__(&self) -> String {
        format!("Tag(term='{}')", &self.inner.term)
    }
}

/// Feed/channel image
#[pyclass(name = "Image", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyImage {
    inner: CoreImage,
}

impl PyImage {
    pub fn from_core(core: CoreImage) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyImage {
    /// Image URL
    #[getter]
    fn url(&self) -> &str {
        &self.inner.url
    }

    /// Image title
    #[getter]
    fn title(&self) -> Option<&str> {
        self.inner.title.as_deref()
    }

    /// Link when image is clicked
    #[getter]
    fn link(&self) -> Option<&str> {
        self.inner.link.as_deref()
    }

    /// Image width in pixels
    #[getter]
    fn width(&self) -> Option<u32> {
        self.inner.width
    }

    /// Image height in pixels
    #[getter]
    fn height(&self) -> Option<u32> {
        self.inner.height
    }

    /// Image description
    #[getter]
    fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }

    fn __repr__(&self) -> String {
        format!("Image(url='{}')", &self.inner.url)
    }
}

/// Media enclosure (audio, video, etc.)
#[pyclass(name = "Enclosure", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyEnclosure {
    inner: CoreEnclosure,
}

impl PyEnclosure {
    pub fn from_core(core: CoreEnclosure) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyEnclosure {
    /// Enclosure URL
    #[getter]
    fn url(&self) -> &str {
        &self.inner.url
    }

    /// File size in bytes
    #[getter]
    fn length(&self) -> Option<u64> {
        self.inner.length
    }

    /// MIME type (e.g., "audio/mpeg", "video/mp4")
    #[getter]
    #[pyo3(name = "type")]
    fn enclosure_type(&self) -> Option<&str> {
        self.inner.enclosure_type.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "Enclosure(url='{}', type='{}')",
            &self.inner.url,
            self.inner.enclosure_type.as_deref().unwrap_or("unknown")
        )
    }
}

/// Content block (for entries with multiple content elements)
#[pyclass(name = "Content", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyContent {
    inner: CoreContent,
}

impl PyContent {
    pub fn from_core(core: CoreContent) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyContent {
    /// Content value
    #[getter]
    fn value(&self) -> &str {
        &self.inner.value
    }

    /// Content MIME type
    #[getter]
    #[pyo3(name = "type")]
    fn content_type(&self) -> Option<&str> {
        self.inner.content_type.as_deref()
    }

    /// Content language
    #[getter]
    fn language(&self) -> Option<&str> {
        self.inner.language.as_deref()
    }

    /// Base URL for relative links
    #[getter]
    fn base(&self) -> Option<&str> {
        self.inner.base.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "Content(type='{}', value='{}')",
            self.inner.content_type.as_deref().unwrap_or("text/plain"),
            &self.inner.value.chars().take(50).collect::<String>()
        )
    }
}

/// Generator (software that created the feed)
#[pyclass(name = "Generator", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyGenerator {
    inner: CoreGenerator,
}

impl PyGenerator {
    pub fn from_core(core: CoreGenerator) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyGenerator {
    /// Generator name
    #[getter]
    fn value(&self) -> &str {
        &self.inner.value
    }

    /// Generator homepage URL
    #[getter]
    fn uri(&self) -> Option<&str> {
        self.inner.uri.as_deref()
    }

    /// Generator version
    #[getter]
    fn version(&self) -> Option<&str> {
        self.inner.version.as_deref()
    }

    fn __repr__(&self) -> String {
        format!(
            "Generator(value='{}', version='{}')",
            &self.inner.value,
            self.inner.version.as_deref().unwrap_or("unknown")
        )
    }
}

/// Source (for entries republished from another feed)
#[pyclass(name = "Source", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PySource {
    inner: CoreSource,
}

impl PySource {
    pub fn from_core(core: CoreSource) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PySource {
    /// Source feed title
    #[getter]
    fn title(&self) -> Option<&str> {
        self.inner.title.as_deref()
    }

    /// Source feed link
    #[getter]
    fn link(&self) -> Option<&str> {
        self.inner.link.as_deref()
    }

    /// Source feed ID
    #[getter]
    fn id(&self) -> Option<&str> {
        self.inner.id.as_deref()
    }

    fn __repr__(&self) -> String {
        if let Some(title) = &self.inner.title {
            format!("Source(title='{}')", title)
        } else {
            "Source()".to_string()
        }
    }
}
