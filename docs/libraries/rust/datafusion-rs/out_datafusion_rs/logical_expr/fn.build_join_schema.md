# Function build_join_schema Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#1575-1579" class="src">Source</a>

``` rust
pub fn build_join_schema(
    left: &DFSchema,
    right: &DFSchema,
    join_type: &JoinType,
) -> Result<DFSchema, DataFusionError>
```

Expand description

Creates a schema for a join operation. The fields from the left side are first
