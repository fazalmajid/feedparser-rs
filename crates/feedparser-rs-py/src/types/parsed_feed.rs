use feedparser_rs::ParsedFeed as CoreParsedFeed;
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
    status: Option<u16>,
    href: Option<String>,
    etag: Option<String>,
    modified: Option<String>,
    #[cfg(feature = "http")]
    headers: Option<Py<PyDict>>,
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

        #[cfg(feature = "http")]
        let headers = if let Some(headers_map) = core.headers {
            let headers_dict = PyDict::new(py);
            for (key, value) in headers_map {
                headers_dict.set_item(key, value)?;
            }
            Some(headers_dict.unbind())
        } else {
            None
        };

        Ok(Self {
            feed,
            entries: entries?,
            bozo: core.bozo,
            bozo_exception: core.bozo_exception,
            encoding: core.encoding,
            version: core.version.to_string(),
            namespaces: namespaces.unbind(),
            status: core.status,
            href: core.href,
            etag: core.etag,
            modified: core.modified,
            #[cfg(feature = "http")]
            headers,
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

    #[getter]
    fn status(&self) -> Option<u16> {
        self.status
    }

    #[getter]
    fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }

    #[getter]
    fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    #[getter]
    fn modified(&self) -> Option<&str> {
        self.modified.as_deref()
    }

    #[cfg(feature = "http")]
    #[getter]
    fn headers(&self, py: Python<'_>) -> Option<Py<PyDict>> {
        self.headers.as_ref().map(|h| h.clone_ref(py))
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
