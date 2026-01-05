# Function with_new_children_if_necessary Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#1116-1119" class="src">Source</a>

``` rust
pub fn with_new_children_if_necessary(
    plan: Arc<dyn ExecutionPlan>,
    children: Vec<Arc<dyn ExecutionPlan>>,
) -> Result<Arc<dyn ExecutionPlan>, DataFusionError>
```

Expand description

Returns a copy of this plan if we change any child according to the pointer comparison. The size of `children` must be equal to the size of `ExecutionPlan::children()`.
