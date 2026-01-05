# Function create_physical_sort_expr Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#167-171" class="src">Source</a>

``` rust
pub fn create_physical_sort_expr(
    e: &Sort,
    input_dfschema: &DFSchema,
    execution_props: &ExecutionProps,
) -> Result<PhysicalSortExpr, DataFusionError>
```

Expand description

Create a physical sort expression from a logical expression
