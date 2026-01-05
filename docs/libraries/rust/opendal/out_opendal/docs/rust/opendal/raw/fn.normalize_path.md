# Function normalize_path Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/path.rs.html#97-121" class="src">Source</a>

``` rust
pub fn normalize_path(path: &str) -> String
```

Expand description

Make sure all operation are constructed by normalized path:

- Path endswith `/` means itâ€™s a dir path.
- Otherwise, itâ€™s a file path.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.normalize_path.html#normalize-rules" class="doc-anchor">Â§</a>Normalize Rules

- All whitespace will be trimmed: `abc/def` =\> `abc/def`
- All leading / will be trimmed: `///abc` =\> `abc`
- Internal // will be replaced by /: `abc///def` =\> `abc/def`
- Empty path will be `/`: \`\` =\> `/`
