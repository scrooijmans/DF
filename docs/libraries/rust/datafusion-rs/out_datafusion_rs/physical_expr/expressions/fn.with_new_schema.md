# Function with_new_schema Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/expressions/column.rs.html#177-180" class="src">Source</a>

``` rust
pub fn with_new_schema(
    expr: Arc<dyn PhysicalExpr>,
    schema: &Arc<Schema>,
) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>
```

Expand description

Rewrites an expression according to new schema; i.e. changes the columns it refers to with the column at corresponding index in the new schema. Returns an error if the given schema has fewer columns than the original schema. Note that the resulting expression may not be valid if data types in the new schema is incompatible with expression nodes.
