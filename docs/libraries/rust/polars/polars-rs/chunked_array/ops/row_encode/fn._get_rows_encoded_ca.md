# Function \_get_rows_encoded_ca Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/row_encode.rs.html#222-227" class="src">Source</a>

``` rust
pub fn _get_rows_encoded_ca(
    name: PlSmallStr,
    by: &[Column],
    descending: &[bool],
    nulls_last: &[bool],
) -> Result<ChunkedArray<BinaryOffsetType>, PolarsError>
```
