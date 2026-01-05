# Function concat_arr Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/concat.rs.html#62" class="src">Source</a>

``` rust
pub fn concat_arr(input: Vec<Expr>) -> Result<Expr, PolarsError>
```

Available on **crate feature `lazy`** only.

Expand description

Horizontally concatenate columns into a single array-type column.
