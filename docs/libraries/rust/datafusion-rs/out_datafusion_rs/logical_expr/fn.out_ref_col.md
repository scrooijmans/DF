# Function out_ref_col Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr_fn.rs.html#73" class="src">Source</a>

``` rust
pub fn out_ref_col(dt: DataType, ident: impl Into<Column>) -> Expr
```

Expand description

Create an out reference column which hold a reference that has been resolved to a field outside of the current plan.
