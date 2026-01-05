# Function is_null Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/expressions/is_null.rs.html#113" class="src">Source</a>

``` rust
pub fn is_null(
    arg: Arc<dyn PhysicalExpr>,
) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>
```

Expand description

Create an IS NULL expression
