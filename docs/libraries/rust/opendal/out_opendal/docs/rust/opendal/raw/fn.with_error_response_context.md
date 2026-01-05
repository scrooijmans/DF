# Function with_error_response_context Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/error.rs.html#55-68" class="src">Source</a>

``` rust
pub fn with_error_response_context(err: Error, parts: Parts) -> Error
```

Expand description

Add response context to error.

This helper function will:

- remove sensitive or useless headers from parts.
- fetch uri if parts extensions contains `Uri`.
