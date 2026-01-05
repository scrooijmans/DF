# Function apply_binary_kernel_broadcast Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#776-789" class="src">Source</a>

``` rust
pub fn apply_binary_kernel_broadcast<'l, 'r, L, R, O, K, LK, RK>(
    lhs: &'l ChunkedArray<L>,
    rhs: &'r ChunkedArray<R>,
    kernel: K,
    lhs_broadcast_kernel: LK,
    rhs_broadcast_kernel: RK,
) -> ChunkedArray<O>where
    L: PolarsDataType,
    R: PolarsDataType,
    O: PolarsDataType,
    K: Fn(&<L as PolarsDataType>::Array, &<R as PolarsDataType>::Array) -> <O as PolarsDataType>::Array,
    LK: Fn(<L as PolarsDataType>::Physical<'l>, &<R as PolarsDataType>::Array) -> <O as PolarsDataType>::Array,
    RK: Fn(&<L as PolarsDataType>::Array, <R as PolarsDataType>::Physical<'r>) -> <O as PolarsDataType>::Array,
```
