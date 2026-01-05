# Function concat_df_horizontal Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/horizontal.rs.html#72" class="src">Source</a>

``` rust
pub fn concat_df_horizontal(
    dfs: &[DataFrame],
    check_duplicates: bool,
) -> Result<DataFrame, PolarsError>
```

Expand description

Concat [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")s horizontally. Concat horizontally and extend with null values if lengths don’t match
