# Function \_struct_arithmetic Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/arithmetic/borrowed.rs.html#416-420" class="src">Source</a>

``` rust
pub fn _struct_arithmetic<F>(
    s: &Series,
    rhs: &Series,
    func: F,
) -> Result<Series, PolarsError>where
    F: FnMut(&Series, &Series) -> Result<Series, PolarsError>,
```
