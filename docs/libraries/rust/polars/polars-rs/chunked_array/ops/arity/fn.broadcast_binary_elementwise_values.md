# Function broadcast_binary_elementwise_values Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#742-752" class="src">Source</a>

``` rust
pub fn broadcast_binary_elementwise_values<T, U, V, F, K>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    U: PolarsDataType,
    V: PolarsDataType,
    F: for<'a> FnMut(<T as PolarsDataType>::Physical<'a>, <U as PolarsDataType>::Physical<'a>) -> K,
    <V as PolarsDataType>::Array: ArrayFromIter<K>,
```
