# Function clip Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/clip.rs.html#6" class="src">Source</a>

``` rust
pub fn clip(
    s: &Series,
    min: &Series,
    max: &Series,
) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Set values outside the given boundaries to the boundary value.
