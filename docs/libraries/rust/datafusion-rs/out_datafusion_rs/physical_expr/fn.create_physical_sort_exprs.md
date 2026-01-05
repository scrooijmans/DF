# Function create_physical_sort_exprs Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#179-183" class="src">Source</a>

``` rust
pub fn create_physical_sort_exprs(
    exprs: &[Sort],
    input_dfschema: &DFSchema,
    execution_props: &ExecutionProps,
) -> Result<Vec<PhysicalSortExpr>, DataFusionError>
```

Expand description

Create vector of physical sort expression from a vector of logical expression
