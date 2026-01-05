# Function unary_elementwise Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#72-77" class="src">Source</a>

``` rust
pub fn unary_elementwise<'a, T, V, F>(
    ca: &'a ChunkedArray<T>,
    op: F,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    V: PolarsDataType,
    F: UnaryFnMut<Option<<T as PolarsDataType>::Physical<'a>>>,
    <V as PolarsDataType>::Array: ArrayFromIter<<F as UnaryFnMut<Option<<T as PolarsDataType>::Physical<'a>>>>::Ret>,
```
