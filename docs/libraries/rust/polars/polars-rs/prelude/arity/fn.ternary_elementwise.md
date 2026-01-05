# Function ternary_elementwise Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#650-672" class="src">Source</a>

``` rust
pub fn ternary_elementwise<T, U, V, G, F>(
    ca1: &ChunkedArray<T>,
    ca2: &ChunkedArray<U>,
    ca3: &ChunkedArray<G>,
    op: F,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    U: PolarsDataType,
    G: PolarsDataType,
    V: PolarsDataType,
    F: for<'a> TernaryFnMut<Option<<T as PolarsDataType>::Physical<'a>>, Option<<U as PolarsDataType>::Physical<'a>>, Option<<G as PolarsDataType>::Physical<'a>>>,
    <V as PolarsDataType>::Array: for<'a> ArrayFromIter<<F as TernaryFnMut<Option<<T as PolarsDataType>::Physical<'a>>, Option<<U as PolarsDataType>::Physical<'a>>, Option<<G as PolarsDataType>::Physical<'a>>>>::Ret>,
```
