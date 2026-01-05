# Function not_in_subquery Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#276" class="src">Source</a>

``` rust
pub fn not_in_subquery(expr: Expr, subquery: Arc<LogicalPlan>) -> Expr
```

Expand description

Create a NOT IN subquery expression
