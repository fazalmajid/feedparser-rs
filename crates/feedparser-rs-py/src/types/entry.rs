use feedparser_rs_core::Entry as CoreEntry;
use pyo3::prelude::*;

use super::common::{PyContent, PyEnclosure, PyLink, PyPerson, PySource, PyTag, PyTextConstruct};
use super::datetime::optional_datetime_to_struct_time;
use super::podcast::PyItunesEntryMeta;

/// Feed entry/item
#[pyclass(name = "Entry", module = "feedparser_rs")]
#[derive(Clone)]
pub struct PyEntry {
    inner: CoreEntry,
}

impl PyEntry {
    pub fn from_core(core: CoreEntry) -> Self {
        Self { inner: core }
    }
}

#[pymethods]
impl PyEntry {
    /// Entry unique identifier
    #[getter]
    fn id(&self) -> Option<&str> {
        self.inner.id.as_deref()
    }

    /// Entry title
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

    /// Primary entry link
    #[getter]
    fn link(&self) -> Option<&str> {
        self.inner.link.as_deref()
    }

    /// All links associated with this entry
    #[getter]
    fn links(&self) -> Vec<PyLink> {
        self.inner
            .links
            .iter()
            .map(|l| PyLink::from_core(l.clone()))
            .collect()
    }

    /// Short description/summary
    #[getter]
    fn summary(&self) -> Option<&str> {
        self.inner.summary.as_deref()
    }

    /// Detailed summary with metadata
    #[getter]
    fn summary_detail(&self) -> Option<PyTextConstruct> {
        self.inner
            .summary_detail
            .as_ref()
            .map(|tc| PyTextConstruct::from_core(tc.clone()))
    }

    /// Full content blocks
    #[getter]
    fn content(&self) -> Vec<PyContent> {
        self.inner
            .content
            .iter()
            .map(|c| PyContent::from_core(c.clone()))
            .collect()
    }

    /// Publication date (ISO 8601 string)
    #[getter]
    fn published(&self) -> Option<String> {
        self.inner.published.map(|dt| dt.to_rfc3339())
    }

    /// Publication date as time.struct_time (feedparser compatibility)
    #[getter]
    fn published_parsed(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_datetime_to_struct_time(py, &self.inner.published)
    }

    /// Last update date (ISO 8601 string)
    #[getter]
    fn updated(&self) -> Option<String> {
        self.inner.updated.map(|dt| dt.to_rfc3339())
    }

    /// Last update date as time.struct_time
    #[getter]
    fn updated_parsed(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_datetime_to_struct_time(py, &self.inner.updated)
    }

    /// Creation date (ISO 8601 string)
    #[getter]
    fn created(&self) -> Option<String> {
        self.inner.created.map(|dt| dt.to_rfc3339())
    }

    /// Creation date as time.struct_time
    #[getter]
    fn created_parsed(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_datetime_to_struct_time(py, &self.inner.created)
    }

    /// Expiration date (ISO 8601 string)
    #[getter]
    fn expired(&self) -> Option<String> {
        self.inner.expired.map(|dt| dt.to_rfc3339())
    }

    /// Expiration date as time.struct_time
    #[getter]
    fn expired_parsed(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_datetime_to_struct_time(py, &self.inner.expired)
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

    /// Contributors to this entry
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

    /// Tags/categories
    #[getter]
    fn tags(&self) -> Vec<PyTag> {
        self.inner
            .tags
            .iter()
            .map(|t| PyTag::from_core(t.clone()))
            .collect()
    }

    /// Media enclosures (audio, video, etc.)
    #[getter]
    fn enclosures(&self) -> Vec<PyEnclosure> {
        self.inner
            .enclosures
            .iter()
            .map(|e| PyEnclosure::from_core(e.clone()))
            .collect()
    }

    /// Comments URL or text
    #[getter]
    fn comments(&self) -> Option<&str> {
        self.inner.comments.as_deref()
    }

    /// Source feed information (for republished entries)
    #[getter]
    fn source(&self) -> Option<PySource> {
        self.inner
            .source
            .as_ref()
            .map(|s| PySource::from_core(s.clone()))
    }

    /// iTunes podcast episode metadata
    #[getter]
    fn itunes(&self) -> Option<PyItunesEntryMeta> {
        self.inner
            .itunes
            .as_ref()
            .map(|i| PyItunesEntryMeta::from_core(i.clone()))
    }

    fn __repr__(&self) -> String {
        format!(
            "Entry(title='{}', id='{}')",
            self.inner.title.as_deref().unwrap_or("untitled"),
            self.inner.id.as_deref().unwrap_or("no-id")
        )
    }
}
