# Function \_arg_bottom_k Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/sort/arg_bottom_k.rs.html#33-37" class="src">Source</a>

``` rust
pub fn _arg_bottom_k(
    k: usize,
    by_column: &[Column],
    sort_options: &mut SortMultipleOptions,
) -> Result<NoNull<ChunkedArray<UInt32Type>>, PolarsError>
```

Expand description

Return the indices of the bottom k elements.

Similar to .argsort() then .slice(0, k) but with a more efficient implementation.
