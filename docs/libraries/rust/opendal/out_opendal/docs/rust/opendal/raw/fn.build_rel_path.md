# Function build_rel_path Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/path.rs.html#68-84" class="src">Source</a>

``` rust
pub fn build_rel_path(root: &str, path: &str) -> String
```

Expand description

build_rel_path will build a relative path towards root.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.build_rel_path.html#rules" class="doc-anchor">Â§</a>Rules

- Input root MUST be the format like `/abc/def/`
- Input path MUST start with root like `/abc/def/path/to/file`
- Output will be the format like `path/to/file`.
