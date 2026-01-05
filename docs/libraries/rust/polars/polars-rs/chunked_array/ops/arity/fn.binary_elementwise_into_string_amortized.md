# Function binary_elementwise_into_string_amortized Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#354-362" class="src">Source</a>

``` rust
pub fn binary_elementwise_into_string_amortized<T, U, F>(
    lhs: &ChunkedArray<T>,
    rhs: &ChunkedArray<U>,
    op: F,
) -> ChunkedArray<StringType>where
    T: PolarsDataType,
    U: PolarsDataType,
    F: for<'a> FnMut(<T as PolarsDataType>::Physical<'a>, <U as PolarsDataType>::Physical<'a>, &mut String),
```

Expand description

Apply elementwise binary function which produces string, amortising allocations.

Currently unused within Polars itself, but it’s a useful utility for plugin authors.
