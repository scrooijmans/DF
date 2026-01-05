# Function rle Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/rle.rs.html#46" class="src">Source</a>

``` rust
pub fn rle(s: &Column) -> Result<Column, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Get the lengths of runs of identical values.
