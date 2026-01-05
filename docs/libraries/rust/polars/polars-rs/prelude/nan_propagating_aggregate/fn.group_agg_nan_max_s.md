# Function group_agg_nan_max_s Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/nan_propagating_aggregate.rs.html#210" class="src">Source</a>

``` rust
pub unsafe fn group_agg_nan_max_s(s: &Series, groups: &GroupsType) -> Series
```

Available on **crate feature `polars-ops`** only.

Expand description

## <a href="https://docs.rs/polars/latest/polars/prelude/nan_propagating_aggregate/fn.group_agg_nan_max_s.html#safety" class="doc-anchor">§</a>Safety

`groups` must be in bounds.
