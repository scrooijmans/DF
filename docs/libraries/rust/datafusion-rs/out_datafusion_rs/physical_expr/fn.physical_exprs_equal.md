# Function physical_exprs_equal Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#66-69" class="src">Source</a>

``` rust
pub fn physical_exprs_equal(
    lhs: &[Arc<dyn PhysicalExpr>],
    rhs: &[Arc<dyn PhysicalExpr>],
) -> bool
```

Expand description

Checks whether the given physical expression slices are equal.
