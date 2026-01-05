# Function build_timestamp_array_impl Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#985-1022" class="src">Source</a>

``` rust
fn build_timestamp_array_impl<T: ArrowTimestampType, Tz: TimeZone>(
    line_number: usize,
    rows: &StringRecords<'_>,
    col_idx: usize,
    timezone: &Tz,
    null_regex: &NullRegex,
) -> Result<PrimitiveArray<T>, ArrowError>
```
