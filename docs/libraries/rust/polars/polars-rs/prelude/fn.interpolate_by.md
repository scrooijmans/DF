# Function interpolate_by Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/interpolation/interpolate_by.rs.html#266" class="src">Source</a>

``` rust
pub fn interpolate_by(
    s: &Column,
    by: &Column,
    by_is_sorted: bool,
) -> Result<Column, PolarsError>
```

Available on **crate feature `polars-ops`** only.
