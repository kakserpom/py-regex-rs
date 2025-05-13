pub extern crate pyo3;
use pyo3::PyResult;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyModule};
use std::collections::HashMap;

/// A wrapper for a compiled regular expression from the Python `regex` library.
#[derive(Debug)]
pub struct PyRegex {
    compiled: Py<PyAny>,
}
impl PyRegex {
    /// Creates a new regular expression by compiling the pattern via Python's `regex.compile`.
    pub fn new(pattern: &str) -> PyResult<Self> {
        Python::with_gil(|py| {
            Ok(PyRegex {
                compiled: PyModule::import(py, "regex")?
                    .call_method("compile", (pattern,), None)?
                    .into(),
            })
        })
    }

    /// Constructs kwargs with `concurrent=True`.
    fn kwargs(py: Python) -> Option<Bound<PyDict>> {
        let kwargs = PyDict::new(py);
        kwargs.set_item("concurrent", true).ok()?;
        Some(kwargs)
    }

    /// Performs a search for the first match and returns a `PyRegexMatch` object.
    pub fn search_match(&self, text: &str) -> PyResult<Option<PyRegexMatch>> {
        Python::with_gil(|py| {
            let result =
                self.compiled
                    .call_method(py, "search", (text,), Self::kwargs(py).as_ref())?;

            Ok(if result.is_none(py) {
                None
            } else {
                Some(PyRegexMatch { inner: result })
            })
        })
    }

    /// Returns a list of `PyRegexMatch` objects from `finditer()`.
    pub fn find_iter(&self, text: &str) -> PyResult<Vec<PyRegexMatch>> {
        Python::with_gil(|py| {
            let mut matches = Vec::new();
            let binding =
                self.compiled
                    .call_method(py, "finditer", (text,), Self::kwargs(py).as_ref())?;
            let iter = binding.downcast_bound::<PyIterator>(py)?;
            for item in iter {
                let match_obj = item?;
                matches.push(PyRegexMatch {
                    inner: match_obj.into(),
                });
            }
            Ok(matches)
        })
    }

    // Other methods remain unchanged.
    pub fn is_match(&self, text: &str) -> PyResult<bool> {
        Python::with_gil(|py| {
            Ok(!self
                .compiled
                .call_method(py, "search", (text,), Self::kwargs(py).as_ref())?
                .is_none(py))
        })
    }

    pub fn find_all(&self, text: &str) -> PyResult<Vec<String>> {
        Python::with_gil(|py| {
            self.compiled
                .call_method(py, "findall", (text,), Self::kwargs(py).as_ref())?
                .extract::<Vec<String>>(py)
        })
    }

    pub fn replace(&self, text: &str, replacement: &str) -> PyResult<String> {
        Python::with_gil(|py| {
            self.compiled
                .call_method(py, "sub", (replacement, text), Self::kwargs(py).as_ref())?
                .extract::<String>(py)
        })
    }

    pub fn split(&self, text: &str) -> PyResult<Vec<String>> {
        Python::with_gil(|py| {
            self.compiled
                .call_method(py, "split", (text,), Self::kwargs(py).as_ref())?
                .extract::<Vec<String>>(py)
        })
    }

    /// Escapes a string.
    pub fn escape(str: &str, special_only: bool, literal_spaces: bool) -> PyResult<String> {
        Python::with_gil(|py| {
            let kwargs = PyDict::new(py);
            kwargs.set_item("special_only", special_only)?;
            kwargs.set_item("literal_spaces", literal_spaces)?;
            PyModule::import(py, "regex")?
                .call_method("escape", (str,), Some::<Bound<PyDict>>(kwargs).as_ref())?
                .extract::<String>()
        })
    }
}

/// A wrapper for the match object from the Python `regex` module.
pub struct PyRegexMatch {
    inner: Py<PyAny>,
}

impl PyRegexMatch {
    /// Returns the match for the specified group.
    /// For example, `group(0)` is the entire match, `group(1)` is the first subgroup, etc.
    pub fn group(&self, group: u16) -> PyResult<Option<String>> {
        Python::with_gil(|py| {
            self.inner
                .call_method1(py, "group", (group as usize,))?
                .extract::<Option<String>>(py)
        })
    }

    /// Returns all captured groups as a vector.
    /// Analogous to Python's `groups()` method, which returns a tuple of all subgroups (starting from 1).
    pub fn groups(&self) -> PyResult<Vec<Option<String>>> {
        Python::with_gil(|py| {
            self.inner
                .call_method1(py, "groups", ())?
                .extract::<Vec<Option<String>>>(py)
        })
    }

    /// Returns the named groups dictionary (`groupdict()`) as a `HashMap`.
    pub fn groupdict(&self) -> PyResult<HashMap<String, Option<String>>> {
        Python::with_gil(|py| {
            self.inner
                .call_method1(py, "groupdict", ())?
                .extract::<HashMap<String, Option<String>>>(py)
        })
    }

    /// Returns the start position of the match for the specified group.
    pub fn start(&self, group: u16) -> PyResult<isize> {
        Python::with_gil(|py| {
            self.inner
                .call_method1(py, "start", (group as usize,))?
                .extract::<isize>(py)
        })
    }

    /// Returns the end position of the match for the specified group.
    pub fn end(&self, group: u16) -> PyResult<isize> {
        Python::with_gil(|py| {
            self.inner
                .call_method1(
                    py,
                    "end",
                    (group as usize,), /* Option<&pyo3::Bound<'_, PyDict>> */
                )?
                .extract::<isize>(py)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape() -> PyResult<()> {
        // Initialize Python for multithreaded usage.
        pyo3::prepare_freethreaded_python();

        assert_eq!(PyRegex::escape("[]", false, false)?, "\\[\\]");

        Ok(())
    }
    #[test]
    fn test_pyregex_match_methods() -> PyResult<()> {
        // Initialize Python for multithreaded usage.
        pyo3::prepare_freethreaded_python();

        // Use a pattern with a named group and multiple subgroups.
        let pattern = r"(?P<word>\w+)-(\d+)";
        let text = "Test-123";
        let re = PyRegex::new(pattern)?;

        if let Some(m) = re.search_match(text)? {
            // Check the full match via group(0)
            assert_eq!(m.group(0)?, Some("Test-123".to_string()));

            // First subgroup (without a name)
            assert_eq!(m.group(1)?, Some("Test".to_string()));

            // Second subgroup (the number)
            assert_eq!(m.group(2)?, Some("123".to_string()));

            // Get the named groups dictionary
            let gd = m.groupdict()?;
            assert_eq!(gd.get("word").cloned(), Some(Some("Test".to_string())));

            // Get the match span for group 0
            let start = m.start(0)?;
            let end = m.end(0)?;
            println!("Match span for group 0: {}..{}", start, end);
        } else {
            panic!("No match found");
        }

        Ok(())
    }
}
