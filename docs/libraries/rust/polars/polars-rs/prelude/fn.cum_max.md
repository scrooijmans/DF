# Function cum_max Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/cum_agg.rs.html#381" class="src">Source</a>

``` rust
pub fn cum_max(s: &Series, reverse: bool) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Get an array with the cumulative max computed at every element.
