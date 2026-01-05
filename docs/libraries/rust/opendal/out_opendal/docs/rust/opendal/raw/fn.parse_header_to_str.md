# Function parse_header_to_str Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#118-145" class="src">Source</a>

``` rust
pub fn parse_header_to_str<K>(
    headers: &HeaderMap,
    name: K,
) -> Result<Option<&str>>where
    HeaderName: TryFrom<K>,
```

Expand description

Parse header value to string according to name.
