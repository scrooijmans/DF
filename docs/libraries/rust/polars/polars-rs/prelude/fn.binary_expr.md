# Function binary_expr Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/arity.rs.html#149" class="src">Source</a>

``` rust
pub fn binary_expr(l: Expr, op: Operator, r: Expr) -> Expr
```

Available on **crate feature `lazy`** only.

Expand description

Compute `op(l, r)` (or equivalently `l op r`). `l` and `r` must have types compatible with the Operator.
