# Function \_agg_helper_idx Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/aggregations/mod.rs.html#163-166" class="src">Source</a>

``` rust
pub fn _agg_helper_idx<T, F>(groups: &GroupsIdx, f: F) -> Serieswhere
    F: Fn((u32, &UnitVec<u32>)) -> Option<<T as PolarsNumericType>::Native> + Send + Sync,
    T: PolarsNumericType,
```

Expand description

Helper that combines the groups into a parallel iterator over `(first, all): (u32, &Vec<u32>)`.
