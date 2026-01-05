# Function \_agg_helper_slice Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/aggregations/mod.rs.html#204-207" class="src">Source</a>

``` rust
pub fn _agg_helper_slice<T, F>(groups: &[[u32; 2]], f: F) -> Serieswhere
    F: Fn([u32; 2]) -> Option<<T as PolarsNumericType>::Native> + Send + Sync,
    T: PolarsNumericType,
```
