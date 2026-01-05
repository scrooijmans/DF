# Function create_aggregate_expr_with_name_and_maybe_filter Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#1718-1783" class="src">Source</a>

``` rust
pub fn create_aggregate_expr_with_name_and_maybe_filter(
    e: &Expr,
    name: Option<String>,
    human_displan: String,
    logical_input_schema: &DFSchema,
    physical_input_schema: &Schema,
    execution_props: &ExecutionProps,
) -> Result<(Arc<AggregateFunctionExpr>, Option<Arc<dyn PhysicalExpr>>, Vec<PhysicalSortExpr>)>
```

Expand description

Create an aggregate expression with a name from a logical expression
