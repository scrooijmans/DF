# Function is_close Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/is_close.rs.html#9-15" class="src">Source</a>

``` rust
pub fn is_close(
    s: &Series,
    other: &Series,
    abs_tol: f64,
    rel_tol: f64,
    nans_equal: bool,
) -> Result<ChunkedArray<BooleanType>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
