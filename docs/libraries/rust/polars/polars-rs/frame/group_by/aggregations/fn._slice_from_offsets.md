# Function \_slice_from_offsets Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/aggregations/mod.rs.html#155-157" class="src">Source</a>

``` rust
pub fn _slice_from_offsets<T>(
    ca: &ChunkedArray<T>,
    first: u32,
    len: u32,
) -> ChunkedArray<T>where
    T: PolarsDataType,
```
