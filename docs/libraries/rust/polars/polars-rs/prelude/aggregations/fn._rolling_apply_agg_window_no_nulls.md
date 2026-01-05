# Function \_rolling_apply_agg_window_no_nulls Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/aggregations/mod.rs.html#123-132" class="src">Source</a>

``` rust
pub fn _rolling_apply_agg_window_no_nulls<'a, Agg, T, O>(
    values: &'a [T],
    offsets: O,
    params: Option<RollingFnParams>,
) -> PrimitiveArray<T>where
    Agg: RollingAggWindowNoNulls<'a, T>,
    O: Iterator<Item = (u32, u32)> + TrustedLen,
    T: IsFloat + NativeType,
```
