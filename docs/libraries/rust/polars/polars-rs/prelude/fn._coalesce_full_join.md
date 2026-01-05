# Function \_coalesce_full_join Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/join/general.rs.html#51-57" class="src">Source</a>

``` rust
pub fn _coalesce_full_join(
    df: DataFrame,
    keys_left: &[PlSmallStr],
    keys_right: &[PlSmallStr],
    suffix: Option<PlSmallStr>,
    df_left: &DataFrame,
) -> DataFrame
```

Available on **crate feature `polars-ops`** only.
