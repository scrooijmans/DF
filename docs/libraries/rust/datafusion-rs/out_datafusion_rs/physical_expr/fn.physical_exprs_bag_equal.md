# Function physical_exprs_bag_equal Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#75-78" class="src">Source</a>

``` rust
pub fn physical_exprs_bag_equal(
    lhs: &[Arc<dyn PhysicalExpr>],
    rhs: &[Arc<dyn PhysicalExpr>],
) -> bool
```

Expand description

Checks whether the given physical expression slices are equal in the sense of bags (multi-sets), disregarding their orderings.
