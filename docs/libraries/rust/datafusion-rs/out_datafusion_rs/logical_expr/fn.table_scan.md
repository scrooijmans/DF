# Function table_scan Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#1912-1916" class="src">Source</a>

``` rust
pub fn table_scan(
    name: Option<impl Into<TableReference>>,
    table_schema: &Schema,
    projection: Option<Vec<usize>>,
) -> Result<LogicalPlanBuilder, DataFusionError>
```

Expand description

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema. This is mostly used for testing and documentation.
