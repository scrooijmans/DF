# Function is_between Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/is_between.rs.html#7-12" class="src">Source</a>

``` rust
pub fn is_between(
    s: &Series,
    lower: &Series,
    upper: &Series,
    closed: ClosedInterval,
) -> Result<ChunkedArray<BooleanType>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
