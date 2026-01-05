# Function concat_df_diagonal Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/functions.rs.html#14" class="src">Source</a>

``` rust
pub fn concat_df_diagonal(dfs: &[DataFrame]) -> Result<DataFrame, PolarsError>
```

Expand description

Concat [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")s diagonally. Concat diagonally thereby combining different schemas.
