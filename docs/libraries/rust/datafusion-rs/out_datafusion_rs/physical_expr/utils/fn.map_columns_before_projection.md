# Function map_columns_before_projection Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/utils/mod.rs.html#107-110" class="src">Source</a>

``` rust
pub fn map_columns_before_projection(
    parent_required: &[Arc<dyn PhysicalExpr>],
    proj_exprs: &[(Arc<dyn PhysicalExpr>, String)],
) -> Vec<Arc<dyn PhysicalExpr>>
```

Expand description

This function maps back requirement after ProjectionExec to the Executor for its input.
