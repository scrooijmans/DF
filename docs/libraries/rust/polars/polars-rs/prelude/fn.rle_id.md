# Function rle_id Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/rle.rs.html#70" class="src">Source</a>

``` rust
pub fn rle_id(s: &Column) -> Result<Column, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Similar to `rle`, but maps values to run IDs.
