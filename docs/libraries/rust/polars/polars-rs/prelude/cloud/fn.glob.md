# Function glob Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/glob.rs.html#203" class="src">Source</a>

``` rust
pub async fn glob(
    url: &str,
    cloud_options: Option<&CloudOptions>,
) -> Result<Vec<String>, PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

List files with a prefix derived from the pattern.
