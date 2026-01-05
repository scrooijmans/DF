# Function parse_content_range Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#88-92" class="src">Source</a>

``` rust
pub fn parse_content_range(
    headers: &HeaderMap,
) -> Result<Option<BytesContentRange>>
```

Expand description

Parse content range from header map.
