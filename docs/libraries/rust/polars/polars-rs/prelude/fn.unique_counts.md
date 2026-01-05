# Function unique_counts Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/series/ops/unique.rs.html#29" class="src">Source</a>

``` rust
pub fn unique_counts(s: &Series) -> Result<Series, PolarsError>
```

Available on **crate feature `polars-ops`** only.

Expand description

Returns a count of the unique values in the order of appearance.
