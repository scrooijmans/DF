# Function create_window_expr_with_name Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#1633-1693" class="src">Source</a>

``` rust
pub fn create_window_expr_with_name(
    e: &Expr,
    name: impl Into<String>,
    logical_schema: &DFSchema,
    execution_props: &ExecutionProps,
) -> Result<Arc<dyn WindowExpr>>
```

Expand description

Create a window expression with a name from a logical expression
