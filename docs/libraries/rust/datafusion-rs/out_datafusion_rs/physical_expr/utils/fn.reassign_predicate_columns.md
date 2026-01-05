# Function reassign_predicate_columns Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/utils/mod.rs.html#243-247" class="src">Source</a>

``` rust
pub fn reassign_predicate_columns(
    pred: Arc<dyn PhysicalExpr>,
    schema: &Schema,
    ignore_not_found: bool,
) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>
```

Expand description

Re-assign column indices referenced in predicate according to given schema. This may be helpful when dealing with projections.
