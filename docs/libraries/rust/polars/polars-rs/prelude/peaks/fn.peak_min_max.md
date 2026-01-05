# Function peak_min_max Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/peaks.rs.html#4-9" class="src">Source</a>

``` rust
pub fn peak_min_max(
    column: &Column,
    start: &AnyValue<'_>,
    end: &AnyValue<'_>,
    is_peak_max: bool,
) -> Result<ChunkedArray<BooleanType>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
