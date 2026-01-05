# Function binary_elementwise Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#206-218" class="src">Source</a>

``` rust
pub fn binary_elementwise<T, U, V, F>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    U: PolarsDataType,
    V: PolarsDataType,
    F: for<'a> BinaryFnMut<Option<<T as PolarsDataType>::Physical<'a>>, Option<<U as PolarsDataType>::Physical<'a>>>,
    <V as PolarsDataType>::Array: for<'a> ArrayFromIter<<F as BinaryFnMut<Option<<T as PolarsDataType>::Physical<'a>>, Option<<U as PolarsDataType>::Physical<'a>>>>::Ret>,
```
