# Function parse_last_modified Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#95-99" class="src">Source</a>

``` rust
pub fn parse_last_modified(headers: &HeaderMap) -> Result<Option<Timestamp>>
```

Expand description

Parse last modified from header map.
