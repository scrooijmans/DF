# Function build_primitive_array Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#937-966" class="src">Source</a>

``` rust
fn build_primitive_array<T: ArrowPrimitiveType + Parser>(
    line_number: usize,
    rows: &StringRecords<'_>,
    col_idx: usize,
    null_regex: &NullRegex,
) -> Result<ArrayRef, ArrowError>
```
