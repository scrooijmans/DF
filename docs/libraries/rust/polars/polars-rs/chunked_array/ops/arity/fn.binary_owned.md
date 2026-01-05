# Function binary_owned Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#481-491" class="src">Source</a>

``` rust
pub fn binary_owned<L, R, V, F, Arr>(
    lhs: ChunkedArray<L>,
    rhs: ChunkedArray<R>,
    op: F,
) -> ChunkedArray<V>where
    L: PolarsDataType,
    R: PolarsDataType,
    V: PolarsDataType<Array = Arr>,
    Arr: Array,
    F: FnMut(<L as PolarsDataType>::Array, <R as PolarsDataType>::Array) -> Arr,
```

Expand description

Applies a kernel that produces `Array` types.
