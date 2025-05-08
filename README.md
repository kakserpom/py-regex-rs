# py_regex

A small Rust library providing a convenient wrapper around the Python [`regex`](https://pypi.org/project/regex/) module
via [PyO3](https://pyo3.rs).  
It allows you to compile fuzzy (approximate) regular expressions with per-pattern thresholds and use Python’s advanced
regex engine directly from Rust.

## Features

- Compile Python `regex` patterns (including fuzzy matching) from Rust
- Set individual error thresholds per pattern (insertion, deletion, substitution)
- Call `search`, `finditer`, `sub`, and extract match groups, start/end positions
- Thread-safe (after calling `pyo3::prepare_freethreaded_python()`)
- Minimal dependencies

## Requirements

- Rust 1.65 or later (edition 2021)
- Python 3.x with the `regex` package installed
- `pyo3` 0.24.0
- `py_regex` crate (local or published)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pyo3 = "0.24.0"
py_regex = "0.1"
```

Ensure that you have the Python `regex` module:

```bash
pip install regex
```

## Quick Start

Call `pyo3::prepare_freethreaded_python()` once at program start:

```rust
fn main() -> pyo3::PyResult<()> {
    // Initialize the Python interpreter for multi-threaded use
    pyo3::prepare_freethreaded_python();

    // Your code here...
    Ok(())
}
```

### Compile a Pattern

```rust
use py_regex::PyRegex;

let re = PyRegex::new(r"(?i)\bhello\b")?;
assert!(re.is_match("Hello, world!")?);
```

### Fuzzy Matching

```rust
// Automatically uses fuzzy thresholds per pattern
let fuzzy = PyRegex::new(r"(rust){e<=2}")?;
let m = fuzzy.find_iter("ruxy").pop();
assert!(m.is_some());
```

### Find All Matches

```rust
let re = PyRegex::new(r"\d+") ?;
for m in re.find_iter("123 abc 456") ? {
    println!("Match: {:?}", m.group(0)?);
}
```

### Replace (Substitution)

```rust
let re = PyRegex::new(r"\d+")?;
let result = re.replace("There are 123 apples", "NUM")?;
assert_eq!(result, "There are NUM apples");
```

### Extract Groups

```rust
let re = PyRegex::new(r"(\w+)@(\w+\.\w+)")?;
if let Some(m) = re.find_iter("user@example.com").pop() {
let parts = m.groups() ?; // Vec<Option<String>>
// parts[0] is the full match, parts[1] username, parts[2] domain
}
```

## API

#### `PyRegex::new(pattern: &str) -> PyResult<PyRegex>`

Compile a Python `regex` pattern.

#### `PyRegex::is_match(text: &str) -> PyResult<bool>`

Return `true` if `search(text)` finds a match.

#### `PyRegex::find_iter(text: &str) -> PyResult<Vec<PyRegexMatch>>`

Return all non-overlapping matches as `PyRegexMatch`.

#### `PyRegex::replace(text: &str, replacement: &str) -> PyResult<String>`

Perform substitution (`sub`) on the input text.

#### `PyRegexMatch`

- `group(idx: usize) -> PyResult<Option<String>>`
- `groups() -> PyResult<Vec<Option<String>>>`
- `start(idx: usize) -> PyResult<isize>`
- `end(idx: usize) -> PyResult<isize>`

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
