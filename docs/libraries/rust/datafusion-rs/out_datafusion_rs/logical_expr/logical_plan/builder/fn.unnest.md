# Function unnest Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#2086" class="src">Source</a>

``` rust
pub fn unnest(
    input: LogicalPlan,
    columns: Vec<Column>,
) -> Result<LogicalPlan, DataFusionError>
```

Expand description

Create a [`LogicalPlan::Unnest`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#variant.Unnest "variant datafusion::logical_expr::LogicalPlan::Unnest") plan
