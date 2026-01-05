# Function clip_min Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/clip.rs.html#91" class="src">Source</a>

``` rust
pub fn clip_min(s: &Series, min: &Series) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Set values below the given minimum to the minimum value.
