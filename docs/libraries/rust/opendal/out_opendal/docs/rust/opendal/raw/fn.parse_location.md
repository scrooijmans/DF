# Function parse_location Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#47-49" class="src">Source</a>

``` rust
pub fn parse_location(headers: &HeaderMap) -> Result<Option<&str>>
```

Expand description

Parse redirect location from header map

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_location.html#note" class="doc-anchor">Â§</a>Note

The returned value maybe a relative path, like `/index.html`, `/robots.txt`, etc.
