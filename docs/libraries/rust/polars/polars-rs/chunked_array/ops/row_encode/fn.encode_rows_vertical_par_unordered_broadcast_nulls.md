# Function encode_rows_vertical_par_unordered_broadcast_nulls Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/row_encode.rs.html#33-35" class="src">Source</a>

``` rust
pub fn encode_rows_vertical_par_unordered_broadcast_nulls(
    by: &[Column],
) -> Result<ChunkedArray<BinaryOffsetType>, PolarsError>
```
