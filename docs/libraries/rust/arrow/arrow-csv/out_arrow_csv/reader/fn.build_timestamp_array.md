# Function build_timestamp_array Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#968-983" class="src">Source</a>

``` rust
fn build_timestamp_array<T: ArrowTimestampType>(
    line_number: usize,
    rows: &StringRecords<'_>,
    col_idx: usize,
    timezone: Option<&str>,
    null_regex: &NullRegex,
) -> Result<ArrayRef, ArrowError>
```
