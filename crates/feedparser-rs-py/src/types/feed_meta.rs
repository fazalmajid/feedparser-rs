use feedparser_rs_core::FeedMeta as CoreFeedMeta;
use pyo3::prelude::*;

use super::common::{PyGenerator, PyImage, PyLink, PyPerson, PyTag, PyTextConstruct};
use super::datetime::optional_datetime_to_struct_time;
use super::podcast::{PyItunesFeedMeta, PyPodcastMeta};

/// Feed-level metadata
#[pyclass(name = "FeedMeta", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyFeedMeta {
    inner: CoreFeedMeta,
}

impl PyFeedMeta {
    pub fn from_core(core: CoreFeedMeta) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyFeedMeta {
    /// Feed title
    #[getter]
    fn title(&self) -> Option<&str> {
        self.inner.title.as_deref()
    }

    /// Detailed title with metadata
    #[getter]
    fn title_detail(&self) -> Option<PyTextConstruct> {
        self.inner
            .title_detail
            .as_ref()
            .map(|tc| PyTextConstruct::from_core(tc.clone()))
    }

    /// Primary feed link
    #[getter]
    fn link(&self) -> Option<&str> {
        self.inner.link.as_deref()
    }

    /// All links associated with this feed
    #[getter]
    fn links(&self) -> Vec<PyLink> {
        self.inner
            .links
            .iter()
            .map(|l| PyLink::from_core(l.clone()))
            .collect()
    }

    /// Feed subtitle/description
    #[getter]
    fn subtitle(&self) -> Option<&str> {
        self.inner.subtitle.as_deref()
    }

    /// Detailed subtitle with metadata
    #[getter]
    fn subtitle_detail(&self) -> Option<PyTextConstruct> {
        self.inner
            .subtitle_detail
            .as_ref()
            .map(|tc| PyTextConstruct::from_core(tc.clone()))
    }

    /// Last update date (ISO 8601 string)
    #[getter]
    fn updated(&self) -> Option<String> {
        self.inner.updated.map(|dt| dt.to_rfc3339())
    }

    /// Last update date as time.struct_time (feedparser compatibility)
    #[getter]
    fn updated_parsed(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_datetime_to_struct_time(py, &self.inner.updated)
    }

    /// Primary author name
    #[getter]
    fn author(&self) -> Option<&str> {
        self.inner.author.as_deref()
    }

    /// Detailed author information
    #[getter]
    fn author_detail(&self) -> Option<PyPerson> {
        self.inner
            .author_detail
            .as_ref()
            .map(|p| PyPerson::from_core(p.clone()))
    }

    /// All authors
    #[getter]
    fn authors(&self) -> Vec<PyPerson> {
        self.inner
            .authors
            .iter()
            .map(|p| PyPerson::from_core(p.clone()))
            .collect()
    }

    /// Contributors to this feed
    #[getter]
    fn contributors(&self) -> Vec<PyPerson> {
        self.inner
            .contributors
            .iter()
            .map(|p| PyPerson::from_core(p.clone()))
            .collect()
    }

    /// Publisher name
    #[getter]
    fn publisher(&self) -> Option<&str> {
        self.inner.publisher.as_deref()
    }

    /// Detailed publisher information
    #[getter]
    fn publisher_detail(&self) -> Option<PyPerson> {
        self.inner
            .publisher_detail
            .as_ref()
            .map(|p| PyPerson::from_core(p.clone()))
    }

    /// Feed language (e.g., "en", "fr")
    #[getter]
    fn language(&self) -> Option<&str> {
        self.inner.language.as_deref()
    }

    /// Copyright/rights statement
    #[getter]
    fn rights(&self) -> Option<&str> {
        self.inner.rights.as_deref()
    }

    /// Detailed rights with metadata
    #[getter]
    fn rights_detail(&self) -> Option<PyTextConstruct> {
        self.inner
            .rights_detail
            .as_ref()
            .map(|tc| PyTextConstruct::from_core(tc.clone()))
    }

    /// Feed generator name
    #[getter]
    fn generator(&self) -> Option<&str> {
        self.inner.generator.as_deref()
    }

    /// Detailed generator information
    #[getter]
    fn generator_detail(&self) -> Option<PyGenerator> {
        self.inner
            .generator_detail
            .as_ref()
            .map(|g| PyGenerator::from_core(g.clone()))
    }

    /// Feed image/logo
    #[getter]
    fn image(&self) -> Option<PyImage> {
        self.inner
            .image
            .as_ref()
            .map(|i| PyImage::from_core(i.clone()))
    }

    /// Feed icon URL (Atom)
    #[getter]
    fn icon(&self) -> Option<&str> {
        self.inner.icon.as_deref()
    }

    /// Feed logo URL (Atom)
    #[getter]
    fn logo(&self) -> Option<&str> {
        self.inner.logo.as_deref()
    }

    /// Feed tags/categories
    #[getter]
    fn tags(&self) -> Vec<PyTag> {
        self.inner
            .tags
            .iter()
            .map(|t| PyTag::from_core(t.clone()))
            .collect()
    }

    /// Feed unique identifier
    #[getter]
    fn id(&self) -> Option<&str> {
        self.inner.id.as_deref()
    }

    /// Time to live (refresh interval in minutes)
    #[getter]
    fn ttl(&self) -> Option<u32> {
        self.inner.ttl
    }

    /// iTunes podcast metadata
    #[getter]
    fn itunes(&self) -> Option<PyItunesFeedMeta> {
        self.inner
            .itunes
            .as_ref()
            .map(|i| PyItunesFeedMeta::from_core(i.clone()))
    }

    /// Podcast 2.0 namespace metadata
    #[getter]
    fn podcast(&self) -> Option<PyPodcastMeta> {
        self.inner
            .podcast
            .as_ref()
            .map(|p| PyPodcastMeta::from_core(p.clone()))
    }

    fn __repr__(&self) -> String {
        format!(
            "FeedMeta(title='{}', link='{}')",
            self.inner.title.as_deref().unwrap_or("untitled"),
            self.inner.link.as_deref().unwrap_or("no-link")
        )
    }
}
