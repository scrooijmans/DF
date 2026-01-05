# Function rle_lengths Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/rle.rs.html#8" class="src">Source</a>

``` rust
pub fn rle_lengths(
    s: &Column,
    lengths: &mut Vec<u32>,
) -> Result<(), PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Get the run-Lengths of values.
