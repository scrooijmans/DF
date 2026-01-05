# Function impl_replace_time_zone Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/datetime/replace_time_zone.rs.html#115-123" class="src">Source</a>

``` rust
pub fn impl_replace_time_zone(
    datetime: &Logical<DatetimeType, Int64Type>,
    ambiguous: &ChunkedArray<StringType>,
    non_existent: NonExistent,
    timestamp_to_datetime: fn(i64) -> NaiveDateTime,
    datetime_to_timestamp: fn(NaiveDateTime) -> i64,
    from_tz: &Tz,
    to_tz: &Tz,
) -> Result<ChunkedArray<Int64Type>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
