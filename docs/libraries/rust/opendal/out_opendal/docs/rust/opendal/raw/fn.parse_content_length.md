# Function parse_content_length Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#62-70" class="src">Source</a>

``` rust
pub fn parse_content_length(headers: &HeaderMap) -> Result<Option<u64>>
```

Expand description

Parse content length from header map.
