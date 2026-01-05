# Function unary_kernel Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#46-51" class="src">Source</a>

``` rust
pub fn unary_kernel<T, V, F, Arr>(
    ca: &ChunkedArray<T>,
    op: F,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    V: PolarsDataType<Array = Arr>,
    Arr: Array,
    F: FnMut(&<T as PolarsDataType>::Array) -> Arr,
```

Expand description

Applies a kernel that produces `Array` types.
