# Function convert_to_unsigned_index Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/index.rs.html#15" class="src">Source</a>

``` rust
pub fn convert_to_unsigned_index(
    s: &Series,
    target_len: usize,
) -> Result<ChunkedArray<UInt32Type>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
