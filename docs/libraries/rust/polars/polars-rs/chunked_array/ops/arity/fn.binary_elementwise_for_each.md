# Function binary_elementwise_for_each Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#235-242" class="src">Source</a>

``` rust
pub fn binary_elementwise_for_each<'a, 'b, T, U, F>(
    lhs: &'a ChunkedArray<T>,
    rhs: &'b ChunkedArray<U>,
    op: F,
)where
    T: PolarsDataType,
    U: PolarsDataType,
    F: FnMut(Option<<T as PolarsDataType>::Physical<'a>>, Option<<U as PolarsDataType>::Physical<'b>>),
```
