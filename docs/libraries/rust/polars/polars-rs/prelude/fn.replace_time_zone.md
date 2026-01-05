# Function replace_time_zone Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/datetime/replace_time_zone.rs.html#12-17" class="src">Source</a>

``` rust
pub fn replace_time_zone(
    datetime: &Logical<DatetimeType, Int64Type>,
    time_zone: Option<&TimeZone>,
    ambiguous: &ChunkedArray<StringType>,
    non_existent: NonExistent,
) -> Result<Logical<DatetimeType, Int64Type>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
