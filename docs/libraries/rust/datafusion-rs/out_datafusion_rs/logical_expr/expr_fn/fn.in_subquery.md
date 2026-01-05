# Function in_subquery Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#262" class="src">Source</a>

``` rust
pub fn in_subquery(expr: Expr, subquery: Arc<LogicalPlan>) -> Expr
```

Expand description

Create an IN subquery expression
