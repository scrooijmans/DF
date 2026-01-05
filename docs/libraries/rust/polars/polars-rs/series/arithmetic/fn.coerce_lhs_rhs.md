# Function coerce_lhs_rhs Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/arithmetic/borrowed.rs.html#328-331" class="src">Source</a>

``` rust
pub fn coerce_lhs_rhs<'a>(
    lhs: &'a Series,
    rhs: &'a Series,
) -> Result<(Cow<'a, Series>, Cow<'a, Series>), PolarsError>
```
