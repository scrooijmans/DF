# Function when Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#365" class="src">Source</a>

``` rust
pub fn when(when: Expr, then: Expr) -> CaseBuilder
```

Expand description

Create a CASE WHEN statement with boolean WHEN expressions and no base expression.
