# Function try_unary_elementwise Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#93-101" class="src">Source</a>

``` rust
pub fn try_unary_elementwise<'a, T, V, F, K, E>(
    ca: &'a ChunkedArray<T>,
    op: F,
) -> Result<ChunkedArray<V>, E>where
    T: PolarsDataType,
    V: PolarsDataType,
    F: FnMut(Option<<T as PolarsDataType>::Physical<'a>>) -> Result<Option<K>, E>,
    <V as PolarsDataType>::Array: ArrayFromIter<Option<K>>,
```
