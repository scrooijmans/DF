# Function format_authorization_by_bearer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/header.rs.html#247-256" class="src">Source</a>

``` rust
pub fn format_authorization_by_bearer(token: &str) -> Result<String>
```

Expand description

format authorization header by bearer token.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.format_authorization_by_bearer.html#errors" class="doc-anchor">Â§</a>Errors

If input token is empty, function will return an unexpected error.
