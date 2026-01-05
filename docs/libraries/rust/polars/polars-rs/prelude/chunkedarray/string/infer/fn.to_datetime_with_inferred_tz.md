# Function to_datetime_with_inferred_tz Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/infer.rs.html#445-451" class="src">Source</a>

``` rust
pub fn to_datetime_with_inferred_tz(
    ca: &ChunkedArray<StringType>,
    tu: TimeUnit,
    strict: bool,
    exact: bool,
    ambiguous: &ChunkedArray<StringType>,
) -> Result<Logical<DatetimeType, Int64Type>, PolarsError>
```

Available on **crate feature `temporal`** only.
