# Function create_physical_exprs Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/planner.rs.html#394-400" class="src">Source</a>

``` rust
pub fn create_physical_exprs<'a, I>(
    exprs: I,
    input_dfschema: &DFSchema,
    execution_props: &ExecutionProps,
) -> Result<Vec<Arc<dyn PhysicalExpr>>, DataFusionError>where
    I: IntoIterator<Item = &'a Expr>,
```

Expand description

Create vector of Physical Expression from a vector of logical expression
