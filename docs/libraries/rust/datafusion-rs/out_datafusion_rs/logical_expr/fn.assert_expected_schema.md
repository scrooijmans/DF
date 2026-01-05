# Function assert_expected_schema Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/invariants.rs.html#114" class="src">Source</a>

``` rust
pub fn assert_expected_schema(
    schema: &Arc<DFSchema>,
    plan: &LogicalPlan,
) -> Result<(), DataFusionError>
```

Expand description

Returns an error if the plan does not have the expected schema. Ignores metadata and nullability.
