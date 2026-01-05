# Function lst_get Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/list/get.rs.html#8" class="src">Source</a>

``` rust
pub fn lst_get(
    ca: &ChunkedArray<ListType>,
    index: &ChunkedArray<Int64Type>,
    null_on_oob: bool,
) -> Result<Column, PolarsError>
```

Available on **crate feature `polars-ops`** only.
