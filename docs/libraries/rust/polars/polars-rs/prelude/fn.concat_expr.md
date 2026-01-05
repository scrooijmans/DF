# Function concat_expr Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/concat.rs.html#73-76" class="src">Source</a>

``` rust
pub fn concat_expr<E, IE>(s: E, rechunk: bool) -> Result<Expr, PolarsError>where
    E: AsRef<[IE]>,
    IE: Into<Expr> + Clone,
```

Available on **crate feature `lazy`** only.
