# Function try_ternary_elementwise Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#615-631" class="src">Source</a>

``` rust
pub fn try_ternary_elementwise<T, U, V, G, F, K, E>(
    ca1: &ChunkedArray<T>,
    ca2: &ChunkedArray<U>,
    ca3: &ChunkedArray<G>,
    op: F,
) -> Result<ChunkedArray<V>, E>where
    T: PolarsDataType,
    U: PolarsDataType,
    V: PolarsDataType,
    G: PolarsDataType,
    F: for<'a> FnMut(Option<<T as PolarsDataType>::Physical<'a>>, Option<<U as PolarsDataType>::Physical<'a>>, Option<<G as PolarsDataType>::Physical<'a>>) -> Result<Option<K>, E>,
    <V as PolarsDataType>::Array: ArrayFromIter<Option<K>>,
```
