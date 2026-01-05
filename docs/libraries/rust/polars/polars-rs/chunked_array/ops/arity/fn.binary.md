# Function binary Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#465-475" class="src">Source</a>

``` rust
pub fn binary<T, U, V, F, Arr>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    U: PolarsDataType,
    V: PolarsDataType<Array = Arr>,
    Arr: Array,
    F: FnMut(&<T as PolarsDataType>::Array, &<U as PolarsDataType>::Array) -> Arr,
```

Expand description

Applies a kernel that produces `Array` types.
