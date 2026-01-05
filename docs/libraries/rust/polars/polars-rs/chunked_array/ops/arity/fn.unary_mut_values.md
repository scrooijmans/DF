# Function unary_mut_values Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#165-170" class="src">Source</a>

``` rust
pub fn unary_mut_values<T, V, F, Arr>(
    ca: &ChunkedArray<T>,
    op: F,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    V: PolarsDataType<Array = Arr>,
    Arr: Array + StaticArray,
    F: FnMut(&<T as PolarsDataType>::Array) -> Arr,
```

Expand description

Applies a kernel that produces `Array` types.

Intended for kernels that apply on values, this function will apply the validity mask afterwards.
