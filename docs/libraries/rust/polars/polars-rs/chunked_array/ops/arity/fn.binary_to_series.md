# Function binary_to_series Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#560-568" class="src">Source</a>

``` rust
pub fn binary_to_series<T, U, F>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
) -> Result<Series, PolarsError>where
    T: PolarsDataType,
    U: PolarsDataType,
    F: FnMut(&<T as PolarsDataType>::Array, &<U as PolarsDataType>::Array) -> Box<dyn Array>,
```
