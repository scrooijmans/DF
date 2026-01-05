# Function physical_exprs_contains Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#56-59" class="src">Source</a>

``` rust
pub fn physical_exprs_contains(
    physical_exprs: &[Arc<dyn PhysicalExpr>],
    expr: &Arc<dyn PhysicalExpr>,
) -> bool
```

Expand description

This function is similar to the `contains` method of `Vec`. It finds whether `expr` is among `physical_exprs`.
