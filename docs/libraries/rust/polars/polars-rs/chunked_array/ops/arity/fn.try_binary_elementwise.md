# Function try_binary_elementwise Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#285-295" class="src">Source</a>

``` rust
pub fn try_binary_elementwise<T, U, V, F, K, E>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
) -> Result<ChunkedArray<V>, E>where
    T: PolarsDataType,
    U: PolarsDataType,
    V: PolarsDataType,
    F: for<'a> FnMut(Option<<T as PolarsDataType>::Physical<'a>>, Option<<U as PolarsDataType>::Physical<'a>>) -> Result<Option<K>, E>,
    <V as PolarsDataType>::Array: ArrayFromIter<Option<K>>,
```
