# Function lower_bound_chunks Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/search_sorted.rs.html#48-56" class="src">Source</a>

``` rust
pub fn lower_bound_chunks<'a, T, F>(
    chunks: &[&'a <T as PolarsDataType>::Array],
    search_values: impl Iterator<Item = Option<<T as PolarsDataType>::Physical<'a>>>,
    null_idx: u32,
    f: F,
) -> Vec<u32>where
    T: PolarsDataType,
    F: Fn(&'a <T as PolarsDataType>::Array, usize, &<T as PolarsDataType>::Physical<'a>) -> bool,
```

Expand description

Search through a series of chunks for the first position where f(x) is true, assuming it is first always false and then always true.

It repeats this for each value in search_values. If the search value is null null_idx is returned.

Assumes the chunks are non-empty.
