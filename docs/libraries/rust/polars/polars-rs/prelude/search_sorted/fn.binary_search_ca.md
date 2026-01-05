# Function binary_search_ca Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/search_sorted.rs.html#124-132" class="src">Source</a>

``` rust
pub fn binary_search_ca<'a, T>(
    ca: &'a ChunkedArray<T>,
    search_values: impl Iterator<Item = Option<<T as PolarsDataType>::Physical<'a>>>,
    side: SearchSortedSide,
    descending: bool,
) -> Vec<u32>where
    T: PolarsDataType,
    <T as PolarsDataType>::Physical<'a>: TotalOrd + Debug + Copy,
```
