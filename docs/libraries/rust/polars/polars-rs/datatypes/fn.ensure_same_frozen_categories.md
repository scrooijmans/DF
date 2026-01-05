# Function ensure_same_frozen_categories Copy item path

<a href="https://docs.rs/polars-dtype/0.51.0/x86_64-unknown-linux-gnu/src/polars_dtype/categorical/mod.rs.html#358-361" class="src">Source</a>

``` rust
pub fn ensure_same_frozen_categories(
    left: &Arc<FrozenCategories>,
    right: &Arc<FrozenCategories>,
) -> Result<(), PolarsError>
```
