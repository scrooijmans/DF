# Function parse_etag Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#102-104" class="src">Source</a>

``` rust
pub fn parse_etag(headers: &HeaderMap) -> Result<Option<&str>>
```

Expand description

Parse etag from header map.
