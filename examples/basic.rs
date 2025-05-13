use py_regex::{PyRegex, pyo3, pyo3::PyResult};
use std::sync::Arc;
use std::thread;

fn main() -> PyResult<()> {
    // Initialize Python for multithreaded usage.
    pyo3::prepare_freethreaded_python();

    let pattern = r"(?P<id>\d+)";
    let text = "IDs: 101, 202, 303";
    let re = Arc::new(PyRegex::new(pattern)?);

    // Example usage of `search_match()` to obtain a `PyRegexMatch` object.
    if let Some(m) = re.search_match(text)? {
        println!("Full match (group 0): {:?}", m.group(0)?);
        println!("Group 'id' (as group 0 here): {:?}", m.group(0)?);
        println!("Groupdict: {:?}", m.groupdict()?);
        println!("Span for group 0: {}..{}", m.start(0)?, m.end(0)?);
    }

    // Example of multithreaded usage of `find_iter()`, returning a `Vec<PyRegexMatch>`.
    let mut handles = vec![];
    for i in 0..4 {
        let re_clone = Arc::clone(&re);
        let text_clone = text.to_string();
        let handle = thread::spawn(move || -> PyResult<()> {
            let matches = re_clone.find_iter(&text_clone)?;
            println!("Thread {}: found {} matches.", i, matches.len());
            for m in matches {
                println!("Thread {}: match group 0: {:?}", i, m.group(0)?);
            }
            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}
