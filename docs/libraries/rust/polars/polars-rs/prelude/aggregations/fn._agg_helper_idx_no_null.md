# Function \_agg_helper_idx_no_null Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/aggregations/mod.rs.html#173-176" class="src">Source</a>

``` rust
pub fn _agg_helper_idx_no_null<T, F>(groups: &GroupsIdx, f: F) -> Serieswhere
    F: Fn((u32, &UnitVec<u32>)) -> <T as PolarsNumericType>::Native + Send + Sync,
    T: PolarsNumericType,
```

Expand description

Same helper as `_agg_helper_idx` but for aggregations that don’t return an Option.
