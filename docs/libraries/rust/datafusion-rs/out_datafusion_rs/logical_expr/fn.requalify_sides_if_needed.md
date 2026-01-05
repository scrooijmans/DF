# Function requalify_sides_if_needed Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#1685-1688" class="src">Source</a>

``` rust
pub fn requalify_sides_if_needed(
    left: LogicalPlanBuilder,
    right: LogicalPlanBuilder,
) -> Result<(LogicalPlanBuilder, LogicalPlanBuilder, bool), DataFusionError>
```

Expand description

(Re)qualify the sides of a join if needed, i.e. if the columns from one side would otherwise conflict with the columns from the other. This is especially useful for queries that come as Substrait, since Substrait doesn’t currently allow specifying aliases, neither for columns nor for tables. DataFusion requires columns to be uniquely identifiable, in some places (see e.g. DFSchema::check_names). The function returns:

- The requalified or original left logical plan
- The requalified or original right logical plan
- If a requalification was needed or not
