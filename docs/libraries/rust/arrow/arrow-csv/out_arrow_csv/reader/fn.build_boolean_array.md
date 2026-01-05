# Function build_boolean_array Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#1025-1054" class="src">Source</a>

``` rust
fn build_boolean_array(
    line_number: usize,
    rows: &StringRecords<'_>,
    col_idx: usize,
    null_regex: &NullRegex,
) -> Result<ArrayRef, ArrowError>
```
