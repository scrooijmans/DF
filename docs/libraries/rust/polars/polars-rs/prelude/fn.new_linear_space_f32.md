# Function new_linear_space_f32 Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/linear_space.rs.html#19-25" class="src">Source</a>

``` rust
pub fn new_linear_space_f32(
    start: f32,
    end: f32,
    n: u64,
    closed: ClosedInterval,
    name: PlSmallStr,
) -> Result<ChunkedArray<Float32Type>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
