# Function binary_unchecked_same_type Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#529-539" class="src">Source</a>

``` rust
pub unsafe fn binary_unchecked_same_type<T, U, F>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
    keep_sorted: bool,
    keep_fast_explode: bool,
) -> ChunkedArray<T>where
    T: PolarsDataType,
    U: PolarsDataType,
    F: FnMut(&<T as PolarsDataType>::Array, &<U as PolarsDataType>::Array) -> Box<dyn Array>,
```

Expand description

Applies a kernel that produces `ArrayRef` of the same type.

## <a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_unchecked_same_type.html#safety" class="doc-anchor">§</a>Safety

Caller must ensure that the returned `ArrayRef` belongs to `T: PolarsDataType`.
