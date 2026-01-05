# Function aggregate_functional_dependencies Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/functional_dependencies.rs.html#418-422" class="src">Source</a>

``` rust
pub fn aggregate_functional_dependencies(
    aggr_input_schema: &DFSchema,
    group_by_expr_names: &[String],
    aggr_schema: &DFSchema,
) -> FunctionalDependencies
```

Expand description

Calculates functional dependencies for aggregate output, when there is a GROUP BY expression.
