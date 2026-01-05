# Function remove_unnecessary_projections Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/projection.rs.html#623-625" class="src">Source</a>

``` rust
pub fn remove_unnecessary_projections(
    plan: Arc<dyn ExecutionPlan>,
) -> Result<Transformed<Arc<dyn ExecutionPlan>>, DataFusionError>
```

Expand description

This function checks if `plan` is a [`ProjectionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html "struct datafusion::physical_plan::projection::ProjectionExec"), and inspects its input(s) to test whether it can push `plan` under its input(s). This function will operate on the entire tree and may ultimately remove `plan` entirely by leveraging source providers with built-in projection capabilities.
