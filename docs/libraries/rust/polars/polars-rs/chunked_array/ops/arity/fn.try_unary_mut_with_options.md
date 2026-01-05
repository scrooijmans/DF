# Function try_unary_mut_with_options Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#191-200" class="src">Source</a>

``` rust
pub fn try_unary_mut_with_options<T, V, F, Arr, E>(
    ca: &ChunkedArray<T>,
    op: F,
) -> Result<ChunkedArray<V>, E>where
    T: PolarsDataType,
    V: PolarsDataType<Array = Arr>,
    Arr: Array + StaticArray,
    F: FnMut(&<T as PolarsDataType>::Array) -> Result<Arr, E>,
    E: Error,
```
