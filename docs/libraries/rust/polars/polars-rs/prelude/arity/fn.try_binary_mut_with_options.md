# Function try_binary_mut_with_options Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#442-454" class="src">Source</a>

``` rust
pub fn try_binary_mut_with_options<T, U, V, F, Arr, E>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
    name: PlSmallStr,
) -> Result<ChunkedArray<V>, E>where
    T: PolarsDataType,
    U: PolarsDataType,
    V: PolarsDataType<Array = Arr>,
    Arr: Array,
    F: FnMut(&<T as PolarsDataType>::Array, &<U as PolarsDataType>::Array) -> Result<Arr, E>,
    E: Error,
```
