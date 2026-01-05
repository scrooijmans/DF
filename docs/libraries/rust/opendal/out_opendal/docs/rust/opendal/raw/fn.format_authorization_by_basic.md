# Function format_authorization_by_basic Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#229-240" class="src">Source</a>

``` rust
pub fn format_authorization_by_basic(
    username: &str,
    password: &str,
) -> Result<String>
```

Expand description

format authorization header by basic auth.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.format_authorization_by_basic.html#errors" class="doc-anchor">Â§</a>Errors

If input username is empty, function will return an unexpected error.
