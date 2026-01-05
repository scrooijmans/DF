# Function percent_encode_path Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/uri.rs.html#54-56" class="src">Source</a>

``` rust
pub fn percent_encode_path(path: &str) -> String
```

Expand description

percent_encode_path will do percent encoding for http encode path.

Follows [encodeURIComponent](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) which will encode all non-ASCII characters except `A-Z a-z 0-9 - _ . ! ~ * ' ( )`

There is a special case for `/` in path: we will allow `/` in path as required by storage services like s3.
