# Function ca_nan_agg Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/nan_propagating_aggregate.rs.html#15-18" class="src">Source</a>

``` rust
pub fn ca_nan_agg<T, Agg>(
    ca: &ChunkedArray<T>,
    min_or_max_fn: Agg,
) -> Option<<T as PolarsNumericType>::Native>where
    T: PolarsFloatType,
    Agg: Fn(<T as PolarsNumericType>::Native, <T as PolarsNumericType>::Native) -> <T as PolarsNumericType>::Native + Copy,
```

Available on **crate feature `polars-ops`** only.
