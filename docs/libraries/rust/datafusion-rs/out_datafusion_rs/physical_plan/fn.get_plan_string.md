# Function get_plan_string Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#1335" class="src">Source</a>

``` rust
pub fn get_plan_string(plan: &Arc<dyn ExecutionPlan>) -> Vec<String>
```

Expand description

Utility function yielding a string representation of the given [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan").
