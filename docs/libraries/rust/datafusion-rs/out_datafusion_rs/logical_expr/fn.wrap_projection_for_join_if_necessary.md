# Function wrap_projection_for_join_if_necessary Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#1979-1982" class="src">Source</a>

``` rust
pub fn wrap_projection_for_join_if_necessary(
    join_keys: &[Expr],
    input: LogicalPlan,
) -> Result<(LogicalPlan, Vec<Column>, bool), DataFusionError>
```

Expand description

Wrap projection for a plan, if the join keys contains normal expression.
