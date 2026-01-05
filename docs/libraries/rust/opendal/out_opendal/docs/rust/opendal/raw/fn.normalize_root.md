# Function normalize_root Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/path.rs.html#135-148" class="src">Source</a>

``` rust
pub fn normalize_root(v: &str) -> String
```

Expand description

Make sure root is normalized to style like `/abc/def/`.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.normalize_root.html#normalize-rules" class="doc-anchor">Â§</a>Normalize Rules

- All whitespace will be trimmed: `abc/def` =\> `abc/def`
- All leading / will be trimmed: `///abc` =\> `abc`
- Internal // will be replaced by /: `abc///def` =\> `abc/def`
- Empty path will be `/`: \`\` =\> `/`
- Add leading `/` if not starts with: `abc/` =\> `/abc/`
- Add trailing `/` if not ends with: `/abc` =\> `/abc/`

Finally, we will get path like `/path/to/root/`.
