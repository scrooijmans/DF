# Function percent_decode_path Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/uri.rs.html#61-66" class="src">Source</a>

``` rust
pub fn percent_decode_path(path: &str) -> String
```

Expand description

percent_decode_path will do percent decoding for http decode path.

If the input is not percent encoded or not valid utf8, return the input.
