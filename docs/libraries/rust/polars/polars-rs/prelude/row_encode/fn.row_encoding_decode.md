# Function row_encoding_decode Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/row_encode.rs.html#249-253" class="src">Source</a>

``` rust
pub fn row_encoding_decode(
    ca: &ChunkedArray<BinaryOffsetType>,
    fields: &[Field],
    opts: &[RowEncodingOptions],
) -> Result<ChunkedArray<StructType>, PolarsError>
```
