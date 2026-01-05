# Function parse_into_metadata Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#154-199" class="src">Source</a>

``` rust
pub fn parse_into_metadata(path: &str, headers: &HeaderMap) -> Result<Metadata>
```

Expand description

parse_into_metadata will parse standards http headers into Metadata.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_into_metadata.html#notes" class="doc-anchor">Â§</a>Notes

parse_into_metadata only handles the standard behavior of http headers. If services have their own logic, they should update the parsed metadata on demand.
