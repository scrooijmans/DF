# Function is_null Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/syntactic_sugar.rs.html#46" class="src">Source</a>

``` rust
pub fn is_null(expr: Expr) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

A column which is `true` wherever `expr` is null, `false` elsewhere.
