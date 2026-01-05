# Function build_decimal_array Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#903-934" class="src">Source</a>

``` rust
fn build_decimal_array<T: DecimalType>(
    _line_number: usize,
    rows: &StringRecords<'_>,
    col_idx: usize,
    precision: u8,
    scale: i8,
    null_regex: &NullRegex,
) -> Result<ArrayRef, ArrowError>
```
