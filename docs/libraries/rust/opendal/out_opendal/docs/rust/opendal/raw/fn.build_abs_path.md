# Function build_abs_path Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/path.rs.html#27-39" class="src">Source</a>

``` rust
pub fn build_abs_path(root: &str, path: &str) -> String
```

Expand description

build_abs_path will build an absolute path with root.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.build_abs_path.html#rules" class="doc-anchor">Â§</a>Rules

- Input root MUST be the format like `/abc/def/`
- Output will be the format like `path/to/root/path`.
