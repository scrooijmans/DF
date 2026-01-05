# Function parse_content_encoding Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#83-85" class="src">Source</a>

``` rust
pub fn parse_content_encoding(headers: &HeaderMap) -> Result<Option<&str>>
```

Expand description

Parse content encoding from header map.
