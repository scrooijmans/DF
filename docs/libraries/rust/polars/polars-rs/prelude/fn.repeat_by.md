# Function repeat_by Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/repeat_by.rs.html#256" class="src">Source</a>

``` rust
pub fn repeat_by(
    s: &Series,
    by: &ChunkedArray<UInt32Type>,
) -> Result<ChunkedArray<ListType>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
