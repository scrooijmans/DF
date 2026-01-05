# Function replace_date Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/replace.rs.html#150-155" class="src">Source</a>

``` rust
pub fn replace_date(
    ca: &Logical<DateType, Int32Type>,
    year: &ChunkedArray<Int32Type>,
    month: &ChunkedArray<Int8Type>,
    day: &ChunkedArray<Int8Type>,
) -> Result<Logical<DateType, Int32Type>, PolarsError>
```

Available on **crate feature `temporal`** only.

Expand description

Replace specific time component of a `DateChunked` with a specified value.
