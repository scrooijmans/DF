# Function apply_binary_kernel_broadcast_owned Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#826-839" class="src">Source</a>

``` rust
pub fn apply_binary_kernel_broadcast_owned<L, R, O, K, LK, RK>(
    lhs: ChunkedArray<L>,
    rhs: ChunkedArray<R>,
    kernel: K,
    lhs_broadcast_kernel: LK,
    rhs_broadcast_kernel: RK,
) -> ChunkedArray<O>where
    L: PolarsDataType,
    R: PolarsDataType,
    O: PolarsDataType,
    K: Fn(<L as PolarsDataType>::Array, <R as PolarsDataType>::Array) -> <O as PolarsDataType>::Array,
    LK: for<'a> Fn(<L as PolarsDataType>::Physical<'a>, <R as PolarsDataType>::Array) -> <O as PolarsDataType>::Array,
    RK: for<'a> Fn(<L as PolarsDataType>::Array, <R as PolarsDataType>::Physical<'a>) -> <O as PolarsDataType>::Array,
```
