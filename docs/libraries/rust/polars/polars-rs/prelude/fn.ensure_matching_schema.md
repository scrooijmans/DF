# Function ensure_matching_schema Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/schema/mod.rs.html#139-144" class="src">Source</a>

``` rust
pub fn ensure_matching_schema<D>(
    lhs: &Schema<D>,
    rhs: &Schema<D>,
) -> Result<(), PolarsError>where
    Schema<D>: SchemaNamesAndDtypes,
```
