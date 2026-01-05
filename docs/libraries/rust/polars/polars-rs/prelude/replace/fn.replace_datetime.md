# Function replace_datetime Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/replace.rs.html#8-18" class="src">Source</a>

``` rust
pub fn replace_datetime(
    ca: &Logical<DatetimeType, Int64Type>,
    year: &ChunkedArray<Int32Type>,
    month: &ChunkedArray<Int8Type>,
    day: &ChunkedArray<Int8Type>,
    hour: &ChunkedArray<Int8Type>,
    minute: &ChunkedArray<Int8Type>,
    second: &ChunkedArray<Int8Type>,
    nanosecond: &ChunkedArray<Int32Type>,
    ambiguous: &ChunkedArray<StringType>,
) -> Result<Logical<DatetimeType, Int64Type>, PolarsError>
```

Available on **crate feature `temporal`** only.

Expand description

Replace specific time component of a `DatetimeChunked` with a specified value.
