# Function binary_mut_values Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#392-403" class="src">Source</a>

``` rust
pub fn binary_mut_values<T, U, V, F, Arr>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
    name: PlSmallStr,
) -> ChunkedArray<V>where
    T: PolarsDataType,
    U: PolarsDataType,
    V: PolarsDataType<Array = Arr>,
    Arr: Array + StaticArray,
    F: FnMut(&<T as PolarsDataType>::Array, &<U as PolarsDataType>::Array) -> Arr,
```

Expand description

Applies a kernel that produces `Array` types.

Intended for kernels that apply on values, this function will filter out any results which do not have two non-null inputs.
