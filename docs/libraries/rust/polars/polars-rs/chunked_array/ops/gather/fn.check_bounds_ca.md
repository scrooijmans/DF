# Function check_bounds_ca Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/gather.rs.html#26" class="src">Source</a>

``` rust
pub fn check_bounds_ca(
    indices: &ChunkedArray<UInt32Type>,
    len: u32,
) -> Result<(), PolarsError>
```
