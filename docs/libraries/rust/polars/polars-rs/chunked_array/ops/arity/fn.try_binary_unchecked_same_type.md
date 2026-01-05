# Function try_binary_unchecked_same_type Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#584-595" class="src">Source</a>

``` rust
pub unsafe fn try_binary_unchecked_same_type<T, U, F, E>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
    keep_sorted: bool,
    keep_fast_explode: bool,
) -> Result<ChunkedArray<T>, E>where
    T: PolarsDataType,
    U: PolarsDataType,
    F: FnMut(&<T as PolarsDataType>::Array, &<U as PolarsDataType>::Array) -> Result<Box<dyn Array>, E>,
    E: Error,
```

Expand description

Applies a kernel that produces `ArrayRef` of the same type.

## <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/fn.try_binary_unchecked_same_type.html#safety" class="doc-anchor">§</a>Safety

Caller must ensure that the returned `ArrayRef` belongs to `T: PolarsDataType`.
