# Function \_rolling_apply_agg_window_nulls Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/aggregations/mod.rs.html#67-76" class="src">Source</a>

``` rust
pub fn _rolling_apply_agg_window_nulls<'a, Agg, T, O>(
    values: &'a [T],
    validity: &'a Bitmap,
    offsets: O,
    params: Option<RollingFnParams>,
) -> PrimitiveArray<T>where
    O: Iterator<Item = (u32, u32)> + TrustedLen,
    Agg: RollingAggWindowNulls<'a, T>,
    T: IsFloat + NativeType,
```
