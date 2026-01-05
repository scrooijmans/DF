# Function add_offset_to_expr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#38-41" class="src">Source</a>

``` rust
pub fn add_offset_to_expr(
    expr: Arc<dyn PhysicalExpr>,
    offset: isize,
) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>
```

Expand description

Adds the `offset` value to `Column` indices inside `expr`. This function is generally used during the update of the right table schema in join operations.
