# Function is_in Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/is_in.rs.html#613" class="src">Source</a>

``` rust
pub fn is_in(
    s: &Series,
    other: &Series,
    nulls_equal: bool,
) -> Result<ChunkedArray<BooleanType>, PolarsError>
```

Available on **crate feature `polars-ops`** only.
