# Function coalesce Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/horizontal.rs.html#174" class="src">Source</a>

``` rust
pub fn coalesce(exprs: &[Expr]) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Folds the expressions from left to right keeping the first non-null values.

It is an error to provide an empty `exprs`.
