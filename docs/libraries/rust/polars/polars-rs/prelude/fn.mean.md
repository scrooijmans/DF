# Function mean Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/syntactic_sugar.rs.html#21" class="src">Source</a>

``` rust
pub fn mean(name: &str) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Find the mean of all the values in the column named `name`. Shorthand for `col(name).mean()`.
