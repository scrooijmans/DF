# Function update_expr Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/projection.rs.html#723-727" class="src">Source</a>

``` rust
pub fn update_expr(
    expr: &Arc<dyn PhysicalExpr>,
    projected_exprs: &[ProjectionExpr],
    sync_with_child: bool,
) -> Result<Option<Arc<dyn PhysicalExpr>>, DataFusionError>
```

Expand description

The function operates in two modes:

1.  When `sync_with_child` is `true`:

    The function updates the indices of `expr` if the expression resides in the input plan. For instance, given the expressions `a@1 + b@2` and `c@0` with the input schema `c@2, a@0, b@1`, the expressions are updated to `a@0 + b@1` and `c@2`.

2.  When `sync_with_child` is `false`:

    The function determines how the expression would be updated if a projection was placed before the plan associated with the expression. If the expression cannot be rewritten after the projection, it returns `None`. For example, given the expressions `c@0`, `a@1` and `b@2`, and the [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec") with an output schema of `a, c_new`, then `c@0` becomes `c_new@1`, `a@1` becomes `a@0`, but `b@2` results in `None` since the projection does not include `b`.
