# Function ensure_duration_matches_dtype Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/windows/duration.rs.html#1046-1050" class="src">Source</a>

``` rust
pub fn ensure_duration_matches_dtype(
    duration: Duration,
    dtype: &DataType,
    variable_name: &str,
) -> Result<(), PolarsError>
```

Available on **crate feature `temporal`** only.
