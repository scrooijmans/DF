# Function displayable Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#1140" class="src">Source</a>

``` rust
pub fn displayable(plan: &dyn ExecutionPlan) -> DisplayableExecutionPlan<'_>
```

Expand description

Return a [`DisplayableExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html "struct datafusion::physical_plan::display::DisplayableExecutionPlan") wrapper around an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") which can be displayed in various easier to understand ways.

See examples on [`DisplayableExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html "struct datafusion::physical_plan::display::DisplayableExecutionPlan")
