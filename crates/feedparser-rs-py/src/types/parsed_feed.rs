use feedparser_rs_core::ParsedFeed as CoreParsedFeed;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::entry::PyEntry;
use super::feed_meta::PyFeedMeta;

#[pyclass(name = "FeedParserDict", module = "feedparser_rs")]
pub struct PyParsedFeed {
    feed: Py<PyFeedMeta>,
    entries: Vec<Py<PyEntry>>,
    bozo: bool,
    bozo_exception: Option<String>,
    encoding: String,
    version: String,
    namespaces: Py<PyDict>,
}

impl PyParsedFeed {
    pub fn from_core(py: Python<'_>, core: CoreParsedFeed) -> PyResult<Self> {
        let feed = Py::new(py, PyFeedMeta::from_core(core.feed))?;

        let entries: PyResult<Vec<_>> = core
            .entries
            .into_iter()
            .map(|e| Py::new(py, PyEntry::from_core(e)))
            .collect();

        let namespaces = PyDict::new(py);
        for (prefix, uri) in core.namespaces {
            namespaces.set_item(prefix, uri)?;
        }

        Ok(Self {
            feed,
            entries: entries?,
            bozo: core.bozo,
            bozo_exception: core.bozo_exception,
            encoding: core.encoding,
            version: core.version.to_string(),
            namespaces: namespaces.unbind(),
        })
    }
}

#[pymethods]
impl PyParsedFeed {
    #[getter]
    fn feed(&self, py: Python<'_>) -> Py<PyFeedMeta> {
        self.feed.clone_ref(py)
    }

    #[getter]
    fn entries(&self, py: Python<'_>) -> Vec<Py<PyEntry>> {
        self.entries.iter().map(|e| e.clone_ref(py)).collect()
    }

    #[getter]
    fn bozo(&self) -> bool {
        self.bozo
    }

    #[getter]
    fn bozo_exception(&self) -> Option<&str> {
        self.bozo_exception.as_deref()
    }

    #[getter]
    fn encoding(&self) -> &str {
        &self.encoding
    }

    #[getter]
    fn version(&self) -> &str {
        &self.version
    }

    #[getter]
    fn namespaces(&self, py: Python<'_>) -> Py<PyDict> {
        self.namespaces.clone_ref(py)
    }

    fn __repr__(&self) -> String {
        format!(
            "FeedParserDict(version='{}', bozo={}, entries={})",
            self.version,
            self.bozo,
            self.entries.len()
        )
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}
