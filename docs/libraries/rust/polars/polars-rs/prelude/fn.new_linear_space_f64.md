# Function new_linear_space_f64 Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/linear_space.rs.html#62-68" class="src">Source</a>

``` rust
pub fn new_linear_space_f64(
    start: f64,
    end: f64,
    n: u64,
    closed: ClosedInterval,
    name: PlSmallStr,
) -> Result<ChunkedArray<Float64Type>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
