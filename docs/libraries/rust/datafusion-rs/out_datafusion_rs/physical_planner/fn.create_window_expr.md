# Function create_window_expr Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#1696-1707" class="src">Source</a>

``` rust
pub fn create_window_expr(
    e: &Expr,
    logical_schema: &DFSchema,
    execution_props: &ExecutionProps,
) -> Result<Arc<dyn WindowExpr>>
```

Expand description

Create a window expression from a logical expression or an alias
