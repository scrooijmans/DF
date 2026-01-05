# Function parse_multipart_boundary Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#112-114" class="src">Source</a>

``` rust
pub fn parse_multipart_boundary(headers: &HeaderMap) -> Result<Option<&str>>
```

Expand description

Parse multipart boundary from header map.
