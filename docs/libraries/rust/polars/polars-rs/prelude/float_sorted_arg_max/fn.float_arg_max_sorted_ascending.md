# Function float_arg_max_sorted_ascending Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/float_sorted_arg_max.rs.html#45-47" class="src">Source</a>

``` rust
pub fn float_arg_max_sorted_ascending<T>(ca: &ChunkedArray<T>) -> usizewhere
    T: PolarsNumericType,
```

Expand description

## <a href="https://docs.rs/polars/latest/polars/prelude/float_sorted_arg_max/fn.float_arg_max_sorted_ascending.html#safety" class="doc-anchor">§</a>Safety

`ca` has a float dtype, has at least 1 non-null value and is sorted ascending
