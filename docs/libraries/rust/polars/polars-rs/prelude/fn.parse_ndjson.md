# Function parse_ndjson Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ndjson/core.rs.html#418-423" class="src">Source</a>

``` rust
pub fn parse_ndjson(
    bytes: &[u8],
    n_rows_hint: Option<usize>,
    schema: &Schema<DataType>,
    ignore_errors: bool,
) -> Result<DataFrame, PolarsError>
```

Available on **crate feature `polars-io`** only.
