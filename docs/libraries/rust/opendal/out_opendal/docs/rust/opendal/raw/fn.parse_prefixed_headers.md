# Function parse_prefixed_headers Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#202-214" class="src">Source</a>

``` rust
pub fn parse_prefixed_headers(
    headers: &HeaderMap,
    prefix: &str,
) -> HashMap<String, String>
```

Expand description

Parse prefixed headers and return a map with the prefix of each header removed.
