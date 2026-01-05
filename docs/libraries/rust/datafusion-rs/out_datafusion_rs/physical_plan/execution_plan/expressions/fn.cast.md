# Function cast Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/expressions/cast.rs.html#243-247" class="src">Source</a>

``` rust
pub fn cast(
    expr: Arc<dyn PhysicalExpr>,
    input_schema: &Schema,
    cast_type: DataType,
) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>
```

Expand description

Return a PhysicalExpression representing `expr` casted to `cast_type`, if any casting is needed.

Note that such casts may lose type information
