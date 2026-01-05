# Function cum_min Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/cum_agg.rs.html#343" class="src">Source</a>

``` rust
pub fn cum_min(s: &Series, reverse: bool) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Get an array with the cumulative min computed at every element.
