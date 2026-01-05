# Function to_datetime Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/infer.rs.html#471-478" class="src">Source</a>

``` rust
pub fn to_datetime(
    ca: &ChunkedArray<StringType>,
    tu: TimeUnit,
    tz: Option<&TimeZone>,
    _ambiguous: &ChunkedArray<StringType>,
    ensure_matching_time_zone: bool,
) -> Result<Logical<DatetimeType, Int64Type>, PolarsError>
```

Available on **crate feature `temporal`** only.
