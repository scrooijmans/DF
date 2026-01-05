# Function quantile Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/syntactic_sugar.rs.html#36" class="src">Source</a>

``` rust
pub fn quantile(name: &str, quantile: Expr, method: QuantileMethod) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Find a specific quantile of all the values in the column named `name`.
