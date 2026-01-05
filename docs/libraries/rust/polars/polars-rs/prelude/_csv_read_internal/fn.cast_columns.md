# Function cast_columns Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/read_impl.rs.html#32-37" class="src">Source</a>

``` rust
pub fn cast_columns(
    df: &mut DataFrame,
    to_cast: &[Field],
    parallel: bool,
    ignore_errors: bool,
) -> Result<(), PolarsError>
```

Available on **crate feature `polars-io`** only.
